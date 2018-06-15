package io.github.zeus.expr;



import org.apache.drill.common.types.TypeProtos.MajorType;

import java.util.Objects;

public class DrillFunctionSignature {
  private final String name;
  private final MajorType[] args;

  public DrillFunctionSignature(String name, MajorType[] args) {
    this.name = name;
    this.args = args;
  }

  @Override
  public boolean equals(Object o) {
    if (this == o) return true;
    if (o == null || getClass() != o.getClass()) return false;
    DrillFunctionSignature that = (DrillFunctionSignature) o;
    return Objects.equals(name, that.name) &&
        Objects.equals(args, that.args);
  }

  @Override
  public int hashCode() {
    return Objects.hash(name, args);
  }
}
