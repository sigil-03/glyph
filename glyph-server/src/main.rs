mod counter_glyph;
use counter_glyph::{Glyph as CounterGlyph, Config as CounterGlyphConfig};
use glyph::Glyphic;

#[tokio::main]
async fn main() {
    let g1 = CounterGlyph::load(CounterGlyphConfig::default()).await;
    let out = g1.run().await;
    println!("output: {:#?}", out);
}
