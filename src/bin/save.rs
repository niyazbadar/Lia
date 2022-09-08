#![no_main]

use std::{
    fs,
    fs::OpenOptions, 
    io::{
        Write,
        Read
    }, 
    fmt::Debug,
    collections::HashMap, 
};

extern crate serde_json;

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Debug , Deserialize)]
pub struct Data {
    pub index: HashMap<String, HashMap<String, String>>
}

impl Data {
    pub async fn new() -> Self {
        let data = Data {
            index: HashMap::new()
        };

        data
    }

    pub async fn get_class(&mut self, class: &str) -> HashMap<String, String>{
        match self.index.get(class) {
            Some(value) => value.clone(),
            None => {
                self.index.insert(String::from(class), HashMap::new());
                self.index.get(class).unwrap().clone()
            }
        }
    }

    pub async fn get_sub_class(&mut self, class: &str, sub_class: &str) -> String{
        match self.index.get_mut(class) {
            Some(i) => {
                match i.get(sub_class) {
                    Some(value) => value.clone(),
                    None => {
                        self.index.get_mut(class).unwrap().insert(String::from(sub_class), String::new());
                        String::new()
                    }
                }
            },
            None => {
                self.index.insert(String::from(class), HashMap::new());
                self.index.get_mut(class).unwrap().insert(String::from(sub_class), String::new());
                String::new()
            }
        }
    }
}

pub async fn save(path: String, name: String, data: &Data){
    let data = serde_json::to_string(&data).unwrap();

    fs::create_dir_all(&path).unwrap();

    let path = format!("{path}/{name}.json");

    let mut file 
        = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .unwrap();

    file.write_all(data.as_bytes()).unwrap();
}

pub async fn load(path: String, name: String) -> Data{
    fs::create_dir_all(&path).unwrap();

    let final_path = format!("{path}/{name}.json");

    let mut file = match OpenOptions::new().read(true).open(&final_path){
        Ok(file) => file,
        _ => {
            save(path, name, &Data::new().await).await;
            let file = OpenOptions::new().read(true).open(final_path).unwrap();
            file
        }
    };

    let mut data = String::new();
    
    file.read_to_string(&mut data).unwrap();

    let data: Data = serde_json::from_str(data.as_str()).unwrap();

    data
}