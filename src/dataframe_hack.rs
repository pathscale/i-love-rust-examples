use virtual_table::*;
use std::sync::Arc;
use std::cell::UnsafeCell;
use std::sync::atomic::AtomicUsize;

#[derive()]
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
        let rows = self.rows.expect("Did not initialize rows");

        let mut frame = Table::create("table".to_owned(), self.definitions);


        DataFrameSync {
            frame: Arc::new(UnsafeCell::new(frame)),
            index: Default::default(),
            size: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct DataFrameSync {
    frame: Arc<UnsafeCell<Table>>,
    index: AtomicUsize,
    size: AtomicUsize,
}

impl DataFrameSync {
    pub fn update_row(&self, row: Row) {
        unsafe {
            &mut *self.frame.get()
        }.update_row(row);
    }
}

unsafe impl Send for DataFrameSync {}

unsafe impl Sync for DataFrameSync {}

static_assertions::assert_impl_all!(DataFrameSync: Sync, Send);
