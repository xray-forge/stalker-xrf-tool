use std::path::PathBuf;
use xray_ltx::{LtxFormatOptions, LtxProject, LtxProjectOptions, LtxVerifyOptions};

#[tauri::command]
pub async fn format_configs_path(path: &str) -> Result<(), String> {
  log::info!("Open ltx folder: {:?}", path);

  let project: LtxProject = match LtxProject::open_at_path(&PathBuf::from(path)) {
    Ok(project) => project,
    Err(error) => return Err(error.to_string()),
  };

  log::info!("Formatting ltx folder: {:?}", path);

  match project.format_all_files_opt(LtxFormatOptions { is_silent: true }) {
    Ok(_) => Ok(()),
    Err(error) => Err(error.to_string()),
  }
}

#[tauri::command]
pub async fn verify_configs_path(path: &str) -> Result<(), String> {
  log::info!("Open ltx folder: {:?}", path);

  let project: LtxProject = match LtxProject::open_at_path_opt(
    &PathBuf::from(path),
    LtxProjectOptions {
      is_with_schemes_check: true,
    },
  ) {
    Ok(project) => project,
    Err(error) => return Err(error.to_string()),
  };

  log::info!("Verifying ltx folder: {:?}", path);

  match project.verify_entries_opt(LtxVerifyOptions {
    is_silent: true,
    is_verbose: false,
    is_strict: false,
  }) {
    Ok(_) => Ok(()),
    Err(error) => Err(error.to_string()),
  }
}
