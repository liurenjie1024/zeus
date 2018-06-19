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
package io.github.zeus.expr.drill;

import io.github.zeus.expr.ZeusFunctionEntry;
{% for item in imported_scalar_func_ids %}
import static io.github.zeus.rpc.ScalarFuncId.{{item}};
{%- endfor %}
{% for item in imported_column_types %}
import static io.github.zeus.rpc.ColumnType.{{item}};
{%- endfor %}

/**
 * This class is generated.
 */
public class ComparatorFunctionSignatures {
  {% for item in items %}
    public static final ZeusFunctionEntry {{item.name}} = ZeusFunctionEntry.from({{item
    .scalar_func_id}}, "{{item.drill_func_name}}", {{item.args|join(', ')}});
  {%- endfor %}
}
