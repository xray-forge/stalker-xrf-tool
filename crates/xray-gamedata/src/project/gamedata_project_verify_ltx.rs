use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataResult};
use colored::Colorize;
use xray_ltx::{
  LtxFormatOptions, LtxProjectFormatResult, LtxProjectVerifyResult, LtxVerifyOptions,
};

impl GamedataProject {
  // todo: Add used LTX files paths based on system ltx / spawn files.
  // todo: Add used LTX files paths based on system ltx / spawn files.
  // todo: Add used LTX files paths based on system ltx / spawn files.

  pub fn verify_ltx_format(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> GamedataResult<LtxProjectFormatResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify gamedata LTX files formatting".green());
    }

    Ok(
      self
        .ltx_project
        .check_format_all_files_opt(LtxFormatOptions {
          is_silent: options.is_silent,
          is_verbose: options.is_verbose,
        })?,
    )
  }

  pub fn verify_ltx_schemes(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> GamedataResult<LtxProjectVerifyResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify gamedata LTX schemas".green());
    };

    Ok(self.ltx_project.verify_entries_opt(LtxVerifyOptions {
      is_silent: options.is_silent,
      is_verbose: options.is_verbose,
      is_strict: options.is_strict,
    })?)
  }
}
