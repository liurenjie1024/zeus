package io.github.zeus.batch

/**
  * Created by liurenjie on 22/03/2018.
  */
trait TableOutput extends AutoCloseable {
  def write(row: Row)
}
