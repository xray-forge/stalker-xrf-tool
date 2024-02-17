use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
use crate::data::alife_object::AlifeObjectInherited;

pub struct AlifeObjectDynamic {
  pub base: AlifeObjectAbstract,
}

impl AlifeObjectInherited<AlifeObjectDynamic> for AlifeObjectDynamic {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectDynamic {
    let base: AlifeObjectAbstract = AlifeObjectAbstract::from_chunk(chunk);

    AlifeObjectDynamic { base }
  }
}