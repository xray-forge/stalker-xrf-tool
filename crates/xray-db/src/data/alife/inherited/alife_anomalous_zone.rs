use crate::data::alife::inherited::alife_object_anomaly_zone::AlifeObjectAnomalyZone;
use crate::data::generic::time::Time;
use crate::export::LtxImportExport;
use crate::file_import::read_ltx_field;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeAnomalousZone {
  pub base: AlifeObjectAnomalyZone,
  pub last_spawn_time: Option<Time>,
}

impl ChunkReadWrite for AlifeAnomalousZone {
  /// Read anomalous zone object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    Ok(Self {
      base: reader.read_xr::<T, _>()?,
      last_spawn_time: reader.read_xr_optional::<T, Time>()?,
    })
  }

  /// Write ALife anomalous zone data to the writer.
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_xr::<T, _>(&self.base)?;
    writer.write_xr_optional::<T, _>(self.last_spawn_time.as_ref())?;

    Ok(())
  }
}

impl LtxImportExport for AlifeAnomalousZone {
  /// Import anomalous zone object data from ltx config section.
  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "ALife object '{}' should be defined in ltx file ({})",
        section_name,
        file!()
      ))
    })?;

    Ok(Self {
      base: AlifeObjectAnomalyZone::import(section_name, ltx)?,
      last_spawn_time: Time::from_str_optional(&read_ltx_field::<String>(
        "anomalous_zone.last_spawn_time",
        section,
      )?)?,
    })
  }

  /// Export object data into ltx file.
  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    self.base.export(section_name, ltx)?;

    ltx.with_section(section_name).set(
      "anomalous_zone.last_spawn_time",
      Time::export_to_string(self.last_spawn_time.as_ref()),
    );

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::alife::inherited::alife_anomalous_zone::AlifeAnomalousZone;
  use crate::data::alife::inherited::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::inherited::alife_object_anomaly_zone::AlifeObjectAnomalyZone;
  use crate::data::alife::inherited::alife_object_custom_zone::AlifeObjectCustomZone;
  use crate::data::alife::inherited::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
  use crate::data::generic::shape::Shape;
  use crate::data::generic::time::Time;
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

    let original: AlifeAnomalousZone = AlifeAnomalousZone {
      base: AlifeObjectAnomalyZone {
        base: AlifeObjectCustomZone {
          base: AlifeObjectSpaceRestrictor {
            base: AlifeObjectAbstract {
              game_vertex_id: 34565,
              distance: 234.0,
              direct_control: 2346,
              level_vertex_id: 7357,
              flags: 55,
              custom_data: String::from("custom-data"),
              story_id: 8567,
              spawn_story_id: 7685,
            },
            shape: vec![
              Shape::Sphere((Vector3d::new(2.5, 5.1, 1.5), 1.0)),
              Shape::Box((
                Vector3d::new(4.1, 1.1, 3.1),
                Vector3d::new(1.1, 3.2, 3.3),
                Vector3d::new(4.0, 5.0, 6.4),
                Vector3d::new(9.2, 8.3, 3.0),
              )),
            ],
            restrictor_type: 4,
          },
          max_power: 1.0,
          owner_id: 64,
          enabled_time: 235,
          disabled_time: 3457,
          start_time_shift: 253,
        },
        offline_interactive_radius: 330.0,
        artefact_spawn_count: 4,
        artefact_position_offset: 12,
      },
      last_spawn_time: Some(Time {
        year: 22,
        month: 10,
        day: 24,
        hour: 20,
        minute: 30,
        second: 50,
        millis: 250,
      }),
    };

    original.write::<XRayByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 145);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 145);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 145 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      AlifeAnomalousZone::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_import_export() -> XRayResult {
    let ltx_filename: String = get_relative_test_sample_file_path(file!(), "import_export.ltx");
    let mut ltx: Ltx = Ltx::new();

    let original: AlifeAnomalousZone = AlifeAnomalousZone {
      base: AlifeObjectAnomalyZone {
        base: AlifeObjectCustomZone {
          base: AlifeObjectSpaceRestrictor {
            base: AlifeObjectAbstract {
              game_vertex_id: 25,
              distance: 234.0,
              direct_control: 4,
              level_vertex_id: 7357,
              flags: 66,
              custom_data: String::from("custom-data"),
              story_id: 25,
              spawn_story_id: 7685,
            },
            shape: vec![
              Shape::Sphere((Vector3d::new(2.5, 5.1, 1.5), 1.0)),
              Shape::Box((
                Vector3d::new(6.1, 1.1, 3.1),
                Vector3d::new(1.1, 3.2, 3.3),
                Vector3d::new(4.0, 7.0, 6.4),
                Vector3d::new(69.2, 28.3, 33.0),
              )),
            ],
            restrictor_type: 4,
          },
          max_power: 1.0,
          owner_id: 64,
          enabled_time: 235,
          disabled_time: 3457,
          start_time_shift: 253,
        },
        offline_interactive_radius: 330.0,
        artefact_spawn_count: 4,
        artefact_position_offset: 12,
      },
      last_spawn_time: Some(Time {
        year: 22,
        month: 10,
        day: 25,
        hour: 20,
        minute: 30,
        second: 45,
        millis: 310,
      }),
    };

    original.export("data", &mut ltx)?;

    ltx.write_to(&mut overwrite_test_relative_resource_as_file(
      &ltx_filename,
    )?)?;

    let source: Ltx = Ltx::read_from_path(get_absolute_test_resource_path(&ltx_filename))?;

    assert_eq!(AlifeAnomalousZone::import("data", &source)?, original);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize() -> XRayResult {
    let original: AlifeAnomalousZone = AlifeAnomalousZone {
      base: AlifeObjectAnomalyZone {
        base: AlifeObjectCustomZone {
          base: AlifeObjectSpaceRestrictor {
            base: AlifeObjectAbstract {
              game_vertex_id: 3,
              distance: 25.0,
              direct_control: 2346,
              level_vertex_id: 7357,
              flags: 55,
              custom_data: String::from("custom-data"),
              story_id: 8567,
              spawn_story_id: 7685,
            },
            shape: vec![
              Shape::Sphere((Vector3d::new(2.5, 5.1, 1.5), 1.0)),
              Shape::Box((
                Vector3d::new(4.1, 1.1, 3.1),
                Vector3d::new(2.1, 1.2, 2.3),
                Vector3d::new(4.0, 5.0, 6.4),
                Vector3d::new(3.2, 4.3, 5.0),
              )),
            ],
            restrictor_type: 4,
          },
          max_power: 1.0,
          owner_id: 25,
          enabled_time: 13,
          disabled_time: 12,
          start_time_shift: 15,
        },
        offline_interactive_radius: 330.0,
        artefact_spawn_count: 4,
        artefact_position_offset: 12,
      },
      last_spawn_time: Some(Time {
        year: 22,
        month: 10,
        day: 24,
        hour: 20,
        minute: 30,
        second: 50,
        millis: 250,
      }),
    };

    let mut file: File = overwrite_test_relative_resource_as_file(
      &get_relative_test_sample_file_path(file!(), "serialize_deserialize.json"),
    )?;

    file.write_all(to_string_pretty(&original)?.as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);

    assert_eq!(
      serde_json::from_str::<AlifeAnomalousZone>(&serialized)?,
      original
    );

    Ok(())
  }
}
