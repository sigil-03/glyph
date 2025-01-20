use clap::Parser;
use counter_glyph::{Config as CounterGlyphConfig, Glyph as CounterGlyph};
use glyph::Glyphic;
use std::fmt::Debug;
use std::fs;
use tokio::net::TcpListener;

#[derive(Parser)]
struct LaunchOptions {
    config_file: String,
}

struct GlyphServer {
    counter: CounterGlyph,
    listener: TcpListener,
}

impl GlyphServer {
    pub async fn load_glyph_from_file<T>(file: &str) -> T
    where
        T: Glyphic,
        for<'de> <T as Glyphic>::Config: serde::Deserialize<'de>,
        <T as Glyphic>::Config: Debug,
    {
        let file_contents = fs::read_to_string(&file).expect("Could not read input file");
        let config: T::Config = toml::from_str(&file_contents).expect("Could not parse TOML file");
        T::load(config).await
    }

    pub async fn new() -> Self {
        Self {
            counter: Self::load_glyph_from_file("../glyphs/counter-glyph/glyph_config.toml").await,
            listener: TcpListener::bind("127.0.0.1:8080")
                .await
                .expect("Failed to start TCP listener"),
        }
    }

    pub async fn run(self) {
        let out = tokio::spawn(self.counter.run());

        let res = tokio::join!(out);
        println!("output: {:#?}", res);
    }
}

// let recipe: TomlRecipe = toml::from_str(&file).expect("Could not parse TOML file");

//

//     pub fn load(glyph: T: Glyphic) {}
// }

#[tokio::main]
async fn main() {
    let s = GlyphServer::new().await;
    s.run().await;
}
