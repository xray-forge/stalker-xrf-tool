use crate::data::particles::particle_action_type::ParticleActionType;
use crate::data::particles::particle_domain::ParticleDomain;
use crate::export::LtxImportExport;
use crate::file_import::read_ltx_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionBounce {
  pub action_flags: u32,
  pub action_type: ParticleActionType,
  pub position: ParticleDomain,
  pub one_minus_friction: f32,
  pub resilience: f32,
  pub cutoff_sqr: f32,
}

impl ChunkReadWrite for ParticleActionBounce {
  /// Read particle_action bounce.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    Ok(Self {
      action_flags: reader.read_u32::<T>()?,
      action_type: reader.read_xr::<T, _>()?,
      position: reader.read_xr::<T, _>()?,
      one_minus_friction: reader.read_f32::<T>()?,
      resilience: reader.read_f32::<T>()?,
      cutoff_sqr: reader.read_f32::<T>()?,
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_u32::<T>(self.action_flags)?;
    writer.write_xr::<T, _>(&self.action_type)?;
    writer.write_xr::<T, _>(&self.position)?;
    writer.write_f32::<T>(self.one_minus_friction)?;
    writer.write_f32::<T>(self.resilience)?;
    writer.write_f32::<T>(self.cutoff_sqr)?;

    Ok(())
  }
}

impl LtxImportExport for ParticleActionBounce {
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
      position: read_ltx_field("position", section)?,
      one_minus_friction: read_ltx_field("one_minus_friction", section)?,
      resilience: read_ltx_field("resilience", section)?,
      cutoff_sqr: read_ltx_field("cutoff_sqr", section)?,
    })
  }

  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    ltx
      .with_section(section_name)
      .set("action_flags", self.action_flags.to_string())
      .set("action_type", self.action_type.to_string())
      .set("position", self.position.to_string())
      .set("one_minus_friction", self.one_minus_friction.to_string())
      .set("resilience", self.resilience.to_string())
      .set("cutoff_sqr", self.cutoff_sqr.to_string());

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::particles::actions::particle_action_bounce::ParticleActionBounce;
  use crate::data::particles::particle_action_type::ParticleActionType;
  use crate::data::particles::particle_domain::ParticleDomain;
  use crate::export::LtxImportExport;
  use serde_json::json;
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

    let original: ParticleActionBounce = ParticleActionBounce {
      action_flags: 1,
      action_type: ParticleActionType::Bounce,
      position: ParticleDomain::new_mock(),
      one_minus_friction: 61.2,
      resilience: 1.0,
      cutoff_sqr: 4.35,
    };

    original.write::<XRayByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 88);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 88);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 88 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      ParticleActionBounce::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_import_export() -> XRayResult {
    let ltx_filename: String = get_relative_test_sample_file_path(file!(), "import_export.ltx");
    let mut ltx: Ltx = Ltx::new();

    let original: ParticleActionBounce = ParticleActionBounce {
      action_flags: 1,
      action_type: ParticleActionType::Bounce,
      position: ParticleDomain::new_mock(),
      one_minus_friction: 30.2,
      resilience: 1.0,
      cutoff_sqr: 0.290,
    };

    original.export("data", &mut ltx)?;

    ltx.write_to(&mut overwrite_test_relative_resource_as_file(
      &ltx_filename,
    )?)?;

    let source: Ltx = Ltx::read_from_path(get_absolute_test_resource_path(&ltx_filename))?;

    assert_eq!(ParticleActionBounce::import("data", &source)?, original);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize() -> XRayResult {
    let original: ParticleActionBounce = ParticleActionBounce {
      action_flags: 1,
      action_type: ParticleActionType::Bounce,
      position: ParticleDomain::new_mock(),
      one_minus_friction: 24.30,
      resilience: 1.1,
      cutoff_sqr: 0.453,
    };

    let mut file: File = overwrite_test_relative_resource_as_file(
      &get_relative_test_sample_file_path(file!(), "serialize_deserialize.json"),
    )?;

    file.write_all(json!(original).to_string().as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);
    assert_eq!(
      original,
      serde_json::from_str::<ParticleActionBounce>(&serialized).unwrap()
    );

    Ok(())
  }
}
