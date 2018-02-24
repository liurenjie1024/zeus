use std::path::Path;

use util::error::Result;
use util::error::Error;
use std::io::ErrorKind;

pub struct NativeFileSegment {
    path: String
}

impl NativeFileSegment {
    pub fn validate(&self) -> Result<()> {
        let path = Path::new(self.path);

        if path.exists() && path.is_file() {
            Ok(())
        } else {
            let msg = format!("{} not found", self.path);
            std::io::Error::new(ErrorKind::NotFound, msg)?
        }
    }
}