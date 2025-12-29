use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct CellId {
    pub col: usize,
    pub row: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CellValue {
    Empty,
    Number(f64),
    Text(String),
    Error(String),
}

#[derive(Debug)]
pub struct Sheet {
    cells: HashMap<CellId, CellValue>,
}

impl Sheet {
    pub fn new() -> Self {
        Sheet {
            cells: HashMap::new(),
        }
    }

    pub fn set_cell_value(&mut self, col: usize, row: usize, value: CellValue) {
        self.cells.insert(CellId { col, row }, value);
    }

    pub fn get_cell_value(&self, col: usize, row: usize) -> Option<&CellValue> {
        self.cells.get(&CellId { col, row })
    }
}
