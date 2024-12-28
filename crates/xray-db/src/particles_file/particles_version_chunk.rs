use crate::chunk::reader::ChunkReader;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticlesVersionChunk {
  pub version: u16,
}

impl ParticlesVersionChunk {
  pub const CHUNK_ID: u32 = 1;

  /// Read version chunk by position descriptor.
  /// Parses binary data into version chunk representation object.
  pub fn read<T: ByteOrder>(mut reader: ChunkReader) -> io::Result<ParticlesVersionChunk> {
    let version_chunk: ParticlesVersionChunk = ParticlesVersionChunk {
      version: reader.read_u16::<T>()?,
    };

    log::info!("Parsed version chunk, {:?} bytes", reader.read_bytes_len());

    assert!(reader.is_ended(), "Expect version chunk to be ended");
    assert_eq!(version_chunk.version, 1, "Expect version chunk to be 1");

    Ok(version_chunk)
  }
}
