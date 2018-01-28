use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Condvar;
use std::collections::LinkedList;

use db::schema::TableSchema;
use db::segment::AppendableSegment;
use db::segment::ImmutableSegment;
use db::DBConfig;
use db::DBResult;
use rpc::zeus_data::Row;

pub struct Tablet {
    inner: Mutex<TabletInner>,
    writable_cnd: Condvar,
}

struct TabletInner {
    config: DBConfig,
    schema: TableSchema,
    active_segment: Arc<AppendableSegment>,
    // Segments waiting to be merged
    pending_segments: LinkedList<Arc<AppendableSegment>>,
    immutable_segments: LinkedList<Arc<ImmutableSegment>>,
}

impl TabletInner {
    fn new(config: DBConfig, schema: TableSchema) -> TabletInner {
        TabletInner {
            config: config,
            schema: schema,
            active_segment: Arc::new(AppendableSegment::new()),
            pending_segments: LinkedList::new(),
            immutable_segments: LinkedList::new(),
        }
    }

    fn append(&mut self, row: Row) -> DBResult {
        self.active_segment.append(row);

        self.roll_over();
        Ok(0)
    }

    /// Roll over the active segment if necessary
    fn roll_over(&mut self) {
        if self.active_segment.size() >= self.config.max_appendable_segment_size {
            self.pending_segments.push_back(self.active_segment.clone())
        }

        self.active_segment = Arc::new(AppendableSegment::new());
    }

    fn can_insert(&self) -> bool {
        (self.pending_segments.len() < self.config.max_pending_segment_num) ||
            (self.active_segment.size() < self.config.max_appendable_segment_size)
    }
}

impl Tablet {
    pub fn new(config: DBConfig, schema: TableSchema) -> Tablet {
        Tablet {
            inner: Mutex::new(TabletInner::new(
                config, schema
            )),
            writable_cnd: Condvar::new(),
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