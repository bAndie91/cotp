
use sodiumoxide;

use otp::otp_helper;
use zeroize::Zeroize;

use serde_json;
use serde::{Deserialize, Serialize};
use crate::otp::otp_helper::get_otp_code;

mod utils;
mod argument_functions;
mod cryptography;
mod importers;
mod otp;
mod database_management;
mod args;

fn init() -> Result<bool, String> {
    match sodiumoxide::init() {
        Err(()) => {
            return Err(String::from("Error during sodiumoxide initialization"));
        }
        _ => {}
    };
    match utils::create_db_if_needed() {
        Ok(value) => {
            if value {
                let mut pw = cryptography::prompt_for_database_password("Choose a password: ", 0, true);
                let result = match database_management::overwrite_database_json("[]", &pw) {
                    Ok(()) => Ok(true),
                    Err(_e) => Err(String::from("An error occurred during database overwriting")),
                };
                pw.zeroize();
                return result;
            }
            Ok(false)
        }
        Err(()) => {
            return Err(String::from("An error occurred during database creation"));
        }
    }
}

fn main() {
    match init() {
        Ok(true) => {
            println!("Database correctly initialized");
        }
        Ok(false) => {}
        Err(e) => {
            println!("{}", e);
            std::process::exit(-1);
        }
    }
    match args::args_parser() {
        // no args, show dashboard
        true => match dashboard(){
            true => std::process::exit(0),
            false => std::process::exit(-2), 
        },
        // args parsed, can exit
        false => std::process::exit(0),
    }
}

#[derive(Serialize, Deserialize)]
struct JsonResult{
    index: usize,
    issuer: String,
    label: String,
    otp_code: String,
}

impl JsonResult {
    pub fn new(index: usize, issuer: String, label: String, otp_code: String) -> JsonResult {
        JsonResult{
            index: index, 
            issuer: issuer,
            label: label,
            otp_code: otp_code
        }
    }
}

fn dashboard() -> bool {
    match otp_helper::read_codes() {
        Ok(elements) => {
            if elements.len() == 0 {
                println!("No codes, type \"cotp -h\" to get help");
                return false;
            } else {
			    let mut results: Vec<JsonResult> = Vec::new();
				
			    for i in 0..elements.len() {
			        let otp_code = get_otp_code(&elements[i]).unwrap();
			        results.push(JsonResult::new(i+1, elements[i].issuer(), elements[i].label(), otp_code))
			    }
				
			    let json_string: &str = &serde_json::to_string_pretty(&results).unwrap();
			    println!("{}", json_string);
            }
        }
        Err(e) => {
            eprintln!("An error occurred: {}", e);
            return false;
        }
    }
    return true;
}
