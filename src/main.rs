#![allow(unused_must_use)]

use crate::modules::cli::analyze::timer_exec::get_time_exec;
use crate::modules::cli::app::start_skyull;
use dotenv::dotenv;
mod modules;

fn main() {
    dotenv().ok();
    get_time_exec(start_skyull);
}
