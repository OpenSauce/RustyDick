use std::sync::Arc;

use markov::Chain;
use serenity::prelude::TypeMapKey;
use tokio::sync::RwLock;

pub struct MarkovChainer;

impl TypeMapKey for MarkovChainer {
    type Value = Arc<RwLock<Chain<String>>>;
}
