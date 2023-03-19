use lazy_static::lazy_static;
use std::sync::Mutex;

extern crate markov;

use markov::Chain;

lazy_static! {
    static ref MARKOV: Mutex<Chain<String>> = Mutex::new(Chain::new());
}

pub fn start() {
    println!("Started");
    MARKOV
        .lock()
        .unwrap()
        .feed_str("I like cats and I like dogs.");
}

pub fn feed(sentence: &str) {
    MARKOV.lock().unwrap().feed_str(sentence);
}

pub fn generate() -> String {
    println!("Generate");
    return MARKOV.lock().unwrap().generate_str();
}
