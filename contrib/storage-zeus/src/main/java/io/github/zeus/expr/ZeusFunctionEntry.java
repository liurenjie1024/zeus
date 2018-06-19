/**
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

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
