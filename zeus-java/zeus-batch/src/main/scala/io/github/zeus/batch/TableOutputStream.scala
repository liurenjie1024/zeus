package io.github.zeus.batch

/**
  * Created by liurenjie on 22/03/2018.
  */
trait TableOutputStream extends AutoCloseable {
  def write(row: Row)
}
