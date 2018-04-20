/*
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
package io.github.zeus;

import org.apache.drill.categories.SlowTest;
import org.apache.drill.categories.ZeusStorageTest;
import org.apache.drill.exec.rpc.user.QueryDataBatch;
import org.junit.Test;
import org.junit.experimental.categories.Category;

import java.util.List;

@Category({ZeusStorageTest.class, SlowTest.class})
public class ZeusQueryTest extends ZeusTestBase {
  @Test
  public void testQueryCount() throws Exception {
    List<QueryDataBatch> result =  testSqlWithResults("select * from logs.realtimelog limit 3");
  }

}
