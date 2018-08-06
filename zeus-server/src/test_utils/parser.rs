use std::process::Command;
use std::fs::File;
use std::ffi::OsString;
use std::ffi::OsStr;
use std::env;

use protobuf::Message;
use protobuf::MessageStatic;
use protobuf::parse_from_reader;
use util::errors::*;

/// This class should only be used for tests
pub struct ProtobufJsonParser {
  jar_path: OsString
}

impl ProtobufJsonParser {
  pub fn new<I: Into<OsString>>(jar_path: I) -> ProtobufJsonParser {
    ProtobufJsonParser {
      jar_path: jar_path.into()
    }
  }

  pub fn parse<P, T, C>(&self, input: P, class_name: C) -> Result<T>
    where P: AsRef<OsStr>,
          T: Message + MessageStatic,
          C: AsRef<OsStr>
  {
    let mut temp_output_file = env::temp_dir();
    temp_output_file.push("zeus_tmp");


    let status =  Command::new("java")
      .arg("-cp").arg(&self.jar_path)
      .arg("ProtobufJsonParser")
      .arg("-i").arg(input)
      .arg("-o").arg(temp_output_file.as_os_str())
      .arg("-c").arg(class_name)
      .status()?;

    if status.success() {
      let mut f = File::open(&temp_output_file)?;
      Ok(parse_from_reader::<T>(&mut f)?)
    } else {
      bail!("Failed to parse json.")
    }
  }
}