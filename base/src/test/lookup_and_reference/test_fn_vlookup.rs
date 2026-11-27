#![allow(clippy::unwrap_used)]

use crate::test::util::new_empty_model;

#[test]
fn test_vlookup_range_sorted() {
    let mut model = new_empty_model();
    // First column is the (sorted) search column, second column holds the results.
    model._set("A1", "10");
    model._set("A2", "20");
    model._set("A3", "30");
    model._set("B1", "a");
    model._set("B2", "b");
    model._set("B3", "c");
    model._set("D1", "=VLOOKUP(20,A1:B3,2)");
    model.evaluate();
    assert_eq!(model._get_text("D1"), "b");
}

#[test]
fn test_vlookup_range_exact() {
    let mut model = new_empty_model();
    model._set("A1", "banana");
    model._set("A2", "apple");
    model._set("A3", "cherry");
    model._set("B1", "1");
    model._set("B2", "2");
    model._set("B3", "3");
    model._set("D1", r#"=VLOOKUP("apple",A1:B3,2,FALSE)"#);
    model.evaluate();
    assert_eq!(model._get_text("D1"), "2");
}

#[test]
fn test_vlookup_array_sorted() {
    let mut model = new_empty_model();
    model._set("A1", "=VLOOKUP(20,{10,\"a\";20,\"b\";30,\"c\"},2)");
    model.evaluate();
    assert_eq!(model._get_text("A1"), "b");
}

#[test]
fn test_vlookup_array_exact() {
    let mut model = new_empty_model();
    model._set(
        "A1",
        r#"=VLOOKUP("banana",{"apple",1;"banana",2;"cherry",3},2,FALSE)"#,
    );
    model.evaluate();
    assert_eq!(model._get_text("A1"), "2");
}

#[test]
fn test_vlookup_array_wildcard() {
    let mut model = new_empty_model();
    model._set(
        "A1",
        r#"=VLOOKUP("ban*",{"apple",1;"banana",2;"cherry",3},2,FALSE)"#,
    );
    model.evaluate();
    assert_eq!(model._get_text("A1"), "2");
}

#[test]
fn test_vlookup_array_not_found() {
    let mut model = new_empty_model();
    model._set("A1", "=VLOOKUP(99,{10,\"a\";20,\"b\";30,\"c\"},2,FALSE)");
    model.evaluate();
    assert_eq!(model._get_text("A1"), "#N/A");
}

#[test]
fn test_vlookup_array_column_index_out_of_range() {
    let mut model = new_empty_model();
    model._set("A1", "=VLOOKUP(20,{10,\"a\";20,\"b\";30,\"c\"},3)");
    model.evaluate();
    assert_eq!(model._get_text("A1"), "#REF!");
}

#[test]
fn test_vlookup_array_sorted_largest_smaller() {
    let mut model = new_empty_model();
    // 25 is not present, sorted search returns the largest value <= 25, i.e. 20.
    model._set("A1", "=VLOOKUP(25,{10,\"a\";20,\"b\";30,\"c\"},2)");
    model.evaluate();
    assert_eq!(model._get_text("A1"), "b");
}

#[test]
fn test_vlookup_column_index_below_one() {
    let mut model = new_empty_model();
    model._set("A1", "1");
    model._set("B1", "one");
    model._set("C1", "10");
    model._set("A2", "2");
    model._set("B2", "two");
    model._set("C2", "20");
    model._set("E1", "=VLOOKUP(2,A1:C2,0,FALSE)");
    model._set("E2", "=VLOOKUP(2,A1:C2,-1,FALSE)");
    model._set("E3", "=VLOOKUP(2,A1:C2,0.5,FALSE)");
    model._set("E4", "=VLOOKUP(2,A1:C2,4,FALSE)");
    model._set("E5", "=VLOOKUP(2,A1:C2,1.9,FALSE)");
    model._set("E6", "=VLOOKUP(2,A1:C2,2.5,FALSE)");
    model._set("E7", "=VLOOKUP(2,A1:C2,TRUE,FALSE)");
    model.evaluate();
    // Below 1 after truncation is #VALUE!, past the table is #REF!
    assert_eq!(model._get_text("E1"), "#VALUE!");
    assert_eq!(model._get_text("E2"), "#VALUE!");
    assert_eq!(model._get_text("E3"), "#VALUE!");
    assert_eq!(model._get_text("E4"), "#REF!");
    assert_eq!(model._get_text("E5"), "2");
    assert_eq!(model._get_text("E6"), "two");
    assert_eq!(model._get_text("E7"), "2");
}

#[test]
fn test_vlookup_ignores_trailing_blanks() {
    let mut model = new_empty_model();
    // The range is larger than the data: rows 6 to 20 are empty.
    for (row, value) in [10, 20, 30, 40, 50].iter().enumerate() {
        model._set(&format!("A{}", row + 1), &value.to_string());
        model._set(&format!("B{}", row + 1), &(row + 1).to_string());
    }
    model._set("D1", "=VLOOKUP(40,A1:B20,2)");
    model._set("D2", "=VLOOKUP(45,A1:B20,2)");
    model._set("D3", "=VLOOKUP(500,A1:B20,2)");
    model._set("D4", "=VLOOKUP(5,A1:B20,2)");
    model._set("D5", "=VLOOKUP(40,A1:B20,2,FALSE)");
    model._set("D6", "=VLOOKUP(45,A1:B20,2,FALSE)");
    model.evaluate();
    assert_eq!(model._get_text("D1"), "4");
    assert_eq!(model._get_text("D2"), "4");
    assert_eq!(model._get_text("D3"), "5");
    assert_eq!(model._get_text("D4"), "#N/A");
    assert_eq!(model._get_text("D5"), "4");
    assert_eq!(model._get_text("D6"), "#N/A");
}

#[test]
fn test_vlookup_ignores_interior_blanks() {
    let mut model = new_empty_model();
    model._set("A1", "10");
    model._set("A2", "20");
    model._set("A4", "30");
    model._set("A5", "40");
    model._set("B1", "1");
    model._set("B2", "2");
    model._set("B3", "99");
    model._set("B4", "4");
    model._set("B5", "5");
    model._set("D1", "=VLOOKUP(25,A1:B8,2)");
    model._set("D2", "=VLOOKUP(40,A1:B8,2)");
    model._set("D3", "=VLOOKUP(35,A1:B8,2)");
    model.evaluate();
    assert_eq!(model._get_text("D1"), "2");
    assert_eq!(model._get_text("D2"), "5");
    assert_eq!(model._get_text("D3"), "4");
}

#[test]
fn test_vlookup_all_blank_and_leading_blanks() {
    let mut model = new_empty_model();
    // Column A is empty, column B has data
    for row in 1..=5 {
        model._set(&format!("B{row}"), &row.to_string());
    }
    // Data starts at row 4
    for (row, value) in [10, 20, 30, 40, 50].iter().enumerate() {
        model._set(&format!("D{}", row + 4), &value.to_string());
        model._set(&format!("E{}", row + 4), &(row + 1).to_string());
    }
    model._set("G1", "=VLOOKUP(5,A1:B5,2)");
    model._set("G2", "=VLOOKUP(0,A1:B5,2)");
    model._set("G3", "=VLOOKUP(40,D1:E20,2)");
    model._set("G4", "=VLOOKUP(45,D1:E20,2)");
    model._set("G5", "=VLOOKUP(5,D1:E20,2)");
    model.evaluate();
    assert_eq!(model._get_text("G1"), "#N/A");
    assert_eq!(model._get_text("G2"), "#N/A");
    assert_eq!(model._get_text("G3"), "4");
    assert_eq!(model._get_text("G4"), "4");
    assert_eq!(model._get_text("G5"), "#N/A");
}

#[test]
fn test_vlookup_blank_lookup_value_is_zero() {
    let mut model = new_empty_model();
    for (row, (key, value)) in [(-5, "neg"), (0, "zero"), (10, "ten"), (20, "twenty")]
        .iter()
        .enumerate()
    {
        model._set(&format!("A{}", row + 1), &key.to_string());
        model._set(&format!("B{}", row + 1), value);
    }
    // Z1 is empty and compares as 0
    model._set("D1", "=VLOOKUP(Z1,A1:B4,2)");
    // With trailing blank rows
    model._set("D2", "=VLOOKUP(Z1,A1:B10,2)");
    model.evaluate();
    assert_eq!(model._get_text("D1"), "zero");
    assert_eq!(model._get_text("D2"), "zero");
}
