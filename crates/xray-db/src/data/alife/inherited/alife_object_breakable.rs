use crate::data::alife::inherited::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
use crate::export::LtxImportExport;
use crate::file_import::read_ltx_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeObjectBreakable {
  pub base: AlifeObjectDynamicVisual,
  pub health: f32,
}

impl ChunkReadWrite for AlifeObjectBreakable {
  /// Read ALife breakable object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    Ok(Self {
      base: reader.read_xr::<T, _>()?,
      health: reader.read_f32::<T>()?,
    })
  }

  /// Write ALife breakable object data into the writer.
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_xr::<T, _>(&self.base)?;
    writer.write_f32::<T>(self.health)?;

    Ok(())
  }
}

impl LtxImportExport for AlifeObjectBreakable {
  /// Import ALife breakable object data from ltx config section.
  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "ALife object '{}' should be defined in ltx file ({})",
        section_name,
        file!()
      ))
    })?;

    Ok(Self {
      base: AlifeObjectDynamicVisual::import(section_name, ltx)?,
      health: read_ltx_field("breakable.health", section)?,
    })
  }

  /// Export object data into ltx file.
  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    self.base.export(section_name, ltx)?;

    ltx
      .with_section(section_name)
      .set("breakable.health", self.health.to_string());

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::alife::inherited::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::inherited::alife_object_breakable::AlifeObjectBreakable;
  use crate::data::alife::inherited::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
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

    let original: AlifeObjectBreakable = AlifeObjectBreakable {
      base: AlifeObjectDynamicVisual {
        base: AlifeObjectAbstract {
          game_vertex_id: 3200,
          distance: 50.0,
          direct_control: 6219,
          level_vertex_id: 239805,
          flags: 562,
          custom_data: String::from("custom-data"),
          story_id: 92378,
          spawn_story_id: 235067,
        },
        visual_name: String::from("visual-name"),
        visual_flags: 237,
      },
      health: 1.0,
    };

    original.write::<XRayByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 55);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 55);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 55 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      AlifeObjectBreakable::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_import_export() -> XRayResult {
    let ltx_filename: String = get_relative_test_sample_file_path(file!(), "import_export.ltx");
    let mut ltx: Ltx = Ltx::new();

    let original: AlifeObjectBreakable = AlifeObjectBreakable {
      base: AlifeObjectDynamicVisual {
        base: AlifeObjectAbstract {
          game_vertex_id: 1,
          distance: 25.0,
          direct_control: 6321,
          level_vertex_id: 246567,
          flags: 67,
          custom_data: String::from("custom-data"),
          story_id: 154,
          spawn_story_id: 364,
        },
        visual_name: String::from("visual-name"),
        visual_flags: 12,
      },
      health: 0.5,
    };

    original.export("data", &mut ltx)?;

    ltx.write_to(&mut overwrite_test_relative_resource_as_file(
      &ltx_filename,
    )?)?;

    let source: Ltx = Ltx::read_from_path(get_absolute_test_resource_path(&ltx_filename))?;

    assert_eq!(AlifeObjectBreakable::import("data", &source)?, original);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize() -> XRayResult {
    let original: AlifeObjectBreakable = AlifeObjectBreakable {
      base: AlifeObjectDynamicVisual {
        base: AlifeObjectAbstract {
          game_vertex_id: 5,
          distance: 3.0,
          direct_control: 265,
          level_vertex_id: 37465,
          flags: 354,
          custom_data: String::from("custom-data"),
          story_id: 3645,
          spawn_story_id: 36445,
        },
        visual_name: String::from("visual-name"),
        visual_flags: 34,
      },
      health: 0.63,
    };

    let mut file: File = overwrite_test_relative_resource_as_file(
      &get_relative_test_sample_file_path(file!(), "serialize_deserialize.json"),
    )?;

    file.write_all(json!(original).to_string().as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);

    assert_eq!(
      serde_json::from_str::<AlifeObjectBreakable>(&serialized).unwrap(),
      original
    );

    Ok(())
  }
}
