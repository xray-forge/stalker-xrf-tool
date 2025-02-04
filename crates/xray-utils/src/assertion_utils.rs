use std::fmt::Debug;
use xray_error::{XRayError, XRayResult};

/// Assert condition.
#[inline(always)]
pub fn assert(condition: bool, message: &str) -> XRayResult {
  if condition {
    Ok(())
  } else {
    Err(XRayError::new_assertion_error(message))
  }
}

/// Assert data values are equal.
#[inline(always)]
pub fn assert_equal<T: PartialEq + Debug>(first: T, second: T, message: &str) -> XRayResult {
  if first == second {
    Ok(())
  } else {
    Err(XRayError::new_assertion_error(format!(
      "Expected values to be equal, left - {:?}, right - {:?}. {}",
      first, second, message
    )))
  }
}

/// Assert data values are not equal.
#[inline(always)]
pub fn assert_not_equal<T: PartialEq + Debug>(first: T, second: T, message: &str) -> XRayResult {
  if first != second {
    Ok(())
  } else {
    Err(XRayError::new_assertion_error(format!(
      "Expected values not to be equal, left - {:?}, right - {:?}. {}",
      first, second, message
    )))
  }
}
