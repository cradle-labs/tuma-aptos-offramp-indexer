use diesel::prelude::*;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use anyhow::{Result, anyhow};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use crate::tuma_schema::off_ramp_requests as OffRampRequestsTable;

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[diesel(table_name = OffRampRequestsTable)]
pub struct CreateOffRampRequest {
    pub requester: String,
    pub from_token: String,
    pub from_token_amount: BigDecimal,
    pub transaction_version: String,
    pub transaction_hash: String,
    pub transaction_code: Option<String>,
    pub data: Option<Value>
}

pub struct OffRamp {
    pool: r2d2::Pool<ConnectionManager<PgConnection>>
}


impl OffRamp {

    pub fn new(pool: r2d2::Pool<ConnectionManager<PgConnection>>)->Self{
        Self {
            pool
        }
    }


    pub async fn create_off_ramp_request(&mut self, req: CreateOffRampRequest)->Result<()> {
        use crate::tuma_schema::off_ramp_requests::dsl::*;

        let mut conn = self.pool.get()?;

        let v = diesel::insert_into(OffRampRequestsTable::table).values(&req).returning(id).get_result::<(Uuid)>(&mut conn)?;

        println!("Successfully created off_ramp request {}", v);

        Ok(())
    }

}