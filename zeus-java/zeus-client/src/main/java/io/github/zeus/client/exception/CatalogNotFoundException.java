package io.github.zeus.client.exception;

public class CatalogNotFoundException extends RuntimeException {
  private CatalogNotFoundException(String message) {
    super(message);
  }

  public static CatalogNotFoundException databaseIdNotFound(int dbId) {
    return new CatalogNotFoundException(String.format("Database with id {%d} not found.", dbId));
  }

  public static CatalogNotFoundException tableIdNotFound(int dbId, int tableId) {
    return new CatalogNotFoundException(
        String.format("Table id {%d} not found in database id {%d}.", tableId, dbId));
  }

  public static CatalogNotFoundException columnNotFound(int dbId, int tableId, int columnId) {
    return new CatalogNotFoundException(
        String.format("Can't find column id {%d}, table id {%d}, database id {%d}.",
            columnId, tableId, dbId));
  }

  public static CatalogNotFoundException databaseNotFound(String dbName) {
    return new CatalogNotFoundException(String.format("Database {%s} not found", dbName));
  }

  public static CatalogNotFoundException tableNotFound(String dbName, String tableName) {
    return new CatalogNotFoundException(String.format("Table {%s}.{%s} not found", dbName, tableName));
  }

  public static CatalogNotFoundException columnNotFound(String dbName, String tableName, String columnName) {
    return new CatalogNotFoundException(String.format("Column {%s}.{%s}.{%s} not found", dbName, tableName, columnName));
  }
}
