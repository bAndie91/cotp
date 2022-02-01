use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use dirs::home_dir;

use crate::otp::otp_element::OTPElement;

#[cfg(debug_assertions)]
pub fn get_db_path() -> PathBuf {
    PathBuf::from("db.cotp")
}

#[cfg(not(debug_assertions))]
pub fn get_db_path() -> PathBuf {
    get_cotp_folder().join("db.cotp")
}

pub fn get_home_folder() -> PathBuf {
    match home_dir() {
        Some(home) => home,
        None => {
            let current_dir = PathBuf::from(".");
            if let Some(str_dir) = current_dir.to_str(){
                eprintln!("Cannot get home folder, using: {}",str_dir);
            }
            else{
                eprintln!("Cannot get home folder, using");
            }
            current_dir
        },
    }
}

#[cfg(debug_assertions)]
fn get_cotp_folder() -> PathBuf {
    PathBuf::from(".")
}

// Pushing an absolute path to a PathBuf replaces the entire PathBuf: https://doc.rust-lang.org/std/path/struct.PathBuf.html#method.push
#[cfg(not(debug_assertions))]
fn get_cotp_folder() -> PathBuf {
    get_home_folder().join(".cotp")
}

pub fn create_db_if_needed() -> Result<bool, ()> {
    let cotp_folder = get_cotp_folder();
    if !cotp_folder.exists() {
        match std::fs::create_dir(cotp_folder) {
            Ok(()) => {}
            Err(_e) => {}
        }
    }
    let db_path = get_db_path();
    if !db_path.exists() {
        return match std::fs::File::create(db_path) {
            Ok(_f) => Ok(true),
            Err(_e) => Err(()),
        }
    }
    Ok(false)
}

pub fn delete_db() -> std::io::Result<()> {
    std::fs::remove_file(get_db_path())
}

pub fn write_to_file(content: &str, file: &mut File) -> Result<(), std::io::Error> {
    file.write_all(content.as_bytes())?;
    file.sync_all()
}

pub fn check_elements(id: usize, elements: &Vec<OTPElement>) -> Result<(), String> {
    if elements.len() == 0 {
        return Err(String::from("there are no elements in your database. Type \"cotp -h\" to get help."));
    }

    if id >= elements.len() {
        return Err(format!("{} is a bad index", id + 1));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::create_db_if_needed;

    #[test]
    fn test_db_creation() {
        assert_eq!(Ok(true), create_db_if_needed());
    }
}