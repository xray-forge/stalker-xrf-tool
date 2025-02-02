use crate::constants::META_TYPE_FIELD;
use crate::file_import::read_ltx_field;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use std::io::Write;
use xray_chunk::{read_till_end_binary_chunk, ChunkReader, ChunkWriter};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};
use xray_utils::{decode_bytes_from_base64, encode_bytes_to_base64};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParticleEffectEditorData {
  pub value: Vec<u8>,
}

impl ParticleEffectEditorData {
  pub const META_TYPE: &'static str = "editor_data";

  /// Read particle effect editor data data from chunk redder.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    let particle_description: Self = Self {
      value: read_till_end_binary_chunk(reader)?,
    };

    assert!(
      reader.is_ended(),
      "Expect particle editor data chunk to be ended"
    );

    Ok(particle_description)
  }

  /// Write particle effect description data into chunk writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_all(&self.value)?;

    Ok(())
  }

  /// Import optional particle effect collision data from provided path.
  pub fn import_optional(section_name: &str, ltx: &Ltx) -> XRayResult<Option<Self>> {
    if ltx.has_section(section_name) {
      Self::import(section_name, ltx).map(Some)
    } else {
      Ok(None)
    }
  }

  /// Import particle effect description data from provided path.
  pub fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "Particle effect editor data section '{}' should be defined in ltx file ({})",
        section_name,
        file!()
      ))
    })?;

    let meta_type: String = read_ltx_field(META_TYPE_FIELD, section)?;

    assert_eq!(
      meta_type,
      Self::META_TYPE,
      "Expected corrected meta type field for '{}' importing",
      Self::META_TYPE
    );

    Ok(Self {
      value: decode_bytes_from_base64(&read_ltx_field::<String>("value", section)?)?,
    })
  }

  /// Export particle effect collision data into provided path.
  pub fn export_optional(data: Option<&Self>, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    if let Some(data) = data {
      data.export(section_name, ltx)
    } else {
      Ok(())
    }
  }

  /// Export particle effect editor data into provided path.
  pub fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    ltx
      .with_section(section_name)
      .set(META_TYPE_FIELD, Self::META_TYPE)
      .set("value", encode_bytes_to_base64(&self.value));

    Ok(())
  }
}
