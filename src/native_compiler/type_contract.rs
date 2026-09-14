use super::CompilerError;

impl std::fmt::Display for CompilerError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Configuration(detail) => write!(formatter, "compiler configuration: {detail}"),
            Self::Process(error) => std::fmt::Display::fmt(error, formatter),
        }
    }
}

impl std::error::Error for CompilerError {}
