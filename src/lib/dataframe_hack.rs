use std::cell::UnsafeCell;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use virtual_table::*;

pub struct DataFrameSyncBuilder {
    rows: Option<usize>,
    name: String,
    definitions: Vec<ColumnDefinition>,
}

impl DataFrameSyncBuilder {
    pub fn new() -> Self {
        Self {
            rows: None,
            name: "table".to_string(),
            definitions: vec![],
        }
    }
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }
    pub fn add_column_definition(&mut self, column: ColumnDefinition) {
        self.definitions.push(column);
    }

    pub fn with_rows(mut self, rows: usize) -> Self {
        self.rows = Some(rows);
        self
    }

    pub fn build(self) -> DataFrameSync {
        let _rows = self.rows.expect("Did not initialize rows");

        let frame = Table::create("table".to_owned(), self.definitions);

        DataFrameSync {
            frame: Arc::new(UnsafeCell::new(frame)),
            index: Default::default(),
            size: Default::default(),
        }
    }
}

#[derive(Debug)]
#[allow(unused)]
pub struct DataFrameSync {
    frame: Arc<UnsafeCell<Table>>,
    index: AtomicUsize,
    size: AtomicUsize,
}

impl DataFrameSync {
    pub fn update_row(&self, row: Row) {
        let _ = unsafe { &mut *self.frame.get() }.update_row(row);
    }
}

unsafe impl Send for DataFrameSync {}

unsafe impl Sync for DataFrameSync {}

static_assertions::assert_impl_all!(DataFrameSync: Sync, Send);
