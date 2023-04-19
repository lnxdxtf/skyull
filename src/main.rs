#![allow(unused_must_use)]
use modules::cli::app::start_skyull;

use crate::modules::cli::analyze::timer_exec::get_time_exec;

mod modules;

fn main() {
    get_time_exec(start_skyull);
}
