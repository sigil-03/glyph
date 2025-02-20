/// A glyph is a plugin for the glyph server to run.
pub trait Glyphic {
    type Config;
    type ExitData;

    /// load() loads the plugin using the given configuration
    fn load(config: Self::Config) -> impl std::future::Future<Output = Self> + Send;

    /// run() consumes `self` and runs
    fn run(self) -> impl std::future::Future<Output = Self::ExitData> + Send;
}
