#![feature(never_type, option_into_flat_iter, iter_intersperse)]

use std::future;

mod schema;
mod db;

mod cmd;

#[tokio::main]
async fn main() {
    let cfg = azel::setup_default_log_and_load_configuration().expect("configuration complete");
    let mut client = azel::build_client(
        cfg,
        |_| future::ready(vec![]),
        |_guild_id, _c| {
            future::ready(cmd::generate_command_descriptions())
        },
        |b| b,
    ).await.expect("build complete");
    client.0.start().await.expect("launch complete");
}
