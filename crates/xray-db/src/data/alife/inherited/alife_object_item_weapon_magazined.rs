use crate::data::alife::inherited::alife_object_item_weapon::AlifeObjectItemWeapon;
use crate::export::LtxImportExport;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::XRayResult;
use xray_ltx::Ltx;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeObjectItemWeaponMagazined {
  pub base: AlifeObjectItemWeapon,
}

impl ChunkReadWrite for AlifeObjectItemWeaponMagazined {
  /// Read magazined weapon data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    Ok(Self {
      base: reader.read_xr::<T, _>()?,
    })
  }

  /// Write magazined weapon item into the writer.
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_xr::<T, _>(&self.base)?;

    Ok(())
  }
}

impl LtxImportExport for AlifeObjectItemWeaponMagazined {
  /// Import ALife item object data from ltx config section.
  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    Ok(Self {
      base: AlifeObjectItemWeapon::import(section_name, ltx)?,
    })
  }

  /// Export object data into ltx file.
  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    self.base.export(section_name, ltx)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::alife::inherited::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::inherited::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
  use crate::data::alife::inherited::alife_object_item::AlifeObjectItem;
  use crate::data::alife::inherited::alife_object_item_weapon::AlifeObjectItemWeapon;
  use crate::data::alife::inherited::alife_object_item_weapon_magazined::AlifeObjectItemWeaponMagazined;
  use crate::data::alife::inherited::alife_object_item_weapon_magazined_wgl::AlifeObjectItemWeaponMagazinedWgl;
  use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter, XRayByteOrder};
  use xray_error::XRayResult;
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };
  use xray_test_utils::FileSlice;

  #[test]
  fn test_read_write() -> XRayResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: AlifeObjectItemWeaponMagazinedWgl = AlifeObjectItemWeaponMagazinedWgl {
      base: AlifeObjectItemWeaponMagazined {
        base: AlifeObjectItemWeapon {
          base: AlifeObjectItem {
            base: AlifeObjectDynamicVisual {
              base: AlifeObjectAbstract {
                game_vertex_id: 5232,
                distance: 53.1213,
                direct_control: 67,
                level_vertex_id: 25313,
                flags: 32,
                custom_data: String::from("custom-data"),
                story_id: 3486,
                spawn_story_id: 37663,
              },
              visual_name: String::from("visual-name"),
              visual_flags: 33,
            },
            condition: 1.0,
            upgrades_count: 0,
          },
          ammo_current: 20,
          ammo_elapsed: 10,
          weapon_state: 1,
          addon_flags: 1,
          ammo_type: 1,
          elapsed_grenades: 0,
        },
      },
    };

    original.write::<XRayByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 67);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 67);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 67 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      AlifeObjectItemWeaponMagazinedWgl::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }
}
