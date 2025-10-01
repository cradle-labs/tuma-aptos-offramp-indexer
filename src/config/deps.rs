

use std::env;
use diesel::{r2d2, PgConnection};
use diesel::r2d2::ConnectionManager;
use anyhow::{anyhow,Result};
use dotenvy::dotenv;
use tuma::controller::aptos_panora_provider::AptosPanoraProvider;
use tuma::kvstore::KVStoreManager;
use tuma::payment_provider::sender::FiatSender;
use tuma::payment_provider::tuma_request_handler::TumaRequestHandler;
use tuma::payments::PaymentSessions;
use tuma::pretium::PretiumService;

#[derive(Debug, Clone)]
pub struct Config {
    pub pool: r2d2::Pool<ConnectionManager<PgConnection>>,
    pub pretium: PretiumService,
    pub panora: AptosPanoraProvider,
    pub fiat_sender: FiatSender,
    pub handler: TumaRequestHandler,
    pub kvStore: KVStoreManager
}

impl Config {
    pub fn new()->Result<Self>{
        dotenv().ok();

        let database_url = match env::var("DATABASE_URL"){Ok(v)=>v, Err(_)=>return Err(anyhow!("db_url_not_found"))};
        let pretium_api_key = match env::var("PRETIUM_API_KEY") {Ok(v)=>v, Err(_)=>return Err(anyhow!("pretium_api_key not found"))};

        let manager = ConnectionManager::<PgConnection>::new(database_url);

        let pool = r2d2::Pool::builder()
            .build(manager)
            .map_err(|_|anyhow!("Failed to create primary database pool"))?;

        let pretium = PretiumService::new(pretium_api_key)?;
        let panora = AptosPanoraProvider::new();

        let fiat_sender = FiatSender::new(pretium.clone());
        let handler = TumaRequestHandler::new(pool.clone(),fiat_sender.clone());
        let kv_store = KVStoreManager::new(pool.clone());

        Ok(Self {
            pool,
            pretium,
            panora,
            fiat_sender,
            handler,
            kvStore: kv_store
        })
    }
}