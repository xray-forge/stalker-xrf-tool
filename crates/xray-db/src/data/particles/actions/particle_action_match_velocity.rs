use crate::export::LtxImportExport;
use crate::file_import::read_ltx_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionMatchVelocity {
  pub magnitude: f32,
  pub epsilon: f32,
  pub max_radius: f32,
}

impl ChunkReadWrite for ParticleActionMatchVelocity {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<ParticleActionMatchVelocity> {
    Ok(Self {
      magnitude: reader.read_f32::<T>()?,
      epsilon: reader.read_f32::<T>()?,
      max_radius: reader.read_f32::<T>()?,
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_f32::<T>(self.magnitude)?;
    writer.write_f32::<T>(self.epsilon)?;
    writer.write_f32::<T>(self.max_radius)?;

    Ok(())
  }
}

impl LtxImportExport for ParticleActionMatchVelocity {
  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "Particle action section '{}' should be defined in ltx file ({})",
        section_name,
        file!()
      ))
    })?;

    Ok(Self {
      magnitude: read_ltx_field("magnitude", section)?,
      epsilon: read_ltx_field("epsilon", section)?,
      max_radius: read_ltx_field("max_radius", section)?,
    })
  }

  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    ltx
      .with_section(section_name)
      .set("magnitude", self.magnitude.to_string())
      .set("epsilon", self.epsilon.to_string())
      .set("max_radius", self.max_radius.to_string());

    Ok(())
  }
}
