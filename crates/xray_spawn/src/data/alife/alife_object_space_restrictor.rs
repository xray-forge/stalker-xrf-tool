use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
use crate::data::alife_object_base::{AlifeObjectGeneric, AlifeObjectInheritedReader};
use crate::data::shape::Shape;
use crate::types::SpawnByteOrder;
use byteorder::ReadBytesExt;

pub struct AlifeObjectSpaceRestrictor {
  pub base: AlifeObjectAbstract,
  pub shape: Vec<Shape>,
  pub restrictor_type: u8,
}

impl AlifeObjectInheritedReader<AlifeObjectSpaceRestrictor> for AlifeObjectSpaceRestrictor {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectSpaceRestrictor {
    let base: AlifeObjectAbstract = AlifeObjectAbstract::from_chunk(chunk);

    let shape: Vec<Shape> = chunk.read_shape_description::<SpawnByteOrder>().unwrap();
    let restrictor_type: u8 = chunk.read_u8().unwrap();

    AlifeObjectSpaceRestrictor {
      base,
      shape,
      restrictor_type,
    }
  }
}

impl AlifeObjectGeneric for AlifeObjectSpaceRestrictor {}
