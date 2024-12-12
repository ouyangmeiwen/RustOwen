use calamine::RangeDeserializerBuilder;
use calamine::{open_workbook_auto, DataType, Reader};
use std::error::Error;
use std::time::Instant;
fn read_excel(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut workbook = open_workbook_auto(file_path)?;
    // Start measuring time for a specific code block
    let start = Instant::now();
    if let Some(Ok(range)) = workbook.worksheet_range("Sheet1") {
        for row in range.rows() {
            // Iterate over each cell in the row
            for cell in row {
                match cell {
                    DataType::Empty => print!("Empty\t"),
                    DataType::String(s) => print!("String: {}\t", s), // Handle as a string
                    DataType::Float(f) => print!("Float: {}\t", f),
                    DataType::Int(i) => print!("Int: {}\t", i),
                    DataType::Bool(b) => print!("Bool: {}\t", b),
                    _ => print!("Other: {:?}\t", cell),
                }
            }
            // After finishing a row, print a newline
            println!();
        }
    } else {
        println!("Sheet not found or unable to read.");
    }
    // Code after the time-measured block
    let duration = start.elapsed();
    println!("Time taken for the block: {} seconds", duration.as_secs());
    Ok(())
}

pub fn import_excle() {
    if let Err(e) = read_excel("D:\\20241020在架图书数据V4.xlsx") {
        eprintln!("Error reading the Excel file: {}", e);
    }
}
