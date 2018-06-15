
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

import com.google.common.collect.ImmutableMap;
import io.github.zeus.rpc.ColumnType;
import io.github.zeus.rpc.ScalarFuncId;

import java.util.Optional;

import static io.github.zeus.rpc.ColumnType.BOOL;
import static io.github.zeus.rpc.ColumnType.FLOAT4;
import static io.github.zeus.rpc.ColumnType.FLOAT8;
import static io.github.zeus.rpc.ColumnType.INT32;
import static io.github.zeus.rpc.ColumnType.INT64;
import static io.github.zeus.rpc.ColumnType.STRING;
import static io.github.zeus.rpc.ScalarFuncId.EQ_BOOL;
import static io.github.zeus.rpc.ScalarFuncId.EQ_F4;
import static io.github.zeus.rpc.ScalarFuncId.EQ_F8;
import static io.github.zeus.rpc.ScalarFuncId.EQ_I32;
import static io.github.zeus.rpc.ScalarFuncId.EQ_I64;
import static io.github.zeus.rpc.ScalarFuncId.EQ_STR;
import static io.github.zeus.rpc.ScalarFuncId.GE_BOOL;
import static io.github.zeus.rpc.ScalarFuncId.GE_F4;
import static io.github.zeus.rpc.ScalarFuncId.GE_F8;
import static io.github.zeus.rpc.ScalarFuncId.GE_I32;
import static io.github.zeus.rpc.ScalarFuncId.GE_I64;
import static io.github.zeus.rpc.ScalarFuncId.GE_STR;
import static io.github.zeus.rpc.ScalarFuncId.GT_BOOL;
import static io.github.zeus.rpc.ScalarFuncId.GT_F4;
import static io.github.zeus.rpc.ScalarFuncId.GT_F8;
import static io.github.zeus.rpc.ScalarFuncId.GT_I32;
import static io.github.zeus.rpc.ScalarFuncId.GT_I64;
import static io.github.zeus.rpc.ScalarFuncId.GT_STR;
import static io.github.zeus.rpc.ScalarFuncId.LE_BOOL;
import static io.github.zeus.rpc.ScalarFuncId.LE_F4;
import static io.github.zeus.rpc.ScalarFuncId.LE_F8;
import static io.github.zeus.rpc.ScalarFuncId.LE_I32;
import static io.github.zeus.rpc.ScalarFuncId.LE_I64;
import static io.github.zeus.rpc.ScalarFuncId.LE_STR;
import static io.github.zeus.rpc.ScalarFuncId.LT_BOOL;
import static io.github.zeus.rpc.ScalarFuncId.LT_F4;
import static io.github.zeus.rpc.ScalarFuncId.LT_F8;
import static io.github.zeus.rpc.ScalarFuncId.LT_I32;
import static io.github.zeus.rpc.ScalarFuncId.LT_I64;
import static io.github.zeus.rpc.ScalarFuncId.LT_STR;
import static io.github.zeus.rpc.ScalarFuncId.NE_BOOL;
import static io.github.zeus.rpc.ScalarFuncId.NE_F4;
import static io.github.zeus.rpc.ScalarFuncId.NE_F8;
import static io.github.zeus.rpc.ScalarFuncId.NE_I32;
import static io.github.zeus.rpc.ScalarFuncId.NE_I64;
import static io.github.zeus.rpc.ScalarFuncId.NE_STR;

public class DrillFunctions {
  private static final ImmutableMap<DrillFunctionSignature, ScalarFuncId> DRILL_FUNCTIONS =
      ImmutableMap.<DrillFunctionSignature, ScalarFuncId>builder()
          .put(signatureOf("greater_than", BOOL, BOOL), GT_BOOL)
          .put(signatureOf("greater_than", INT32, INT32), GT_I32)
          .put(signatureOf("greater_than", INT64, INT64), GT_I64)
          .put(signatureOf("greater_than", FLOAT4, FLOAT4), GT_F4)
          .put(signatureOf("greater_than", FLOAT8, FLOAT8), GT_F8)
          .put(signatureOf("greater_than", STRING, STRING), GT_STR)
          .put(signatureOf("greater_than_or_equal_to", BOOL, BOOL), GE_BOOL)
          .put(signatureOf("greater_than_or_equal_to", INT32, INT32), GE_I32)
          .put(signatureOf("greater_than_or_equal_to", INT64, INT64), GE_I64)
          .put(signatureOf("greater_than_or_equal_to", FLOAT4, FLOAT4), GE_F4)
          .put(signatureOf("greater_than_or_equal_to", FLOAT8, FLOAT8), GE_F8)
          .put(signatureOf("greater_than_or_equal_to", STRING, STRING), GE_STR)
          .put(signatureOf("less_than", BOOL, BOOL), LT_BOOL)
          .put(signatureOf("less_than", INT32, INT32), LT_I32)
          .put(signatureOf("less_than", INT64, INT64), LT_I64)
          .put(signatureOf("less_than", FLOAT4, FLOAT4), LT_F4)
          .put(signatureOf("less_than", FLOAT8, FLOAT8), LT_F8)
          .put(signatureOf("less_than", STRING, STRING), LT_STR)
          .put(signatureOf("less_than_or_equal_to", BOOL, BOOL), LE_BOOL)
          .put(signatureOf("less_than_or_equal_to", INT32, INT32), LE_I32)
          .put(signatureOf("less_than_or_equal_to", INT64, INT64), LE_I64)
          .put(signatureOf("less_than_or_equal_to", FLOAT4, FLOAT4), LE_F4)
          .put(signatureOf("less_than_or_equal_to", FLOAT8, FLOAT8), LE_F8)
          .put(signatureOf("less_than_or_equal_to", STRING, STRING), LE_STR)
          .put(signatureOf("equal", BOOL, BOOL), EQ_BOOL)
          .put(signatureOf("equal", INT32, INT32), EQ_I32)
          .put(signatureOf("equal", INT64, INT64), EQ_I64)
          .put(signatureOf("equal", FLOAT4, FLOAT4), EQ_F4)
          .put(signatureOf("equal", FLOAT8, FLOAT8), EQ_F8)
          .put(signatureOf("equal", STRING, STRING), EQ_STR)
          .put(signatureOf("not_equal", BOOL, BOOL), NE_BOOL)
          .put(signatureOf("not_equal", INT32, INT32), NE_I32)
          .put(signatureOf("not_equal", INT64, INT64), NE_I64)
          .put(signatureOf("not_equal", FLOAT4, FLOAT4), NE_F4)
          .put(signatureOf("not_equal", FLOAT8, FLOAT8), NE_F8)
          .put(signatureOf("not_equal", STRING, STRING), NE_STR)
          .build();

  private static DrillFunctionSignature signatureOf(String name, ColumnType... args) {
    return new DrillFunctionSignature(name, args);
  }

  public static Optional<ScalarFuncId> zeusScalarFuncOf(DrillFunctionSignature signature) {
    return Optional.ofNullable(DRILL_FUNCTIONS.get(signature));
  }

  public static void main(String[] args) {
    DrillFunctionSignature sig = new DrillFunctionSignature("less_than", new ColumnType[] {ColumnType.INT32, ColumnType.INT32});

    System.out.println(zeusScalarFuncOf(sig));

  }
}
