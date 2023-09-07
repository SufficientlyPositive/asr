// REMOVE THIS WHEN SOMETHING CLOSE TO DECENT IS MADE
#![ allow( dead_code, unused_imports, unused_variables ) ]

// logging, use trace!(), debug!(), info!(), warn!() or error!() macros
extern crate pretty_env_logger;
#[macro_use] extern crate log;

mod cli;
mod bounded_float;
mod audio;
mod parsing;
mod operation;
mod multithreading;
mod models;

fn main() {
    pretty_env_logger::init_timed();
    cli::cli_start();
}