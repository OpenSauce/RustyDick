use log::info;
use markov::Chain;
use serenity::prelude::TypeMapKey;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

pub struct MarkovChainer;

impl TypeMapKey for MarkovChainer {
    type Value = Arc<RwLock<Chain<String>>>;
}

pub fn load_markov_chain() -> Chain<String> {
    let mut chain: Chain<String> = Chain::new();

    let start = Instant::now();
    let lines = read_lines("./history/log.txt".to_string());
    for line in lines {
        let line = line.unwrap();
        chain.feed_str(line.as_str());
    }

    info!("Log load time is: {:?}", start.elapsed());

    chain
}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}
