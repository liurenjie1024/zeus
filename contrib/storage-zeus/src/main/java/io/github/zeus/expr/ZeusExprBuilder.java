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
import io.github.zeus.rpc.ColumnRef;
import io.github.zeus.rpc.ColumnValue;
import io.github.zeus.rpc.Expression;
import io.github.zeus.rpc.ExpressionType;
import io.github.zeus.rpc.LiteralExpression;
import io.github.zeus.rpc.ScalarFuncId;
import io.github.zeus.rpc.ScalarFunction;
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
import org.apache.drill.common.types.TypeProtos.MajorType;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.io.StringWriter;
import java.util.Optional;

public class ZeusExprBuilder extends AbstractExprVisitor<Optional<Expression>, Void, RuntimeException> {
  private static final Logger LOG = LoggerFactory.getLogger(ZeusExprBuilder.class);

  @Override
  public Optional<Expression> visitUnknown(LogicalExpression e, Void value) throws RuntimeException {
    LOG.info("Unable to transform expression: {}", serializeLogicalExpression(e));
    return Optional.empty();
  }

  @Override
  public Optional<Expression> visitFunctionCall(FunctionCall call, Void value) throws RuntimeException {
    MajorType[] argTypes = call.args.stream()
        .map(LogicalExpression::getMajorType)
        .toArray(MajorType[]::new);

    Optional<ScalarFuncId> funcId = DrillFunctions.zeusScalarFuncOf(
        new DrillFunctionSignature(call.getName(), argTypes));

    if (funcId.isPresent()) {
      ScalarFunction.Builder builder = ScalarFunction.newBuilder()
          .setFuncId(funcId.get());

      for (LogicalExpression arg: call.args) {
        Optional<Expression> argExpr = arg.accept(this, null);
        if (!argExpr.isPresent()) {
          return Optional.empty();
        }

        builder.addChildren(argExpr.get());
      }

      return Optional.of(Expression.newBuilder()
          .setExpressionType(ExpressionType.SCALAR_FUNCTION)
          .setScalarFunc(builder.build())
          .build()
      );
    }

    return Optional.empty();
  }

  @Override
  public Optional<Expression> visitSchemaPath(SchemaPath path, Void value) {
    ColumnRef columnRef = ColumnRef.newBuilder()
        .setName(path.getLastSegment().getNameSegment().getPath())
        .build();
    
    return Optional.of(Expression.newBuilder()
        .setExpressionType(ExpressionType.COLUMN_REF)
        .setColumn(columnRef)
        .build()
    );
  }

  @Override
  public Optional<Expression> visitFloatConstant(FloatExpression fExpr, Void value) {
    ColumnValue columnValue = ColumnValue.newBuilder()
        .setFloatValue(fExpr.getFloat())
        .build();
    
    return Optional.of(create(columnValue));
  }

  @Override
  public Optional<Expression> visitIntConstant(IntExpression intExpr, Void value) {
    ColumnValue columnValue = ColumnValue.newBuilder()
        .setI32Value(intExpr.getInt())
        .build();

    return Optional.of(create(columnValue));
  }

  @Override
  public Optional<Expression> visitLongConstant(LongExpression intExpr, Void value)  {
    ColumnValue columnValue = ColumnValue.newBuilder()
        .setI64Value(intExpr.getLong())
        .build();

    return Optional.of(create(columnValue));
  }

  @Override
  public Optional<Expression> visitTimeStampConstant(TimeStampExpression intExpr, Void value)  {
    ColumnValue columnValue = ColumnValue.newBuilder()
        .setI64Value(intExpr.getTimeStamp())
        .build();

    return Optional.of(create(columnValue));
  }

  @Override
  public Optional<Expression> visitDoubleConstant(DoubleExpression dExpr, Void value)  {
    ColumnValue columnValue = ColumnValue.newBuilder()
        .setDoubleValue(dExpr.getDouble())
        .build();

    return Optional.of(create(columnValue));
  }

  @Override
  public Optional<Expression> visitBooleanConstant(BooleanExpression e, Void value)  {
    ColumnValue columnValue = ColumnValue.newBuilder()
        .setBoolValue(e.getBoolean())
        .build();

    return Optional.of(create(columnValue));
  }

  @Override
  public Optional<Expression> visitQuotedStringConstant(QuotedString e, Void value)  {
    ColumnValue columnValue = ColumnValue.newBuilder()
        .setStringValue(e.getString())
        .build();

    return Optional.of(create(columnValue));
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
  
  private static Expression create(ColumnValue columnValue) {
    LiteralExpression literalExpr = LiteralExpression.newBuilder()
        .setValue(columnValue)
        .build();
    
    return Expression.newBuilder()
        .setExpressionType(ExpressionType.LITERAL)
        .setLiteral(literalExpr)
        .build();
  }
}
