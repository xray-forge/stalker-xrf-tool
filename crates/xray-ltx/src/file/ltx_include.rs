use crate::file::error::LtxConvertError;
use crate::{Ltx, LtxError, ParseOptions};
use std::io;
use std::path::{Path, PathBuf};

/// Converter object to process and inject all child #include statements.
#[derive(Default)]
pub struct LtxIncludeConvertor {}

impl LtxIncludeConvertor {
  fn new() -> LtxIncludeConvertor {
    LtxIncludeConvertor {}
  }

  /// Cast LTX file to fully parsed with include sections.
  pub fn convert(ltx: Ltx, options: ParseOptions) -> Result<Ltx, LtxError> {
    LtxIncludeConvertor::new().convert_ltx(ltx, options)
  }
}

impl LtxIncludeConvertor {
  /// Convert ltx file with inclusion of nested files.
  fn convert_ltx(&self, ltx: Ltx, options: ParseOptions) -> Result<Ltx, LtxError> {
    if ltx.directory.is_none() {
      return Err(LtxConvertError::new_ltx_error(
        "Failed to convert ltx file, parent directory is not specified",
      ));
    }

    // Nothing to parse - no include statements.
    if ltx.includes.is_empty() {
      return Ok(ltx);
    }

    let mut result: Ltx = Ltx {
      path: ltx.path,
      directory: ltx.directory,
      includes: Default::default(),
      sections: Default::default(),
    };

    for included in &ltx.includes {
      let mut included_path: PathBuf = result.directory.as_ref().unwrap().clone();

      included_path.push(PathBuf::from(included));

      self.include_children(&mut result, &included_path, options.clone())?;
    }

    for (key, value) in ltx.sections {
      if result.has_section(key.clone()) {
        return Err(LtxConvertError::new_ltx_error(format!(
          "Failed to convert ltx file, duplicate section {key} found",
        )));
      }

      result.sections.insert(key, value);
    }

    Ok(result)
  }

  /// Include children ltx into provided ltx.
  fn include_children(
    &self,
    into: &mut Ltx,
    path: &Path,
    options: ParseOptions,
  ) -> Result<(), LtxError> {
    let ltx: Ltx = match self.parse_nested_file(path, options.clone()) {
      Ok(value) => match value {
        Some(ltx) => ltx,
        None => return Ok(()),
      },
      Err(error) => {
        return Err(LtxConvertError::new_ltx_error(format!(
          "Failed to convert ltx file, nested file '{:?}' in {:?} error: {error}",
          path,
          into.path.as_ref().unwrap(),
        )))
      }
    };

    for (key, value) in ltx.into_included_opt(options)?.sections {
      if into.has_section(key.clone()) {
        return Err(LtxConvertError::new_ltx_error(format!(
          "Failed to include ltx file '{:?}' in {:?}, duplicate section {key} found",
          path,
          into.path.as_ref().unwrap()
        )));
      }

      into.sections.insert(key, value);
    }

    Ok(())
  }

  /// Open nested file for importing in current context.
  /// Skips '.ts' variant of configuration file as None.
  fn parse_nested_file(&self, path: &Path, options: ParseOptions) -> Result<Option<Ltx>, LtxError> {
    match Ltx::load_from_file_opt(path, options) {
      Ok(ltx) => Ok(Some(ltx)),
      Err(error) => match error {
        LtxError::Io(ref io_error) => {
          if io_error.kind() == io::ErrorKind::NotFound {
            if self.is_raw_ts_variant_existing(path) {
              Ok(None)
            } else {
              Err(error)
            }
          } else {
            Err(error)
          }
        }
        _ => Err(error),
      },
    }
  }

  /// Check if similar TS counterpart exists for provided ltx path.
  fn is_raw_ts_variant_existing(&self, path: &Path) -> bool {
    if path.extension().is_some_and(|extension| extension == "ltx") {
      path.with_extension("ts").exists()
    } else {
      false
    }
  }
}
