use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Condvar;
use std::collections::LinkedList;

use db::schema::TableSchema;
use db::segment::AppendableSegment;
use db::segment::MemSegment;
use db::DBConfig;
use db::DBResult;
use rpc::zeus_meta::ZeusTableSchema;

pub struct Tablet {
    inner: Mutex<TabletInner>,
}

struct TabletInner {
    config: DBConfig,
    schema: ZeusTableSchema,
    mem_segments: LinkedList<Arc<MemSegment>>,
}

impl TabletInner {
    fn new(config: DBConfig, schema: ZeusTableSchema) -> TabletInner {
        TabletInner {
            config,
            schema,
            mem_segments: LinkedList::new(),
        }
    }
}

impl Tablet {
    pub fn new(config: DBConfig, schema: ZeusTableSchema) -> Tablet {
        Tablet {
            inner: Mutex::new(TabletInner::new(
                config, schema
            )),
        }
    }

    pub fn append(&self, row: Row) -> DBResult {
        let mut guard = match self.inner.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner()
        };

        while !guard.can_insert() {
            guard = match self.writable_cnd.wait(guard) {
                Ok(guard) => guard,
                Err(poisoned) => poisoned.into_inner()
            }
        }

        guard.append(row)
    }
}