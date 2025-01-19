mod counter_glyph;
use counter_glyph::{Glyph as CounterGlyph, Config as CounterGlyphConfig};
use glyph::Glyphic;
use std::fs;

use std::fmt::Debug;


struct GlyphServer {
    counter: CounterGlyph,
}

impl GlyphServer {
    pub async fn load_glyph_from_file<T>(file: &str) -> T 
    where T: Glyphic, 
    for<'de> <T as Glyphic>::Config: serde::Deserialize<'de>,
    <T as Glyphic>::Config: Debug,
    {
        let file_contents = fs::read_to_string(&file).expect("Could not read input file");
        let config: T::Config = toml::from_str(&file_contents).expect("Could not parse TOML file");
        T::load(config).await

    }

    pub async fn new() -> Self {
        Self {
            counter: Self::load_glyph_from_file("./test/counter.toml").await,
        }
    }

    pub async fn run(self) {
        let out = self.counter.run().await;
        println!("output: {:#?}", out);
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
