use crate::chunk::reader::ChunkReader;
use crate::data::particle::effect_action::particle_action_generic::ParticleActionGeneric;
use crate::data::particle::particle_domain::ParticleDomain;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionRandomVelocity {
  pub gen_vel: ParticleDomain,
}

impl ParticleActionRandomVelocity {
  /// Read effect_action random velocity.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<ParticleActionRandomVelocity> {
    Ok(ParticleActionRandomVelocity {
      gen_vel: ParticleDomain::read::<T>(reader)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionGeneric for ParticleActionRandomVelocity {}