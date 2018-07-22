package io.github.zeus.utils.template

class TemplateEngine {
  val regex = raw"\{\{(\w+)\}\}".r

  def transform(input: String, env: Map[String, String]): String = {
    regex.replaceAllIn(input, m => {
      val key = m.group(1).trim
      env.get(key) match {
        case Some(v) => v
        case None => throw new NoSuchElementException(s"$key not found in environment.")
      }
    })
  }
}
