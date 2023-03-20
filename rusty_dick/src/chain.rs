use std::collections::HashMap;

use std::sync::{Arc, Mutex};

extern crate markov;

use markov::Chain;
use once_cell::sync::Lazy;

static GLOBAL_DATA: Lazy<Arc<Mutex<Chain<String>>>> =
    Lazy::new(|| Arc::new(Mutex::new(Chain::new())));

static GLOBAL_MAP: Lazy<Arc<Mutex<HashMap<i32, String>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));


pub fn start() {
    println!("Started");
    GLOBAL_MAP.lock().unwrap().push(1);
    println!("{}", GLOBAL_DATA.lock().unwrap().generate_str());
}

pub fn feed(sentence: &str) {
    GLOBAL_DATA.lock().unwrap().feed_str(sentence);
}

pub fn generate() -> String {
    println!("called {}", TEST.lock().unwrap().len());
    return String::from("Hi");
}
