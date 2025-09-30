use serde::{Deserialize, Serialize};

#[derive(
    Deserialize,
    Serialize,
    strum::VariantNames,
    strum::IntoStaticStr,
    strum::Display,
    clap::ValueEnum,
    Clone,
    Debug
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[clap(rename_all = "snake_case")]
pub enum ProcessorConfig {
    TumaProcessor
}

impl ProcessorConfig {
    pub fn name(&self) -> &'static str {
        self.into()
    }
}

#[derive(strum::EnumDiscriminants)]
#[strum_discriminants(
    derive(strum::VariantNames),
    name(ProcessorDiscriminants),
    strum(serialize_all = "snake_case")
)]
#[cfg_attr(test, derive(Debug))]
pub enum Processor {
    TumaProcessor
}