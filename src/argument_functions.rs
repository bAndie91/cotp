use std::path::PathBuf;

use crate::{cryptography, database_management};
use crate::cryptography::prompt_for_passwords;
use crate::importers;
use crate::otp::otp_helper;
use clap::ArgMatches;
use zeroize::Zeroize;

pub fn import(matches: &ArgMatches) {
    let path = matches.value_of("path").unwrap();

    let result = if matches.is_present("cotp") ||
        matches.is_present("andotp") {
        importers::and_otp::import(path)
    }
    else if matches.is_present("aegis") {
        importers::aegis::import(path)
    }
    else if matches.is_present("freeotp-plus") {
        importers::freeotp_plus::import(path)
    }
    else if matches.is_present("google-authenticator") ||
        matches.is_present("authy") ||
        matches.is_present("microsoft-authenticator") ||
        matches.is_present("freeotp") {
        importers::converted::import(path)
    }
    else {
        eprintln!("Invalid arguments provided");
        return;
    };

    let elements = match result {
        Ok(result) => result,
        Err(e) => {
            eprintln!("An error occurred: {}", e);
            return;
        }
    };

    let mut pw = cryptography::prompt_for_database_password("Choose a password: ", 0, true);
    match database_management::overwrite_database(elements, &pw) {
        Ok(()) => {
            println!("Successfully imported database");
        }
        Err(e) => {
            eprintln!("An error occurred during database overwriting: {}", e);
        }
    }
    pw.zeroize();
}

pub fn add(matches: &ArgMatches) {   
    let mut secret = prompt_for_passwords("Insert the secret: ", 0, false);
    match database_management::add_element(
        secret.as_str(),
        // Safe to unwrap due to default values
        matches.value_of("issuer").unwrap(),
        matches.value_of("label").unwrap(),
        matches.value_of("algorithm").unwrap(), 
        matches.value_of_t("digits").unwrap_or(6),
        matches.value_of_t("counter").unwrap_or_default(),
        matches.is_present("hotp"),
        ) {
        Ok(()) => println!("Success"),
        Err(e) => eprintln!("An error occurred: {}", e)
    }
    secret.zeroize();
}

pub fn remove(matches: &ArgMatches) {
    match database_management::remove_element_from_db(
        matches.values_of("index").unwrap()
        .map(|s| s.parse::<usize>().unwrap())
        .collect())
     {
        Ok(()) => println!("Success"),
        Err(e) => eprintln!("An error has occurred: {}", e)
    }
}

pub fn edit(matches: &ArgMatches) {
    let mut secret = match matches.is_present("change-secret") {
        true => prompt_for_passwords("Insert the secret (type ENTER to skip): ", 0, false),
        false => String::from(""),
    };
    match database_management::edit_element(
        matches.value_of_t_or_exit("index"), 
        secret.as_str(), 
        matches.value_of("issuer").unwrap_or(""), 
        matches.value_of("label").unwrap_or(""), 
        matches.value_of("algorithm").unwrap_or(""), 
        matches.value_of_t("digits").unwrap_or(0), 
        matches.value_of_t("counter").unwrap_or(0)
    ) {
        Ok(()) => println!("Success"),
        Err(e) => eprintln!("An error occurred: {}", e)
    }
    secret.zeroize();
}

pub fn export(matches: &ArgMatches) {
    match database_management::export_database(PathBuf::from(matches.value_of("path").unwrap())) {
        Ok(export_result) => {
            println!("Database was successfully exported at {}", export_result.to_str().unwrap_or("**Invalid path**"));
        }
        Err(e) => {
            eprintln!("An error occurred while exporting database: {}", e);
        }
    }
}

pub fn info(matches: &ArgMatches) {
    match otp_helper::print_element_info(matches.value_of_t_or_exit("index")) {
        Ok(()) => {}
        Err(e) => eprintln!("An error occurred: {}", e),
    }
}

pub fn change_password() {
    let mut old_password = cryptography::prompt_for_database_password("Old password: ", 0, false);
    let decrypted_text = database_management::read_decrypted_text(&old_password);
    old_password.zeroize();
    match decrypted_text {
        Ok(mut s) => {
            let mut new_password = cryptography::prompt_for_passwords("New password: ", 0, true);
            match database_management::overwrite_database_json(&s, &new_password) {
                Ok(()) => println!("Password changed"),
                Err(e) => eprintln!("An error has occurred: {}", e),
            }
            s.zeroize();
            new_password.zeroize();
        }
        Err(e) => {
            eprintln!("An error has occurred: {}", e);
        }
    }
} 