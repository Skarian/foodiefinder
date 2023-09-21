use crate::error::DBError;
use serde_json::{from_str, to_string};

pub fn vec_to_string(vec: &Option<Vec<String>>) -> Result<Option<String>, DBError> {
    match vec {
        Some(vec) => match to_string(&vec) {
            Ok(result) => Ok(Some(result)),
            Err(e) => Err(DBError::SerdeJson(e)),
        },
        None => Ok(None),
    }
}

pub fn string_to_vec(string: &Option<String>) -> Result<Option<Vec<String>>, DBError> {
    match string {
        Some(string) => match from_str(string) {
            Ok(result) => Ok(Some(result)),
            Err(e) => Err(DBError::SerdeJson(e)),
        },
        None => Ok(None),
    }
}
