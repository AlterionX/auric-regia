#![feature(never_type, option_into_flat_iter)]

mod schema;
mod db;

mod cmd;

#[tokio::main]
async fn main() {
    azel::easy_setup_and_run(cmd::generate_command_descriptions()).await;
}
