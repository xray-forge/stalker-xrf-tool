use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
use crate::data::meta::alife_object_generic::AlifeObjectWriter;
use crate::data::meta::alife_object_reader::AlifeObjectReader;
use crate::error::DatabaseError;
use crate::export::file_import::read_ltx_field;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeObjectItem {
  pub base: AlifeObjectDynamicVisual,
  pub condition: f32,
  pub upgrades_count: u32,
}

impl AlifeObjectReader for AlifeObjectItem {
  /// Read alife item object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    let object: Self = Self {
      base: AlifeObjectDynamicVisual::read::<T>(reader)?,
      condition: reader.read_f32::<XRayByteOrder>()?,
      upgrades_count: reader.read_u32::<XRayByteOrder>()?,
    };

    assert_eq!(
      object.upgrades_count, 0,
      "Unexpected upgraded item provided"
    );

    Ok(object)
  }

  /// Import alife item object data from ltx config section.
  fn import(section_name: &str, ltx: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      DatabaseError::new_parse_error(format!(
        "ALife object '{section_name}' should be defined in ltx file ({})",
        file!()
      ))
    })?;

    Ok(Self {
      base: AlifeObjectDynamicVisual::import(section_name, ltx)?,
      condition: read_ltx_field("condition", section)?,
      upgrades_count: read_ltx_field("upgrades_count", section)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectWriter for AlifeObjectItem {
  /// Write item data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    self.base.write(writer)?;

    writer.write_f32::<XRayByteOrder>(self.condition)?;
    writer.write_u32::<XRayByteOrder>(self.upgrades_count)?;

    Ok(())
  }

  /// Export object data into ltx file.
  fn export(&self, section_name: &str, ltx: &mut Ltx) -> DatabaseResult {
    self.base.export(section_name, ltx)?;

    ltx
      .with_section(section_name)
      .set("condition", self.condition.to_string())
      .set("is_closed", self.upgrades_count.to_string())
      .set("upgrades_count", self.upgrades_count.to_string());

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
  use crate::data::alife::alife_object_item::AlifeObjectItem;
  use crate::data::meta::alife_object_generic::AlifeObjectWriter;
  use crate::data::meta::alife_object_reader::AlifeObjectReader;
  use crate::types::DatabaseResult;
  use fileslice::FileSlice;
  use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write() -> DatabaseResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: AlifeObjectItem = AlifeObjectItem {
      base: AlifeObjectDynamicVisual {
        base: AlifeObjectAbstract {
          game_vertex_id: 1002,
          distance: 65.25,
          direct_control: 41262,
          level_vertex_id: 618923,
          flags: 32,
          custom_data: String::from("custom_data"),
          story_id: 500,
          spawn_story_id: 35,
        },
        visual_name: String::from("abcd"),
        visual_flags: 33,
      },
      condition: 0.5,
      upgrades_count: 0,
    };

    original.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 52);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 52);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 52 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      AlifeObjectItem::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }
}
