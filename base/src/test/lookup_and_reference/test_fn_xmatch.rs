#![allow(clippy::unwrap_used)]

use crate::test::util::new_empty_model;

// ── exact match (default) ─────────────────────────────────────────────────────

#[test]
fn test_xmatch_exact_found() {
    let mut model = new_empty_model();
    model._set("A1", "apple");
    model._set("A2", "banana");
    model._set("A3", "cherry");
    model._set("C1", "=XMATCH(\"banana\",A1:A3)");
    model.evaluate();
    assert_eq!(model._get_text("C1"), "2");
}

#[test]
fn test_xmatch_exact_not_found() {
    let mut model = new_empty_model();
    model._set("A1", "apple");
    model._set("A2", "banana");
    model._set("C1", "=XMATCH(\"cherry\",A1:A2)");
    model.evaluate();
    assert_eq!(model._get_text("C1"), "#N/A");
}

#[test]
fn test_xmatch_exact_number() {
    let mut model = new_empty_model();
    model._set("A1", "10");
    model._set("A2", "20");
    model._set("A3", "30");
    model._set("C1", "=XMATCH(20,A1:A3)");
    model.evaluate();
    assert_eq!(model._get_text("C1"), "2");
}

#[test]
fn test_xmatch_empty_optional_arguments_use_defaults() {
    for (formula, expected) in [
        ("=XMATCH(8,A1:A4)", "2"),
        ("=XMATCH(8,A1:A4,)", "2"),
        ("=XMATCH(8,A1:A4,,)", "2"),
        ("=XMATCH(8,A1:A4,0,)", "2"),
        ("=XMATCH(8,A1:A4,,1)", "2"),
        ("=XMATCH(8,A1:A4,,-1)", "4"),
        ("=XMATCH(6,A1:A4,-1,)", "1"),
        ("=XMATCH(7,A1:A4,,)", "#N/A"),
        ("=XMATCH(8,{5,8,1,8},,)", "2"),
    ] {
        let mut model = new_empty_model();
        model._set("A1", "5");
        model._set("A2", "8");
        model._set("A3", "1");
        model._set("A4", "8");
        model._set("C1", formula);
        model.evaluate();
        assert_eq!(model._get_text("C1"), expected, "{formula}");
    }
}

#[test]
fn test_xmatch_explicit_optional_arguments_still_validate() {
    for (formula, expected) in [
        ("=XMATCH(2,A1:A3,0,0)", "#VALUE!"),
        ("=XMATCH(2,A1:A3,0,Z99)", "#VALUE!"),
        ("=XMATCH(2,A1:A3,0,\"\")", "#VALUE!"),
        ("=XMATCH(2,A1:A3,0,NA())", "#N/A"),
        ("=XMATCH(2,A1:A3,NA(),)", "#N/A"),
        ("=XMATCH(2,A1:A3,0,1/0)", "#DIV/0!"),
        ("=XMATCH(2,A1:A3,99,)", "#VALUE!"),
    ] {
        let mut model = new_empty_model();
        model._set("A1", "1");
        model._set("A2", "2");
        model._set("A3", "3");
        model._set("C1", formula);
        model.evaluate();
        assert_eq!(model._get_text("C1"), expected, "{formula}");
    }
}

// ── next-smaller / next-larger (unsorted) ────────────────────────────────────

#[test]
fn test_xmatch_next_smaller() {
    // match_mode=-1: exact or next smaller
    let mut model = new_empty_model();
    model._set("A1", "5");
    model._set("A2", "10");
    model._set("A3", "15");
    // Looking for 12: no exact match, next smaller is 10 at position 2
    model._set("C1", "=XMATCH(12,A1:A3,-1)");
    model.evaluate();
    assert_eq!(model._get_text("C1"), "2");
}

