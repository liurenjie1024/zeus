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
import com.google.common.collect.ImmutableSet;
import io.github.zeus.expr.drill.ComparatorFunctionSignatures;
import io.github.zeus.rpc.ScalarFuncId;

import java.lang.reflect.Modifier;
import java.util.Arrays;
import java.util.List;
import java.util.Optional;
import java.util.stream.Collectors;

import static io.github.zeus.rpc.ColumnType.INT8;

public class DrillFunctions {
  private static final ImmutableMap<ZeusFunctionSignature, ScalarFuncId> DRILL_FUNCTIONS;

  static {
    ImmutableSet<Class<?>> DRILL_FUNCTIONS_CLASSES =
      ImmutableSet.<Class<?>>builder()
        .add(ComparatorFunctionSignatures.class)
       .build();

    ImmutableMap.Builder<ZeusFunctionSignature, ScalarFuncId> builder = ImmutableMap.builder();

    DRILL_FUNCTIONS_CLASSES.stream()
      .map(DrillFunctions::listEntries)
      .flatMap(List::stream)
      .forEach(entry -> builder.put(entry.getSignature(), entry.getFuncId()));


    DRILL_FUNCTIONS = builder.build();

    System.out.println("functions: " + DRILL_FUNCTIONS);
  }

  private static List<ZeusFunctionEntry> listEntries(Class<?> klass) {
    return Arrays.stream(klass.getDeclaredFields())
      .filter(f -> Modifier.isStatic(f.getModifiers()))
//      .filter(f -> ZeusFunctionEntry.class == f.getDeclaringClass())
      .map(f -> {
        try {
          return (ZeusFunctionEntry)f.get(null);
        } catch (IllegalAccessException e) {
          throw new RuntimeException(e);
        }
      })
      .collect(Collectors.toList());
  }

  static Optional<ScalarFuncId> zeusScalarFuncOf(ZeusFunctionSignature signature) {
    return Optional.ofNullable(DRILL_FUNCTIONS.get(signature));
  }

  public static void main(String[] args) {
    System.out.println(zeusScalarFuncOf(ZeusFunctionSignature.from("less_than", INT8, INT8)));
  }
}
