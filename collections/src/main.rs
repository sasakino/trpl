enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("blue")),
    ];

    match row.get(1) {
        Some(SpreadSheetCell::Int(val)) => println!("Int: {}", val),
        Some(SpreadSheetCell::Float(val)) => println!("Float: {}", val),
        Some(SpreadSheetCell::Text(val)) => println!("Text: {}", val),
        None => println!("None"),
    }
}