#[test]
fn test_xmatch_next_larger() {
    // match_mode=1: exact or next larger
    let mut model = new_empty_model();
    model._set("A1", "5");
    model._set("A2", "10");
    model._set("A3", "15");
    // Looking for 12: no exact match, next larger is 15 at position 3
    model._set("C1", "=XMATCH(12,A1:A3,1)");
    model.evaluate();
    assert_eq!(model._get_text("C1"), "3");
}

// ── wildcard match ────────────────────────────────────────────────────────────

#[test]
fn test_xmatch_wildcard() {
    let mut model = new_empty_model();
    model._set("A1", "apple");
    model._set("A2", "apricot");
    model._set("A3", "banana");
    // "ap*" should match the first entry starting with "ap"
    model._set("C1", "=XMATCH(\"ap*\",A1:A3,2)");
    model.evaluate();
    assert_eq!(model._get_text("C1"), "1");
}

#[test]
fn test_xmatch_wildcard_question_mark() {
    let mut model = new_empty_model();
    model._set("A1", "cat");
    model._set("A2", "bat");
    model._set("A3", "hat");
    // "?at" matches all three; should return first (position 1)
    model._set("C1", "=XMATCH(\"?at\",A1:A3,2)");
    model.evaluate();
    assert_eq!(model._get_text("C1"), "1");
}

// ── reverse search ────────────────────────────────────────────────────────────

#[test]
fn test_xmatch_reverse_search() {
    let mut model = new_empty_model();
    model._set("A1", "x");
    model._set("A2", "y");
    model._set("A3", "x");
    // Last-to-first: last "x" is at position 3
    model._set("C1", "=XMATCH(\"x\",A1:A3,0,-1)");
    model.evaluate();
    assert_eq!(model._get_text("C1"), "3");
}

// ── binary search ─────────────────────────────────────────────────────────────

#[test]
fn test_xmatch_binary_ascending() {
    let mut model = new_empty_model();
    model._set("A1", "1");
    model._set("A2", "3");
    model._set("A3", "5");
    model._set("A4", "7");
    // Exact match in sorted array
    model._set("C1", "=XMATCH(5,A1:A4,0,2)");
    model.evaluate();
    assert_eq!(model._get_text("C1"), "3");
}

#[test]
fn test_xmatch_binary_descending() {
    let mut model = new_empty_model();
    model._set("A1", "7");
    model._set("A2", "5");
    model._set("A3", "3");
    model._set("A4", "1");
    model._set("C1", "=XMATCH(5,A1:A4,0,-2)");
    model.evaluate();
    assert_eq!(model._get_text("C1"), "2");
}

// ── regex match (match_mode=3) ────────────────────────────────────────────────

#[test]
fn test_xmatch_regex_basic() {
    let mut model = new_empty_model();
    model._set("A1", "Monday");
    model._set("A2", "Tuesday");
    model._set("A3", "Wednesday");
    // Matches words ending in "day"
    model._set("C1", "=XMATCH(\"[A-Z][a-z]*day\",A1:A3,3)");
    model.evaluate();
    assert_eq!(model._get_text("C1"), "1");
}

#[test]
fn test_xmatch_regex_second_match() {
    let mut model = new_empty_model();
    model._set("A1", "cat");
    model._set("A2", "Cart");
    model._set("A3", "car");
    // Capital letter followed by lowercase letters ending in "t"
    model._set("C1", "=XMATCH(\"[A-Z][a-z]*t\",A1:A3,3)");
    model.evaluate();
    assert_eq!(model._get_text("C1"), "2");
}

#[test]
fn test_xmatch_regex_not_found() {
    let mut model = new_empty_model();
    model._set("A1", "apple");
    model._set("A2", "banana");
    model._set("C1", "=XMATCH(\"^\\d+$\",A1:A2,3)");
    model.evaluate();
    assert_eq!(model._get_text("C1"), "#N/A");
}

#[test]
fn test_xmatch_regex_invalid_pattern() {
    let mut model = new_empty_model();
    model._set("A1", "apple");
    model._set("C1", "=XMATCH(\"[\",A1:A1,3)");
    model.evaluate();
    assert_eq!(model._get_text("C1"), "#VALUE!");
}

