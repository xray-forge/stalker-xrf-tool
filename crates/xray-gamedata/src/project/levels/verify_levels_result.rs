use crate::project::gamedata_generic_result::GamedataGenericVerificationResult;

#[derive(Default)]
pub struct GamedataLevelVerificationResult {}

impl GamedataGenericVerificationResult for GamedataLevelVerificationResult {
  fn is_valid(&self) -> bool {
    true
  }

  fn get_failure_message(&self) -> String {
    String::from("todo;")
  }
}
