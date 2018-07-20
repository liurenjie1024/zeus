package io.github.zeus.utils

object AutoManageResources {
  implicit class ImplicitManagedResource[A <: AutoCloseable](resource: => A)
    extends ManagedResource(resource)
}

class ManagedResource[A <: AutoCloseable](r: => A) {
  def map[B](f: (A) => B): B = {
    val resource = r
    var currentException: Throwable = null
    try {
      f(resource)
    } catch {
      case e: Throwable =>
        currentException = e
        throw e
    } finally {
      if (resource != null) {
        if (currentException != null) {
          try {
            resource.close()
          } catch {
            case e: Throwable =>
              currentException.addSuppressed(e)
          }
        } else {
          resource.close()
        }
      }
    }
  }
  def flatMap[B](f: (A) => B): B = map(f)
  def foreach[U](f: (A) => U): Unit = map(f)
}

