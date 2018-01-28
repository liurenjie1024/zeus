package io.github.zeus;

import com.google.common.collect.ImmutableList;
import io.github.zeus.rpc.ZeusDBSchema;
import io.github.zeus.rpc.ZeusTableSchema;
import org.apache.calcite.schema.Table;
import org.apache.drill.exec.store.AbstractSchema;

import java.util.Set;
import java.util.stream.Collectors;


public class ZeusDB extends AbstractSchema {
  private final ZeusStoragePlugin plugin;
  private final String storageEngineName;
  private final ZeusDBSchema zeusDBSchema;

  public ZeusDB(ZeusStoragePlugin plugin, String storageEngineName, ZeusDBSchema zeusDBSchema) {
    super(ImmutableList.of(), zeusDBSchema.getName());
    this.plugin = plugin;
    this.storageEngineName = storageEngineName;
    this.zeusDBSchema = zeusDBSchema;
  }

  @Override
  public String getTypeName() {
    return getName();
  }

  @Override
  public Set<String> getTableNames() {
    return zeusDBSchema.getTablesList()
      .stream()
      .map(ZeusTableSchema::getName)
      .collect(Collectors.toSet());
  }

  public Table getTable(String name) {
    return zeusDBSchema.getTablesList()
      .stream()
      .filter(t -> t.getName().equals(name))
      .findFirst()
      .map(t -> new ZeusTable(plugin, storageEngineName, null, t))
      .get();
  }
}