#[test]
fn test_xmatch_regex_reverse_search() {
    let mut model = new_empty_model();
    model._set("A1", "Bay");
    model._set("A2", "Cat");
    model._set("A3", "Day");
    // Last-to-first: last match for capital+lowercase+y pattern is "Day" at position 3
    model._set("C1", "=XMATCH(\"[A-Z][a-z]*y\",A1:A3,3,-1)");
    model.evaluate();
    assert_eq!(model._get_text("C1"), "3");
}

// ── regex mode: non-string lookup_value → #N/A (matches Excel) ───────────────

#[test]
fn test_xmatch_regex_non_string_lookup_is_na() {
    let mut model = new_empty_model();
    model._set("A1", "apple");
    model._set("A2", "banana");
    // Excel returns #N/A (not found) when lookup_value is not a string in regex mode
    model._set("C1", "=XMATCH(42,A1:A2,3)");
    model.evaluate();
    assert_eq!(model._get_text("C1"), "#N/A");
}

// ── row vector ────────────────────────────────────────────────────────────────

#[test]
fn test_xmatch_row_vector() {
    let mut model = new_empty_model();
    model._set("A1", "red");
    model._set("B1", "green");
    model._set("C1", "blue");
    model._set("A3", "=XMATCH(\"green\",A1:C1)");
    model.evaluate();
    assert_eq!(model._get_text("A3"), "2");
}

// ── array constants as lookup_array ─────────────────────────────────────────────

