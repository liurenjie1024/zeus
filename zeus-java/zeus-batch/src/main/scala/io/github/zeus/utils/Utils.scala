package io.github.zeus.utils

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

  def withResource[A <: AutoCloseable, R](resource: A)(f: A => R): R = {
    require(resource != null, "resource can't be null")

    var exception: Throwable = _
    try {
      f(resource)
    } catch {
      case t: Throwable =>
        exception = t
        throw exception
    } finally {
      try {
        resource.close()
      } catch {
        case t: Throwable =>
          if (exception != null) {
            exception.addSuppressed(t)
          } else {
            throw t
          }
      }
    }
  }
}
