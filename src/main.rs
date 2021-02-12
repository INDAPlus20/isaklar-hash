mod hash_map;
use std::{fmt::Error, fs::File};

use hash_map::HashMap;

const FILE_NAME: &str = "table.json";

fn main() {
    let qurey_type = std::env::args().nth(1).expect("<command> argument not found");
    match qurey_type.as_str() {
        "get"       => get(),
        "insert"    => insert(),    
        "remove"    => remove(),    
        "all"       => all(),
        "help"      => help(),
        _           => println!("Invalid command. Type \"help\" for help")
    }
}

fn get(){
    
    if let Some(key) = std::env::args().nth(2){
        let map = load_table();
        if let Some(value) = map.get(&key) {
            println!("{:?}, {:?}", key, value);
        } else {
            println!("Entry not found");
        }
    } else {
        println!("<key> argument not found");
    }
    
}

fn insert(){
    if let Some(key) = std::env::args().nth(2){
        if let Some(value) = std::env::args().nth(3){
            let mut map = load_table();
            match map.insert(&key, &value) {
                Ok(_) => save_table(map),
                Err(e) => println!("{:?}",e)
            }
            
        } else {
            println!("<value> argument not found");
        }
    } else {
        println!("<key> argument not found");
    }
}

fn remove(){
    if let Some(key) = std::env::args().nth(2){
        let mut map = load_table();
        map.remove(&key);
        save_table(map);
    } else {
        println!("<key> argument not found");
    }
}

fn all(){
    let map = load_table();
    map.print_all();
}

fn help() {

    println!("Welcome to Isak's very useful database! The available commands are: ");
    println!("\tget \t<key>");
    println!("\tinsert \t<key> <value>");
    println!("\tremove \t<key>");
    println!("\tall ");
    println!("\thelp");
}

fn load_table() -> HashMap{
    let file: String = serde_json::from_reader(File::open(FILE_NAME).unwrap()).unwrap();
    let mut map: HashMap = serde_json::from_str(&file).unwrap();

    return map;
}

fn save_table(map: HashMap) {
    let serialized = serde_json::to_string(&map).unwrap();
    serde_json::to_writer(&File::create(FILE_NAME).unwrap(), &serialized);
}
