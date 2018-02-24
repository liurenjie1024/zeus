use std::path::Path;
use std::io::ErrorKind;
use std::io::Error as StdIoError;

use util::error::Result;
use util::error::Error;

pub struct NativeFileSegment {
    pub path: String
}

impl NativeFileSegment {
    pub fn validate(&self) -> Result<()> {
        let path = Path::new(&self.path);

        if path.exists() && path.is_file() {
            Ok(())
        } else {
            let msg = format!("{} not found", self.path);
            Err(Error::IoError(StdIoError::new(ErrorKind::NotFound, msg)))
        }
    }
}