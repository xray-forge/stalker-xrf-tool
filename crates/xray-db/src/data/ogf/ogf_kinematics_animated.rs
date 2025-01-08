use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::DatabaseResult;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfKinematicsAnimated {}

impl OgfKinematicsAnimated {
  pub fn read<T: ByteOrder>(_: &mut ChunkReader) -> DatabaseResult<Self> {
    todo!("Implement")
  }

  pub fn write<T: ByteOrder>(&self, _: &mut ChunkWriter) -> DatabaseResult {
    todo!("Implement")
  }
}

/*
sub read_kinematics_animated {
  my $self = shift;
  my ($cf) = @_;
  $self->read_kinematics($cf);
  if ($self->{ogf_version} == 4 && $cf->find_chunk($chunk_names{$self->{ogf_version}}{'OGF_S_MOTION_REFS_1'})) {
    $self->read_smotion_refs_1($cf);
    $cf->close_found_chunk();
    return;
  } elsif ($cf->find_chunk($chunk_names{$self->{ogf_version}}{'OGF_S_MOTION_REFS_0'})) {
    $self->read_smotion_refs_0($cf);
    $cf->close_found_chunk();
    return;
  } elsif ($cf->find_chunk($chunk_names{$self->{ogf_version}}{'OGF_S_SMPARAMS_1'})) {
    $self->read_s_smparams($cf, 1);
    $cf->close_found_chunk();
  } elsif ($cf->find_chunk($chunk_names{$self->{ogf_version}}{'OGF_S_SMPARAMS_0'})) {
    $self->read_s_smparams($cf, 0);
    $cf->close_found_chunk();
  }
  if ($cf->find_chunk($chunk_names{$self->{ogf_version}}{'OGF_S_MOTIONS_1'})) {
    $self->read_smotions($cf, 1);
    $cf->close_found_chunk();
  } elsif ($cf->find_chunk($chunk_names{$self->{ogf_version}}{'OGF_S_MOTIONS_0'})) {
    $self->read_smotions($cf, 0);
    $cf->close_found_chunk();
  } else {
    fail('Invalid visual, no motions');
  }
}
 */
