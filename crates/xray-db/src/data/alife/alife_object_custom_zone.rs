use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct AlifeObjectCustomZone {
  pub base: AlifeObjectSpaceRestrictor,
  pub max_power: f32,
  pub owner_id: u32,
  pub enabled_time: u32,
  pub disabled_time: u32,
  pub start_time_shift: u32,
}

impl AlifeObjectInheritedReader<AlifeObjectCustomZone> for AlifeObjectCustomZone {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectCustomZone> {
    let base: AlifeObjectSpaceRestrictor = AlifeObjectSpaceRestrictor::read_from_chunk::<T>(chunk)?;

    let max_power: f32 = chunk.read_f32::<SpawnByteOrder>()?;
    let owner_id: u32 = chunk.read_u32::<SpawnByteOrder>()?;
    let enabled_time: u32 = chunk.read_u32::<SpawnByteOrder>()?;
    let disabled_time: u32 = chunk.read_u32::<SpawnByteOrder>()?;
    let start_time_shift: u32 = chunk.read_u32::<SpawnByteOrder>()?;

    Ok(AlifeObjectCustomZone {
      base,
      max_power,
      owner_id,
      enabled_time,
      disabled_time,
      start_time_shift,
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write::<T>(writer)?;

    writer.write_f32::<T>(self.max_power)?;
    writer.write_u32::<T>(self.owner_id)?;
    writer.write_u32::<T>(self.enabled_time)?;
    writer.write_u32::<T>(self.disabled_time)?;
    writer.write_u32::<T>(self.start_time_shift)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_custom_zone::AlifeObjectCustomZone;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
  use crate::data::shape::Shape;
  use crate::test::utils::{
    get_test_chunk_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String =
      get_test_chunk_file_sub_dir(file!(), &String::from("alife_object_custom_zone.chunk"));

    let object: AlifeObjectCustomZone = AlifeObjectCustomZone {
      base: AlifeObjectSpaceRestrictor {
        base: AlifeObjectAbstract {
          game_vertex_id: 42343,
          distance: 255.4,
          direct_control: 3,
          level_vertex_id: 1003,
          flags: 32,
          custom_data: String::from("custom-data"),
          story_id: 441,
          spawn_story_id: 254,
        },
        shape: vec![
          Shape::Sphere(((2.5, 3.5, 1.5), 1.0)),
          Shape::Box((
            (1.1, 1.1, 3.1),
            (1.1, 2.2, 3.3),
            (4.0, 5.0, 6.4),
            (9.2, 8.3, 7.0),
          )),
        ],
        restrictor_type: 3,
      },
      max_power: 2.0,
      owner_id: 553,
      enabled_time: 100,
      disabled_time: 220,
      start_time_shift: 300,
    };

    object.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 126);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 126);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 126 + 8);

    let mut chunk: Chunk = Chunk::from_file(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectCustomZone =
      AlifeObjectCustomZone::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}