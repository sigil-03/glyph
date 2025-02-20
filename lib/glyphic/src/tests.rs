use super::Glyphic;

#[derive(Clone)]
struct TestGlyphConfig {
    name: String,
}
struct TestGlyphExitData {
    name: String,
}
struct TestGlyph {
    name: String,
}

impl Glyphic for TestGlyph {
    type Config = TestGlyphConfig;
    type ExitData = TestGlyphExitData;

    async fn load(config: Self::Config) -> Self {
        Self { name: config.name }
    }

    async fn run(self) -> Self::ExitData {
        Self::ExitData { name: self.name }
    }
}

#[tokio::test]
/// test loading a glyph, running it, and collecting the exit data
async fn load_and_run() {
    let config = TestGlyphConfig {
        name: String::from("Test"),
    };
    let glyph = TestGlyph::load(config.clone()).await;
    let exit_data = glyph.run().await;
    assert_eq!(exit_data.name, config.name);
}
