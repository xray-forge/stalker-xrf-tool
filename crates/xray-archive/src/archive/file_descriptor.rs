use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct ArchiveFileDescriptor {
  pub crc: u32,
  pub name: String,
  pub offset: u32,
  pub size_compressed: u32,
  pub size_real: u32,
}

#[derive(Clone)]
pub struct ArchiveFileReplicationDescriptor {
  pub crc: u32,
  pub source: PathBuf,
  pub destination: PathBuf,
  pub name: String,
  pub offset: u32,
  pub size_compressed: u32,
  pub size_real: u32,
}

impl ArchiveFileReplicationDescriptor {
  pub fn from_descriptor(
    descriptor: &ArchiveFileDescriptor,
    source: &Path,
    destination: &Path,
  ) -> ArchiveFileReplicationDescriptor {
    ArchiveFileReplicationDescriptor {
      crc: descriptor.crc,
      source: source.into(),
      destination: destination.into(),
      name: descriptor.name.clone(),
      offset: descriptor.offset,
      size_compressed: descriptor.size_compressed,
      size_real: descriptor.size_real,
    }
  }
}
