package io.github.zeus.expr;

import io.github.zeus.rpc.ColumnType;
import io.github.zeus.rpc.ScalarFuncId;

public class ZeusFunctionEntry {
  private final ZeusFunctionSignature signature;
  private final ScalarFuncId funcId;

  public ZeusFunctionEntry(ZeusFunctionSignature signature, ScalarFuncId funcId) {
    this.signature = signature;
    this.funcId = funcId;
  }

  public ZeusFunctionSignature getSignature() {
    return signature;
  }

  public ScalarFuncId getFuncId() {
    return funcId;
  }

  public static ZeusFunctionEntry from(ScalarFuncId funcId, String name, ColumnType... argTypes) {
    return new ZeusFunctionEntry(ZeusFunctionSignature.from(name, argTypes), funcId);
  }
}
