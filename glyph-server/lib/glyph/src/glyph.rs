/// A glyph is a plugin for the glyph server to run.
pub trait Glyph {
    type Config;
    type ExitData;
    /// load() loads the plugin using the given configuration
    fn load(config: Self::Config) -> Self;

    /// run() consumes `self` and runs
    fn run(self) -> Self::ExitData;
}
