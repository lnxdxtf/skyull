#![allow(unused_must_use)]

use crate::modules::cli::analyze::timer_exec::get_time_exec;
use crate::modules::cli::app::start_skyull;
mod modules;

fn main() {
    get_time_exec(start_skyull);
}
