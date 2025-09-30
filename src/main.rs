use anyhow::Result;
use aptos_indexer_processor_sdk::server_framework::ServerArgs;
use clap::Parser;
use crate::config::indexer_processor_config::IndexerProcessorConfig;

mod setup;
mod config;
mod tuma_stream;
mod tuma_schema;

#[cfg(unix)]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() -> Result<()> {
    setup::load_config_file();
    let num_cpus = num_cpus::get();
    let worker_threads = (num_cpus).max(16);

    let mut builder = tokio::runtime::Builder::new_multi_thread();

    builder
        .enable_all()
        .worker_threads(worker_threads)
        .build()?
        .block_on(async {
            let args = ServerArgs::parse();
            args.run::<IndexerProcessorConfig>(tokio::runtime::Handle::current()).await
        })
}