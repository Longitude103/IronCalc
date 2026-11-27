#![allow(clippy::unwrap_used)]

use crate::test::util::new_empty_model;

#[test]
fn test_match_array_row_vector() {
    let mut model = new_empty_model();
    model._set("A1", "=MATCH(69,{24,43.5,52.8,69,269,387,770},0)");
    model.evaluate();
    assert_eq!(model._get_text("A1"), "4");
}

#[test]
fn test_match_array_column_vector() {
    let mut model = new_empty_model();
    model._set("A1", "=MATCH(52.8,{24;43.5;52.8;69;269;387;770},0)");
    model.evaluate();
    assert_eq!(model._get_text("A1"), "3");
}

#[test]
fn test_match_array_exact_string() {
    let mut model = new_empty_model();
    model._set("A1", r#"=MATCH("banana",{"apple","banana","cherry"},0)"#);
    model.evaluate();
    assert_eq!(model._get_text("A1"), "2");
}

#[test]
fn test_match_array_not_found() {
    let mut model = new_empty_model();
    model._set("A1", "=MATCH(100,{24,43.5,52.8,69},0)");
    model.evaluate();
    assert_eq!(model._get_text("A1"), "#N/A");
}

#[test]
fn test_match_array_ascending_default() {
    let mut model = new_empty_model();
    // match_type 1 (default): largest value <= target in an ascending array
    model._set("A1", "=MATCH(70,{24,43.5,52.8,69,269,387,770})");
    model.evaluate();
    assert_eq!(model._get_text("A1"), "4");
}

#[test]
fn test_match_array_descending() {
    let mut model = new_empty_model();
    // match_type -1: smallest value >= target in a descending array.
    // Values >= 70 are 770, 387, 269; the smallest of those is 269 at position 3.
    model._set("A1", "=MATCH(70,{770,387,269,69,52.8,43.5,24},-1)");
    model.evaluate();
    assert_eq!(model._get_text("A1"), "3");
}

#[test]
fn test_match_array_wildcard() {
    let mut model = new_empty_model();
    model._set("A1", r#"=MATCH("ban*",{"apple","banana","cherry"},0)"#);
    model.evaluate();
    assert_eq!(model._get_text("A1"), "2");
}

#[test]
fn test_match_array_not_a_vector() {
    let mut model = new_empty_model();
    model._set("A1", "=MATCH(1,{1,2;3,4},0)");
    model.evaluate();
    assert_eq!(model._get_text("A1"), "#ERROR!");
}

#[test]
fn test_match_ignores_blanks() {
    let mut model = new_empty_model();
    // Ascending data followed by empty rows
    for (row, value) in [10, 20, 30, 40, 50].iter().enumerate() {
        model._set(&format!("A{}", row + 1), &value.to_string());
    }
    // Descending data followed by empty rows
    for (row, value) in [50, 40, 30, 20, 10].iter().enumerate() {
        model._set(&format!("C{}", row + 1), &value.to_string());
    }
    model._set("E1", "=MATCH(40,A1:A20,1)");
    model._set("E2", "=MATCH(45,A1:A20)");
    model._set("E3", "=MATCH(500,A1:A20)");
    model._set("E4", "=MATCH(25,C1:C20,-1)");
    model._set("E5", "=MATCH(40,C1:C20,-1)");
    model._set("E6", "=MATCH(100,C1:C20,-1)");
    model.evaluate();
    assert_eq!(model._get_text("E1"), "4");
    assert_eq!(model._get_text("E2"), "4");
    assert_eq!(model._get_text("E3"), "5");
    assert_eq!(model._get_text("E4"), "3");
    assert_eq!(model._get_text("E5"), "2");
    assert_eq!(model._get_text("E6"), "#N/A");
}

#[test]
fn test_match_all_blank_leading_and_interior_blanks() {
    let mut model = new_empty_model();
    // Column A is empty. Column C has leading blanks, column E descending
    // data with interior blanks.
    for (row, value) in [10, 20, 30, 40, 50].iter().enumerate() {
        model._set(&format!("C{}", row + 4), &value.to_string());
    }
    for (row, value) in [(1, 50), (2, 40), (4, 30), (5, 20)] {
        model._set(&format!("E{row}"), &value.to_string());
    }
    model._set("G1", "=MATCH(5,A1:A5,1)");
    model._set("G2", "=MATCH(5,A1:A5,-1)");
    model._set("G3", "=MATCH(0,A1:A5,1)");
    model._set("G4", "=MATCH(40,C1:C20,1)");
    model._set("G5", "=MATCH(5,C1:C20,1)");
    model._set("G6", "=MATCH(35,E1:E10,-1)");
    model._set("G7", "=MATCH(30,E1:E10,-1)");
    model._set("G8", "=MATCH(25,E1:E10,-1)");
    model._set("G9", "=MATCH(Z1,C1:C20,1)");
    model.evaluate();
    assert_eq!(model._get_text("G1"), "#N/A");
    assert_eq!(model._get_text("G2"), "#N/A");
    assert_eq!(model._get_text("G3"), "#N/A");
    assert_eq!(model._get_text("G4"), "7");
    assert_eq!(model._get_text("G5"), "#N/A");
    assert_eq!(model._get_text("G6"), "2");
    assert_eq!(model._get_text("G7"), "4");
    assert_eq!(model._get_text("G8"), "4");
    assert_eq!(model._get_text("G9"), "#N/A");
}
