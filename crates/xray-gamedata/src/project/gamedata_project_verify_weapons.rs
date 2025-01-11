use crate::constants::NO_SOUND;
use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataResult};
use colored::Colorize;
use std::path::PathBuf;
use xray_db::{OgfFile, XRayByteOrder};
use xray_ltx::{Ltx, Section};

impl GamedataProject {
  pub fn verify_ltx_weapons(&mut self, options: &GamedataProjectVerifyOptions) -> GamedataResult {
    log::info!("Verify gamedata weapons");

    if options.is_logging_enabled() {
      println!("{}", "Verify gamedata LTX weapons".green());
    }

    let system_ltx: Ltx = self.ltx_project.get_system_ltx()?;

    let mut checked_weapons_count: usize = 0;
    let mut invalid_weapons_count: usize = 0;

    for (section_name, section) in &system_ltx.sections {
      if Self::is_weapon_section(section) {
        checked_weapons_count += 1;
      } else {
        continue;
      }

      match self.verify_ltx_weapon(options, &system_ltx, section_name, section) {
        Ok(is_valid) => {
          if !is_valid {
            log::warn!("Invalid weapon section: [{section_name}]");

            if options.is_logging_enabled() {
              eprintln!("Invalid weapon section: [{section_name}]");
            }

            invalid_weapons_count += 1;
          }
        }
        Err(error) => {
          log::warn!("Invalid weapon section: [{section_name}], {error:?}");

          if options.is_logging_enabled() {
            eprintln!("Invalid weapon section: [{section_name}], {error:?}");
          }

          invalid_weapons_count += 1;
        }
      }
    }

    if options.is_logging_enabled() {
      log::info!(
        "Verified gamedata weapons, {}/{} valid",
        checked_weapons_count - invalid_weapons_count,
        checked_weapons_count
      );

      println!(
        "Verified gamedata weapons, {}/{} valid",
        checked_weapons_count - invalid_weapons_count,
        checked_weapons_count
      );
    }

    Ok(())
  }

  pub fn verify_ltx_weapon(
    &mut self,
    options: &GamedataProjectVerifyOptions,
    ltx: &Ltx,
    section_name: &str,
    section: &Section,
  ) -> GamedataResult<bool> {
    log::info!("Verify weapon ltx config [{section_name}]");

    if options.is_verbose_logging_enabled() {
      println!("Verify weapon ltx config [{section_name}]");
    }

    let mut is_weapon_valid: bool = true;

    let visual: Option<PathBuf> = self.get_section_ogf_visual(section, "visual");
    let hud_section: Option<&Section> = section.get("hud").and_then(|it| ltx.section(it));
    let hud_visual: Option<PathBuf> =
      hud_section.and_then(|it| self.get_section_ogf_visual(it, "item_visual"));

    // todo: Check animations as separate util checker for all existing meshes.
    // todo: Check textures as separate util checker for all existing meshes.

    if let Some(visual) = &visual {
      OgfFile::read_from_path::<XRayByteOrder>(visual)?;
    }

    if let Some(hud_visual) = &hud_visual {
      OgfFile::read_from_path::<XRayByteOrder>(hud_visual)?;
    }

    if visual.is_none() {
      log::info!(
        "Not found visual: [{section_name}] - {:?}",
        section.get("visual")
      );

      if options.is_logging_enabled() {
        eprintln!("Not found hud visual: [{section_name}]");
      }

      is_weapon_valid = false;
    }

    if hud_visual.is_none() {
      log::warn!("Not found hud visual: [{section_name}]");

      if options.is_logging_enabled() {
        eprintln!("Not found hud visual: [{section_name}]");
      }

      is_weapon_valid = false;
    }

    if !self
      .verify_weapon_sounds(options, ltx, section_name, section)
      .is_ok_and(|it| it)
    {
      log::warn!("Weapon sounds are invalid: [{section_name}]");

      if options.is_logging_enabled() {
        eprintln!("Weapon sounds are invalid: [{section_name}]");
      }

      is_weapon_valid = false;
    }

    Ok(is_weapon_valid)
  }

  pub fn get_section_ogf_visual(&mut self, section: &Section, field_name: &str) -> Option<PathBuf> {
    section
      .get(field_name)
      .map(|it| {
        let mut visual_path: String = String::from(it);

        if !it.ends_with(".ogf") {
          visual_path.push_str(".ogf");
        }

        visual_path
      })
      .and_then(|it| self.get_prefixed_relative_asset_path("meshes", &it))
  }

  pub fn verify_weapon_sounds(
    &mut self,
    options: &GamedataProjectVerifyOptions,
    ltx: &Ltx,
    section_name: &str,
    section: &Section,
  ) -> GamedataResult<bool> {
    let mut are_sounds_valid: bool = true;

    for sound_section in [
      "snd_draw",
      "snd_empty",
      "snd_holster",
      "snd_reload",
      "snd_shoot",
    ] {
      if !section.contains_key(sound_section) {
        log::warn!("Missing section required weapon sound: [{section_name}] : {sound_section}");

        if options.is_logging_enabled() {
          eprintln!("Missing section required weapon sound: [{section_name}] : {sound_section}");
        }

        are_sounds_valid = false;
      }
    }

    for (field_name, field_value) in section {
      if !field_name.starts_with("snd_") {
        continue;
      }

      if field_value == NO_SOUND {
        continue;
      }

      // Layered sounds from OXR/COC.
      if ltx.has_section(field_name) {
        // todo: Check sound layer structure here and linked sounds?
        //
        // [wpn_abakan_snd_shoot]
        // snd_1_layer = weapons\abakan\abakan_shoot
        // snd_1_layer1 = weapons\abakan\abakan_shoot1
        continue;
      }

      // Sounds field is 1-3 comma separated values:
      let mut sound_object_path: String = String::from(
        *field_value
          .split(",")
          .collect::<Vec<&str>>()
          .first()
          .unwrap_or(&"~failed-to-parse~"),
      );

      if !sound_object_path.ends_with(".ogg") {
        sound_object_path.push_str(".ogg");
      }

      // todo: Check OGG file, check existing.
      if let Some(sound_path) = self.get_prefixed_relative_asset_path("sounds", &sound_object_path)
      {
        if sound_path.is_file() && sound_path.exists() {
          if options.is_verbose_logging_enabled() {
            eprintln!("Sound verified in weapon section: [{section_name}] : {field_name} -> {sound_object_path}");
          }
        } else {
          are_sounds_valid = true
        }
      } else {
        log::warn!("Sound not found in weapon section: [{section_name}] : {field_name} -> {sound_object_path}");

        if options.is_logging_enabled() {
          eprintln!("Sound not found in weapon section: [{section_name}] : {field_name} -> {sound_object_path}");
        }

        are_sounds_valid = true;
      }
    }

    Ok(are_sounds_valid)
  }
}
