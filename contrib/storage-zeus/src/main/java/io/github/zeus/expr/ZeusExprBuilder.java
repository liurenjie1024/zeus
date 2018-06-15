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

import com.fasterxml.jackson.databind.ObjectMapper;
import com.google.common.collect.ImmutableList;
import io.github.zeus.client.exception.CatalogNotFoundException;
import io.github.zeus.rpc.ColumnRef;
import io.github.zeus.rpc.ColumnType;
import io.github.zeus.rpc.ColumnValue;
import io.github.zeus.rpc.Expression;
import io.github.zeus.rpc.ExpressionType;
import io.github.zeus.rpc.LiteralExpression;
import io.github.zeus.rpc.ScalarFuncId;
import io.github.zeus.rpc.ScalarFunction;
import io.github.zeus.schema.ZeusTable;
import org.apache.drill.common.expression.FunctionCall;
import org.apache.drill.common.expression.LogicalExpression;
import org.apache.drill.common.expression.SchemaPath;
import org.apache.drill.common.expression.ValueExpressions.BooleanExpression;
import org.apache.drill.common.expression.ValueExpressions.DoubleExpression;
import org.apache.drill.common.expression.ValueExpressions.FloatExpression;
import org.apache.drill.common.expression.ValueExpressions.IntExpression;
import org.apache.drill.common.expression.ValueExpressions.LongExpression;
import org.apache.drill.common.expression.ValueExpressions.QuotedString;
import org.apache.drill.common.expression.ValueExpressions.TimeStampExpression;
import org.apache.drill.common.expression.visitors.AbstractExprVisitor;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.io.StringWriter;
import java.util.Optional;

public class ZeusExprBuilder extends AbstractExprVisitor<Optional<Expression>, Void, RuntimeException> {
  private static final Logger LOG = LoggerFactory.getLogger(ZeusExprBuilder.class);
  private final ZeusTable table;
  private int nextID;

  public ZeusExprBuilder(ZeusTable table) {
    this.table = table;
    this.nextID = 0;
  }

  @Override
  public Optional<Expression> visitUnknown(LogicalExpression e, Void value) throws RuntimeException {
    LOG.info("Unable to transform expression: {}", serializeLogicalExpression(e));
    return Optional.empty();
  }

  @Override
  public Optional<Expression> visitFunctionCall(FunctionCall call, Void value) throws RuntimeException {
    ScalarFunction.Builder builder = ScalarFunction.newBuilder();

    ImmutableList.Builder<ColumnType> argTypesBuilder = ImmutableList.builder();

    for (LogicalExpression arg: call.args) {
      Optional<Expression> argExpr = arg.accept(this, null);
      if (!argExpr.isPresent()) {
        return Optional.empty();
      }

      argTypesBuilder.add(argExpr.get().getFieldType());
      builder.addChildren(argExpr.get());
    }

    Optional<ScalarFuncId> scalarFuncIdOpt = DrillFunctions.zeusScalarFuncOf(
        new DrillFunctionSignature(call.getName(), argTypesBuilder.build()));

    if (!scalarFuncIdOpt.isPresent()) {
      LOG.info("Unable to transform expression: {}", serializeLogicalExpression(call));
      return Optional.empty();
    }

    builder.setFuncId(scalarFuncIdOpt.get());

    return Optional.of(Expression.newBuilder()
        .setExpressionType(ExpressionType.SCALAR_FUNCTION)
        .setScalarFunc(builder.build())
        .setFieldType(ColumnType.BOOL)
        .setAlias(nextAnonymousName())
        .build()
    );
  }

  @Override
  public Optional<Expression> visitSchemaPath(SchemaPath path, Void value) {
    String columnName = path.getLastSegment().getNameSegment().getPath();

    ColumnType columnType = table.getColumnType(columnName)
        .orElseThrow(
            () -> CatalogNotFoundException.columnNotFound(table.getDBName(), table.getTableName(), columnName));

    ColumnRef columnRef = ColumnRef.newBuilder()
        .setName(columnName)
        .build();

    
    return Optional.of(Expression.newBuilder()
        .setExpressionType(ExpressionType.COLUMN_REF)
        .setColumn(columnRef)
        .setAlias(columnName)
        .setFieldType(columnType)
        .build()
    );
  }

  @Override
  public Optional<Expression> visitFloatConstant(FloatExpression fExpr, Void value) {
    ColumnValue columnValue = ColumnValue.newBuilder()
        .setFloatValue(fExpr.getFloat())
        .build();
    
    return Optional.of(create(columnValue, ColumnType.FLOAT4));
  }

  @Override
  public Optional<Expression> visitIntConstant(IntExpression intExpr, Void value) {
    ColumnValue columnValue = ColumnValue.newBuilder()
        .setI32Value(intExpr.getInt())
        .build();

    return Optional.of(create(columnValue, ColumnType.INT32));
  }

  @Override
  public Optional<Expression> visitLongConstant(LongExpression intExpr, Void value)  {
    ColumnValue columnValue = ColumnValue.newBuilder()
        .setI64Value(intExpr.getLong())
        .build();

    return Optional.of(create(columnValue, ColumnType.INT64));
  }

  @Override
  public Optional<Expression> visitTimeStampConstant(TimeStampExpression intExpr, Void value)  {
    ColumnValue columnValue = ColumnValue.newBuilder()
        .setI64Value(intExpr.getTimeStamp())
        .build();

    return Optional.of(create(columnValue, ColumnType.INT64));
  }

  @Override
  public Optional<Expression> visitDoubleConstant(DoubleExpression dExpr, Void value)  {
    ColumnValue columnValue = ColumnValue.newBuilder()
        .setDoubleValue(dExpr.getDouble())
        .build();

    return Optional.of(create(columnValue, ColumnType.FLOAT8));
  }

  @Override
  public Optional<Expression> visitBooleanConstant(BooleanExpression e, Void value)  {
    ColumnValue columnValue = ColumnValue.newBuilder()
        .setBoolValue(e.getBoolean())
        .build();

    return Optional.of(create(columnValue, ColumnType.BOOL));
  }

  @Override
  public Optional<Expression> visitQuotedStringConstant(QuotedString e, Void value)  {
    ColumnValue columnValue = ColumnValue.newBuilder()
        .setStringValue(e.getString())
        .build();

    return Optional.of(create(columnValue, ColumnType.STRING));
  }

  private String nextAnonymousName() {
    nextID += 1;
    return "$_"+nextID;
  }

  private static String serializeLogicalExpression(LogicalExpression e) {
    ObjectMapper mapper = new ObjectMapper();
    StringWriter sw = new StringWriter();
    try {
      mapper.writeValue(sw, e);
    } catch (IOException exp) {
      throw new RuntimeException(exp);
    }
    return sw.toString();
  }
  
  private static Expression create(ColumnValue columnValue, ColumnType columnType) {
    LiteralExpression literalExpr = LiteralExpression.newBuilder()
        .setValue(columnValue)
        .build();
    
    return Expression.newBuilder()
        .setExpressionType(ExpressionType.LITERAL)
        .setFieldType(columnType)
        .setLiteral(literalExpr)
        .build();
  }
}
