use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Arc;

use db::schema::DBSchema;
use db::table::Tablet;
use db::DBResult;
use db::DBErr;
use db::DBConfig;
use rpc::zeus_data::Row;



pub struct DB {
    inner: Mutex<DBInner>
}

struct DBInner {
    config: DBConfig,
    schema: DBSchema,
    tablets: HashMap<i32, Arc<Tablet>>
}

impl DBInner {
    fn new(config: DBConfig, schema: DBSchema) -> DBInner {
        DBInner {
            config: config,
            schema: schema,
            tablets: HashMap::new()
        }
    }

    fn get_table(&self, table_id: i32) -> Option<Arc<Tablet>> {
        self.tablets.get(&table_id).map(|x| x.clone())
    }
}


impl DB {
    pub fn new(config: DBConfig, schema: DBSchema) -> DB {
        DB {
            inner: Mutex::new(DBInner::new(config, schema))
        }
    }



    pub fn append(&self, row: Row) -> DBResult {
        let db = match self.inner.lock() {
            Ok(guard) => guard,
            Err(poisioned) => poisioned.into_inner()
        };

        assert_eq!(db.schema.id, row.db_id);

        match db.get_table(row.table_id) {
            Some(table) => table.append(row),
            None => Result::Err(DBErr::TableNotFound(row.table_id))
        }

    }
}