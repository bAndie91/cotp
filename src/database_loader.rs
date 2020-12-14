use std::fs::File;
use std::io::Read;
use serde_json;
use serde::{Deserialize, Serialize};
use super::utils;
use utils::get_db_path;


#[derive(Serialize, Deserialize)]
pub struct OTPElement {
    secret: String,
    label: String,
    algorithm: String,
    issuer: String,
    period: u64
}

impl OTPElement {
    pub fn new(secret: String, label: String, algorithm: String,issuer: String, period: u64) -> OTPElement {
        OTPElement {
            secret: secret,
            algorithm: algorithm,
            label: label,
            issuer: issuer,
            period: period,
        }
    }
    pub fn secret(&self) -> String {
        self.secret.to_string().replace("=", "")
    }
    pub fn label(&self) -> String{
        self.label.to_string()
    }
    pub fn algorithm(&self) -> String{
        self.algorithm.to_string()
    }
    pub fn issuer(&self) -> String{
        self.issuer.to_string()
    }
    pub fn period(&self) -> u64{
        self.period
    }
}

pub fn read_from_file() -> Vec<OTPElement>{
    let mut file = File::open(&get_db_path()).expect("File not found!");
    //rust close files at the end of the function
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let vector: Vec<OTPElement> = serde_json::from_str(&contents).unwrap();
    vector
}

pub fn remove_element_from_db(mut id: usize) -> bool{
    if id == 0{
        return false;
    }
    //user inserts numbers starting from 1, so we will decrement the value becouse we use array indexes instead
    id = id - 1;

    let mut elements: Vec<OTPElement> = read_from_file();

    if id >= elements.len(){
        return false;
    }

    for i in 0..elements.len(){
        if i == id {
            elements.remove(i);
        }
    }
    overwrite_database(elements);
    true
}


pub fn overwrite_database(elements: Vec<OTPElement>){
    let json_string: &str = &serde_json::to_string(&elements).unwrap();
    utils::write_to_file(json_string, &utils::get_db_path())
}

