use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::meta::particle_action_reader::ParticleActionReader;
use crate::data::meta::particle_action_writer::ParticleActionWriter;
use crate::data::vector_3d::Vector3d;
use crate::error::database_parse_error::DatabaseParseError;
use crate::export::file_import::read_ini_field;
use crate::types::{DatabaseResult, ParticlesByteOrder};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_ltx::{Ltx, Section};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionOrbitPoint {
  pub center: Vector3d,
  pub magnitude: f32,
  pub epsilon: f32,
  pub max_radius: f32,
}

impl ParticleActionReader for ParticleActionOrbitPoint {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    Ok(Self {
      center: reader.read_f32_3d_vector::<T>()?,
      magnitude: reader.read_f32::<T>()?,
      epsilon: reader.read_f32::<T>()?,
      max_radius: reader.read_f32::<T>()?,
    })
  }

  fn import(section_name: &str, ini: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ini.section("header").ok_or_else(|| {
      DatabaseParseError::new_database_error(format!(
        "Particle action section '{section_name}' should be defined in ltx file ({})",
        file!()
      ))
    })?;

    Ok(Self {
      center: read_ini_field("center", section)?,
      magnitude: read_ini_field("magnitude", section)?,
      epsilon: read_ini_field("epsilon", section)?,
      max_radius: read_ini_field("max_radius", section)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionWriter for ParticleActionOrbitPoint {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    writer.write_f32_3d_vector::<ParticlesByteOrder>(&self.center)?;
    writer.write_f32::<ParticlesByteOrder>(self.magnitude)?;
    writer.write_f32::<ParticlesByteOrder>(self.epsilon)?;
    writer.write_f32::<ParticlesByteOrder>(self.max_radius)?;

    Ok(())
  }

  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    ini
      .with_section(section)
      .set("center", self.center.to_string())
      .set("magnitude", self.magnitude.to_string())
      .set("epsilon", self.epsilon.to_string())
      .set("max_radius", self.max_radius.to_string());

    Ok(())
  }
}
