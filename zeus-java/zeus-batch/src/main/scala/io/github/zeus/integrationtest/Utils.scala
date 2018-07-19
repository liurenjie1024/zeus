package io.github.zeus.integrationtest

import org.apache.hadoop.fs.{FileSystem, Path}

object Utils {
  def isEmptyDir(path: Path, fs: FileSystem): Boolean = {
    if (fs.exists(path)) {
      if (fs.isDirectory(path) && fs.listStatus(path).length == 0) {
        true
      } else{
        false
      }
    } else {
      true
    }
  }
}