#[test]
fn test_xmatch_array_constant_wildcard() {
    let mut model = new_empty_model();
    // Wildcard match against an in-formula array constant.
    model._set("A1", r#"=XMATCH("ban*", {"apple","banana","cherry"}, 2)"#);
    model.evaluate();
    assert_eq!(model._get_text("A1"), "2");
}

#[test]
fn test_xmatch_array_constant_numeric() {
    let mut model = new_empty_model();
    // Exact numeric match against a numeric array constant.
    model._set("A1", "=XMATCH(30, {10,20,30,40,50})");
    // Exact-or-next-larger against a numeric array constant.
    model._set("A2", "=XMATCH(35, {10,20,30,40,50}, 1)");
    // A column-oriented array constant works too.
    model._set("A3", "=XMATCH(30, {10;20;30;40;50})");
    // Not found is #N/A, not #VALUE!.
    model._set("A4", "=XMATCH(99, {10,20,30})");
    model.evaluate();
    assert_eq!(model._get_text("A1"), "3");
    assert_eq!(model._get_text("A2"), "4");
    assert_eq!(model._get_text("A3"), "3");
    assert_eq!(model._get_text("A4"), "#N/A");
}

// ── whole-column lookup array and a spill beyond the used area ───────────────

// XMATCH clips a whole-column lookup array to the sheet's used area. When it
// runs inside an anchor, the array it looks in may be spilled on demand
// during the walk, past the bound the walk was clipped to. The skipped part
// is on record, so the spill restarts the pass and the match is found.
#[test]
fn test_xmatch_whole_column_sees_a_spill_beyond_the_used_area() {
    let mut model = new_empty_model();
    model._set("A1", "=SEQUENCE(1,1,XMATCH(5,D:D))");
    model._set("C1", "=SEQUENCE(3,1,10)");
    model._set("D1", "=SEQUENCE(5)");
    model.evaluate();
    assert_eq!(model._get_text("A1"), "5");
    assert_eq!(model.evaluation.restarts_in_last_evaluation, 1);

    model.evaluate();
    assert_eq!(model._get_text("A1"), "5");
    assert_eq!(model.evaluation.restarts_in_last_evaluation, 0);
}

// ── binary search ignores empty cells ─────────────────────────────────────────

#[test]
fn test_xmatch_xlookup_binary_ignores_blanks() {
    let mut model = new_empty_model();
    for (row, value) in [10, 20, 30, 40, 50].iter().enumerate() {
        model._set(&format!("A{}", row + 1), &value.to_string());
        model._set(&format!("B{}", row + 1), &(row + 1).to_string());
    }
    for (row, value) in [50, 40, 30, 20, 10].iter().enumerate() {
        model._set(&format!("C{}", row + 1), &value.to_string());
    }
    model._set("E1", "=XMATCH(45,A1:A20,-1,2)");
    model._set("E2", "=XMATCH(45,A1:A20,1,2)");
    model._set("E3", "=XMATCH(25,C1:C20,1,-2)");
    model._set("E4", "=XMATCH(25,C1:C20,-1,-2)");
    model._set("E5", "=XLOOKUP(45,A1:A20,B1:B20,\"none\",-1,2)");
    model._set("E6", "=XLOOKUP(40,A1:A20,B1:B20,\"none\",0,2)");
    model.evaluate();
    assert_eq!(model._get_text("E1"), "4");
    assert_eq!(model._get_text("E2"), "5");
    assert_eq!(model._get_text("E3"), "3");
    assert_eq!(model._get_text("E4"), "4");
    assert_eq!(model._get_text("E5"), "4");
    assert_eq!(model._get_text("E6"), "4");
}

#[test]
fn test_xmatch_xlookup_binary_all_blank_leading_and_interior_blanks() {
    let mut model = new_empty_model();
    // Column A is empty. Column C has leading blanks, column E descending
    // data with interior blanks, column G the results.
    for (row, value) in [10, 20, 30, 40, 50].iter().enumerate() {
        model._set(&format!("C{}", row + 4), &value.to_string());
        model._set(&format!("G{}", row + 4), &(row + 1).to_string());
    }
    for (row, value) in [(1, 50), (2, 40), (4, 30), (5, 20)] {
        model._set(&format!("E{row}"), &value.to_string());
    }
    // All blank
    model._set("I1", "=XMATCH(5,A1:A20,-1,2)");
    model._set("I2", "=XMATCH(5,A1:A5,0,2)");
    model._set("I3", "=XMATCH(5,A1:A5,1,-2)");
    model._set("I4", "=XLOOKUP(5,A1:A5,G1:G5,\"none\",-1,2)");
    model._set("I5", "=XLOOKUP(5,A1:A5,G1:G5,\"none\",0,2)");
    model._set("I6", "=XLOOKUP(5,A1:A5,G1:G5,\"none\",1,2)");
    // Leading blanks
    model._set("J1", "=XMATCH(45,C1:C20,-1,2)");
    model._set("J2", "=XMATCH(40,C1:C20,0,2)");
    model._set("J3", "=XMATCH(500,C1:C20,1,2)");
    model._set("J4", "=XMATCH(5,C1:C20,-1,2)");
    model._set("J5", "=XLOOKUP(45,C1:C20,G1:G20,\"none\",-1,2)");
    // Descending with interior blanks
    model._set("K1", "=XMATCH(25,E1:E10,1,-2)");
    model._set("K2", "=XMATCH(25,E1:E10,-1,-2)");
    model._set("K3", "=XMATCH(30,E1:E10,0,-2)");
    // An empty lookup value is not found
    model._set("L1", "=XMATCH(Z1,C1:C20,0,2)");
    model._set("L2", "=XMATCH(Z1,C1:C20,-1,2)");
    model.evaluate();
    for cell in ["I1", "I2", "I3", "J3", "J4", "L1", "L2"] {
        assert_eq!(model._get_text(cell), "#N/A", "{cell}");
    }
    for cell in ["I4", "I5", "I6"] {
        assert_eq!(model._get_text(cell), "none", "{cell}");
    }
    assert_eq!(model._get_text("J1"), "7");
    assert_eq!(model._get_text("J2"), "7");
    assert_eq!(model._get_text("J5"), "4");
    assert_eq!(model._get_text("K1"), "4");
    assert_eq!(model._get_text("K2"), "5");
    assert_eq!(model._get_text("K3"), "4");
}
