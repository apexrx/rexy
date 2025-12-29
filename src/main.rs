use rexy::{CellValue, Sheet};

fn main() {
    let mut sheet = Sheet::new();
    sheet.set_cell_value(0, 0, CellValue::Number(10.0));
    if let Some(value) = sheet.get_cell_value(0, 0) {
        println!("Cell value: {:?}", value);
    } else {
        println!("Cell is empty");
    }

    println!("Spreadsheet engine... placeholder");
}
