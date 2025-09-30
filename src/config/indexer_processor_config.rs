use anyhow::{Result};
use aptos_indexer_processor_sdk::aptos_indexer_transaction_stream::TransactionStreamConfig;
use aptos_indexer_processor_sdk::server_framework::RunnableConfig;
use serde::{Deserialize, Serialize};
use crate::config::deps::Config;
use crate::config::processor_config::ProcessorConfig;
use crate::tuma_stream::processor::TumaStreamProcessor;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IndexerProcessorConfig {
    pub processor_config: ProcessorConfig,
    pub transaction_stream_config: TransactionStreamConfig,

}

#[async_trait::async_trait]
impl RunnableConfig for IndexerProcessorConfig {
    async fn run(&self) ->  Result<()> {
        let config = Config::new()?;
        match self.processor_config {
            ProcessorConfig::TumaProcessor => {
                let mut processor = TumaStreamProcessor::new(self.clone(), config).await?;
                processor.run_processor().await
            }
        }
    }

    fn get_server_name(&self) -> String {
        let before_underscore = self.processor_config.name().split("-").next().unwrap_or("unknown");
        before_underscore[..before_underscore.len().min(12)].to_string()
    }
}