use crate::error::ltx_scheme_error::LtxSchemeError;
use crate::file::configuration::constants::{LTX_SCHEME_FIELD, LTX_SYMBOL_ANY};
use crate::project::verify_options::LtxVerifyOptions;
use crate::{Ltx, LtxProject, LtxProjectVerifyResult, LtxResult};
use fxhash::FxBuildHasher;
use indexmap::IndexSet;
use std::path::Path;
use std::time::Instant;

impl LtxProject {
  /// Verify all the entries in current ltx project.
  /// Make sure that:
  /// - All included files exist or `.ts` counterpart is declared
  /// - All the inherited sections are valid and declared before inherit attempt
  pub fn verify_entries_opt(&self, options: LtxVerifyOptions) -> LtxResult<LtxProjectVerifyResult> {
    let mut result: LtxProjectVerifyResult = LtxProjectVerifyResult::new();
    let started_at: Instant = Instant::now();

    if !options.is_silent {
      println!("Verify path: {:?}", self.root);
    }

    // For each file entry in the project:
    for entry in &self.ltx_file_entries {
      let entry_path: &Path = entry.path();

      // Do not check scheme definitions for scheme files - makes no sense.
      if Self::is_ltx_scheme_path(entry_path) {
        continue;
      } else {
        result.total_files += 1;
      }

      let ltx: Ltx = Ltx::load_from_file_full(entry_path)?;

      // For each section in file:
      for (section_name, section) in &ltx {
        result.total_sections += 1;

        // Check only if schema is defined:
        if let Some(scheme_name) = section.get(LTX_SCHEME_FIELD) {
          let mut section_has_error: bool = false;

          result.checked_sections += 1;

          // Check if definition or required schema exists:
          if let Some(scheme_definition) = self.ltx_scheme_declarations.get(scheme_name) {
            let mut validated: IndexSet<String, FxBuildHasher> = Default::default();

            // Check all fields in section data.
            for (field_name, value) in section {
              validated.insert(field_name.into());

              // Respect `*` definition for mapping sections.
              if let Some(field_definition) = scheme_definition
                .fields
                .get(field_name)
                .or_else(|| scheme_definition.fields.get(LTX_SYMBOL_ANY))
              {
                if options.is_verbose && !options.is_silent {
                  println!("Checking {:?} [{section_name}] {field_name}", entry_path);
                }

                result.checked_fields += 1;

                if let Some(mut error) = field_definition.validate_value(&ltx, value) {
                  error.section = section_name.into();
                  error.at = Some(entry_path.to_str().unwrap().into());

                  section_has_error = true;

                  result.errors.push(error);
                }
              } else if scheme_definition.is_strict {
                section_has_error = true;

                result.errors.push(LtxSchemeError::new_at(
                  section_name,
                  field_name,
                  "Unexpected field, definition is required in strict mode",
                  entry_path.to_str().unwrap(),
                ));
              }
            }

            if scheme_definition.is_strict {
              for (field_name, definition) in &scheme_definition.fields {
                if !definition.is_optional
                  && field_name != LTX_SYMBOL_ANY
                  && !validated.contains(field_name)
                {
                  section_has_error = true;

                  result.errors.push(LtxSchemeError::new_at(
                    section_name,
                    field_name,
                    "Required field was not provided",
                    entry_path.to_str().unwrap(),
                  ));
                }
              }
            }
          } else {
            section_has_error = true;

            result.errors.push(LtxSchemeError::new_at(
              section_name,
              "*",
              format!("Required schema '{scheme_name}' definition is not found"),
              entry_path.to_str().unwrap(),
            ));
          }

          if section_has_error {
            result.invalid_sections += 1;
          } else {
            result.valid_sections += 1;
          }
        } else if options.is_strict {
          result.invalid_sections += 1;
          result.errors.push(LtxSchemeError::new_at(
            section_name,
            "*",
            "Expected '$schema' field to be defined in strict mode check",
            entry_path.to_str().unwrap(),
          ));
        } else {
          result.skipped_sections += 1
        }
      }
    }

    result.duration = started_at.elapsed().as_millis();

    if !options.is_silent {
      for error in &result.errors {
        println!("{}", error);
      }

      println!(
        "Checked {} files, {} sections in {} sec",
        self.ltx_files.len(),
        result.total_sections,
        (result.duration as f64) / 1000.0
      );
      println!(
        "Verified {:.2}%, {} files, {} sections, {} fields",
        (result.checked_sections as f32 * 100.0) / result.total_sections as f32,
        result.total_files,
        result.checked_sections,
        result.checked_fields
      );
      println!("Found {} error(s)", result.errors.len());
    }

    Ok(result)
  }

  /// Verify all the section/field entries in current ltx project.
  pub fn verify_entries(&self) -> LtxResult<LtxProjectVerifyResult> {
    self.verify_entries_opt(Default::default())
  }

  /// Format single LTX file by provided path
  pub fn verify_file(path: &Path) -> LtxResult<()> {
    Ltx::load_from_file_full(path)?;

    Ok(())
  }
}
