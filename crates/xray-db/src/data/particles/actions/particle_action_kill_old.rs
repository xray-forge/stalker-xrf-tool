use crate::data::meta::particle_action_reader::ParticleActionReader;
use crate::data::meta::particle_action_writer::ParticleActionWriter;
use crate::file_import::read_ltx_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionKillOld {
  pub age_limit: f32,
  pub kill_less_than: u32,
}

impl ParticleActionReader for ParticleActionKillOld {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<ParticleActionKillOld> {
    Ok(ParticleActionKillOld {
      age_limit: reader.read_f32::<T>()?,
      kill_less_than: reader.read_u32::<T>()?,
    })
  }

  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "Particle action section '{}' should be defined in ltx file ({})",
        section_name,
        file!()
      ))
    })?;

    Ok(Self {
      age_limit: read_ltx_field("age_limit", section)?,
      kill_less_than: read_ltx_field("kill_less_than", section)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionWriter for ParticleActionKillOld {
  fn write(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_f32::<XRayByteOrder>(self.age_limit)?;
    writer.write_u32::<XRayByteOrder>(self.kill_less_than)?;

    Ok(())
  }

  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    ltx
      .with_section(section_name)
      .set("age_limit", self.age_limit.to_string())
      .set("kill_less_than", self.kill_less_than.to_string());

    Ok(())
  }
}
