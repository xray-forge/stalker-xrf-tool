use std::sync::{Arc, Mutex};
use xray_archive::ArchiveProject;

pub struct ArchivesEditorState {
  pub project: Arc<Mutex<Option<ArchiveProject>>>,
}
