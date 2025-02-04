use crate::data::generic::vector_3d::Vector3d;
use crate::data::particles::particle_action_type::ParticleActionType;
use crate::export::LtxImportExport;
use crate::file_import::read_ltx_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionExplosion {
  pub action_flags: u32,
  pub action_type: ParticleActionType,
  pub center: Vector3d,
  pub velocity: f32,
  pub magnitude: f32,
  pub st_dev: f32,
  pub age: f32,
  pub epsilon: f32,
}

impl ChunkReadWrite for ParticleActionExplosion {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    Ok(Self {
      action_flags: reader.read_u32::<T>()?,
      action_type: reader.read_xr::<T, _>()?,
      center: reader.read_xr::<T, _>()?,
      velocity: reader.read_f32::<T>()?,
      magnitude: reader.read_f32::<T>()?,
      st_dev: reader.read_f32::<T>()?,
      age: reader.read_f32::<T>()?,
      epsilon: reader.read_f32::<T>()?,
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_u32::<T>(self.action_flags)?;
    writer.write_xr::<T, _>(&self.action_type)?;
    writer.write_xr::<T, _>(&self.center)?;
    writer.write_f32::<T>(self.velocity)?;
    writer.write_f32::<T>(self.magnitude)?;
    writer.write_f32::<T>(self.st_dev)?;
    writer.write_f32::<T>(self.age)?;
    writer.write_f32::<T>(self.epsilon)?;

    Ok(())
  }
}

impl LtxImportExport for ParticleActionExplosion {
  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "Particle action section '{}' should be defined in ltx file ({})",
        section_name,
        file!()
      ))
    })?;

    Ok(Self {
      action_flags: read_ltx_field("action_flags", section)?,
      action_type: read_ltx_field("action_type", section)?,
      center: read_ltx_field("center", section)?,
      velocity: read_ltx_field("velocity", section)?,
      magnitude: read_ltx_field("magnitude", section)?,
      st_dev: read_ltx_field("st_dev", section)?,
      age: read_ltx_field("age", section)?,
      epsilon: read_ltx_field("epsilon", section)?,
    })
  }

  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    ltx
      .with_section(section_name)
      .set("action_flags", self.action_flags.to_string())
      .set("action_type", self.action_type.to_string())
      .set("center", self.center.to_string())
      .set("velocity", self.velocity.to_string())
      .set("magnitude", self.magnitude.to_string())
      .set("st_dev", self.st_dev.to_string())
      .set("age", self.age.to_string())
      .set("epsilon", self.epsilon.to_string());

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::generic::vector_3d::Vector3d;
  use crate::data::particles::actions::particle_action_explosion::ParticleActionExplosion;
  use crate::data::particles::particle_action_type::ParticleActionType;
  use crate::export::LtxImportExport;
  use serde_json::to_string_pretty;
  use std::fs::File;
  use std::io::{Seek, SeekFrom, Write};
  use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter, XRayByteOrder};
  use xray_error::XRayResult;
  use xray_ltx::Ltx;
  use xray_test_utils::file::read_file_as_string;
  use xray_test_utils::utils::{
    get_absolute_test_resource_path, get_relative_test_sample_file_path,
    open_test_resource_as_slice, overwrite_test_relative_resource_as_file,
  };
  use xray_test_utils::FileSlice;

  #[test]
  fn test_read_write() -> XRayResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: ParticleActionExplosion = ParticleActionExplosion {
      action_flags: 1,
      action_type: ParticleActionType::Explosion,
      center: Vector3d::new_mock(),
      velocity: 36.3,
      magnitude: 20.0,
      st_dev: 0.2,
      age: 430.0,
      epsilon: 0.0001,
    };

    ParticleActionExplosion::write::<XRayByteOrder>(&original, &mut writer)?;

    assert_eq!(writer.bytes_written(), 40);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 40);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 40 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      ParticleActionExplosion::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_import_export() -> XRayResult {
    let ltx_filename: String = get_relative_test_sample_file_path(file!(), "import_export.ltx");
    let mut ltx: Ltx = Ltx::new();

    let original: ParticleActionExplosion = ParticleActionExplosion {
      action_flags: 1,
      action_type: ParticleActionType::Explosion,
      center: Vector3d::new_mock(),
      velocity: 30.5,
      magnitude: 1.0,
      st_dev: 0.14,
      age: 427.0,
      epsilon: 0.00001,
    };

    original.export("data", &mut ltx)?;

    ltx.write_to(&mut overwrite_test_relative_resource_as_file(
      &ltx_filename,
    )?)?;

    let source: Ltx = Ltx::read_from_path(get_absolute_test_resource_path(&ltx_filename))?;

    assert_eq!(ParticleActionExplosion::import("data", &source)?, original);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize() -> XRayResult {
    let original: ParticleActionExplosion = ParticleActionExplosion {
      action_flags: 1,
      action_type: ParticleActionType::Explosion,
      center: Vector3d::new_mock(),
      velocity: 36.422,
      magnitude: 1.2,
      st_dev: 0.12,
      age: 544.0,
      epsilon: 0.00001,
    };

    let mut file: File = overwrite_test_relative_resource_as_file(
      &get_relative_test_sample_file_path(file!(), "serialize_deserialize.json"),
    )?;

    file.write_all(to_string_pretty(&original)?.as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);
    assert_eq!(
      original,
      serde_json::from_str::<ParticleActionExplosion>(&serialized)?
    );

    Ok(())
  }
}
