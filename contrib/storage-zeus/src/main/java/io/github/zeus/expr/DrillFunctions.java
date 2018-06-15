package io.github.zeus.expr;

import com.google.common.collect.ImmutableMap;
import io.github.zeus.rpc.ScalarFuncId;
import org.apache.drill.common.types.TypeProtos.MajorType;
import org.apache.drill.common.types.TypeProtos.MinorType;
import org.apache.drill.common.types.Types;

import java.util.Optional;
import java.util.stream.Collectors;
import java.util.stream.Stream;

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
          .put(signatureOf("greater_than", MinorType.BIT, MinorType.BIT), GT_BOOL)
          .put(signatureOf("greater_than", MinorType.INT, MinorType.INT), GT_I32)
          .put(signatureOf("greater_than", MinorType.BIGINT, MinorType.BIGINT), GT_I64)
          .put(signatureOf("greater_than", MinorType.FLOAT4, MinorType.FLOAT4), GT_F4)
          .put(signatureOf("greater_than", MinorType.FLOAT8, MinorType.FLOAT8), GT_F8)
          .put(signatureOf("greater_than", MinorType.VARCHAR, MinorType.VARCHAR), GT_STR)
          .put(signatureOf("greater_than_or_equal_to", MinorType.BIT, MinorType.BIT), GE_BOOL)
          .put(signatureOf("greater_than_or_equal_to", MinorType.INT, MinorType.INT), GE_I32)
          .put(signatureOf("greater_than_or_equal_to", MinorType.BIGINT, MinorType.BIGINT), GE_I64)
          .put(signatureOf("greater_than_or_equal_to", MinorType.FLOAT4, MinorType.FLOAT4), GE_F4)
          .put(signatureOf("greater_than_or_equal_to", MinorType.FLOAT8, MinorType.FLOAT8), GE_F8)
          .put(signatureOf("greater_than_or_equal_to", MinorType.VARCHAR, MinorType.VARCHAR), GE_STR)
          .put(signatureOf("less_than", MinorType.BIT, MinorType.BIT), LT_BOOL)
          .put(signatureOf("less_than", MinorType.INT, MinorType.INT), LT_I32)
          .put(signatureOf("less_than", MinorType.BIGINT, MinorType.BIGINT), LT_I64)
          .put(signatureOf("less_than", MinorType.FLOAT4, MinorType.FLOAT4), LT_F4)
          .put(signatureOf("less_than", MinorType.FLOAT8, MinorType.FLOAT8), LT_F8)
          .put(signatureOf("less_than", MinorType.VARCHAR, MinorType.VARCHAR), LT_STR)
          .put(signatureOf("less_than_or_equal_to", MinorType.BIT, MinorType.BIT), LE_BOOL)
          .put(signatureOf("less_than_or_equal_to", MinorType.INT, MinorType.INT), LE_I32)
          .put(signatureOf("less_than_or_equal_to", MinorType.BIGINT, MinorType.BIGINT), LE_I64)
          .put(signatureOf("less_than_or_equal_to", MinorType.FLOAT4, MinorType.FLOAT4), LE_F4)
          .put(signatureOf("less_than_or_equal_to", MinorType.FLOAT8, MinorType.FLOAT8), LE_F8)
          .put(signatureOf("less_than_or_equal_to", MinorType.VARCHAR, MinorType.VARCHAR), LE_STR)
          .put(signatureOf("equal", MinorType.BIT, MinorType.BIT), EQ_BOOL)
          .put(signatureOf("equal", MinorType.INT, MinorType.INT), EQ_I32)
          .put(signatureOf("equal", MinorType.BIGINT, MinorType.BIGINT), EQ_I64)
          .put(signatureOf("equal", MinorType.FLOAT4, MinorType.FLOAT4), EQ_F4)
          .put(signatureOf("equal", MinorType.FLOAT8, MinorType.FLOAT8), EQ_F8)
          .put(signatureOf("equal", MinorType.VARCHAR, MinorType.VARCHAR), EQ_STR)
          .put(signatureOf("not_equal", MinorType.BIT, MinorType.BIT), NE_BOOL)
          .put(signatureOf("not_equal", MinorType.INT, MinorType.INT), NE_I32)
          .put(signatureOf("not_equal", MinorType.BIGINT, MinorType.BIGINT), NE_I64)
          .put(signatureOf("not_equal", MinorType.FLOAT4, MinorType.FLOAT4), NE_F4)
          .put(signatureOf("not_equal", MinorType.FLOAT8, MinorType.FLOAT8), NE_F8)
          .put(signatureOf("not_equal", MinorType.VARCHAR, MinorType.VARCHAR), NE_STR)
          .build();

  private static DrillFunctionSignature signatureOf(String name, MinorType... args) {
    MajorType[] argTypes = Stream.of(args)
        .map(Types::required)
        .toArray(MajorType[]::new);
    return new DrillFunctionSignature(name, argTypes);
  }

  public static Optional<ScalarFuncId> zeusScalarFuncOf(DrillFunctionSignature signature) {
    return Optional.ofNullable(DRILL_FUNCTIONS.get(signature));
  }
}
