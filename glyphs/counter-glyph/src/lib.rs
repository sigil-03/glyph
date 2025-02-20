use glyphic::Glyphic;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Config {
    initial_value: usize,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            initial_value: 0,
        }
    }
}

#[derive(Debug)]
pub struct ExitData {
    final_value: usize,
}

pub struct Glyph {
    counter: usize,
}

impl Glyphic for Glyph {
    type Config = Config;
    type ExitData = ExitData;
    
    async fn load(config: Self::Config) -> Self {
        Self {
            counter: config.initial_value,
        }
    }

    async fn run(mut self) -> Self::ExitData {
        loop {
            println!("Counter: {}", self.counter);
            self.counter += 1;
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        }
    }
}
