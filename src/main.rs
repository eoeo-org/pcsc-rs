#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenvy::dotenv;
use std::{env, path::Path, process};
use sysinfo::IS_SUPPORTED_SYSTEM;

fn main() {
    let rs = Path::new(".env").exists();
    if rs {
        dotenv().expect(".env file not found");
    }

    if !IS_SUPPORTED_SYSTEM {
        println!("This OS isn't supported (yet?).");
        process::exit(95);
    }

    if !env::var("PASS").is_ok() {
        println!("The environment variable Password (PASS) is not specified.");
        process::exit(95);
    }

    pcsc_rs::start();
}
