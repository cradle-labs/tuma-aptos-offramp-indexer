use std::env;

use anyhow::Result;
use aptos_indexer_processor_sdk::{ aptos_indexer_transaction_stream::TransactionStreamConfig, builder::ProcessorBuilder, common_steps::TransactionStreamStep, traits::IntoRunnableStep};
use dotenvy::dotenv;
use tracing::info;

use crate::config::deps::Config;
use crate::config::indexer_processor_config::IndexerProcessorConfig;
use crate::tuma_stream::step::TumaTransactionStreamProcessor;

pub struct TumaStreamProcessor {
    pub config: IndexerProcessorConfig,
    pub app_config: Config
}

impl TumaStreamProcessor {
    pub async fn new(config: IndexerProcessorConfig, app_config: Config) -> Result<Self> {
        Ok(Self {config, app_config })
    }

    pub async fn run_processor(&mut self) -> Result<()> {

        let _ = dotenv().ok();

        let stream_config = self.config.transaction_stream_config.clone();
        let env_transaction_value = env::var("STARTING_VERSION")?;
        let latest_transaction_version_string = {
            // env_transaction_value
          match self.app_config.kvStore.get("latest_transaction_version".to_string()).await {
              Ok(res)=>{
                  match res {
                      Some(v)=>{
                          v
                      },
                      None=> env_transaction_value
                  }
              },
              Err(e)=>{
                  println!("Unable to check db {e}");
                  env_transaction_value
              }
          }
        };

        let latest_transaction_version = latest_transaction_version_string.parse::<u64>()?;

        let stream = TransactionStreamStep::new(TransactionStreamConfig {
            starting_version: Some(latest_transaction_version),
            ..stream_config
        }).await?;


        let processor_step = TumaTransactionStreamProcessor { pool: self.app_config.pool.clone(), app_config: self.app_config.clone() };

        let (_, buffer_receiver) = ProcessorBuilder::new_with_inputless_first_step(
            stream.into_runnable_step()
        )
            .connect_to(processor_step.into_runnable_step(), 10)
            .end_and_return_output_receiver(10);


        loop {
            match  buffer_receiver.recv().await {
                Ok(txn_context) => {

                    match self.app_config.kvStore.set("latest_transaction_version".parse()?, format!("{:?}", txn_context.metadata.end_version)).await {
                        Ok(_)=>{
                            info!(
                                "Finished processing events from versions [{:?}, {:?}]",
                                txn_context.metadata.start_version, txn_context.metadata.end_version,
                            );
                        },
                        Err(err)=>{
                            println!("Failed to update latest transaction version {:?}", err);
                            // panic!("Failed to update transaction version")
                        }
                    }
                },
                Err(_)=> {
                    info!("Channel is closed");
                    return Ok(())
                }
            }
        }
    }
}