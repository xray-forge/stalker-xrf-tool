use crate::icons_editor::state::{IconsEditorEquipmentResponse, IconsEditorState};
use image::codecs::png::PngEncoder;
use image::{ColorType, ImageEncoder, RgbaImage};
use serde_json::{json, Value};
use std::io::Write;
use std::path::PathBuf;
use std::sync::MutexGuard;
use xray_icon::{
  dds_to_image, get_ltx_inventory_descriptors, read_dds_by_path, ConfigInventorySectionDescriptor,
};
use xray_ltx::Ltx;

#[tauri::command]
pub async fn open_equipment_sprite(
  equipment_dds_path: &str,
  system_ltx_path: &str,
  state: tauri::State<'_, IconsEditorState>,
) -> Result<Value, String> {
  log::info!("Opening equipment file: {equipment_dds_path} - {system_ltx_path}");

  let name: &str = "equipment.png";

  let image: RgbaImage =
    match read_dds_by_path(&PathBuf::from(equipment_dds_path)).and_then(|dds| dds_to_image(&dds)) {
      Ok(image) => image,
      Err(error) => return Err(format!("Failed to open provided image file: {:?}", error,)),
    };

  let mut preview_buffer: Vec<u8> = Vec::new();

  PngEncoder::new(preview_buffer.by_ref())
    .write_image(
      image.as_raw(),
      image.width(),
      image.height(),
      ColorType::Rgba8,
    )
    .expect("error encoding pixels as PNG");

  log::info!("Opened equipment dds file");

  let descriptors: Vec<ConfigInventorySectionDescriptor> = get_ltx_inventory_descriptors(
    &Ltx::load_from_file_full(system_ltx_path).map_err(|error| error.to_string())?,
  );

  let response = IconsEditorEquipmentResponse {
    path: equipment_dds_path.into(),
    name: name.into(),
    equipment_descriptors: descriptors.clone(),
  };

  *state.equipment_sprite_name.lock().unwrap() = Some(name.into());
  *state.equipment_sprite_path.lock().unwrap() = Some(equipment_dds_path.into());
  *state.equipment_sprite.lock().unwrap() = Some(image);
  *state.equipment_sprite_preview.lock().unwrap() = Some(preview_buffer);
  *state.equipment_descriptors.lock().unwrap() = Some(descriptors);

  Ok(json!(response))
}

#[tauri::command]
pub async fn get_equipment_sprite(
  state: tauri::State<'_, IconsEditorState>,
) -> Result<Option<Value>, String> {
  let path_lock: MutexGuard<Option<String>> = state.equipment_sprite_path.lock().unwrap();
  let name_lock: MutexGuard<Option<String>> = state.equipment_sprite_name.lock().unwrap();
  let equipment_lock: MutexGuard<Option<Vec<ConfigInventorySectionDescriptor>>> =
    state.equipment_descriptors.lock().unwrap();

  if (*equipment_lock).is_none() || (*name_lock).is_none() {
    return Ok(None);
  }

  Ok(Some(json!(IconsEditorEquipmentResponse {
    path: path_lock.as_ref().unwrap().clone(),
    name: name_lock.as_ref().unwrap().clone(),
    equipment_descriptors: equipment_lock.as_ref().unwrap().clone(),
  })))
}

#[tauri::command]
pub async fn close_equipment_sprite(
  state: tauri::State<'_, IconsEditorState>,
) -> Result<(), String> {
  log::info!("Closing equipment file:");

  *state.equipment_sprite_path.lock().unwrap() = None;
  *state.equipment_sprite_name.lock().unwrap() = None;
  *state.equipment_descriptors.lock().unwrap() = None;
  *state.equipment_sprite.lock().unwrap() = None;
  *state.equipment_sprite_preview.lock().unwrap() = None;

  Ok(())
}