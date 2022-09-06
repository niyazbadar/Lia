#![no_main]

use std::{
    fs,
    fs::OpenOptions, 
    io::{
        Write,
        Read
    }, fmt::Debug
};

extern crate serde_json;

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Debug , Deserialize)]
pub struct Profile{
    pub money: u32, 
}

pub async fn save(path: String, name: String, money: u32){
    let profile = Profile {
        money: money
    };

    let profile = serde_json::to_string(&profile).unwrap();

    fs::create_dir_all(&path).unwrap();

    let path = format!("{path}/{name}.json");

    let mut file 
        = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .unwrap();

    file.write_all(profile.as_bytes()).unwrap();
}

pub async fn load(path: String, name: String) -> Profile{
    fs::create_dir_all(&path).unwrap();

    let final_path = format!("{path}/{name}.json");

    let mut file = match OpenOptions::new().read(true).open(&final_path){
        Ok(file) => file,
        _ => {
            save(path, name, 0).await;
            let file = OpenOptions::new().read(true).open(final_path).unwrap();
            file
        }
    };

    let mut profile = String::new();
    
    file.read_to_string(&mut profile).unwrap();

    let profile: Profile = serde_json::from_str(profile.as_str()).unwrap();

    profile
}