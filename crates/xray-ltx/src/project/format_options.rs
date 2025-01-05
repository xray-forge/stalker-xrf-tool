/// Formatting configuration.
#[derive(Clone, Default)]
pub struct LtxFormatOptions {
  /// Whether log is in silent mode.
  pub is_silent: bool,
}

impl LtxFormatOptions {
  pub fn new() -> Self {
    Self { is_silent: false }
  }
}
