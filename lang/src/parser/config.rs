/// Parser configuration options
#[derive(Debug, Clone)]
pub struct ParserConfig {
    /// Allow trailing commas in lists
    pub(super) allow_trailing_commas: bool,
    /// Maximum recursion depth to prevent stack overflow
    pub(super) max_recursion_depth: usize,
}

impl ParserConfig {
    pub fn allow_trailing_commas(&self) -> bool {
        self.allow_trailing_commas
    }

    pub fn max_recursion_depth(&self) -> usize {
        self.max_recursion_depth
    }
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            allow_trailing_commas: true,
            max_recursion_depth: 1000,
        }
    }
}
