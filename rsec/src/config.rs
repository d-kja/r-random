/// Language of choice for your application
pub enum Languages {
    Javascript,
    Typescript,
}

/// Style of choice for your application
pub enum Style {
    Tailwind,
}

/// Configuration for the CLI execution
pub struct CliConfig {
    /// Defines the path for the output
    path: String,

    /// Defines the language choice
    language: Languages,
    /// Defines the styling choice
    style: Style,
}

impl CliConfig {
    /// Creates a new instance with the default configuration
    fn new(path: &str) -> Self {
        Self {
            path: path.to_owned(),
            language: Languages::Typescript,
            style: Style::Tailwind,
        }
    }
}
