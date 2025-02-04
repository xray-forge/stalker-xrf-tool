use crate::data::alife::inherited::alife_object_shape::AlifeObjectShape;
use crate::export::LtxImportExport;
use crate::file_import::read_ltx_field;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeObjectClimable {
  pub base: AlifeObjectShape,
  pub game_material: String,
}

impl ChunkReadWrite for AlifeObjectClimable {
  /// Read climable object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    Ok(Self {
      base: reader.read_xr::<T, _>()?,
      game_material: reader.read_w1251_string()?,
    })
  }

  /// Write climable object data into the chunk.
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_xr::<T, _>(&self.base)?;
    writer.write_w1251_string(&self.game_material)?;

    Ok(())
  }
}

impl LtxImportExport for AlifeObjectClimable {
  /// Import climable object data from ltx config section.
  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "ALife object '{}' should be defined in ltx file ({})",
        section_name,
        file!()
      ))
    })?;

    Ok(Self {
      base: AlifeObjectShape::import(section_name, ltx)?,
      game_material: read_ltx_field("climbable.game_material", section)?,
    })
  }

  /// Export object data into ltx file.
  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    self.base.export(section_name, ltx)?;

    ltx
      .with_section(section_name)
      .set("climbable.game_material", &self.game_material);

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::alife::inherited::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::inherited::alife_object_climable::AlifeObjectClimable;
  use crate::data::alife::inherited::alife_object_shape::AlifeObjectShape;
  use crate::data::generic::shape::Shape;
  use crate::data::generic::vector_3d::Vector3d;
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

    let original: AlifeObjectClimable = AlifeObjectClimable {
      base: AlifeObjectShape {
        base: AlifeObjectAbstract {
          game_vertex_id: 4223,
          distance: 723.23,
          direct_control: 0,
          level_vertex_id: 0,
          flags: 0,
          custom_data: String::from("custom-data"),
          story_id: 0,
          spawn_story_id: 0,
        },
        shape: vec![
          Shape::Sphere((Vector3d::new(54.5, 0.5, 11.5), 1.0)),
          Shape::Box((
            Vector3d::new(51.5, 2.5, 73.1),
            Vector3d::new(55.1, 3.2, 2.3),
            Vector3d::new(51.0, 3.0, 6.4),
            Vector3d::new(59.2, 3.3, 3.0),
          )),
        ],
      },
      game_material: String::from("dest-material"),
    };

    original.write::<XRayByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 119);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 119);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 119 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      AlifeObjectClimable::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_import_export() -> XRayResult {
    let ltx_filename: String = get_relative_test_sample_file_path(file!(), "import_export.ltx");
    let mut ltx: Ltx = Ltx::new();

    let original: AlifeObjectClimable = AlifeObjectClimable {
      base: AlifeObjectShape {
        base: AlifeObjectAbstract {
          game_vertex_id: 364,
          distance: 64.23,
          direct_control: 20,
          level_vertex_id: 10,
          flags: 6,
          custom_data: String::from("custom-data"),
          story_id: 4,
          spawn_story_id: 53,
        },
        shape: vec![
          Shape::Sphere((Vector3d::new(54.5, 0.5, 11.5), 1.0)),
          Shape::Box((
            Vector3d::new(5.5, 6.5, 3.1),
            Vector3d::new(5.1, 3.2, 2.3),
            Vector3d::new(5.0, 6.0, 2.4),
            Vector3d::new(5.2, 3.3, 3.0),
          )),
        ],
      },
      game_material: String::from("dest-material"),
    };

    original.export("data", &mut ltx)?;

    ltx.write_to(&mut overwrite_test_relative_resource_as_file(
      &ltx_filename,
    )?)?;

    let source: Ltx = Ltx::read_from_path(get_absolute_test_resource_path(&ltx_filename))?;

    assert_eq!(AlifeObjectClimable::import("data", &source)?, original);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize() -> XRayResult {
    let original: AlifeObjectClimable = AlifeObjectClimable {
      base: AlifeObjectShape {
        base: AlifeObjectAbstract {
          game_vertex_id: 253,
          distance: 47.23,
          direct_control: 5,
          level_vertex_id: 50,
          flags: 64,
          custom_data: String::from("custom-data"),
          story_id: 75,
          spawn_story_id: 35,
        },
        shape: vec![
          Shape::Sphere((Vector3d::new(54.5, 0.5, 11.5), 1.0)),
          Shape::Box((
            Vector3d::new(7.5, 2.5, 4.1),
            Vector3d::new(7.1, 3.2, 4.3),
            Vector3d::new(7.0, 3.0, 4.4),
            Vector3d::new(7.2, 3.3, 4.0),
          )),
        ],
      },
      game_material: String::from("dest-material"),
    };

    let mut file: File = overwrite_test_relative_resource_as_file(
      &get_relative_test_sample_file_path(file!(), "serialize_deserialize.json"),
    )?;

    file.write_all(to_string_pretty(&original)?.as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);

    assert_eq!(
      serde_json::from_str::<AlifeObjectClimable>(&serialized)?,
      original
    );

    Ok(())
  }
}
