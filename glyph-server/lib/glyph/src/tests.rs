use super::Glyph;

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

impl Glyph for TestGlyph {
    type Config = TestGlyphConfig;
    type ExitData = TestGlyphExitData;

    fn load(config: Self::Config) -> Self {
        Self { name: config.name }
    }

    fn run(self) -> Self::ExitData {
        Self::ExitData { name: self.name }
    }
}

#[test]
/// test loading a glyph, running it, and collecting the exit data
fn load_and_run() {
    let config = TestGlyphConfig {
        name: String::from("Test"),
    };
    let glyph = TestGlyph::load(config.clone());
    let exit_data = glyph.run();
    assert_eq!(exit_data.name, config.name);
}
