use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
use crate::data::alife_object_base::AlifeObjectInheritedReader;

pub struct AlifeSmartZone {
  pub base: AlifeObjectSpaceRestrictor,
}

impl AlifeObjectInheritedReader<AlifeSmartZone> for AlifeSmartZone {
  fn from_chunk(chunk: &mut Chunk) -> AlifeSmartZone {
    let base: AlifeObjectSpaceRestrictor = AlifeObjectSpaceRestrictor::from_chunk(chunk);

    AlifeSmartZone { base }
  }
}
