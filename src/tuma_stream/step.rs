use std::str::FromStr;
use anyhow::anyhow;
use aptos_indexer_processor_sdk::{aptos_protos::transaction::v1::{transaction::TxnData, transaction_payload::Payload, Transaction}, traits::{AsyncRunType, AsyncStep, NamedStep, Processable}, types::transaction_context::TransactionContext, utils::errors::ProcessorError};
use aptos_indexer_processor_sdk::utils::errors::ProcessorError::ProcessError;
use bigdecimal::{BigDecimal, Pow};
use diesel::{r2d2::{self, ConnectionManager}, PgConnection};
use serde::{Deserialize, Serialize};
use tuma::payments::PaymentSessions;
use tuma::r#static::currency::CurrencyStaticData;
use uuid::Uuid;
use crate::config::deps::Config;
use crate::tuma_stream::offramp::{CreateOffRampRequest, OffRamp};

pub struct TumaTransactionStreamProcessor where Self: Sized + Send + 'static, {
    pub pool: r2d2::Pool<ConnectionManager<PgConnection>>,
    pub app_config: Config
}


#[async_trait::async_trait]
impl Processable for TumaTransactionStreamProcessor {
    type Input = Vec<Transaction>;

    type Output = Option<()>;

    type RunType = AsyncRunType;

    async fn process(
        &mut self,
        input: TransactionContext<Vec<Transaction>>
    )-> Result<Option<TransactionContext<Option<()>>>, ProcessorError> {

        let  mut off_ramp_registration = OffRamp::new(self.pool.clone());

        let mut payment_session_handler = PaymentSessions::new(self.pool.clone()).unwrap();

        let transactions = input.data;
        for transaction in transactions {
            // println!("New transaction :: {:?}", transaction.version);
            if let Some(tx_data) = &transaction.txn_data {
                if let TxnData::User(user_transaction) = &tx_data {
                    if let Some(request) = &user_transaction.request {
                        if let Some(request_payload) = &request.payload {
                            if let Some(payload) = &request_payload.payload {
                                let tx = transaction.clone();
                                if let Payload::EntryFunctionPayload(data) = payload {
                                    let currency_data = CurrencyStaticData::new();

                                    #[derive(Deserialize,Serialize)]
                                    struct Inner {
                                        pub inner: String
                                    }

                                    let args = data.arguments.clone();
                                    match data.entry_function_id_str.as_str() {
                                        "0xce349ffbde2e28c21a4a7de7c4e1b3d72f1fe079494c7f8f8832bd6c8502e559::tuma::deposit_fungible"=>{
                                            println!("ALl Args :: {:?}", args);

                                            let token_metadata = args[0].clone();
                                            let token = match serde_json::from_str::<Inner>(token_metadata.as_str()) {
                                                Ok(t) => t.inner,
                                                Err(e)=>{
                                                    println!("Unable to deserialize token {} metadata {}", e, token_metadata);
                                                    return continue;
                                                }
                                            };
                                            let token_amount_str = match serde_json::from_str::<String>(args[1].as_str()) {
                                                Ok(s) => s,
                                                Err(e) => {
                                                    println!("Unable to deserialize token amount string: {}", e);
                                                    continue;
                                                }
                                            };

                                            let token_amount = match token_amount_str.parse::<u64>() {
                                                Ok(v) => v,
                                                Err(e) => {
                                                    println!("Unable to parse token amount successfully: {}", e);
                                                    continue;
                                                }
                                            };

                                            let requester = request.sender.clone();
                                            let version = transaction.version.clone().to_string();

                                            match off_ramp_registration.create_off_ramp_request(CreateOffRampRequest {
                                                data: None,
                                                requester,
                                                transaction_code: None,
                                                transaction_version: version.clone(),
                                                from_token: token,
                                                from_token_amount: BigDecimal::from(token_amount),
                                                transaction_hash: version.clone()
                                            }).await {
                                                Ok(_)=>{
                                                    println!("Successfully recorded on-ramp {}", version.clone())
                                                },
                                                Err(e)=>{
                                                    println!("Error:: {}",e);
                                                    println!("Failed to off ramp successfully for transaction {}", version.clone());
                                                    continue;
                                                }
                                            }

                                        }
                                        "0xce349ffbde2e28c21a4a7de7c4e1b3d72f1fe079494c7f8f8832bd6c8502e559::tuma::make_payment_fungible"=>{
                                            println!("ALl Args :: {:?}", args);

                                            let token_metadata = args[0].clone();
                                            let token = match serde_json::from_str::<Inner>(token_metadata.as_str()) {
                                                Ok(t) => t.inner,
                                                Err(e)=>{
                                                    println!("Unable to deserialize token {} metadata {}", e, token_metadata);
                                                    return continue;
                                                }
                                            };
                                            let token_amount_str = match serde_json::from_str::<String>(args[1].as_str()) {
                                                Ok(s) => s,
                                                Err(e) => {
                                                    println!("Unable to deserialize token amount string: {}", e);
                                                    continue;
                                                }
                                            };

                                            let session_id = match serde_json::from_str::<String>(args[2].as_str()) {
                                                Ok(s)=>match Uuid::from_str(s.as_str()) {Ok(v)=>v, Err(e)=>{
                                                    println!("Unable to parse session id");
                                                    continue
                                                }},
                                                Err(e)=> {
                                                    println!("Unable to deserialize session as string: {}", e);
                                                    continue;
                                                }
                                            };

                                            let token_amount = match token_amount_str.parse::<u64>() {
                                                Ok(v) => v,
                                                Err(e) => {
                                                    println!("Unable to parse token amount successfully: {}", e);
                                                    continue;
                                                }
                                            };

                                            let token_currency_decimals = match currency_data.get_currency_by_token(token.clone()) {
                                                Some(c)=> match c.decimals { Some(v)=>v, None=>continue},
                                                None=>continue
                                            };

                                            let normalized_token_a = (token_amount as f64) / (10_u64.pow(token_currency_decimals as u32) as f64);

                                            let requester = request.sender.clone();
                                            let version = transaction.version.clone().to_string();

                                            match payment_session_handler.off_ramp_payment_session(session_id.to_string(), normalized_token_a, token, version.clone()).await {
                                                Ok(v)=>{
                                                    println!("Successfully recorded payment {}",v)
                                                },
                                                Err(e)=>{
                                                    println!("Something went wrong :: {e}");
                                                    continue;
                                                }
                                            }
                                        }
                                        _=>{

                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }


        Ok(Some(
            TransactionContext { data: None, metadata: input.metadata }
        ))
    }
}

impl AsyncStep for TumaTransactionStreamProcessor {}

impl NamedStep for TumaTransactionStreamProcessor {
    fn name(&self) -> String {
        "TumaTransactionStreamProcessor".to_string()
    }
}