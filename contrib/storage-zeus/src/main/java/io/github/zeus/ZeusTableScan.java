package io.github.zeus;

import com.fasterxml.jackson.annotation.JacksonInject;
import com.fasterxml.jackson.annotation.JsonIgnore;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonTypeName;
import io.github.zeus.client.exception.CatalogNotFoundException;
import io.github.zeus.rpc.PlanNode;
import io.github.zeus.rpc.PlanNodeType;
import io.github.zeus.rpc.ScanNode;
import io.github.zeus.schema.ZeusDB;
import org.apache.drill.common.exceptions.ExecutionSetupException;
import org.apache.drill.common.expression.SchemaPath;
import org.apache.drill.exec.physical.PhysicalOperatorSetupException;
import org.apache.drill.exec.physical.base.AbstractGroupScan;
import org.apache.drill.exec.physical.base.PhysicalOperator;
import org.apache.drill.exec.physical.base.SubScan;
import org.apache.drill.exec.proto.CoordinationProtos.DrillbitEndpoint;

import java.util.Collections;
import java.util.List;

@JsonTypeName("zeus-table-scan")
public class ZeusTableScan extends AbstractGroupScan {
  private final int dbId;
  private final int tableId;
  private final boolean isStarSchema;
  private final List<Integer> columnIds;
  private final ZeusStoragePlugin plugin;

  public ZeusTableScan(
      @JsonProperty("dbId") int dbId,
      @JsonProperty("tableId") int tableId,
      @JsonProperty("isStarSchema") boolean isStarSchema,
      @JsonProperty("columnIds") List<Integer> columnIds,
      @JacksonInject ZeusStoragePlugin plugin) {
    super("");
    this.dbId = dbId;
    this.tableId = tableId;
    this.isStarSchema = isStarSchema;
    this.columnIds = columnIds;
    this.plugin = plugin;
  }


  @Override
  public void applyAssignments(List<DrillbitEndpoint> endpoints) throws PhysicalOperatorSetupException {
  }

  @Override
  public SubScan getSpecificScan(int minorFragmentId) throws ExecutionSetupException {
    throw new UnsupportedOperationException();
  }

  @Override
  public boolean canPushdownProjects(List<SchemaPath> columns) {
    return true;
  }

  @Override
  public ZeusTableScan clone(List<SchemaPath> columns) {
    return ZeusTableScan.from(plugin, dbId, tableId, columns);
  }

  @Override
  public int getMaxParallelizationWidth() {
    return 0;
  }

  @Override
  public String getDigest() {
    return null;
  }

  @Override
  public PhysicalOperator getNewWithChildren(List<PhysicalOperator> children) throws ExecutionSetupException {
    return null;
  }

  @JsonProperty
  public int getDbId() {
    return dbId;
  }

  @JsonProperty
  public int getTableId() {
    return tableId;
  }

  @JsonProperty
  public boolean isStarSchema() {
    return isStarSchema;
  }

  @JsonProperty
  public List<Integer> getColumnIds() {
    return columnIds;
  }

  @JsonIgnore
  public ZeusStoragePlugin getPlugin() {
    return plugin;
  }

  public PlanNode toPlanNode() {
    ScanNode scanNode = ScanNode.newBuilder()
        .setDbId(dbId)
        .setTableId(tableId)
        .addAllColumns(columnIds)
        .build();

    return PlanNode.newBuilder()
        .setPlanNodeType(PlanNodeType.SCAN_NODE)
        .setScanNode(scanNode)
        .build();
  }

  public static ZeusTableScan from(ZeusStoragePlugin plugin, int dbId, int tableId, List<SchemaPath> columns) {
    if (ZeusDB.isStarSchema(columns)) {
      return new ZeusTableScan(dbId, tableId, true, Collections.emptyList(), plugin);
    } else {
      List<Integer> columnIds = plugin.getDbSchema().getTable(tableId)
          .map(t -> t.getColumnIds(columns))
          .orElseThrow(() -> CatalogNotFoundException.tableIdNotFound(dbId, tableId));

      return new ZeusTableScan(dbId, tableId, false, columnIds, plugin);
    }
  }
}
