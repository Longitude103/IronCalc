#![allow(clippy::unwrap_used)]

use crate::test::util::new_empty_model;

#[test]
fn test_fn_averageifs_arguments() {
    let mut model = new_empty_model();

    // Incorrect number of arguments
    model._set("A1", "=AVERAGEIFS()");
    model._set("A2", "=AVERAGEIFS(B2:B9)");
    model._set("A3", "=AVERAGEIFS(B2:B9,C2:C9)");
    model._set("A4", "=AVERAGEIFS(B2:B9,C2:C9,\"=A*\",D2:D9)");

    // Correct (Sum everything in column 'B' if column 'C' starts with "A")
    model._set("A5", "=AVERAGEIFS(B2:B9,C2:C9,\"=A*\")");

    // Data
    model._set("B2", "5");
    model._set("B3", "4");
    model._set("B4", "15");
    model._set("B5", "22");
    model._set("B6", "=NA()");
    model._set("C2", "Apples");
    model._set("C3", "Bananas");
    model._set("C4", "Almonds");
    model._set("C5", "Yoni");
    model._set("C6", "Mandarin");

    model.evaluate();

    // Error (Incorrect number of arguments)
    assert_eq!(model._get_text("A1"), *"#ERROR!");
    assert_eq!(model._get_text("A2"), *"#ERROR!");
    assert_eq!(model._get_text("A3"), *"#ERROR!");
    assert_eq!(model._get_text("A4"), *"#ERROR!");

    // Correct
    assert_eq!(model._get_text("A5"), *"10");
}

// ── whole-column range and a spill beyond the used area ─────────────────────

// A whole-column range is clipped to the sheet's used area before it is
// walked. Inside an anchor, the column may be spilled on demand during the
// walk, past the bound the walk was clipped to. The skipped part is on
// record, so the spill restarts the pass and the second run sees it all.
#[test]
fn test_fn_averageifs_whole_column_sees_a_spill_beyond_the_used_area() {
    let cases = [
        ("AVERAGEIF(D:D,\">2\")", "4"),
        ("AVERAGEIFS(D:D,D:D,\">2\")", "4"),
    ];
    for (formula, expected) in cases {
        let mut model = new_empty_model();
        model._set("A1", &format!("=SEQUENCE(1,1,{formula})"));
        model._set("C1", "=SEQUENCE(3,1,10)");
        model._set("D1", "=SEQUENCE(5)");
        model.evaluate();
        assert_eq!(model._get_text("A1"), expected, "{formula}");
        assert_eq!(model.evaluation.restarts_in_last_evaluation, 1, "{formula}");
    }
}

// Like SUMIF, AVERAGEIF takes the average range as starting at its own
// top-left cell with the size of the criteria range. AVERAGEIFS needs equal
// sizes instead.
#[test]
fn test_fn_averageif_resizes_average_range() {
    let mut model = new_empty_model();
    for row in 1..=4 {
        model._set(&format!("A{row}"), &row.to_string());
        model._set(&format!("B{row}"), &(row * 10).to_string());
    }
    model._set("D1", "=AVERAGEIF(A1:A4,\">1\",B1)");
    model._set("D2", "=AVERAGEIF(A1:A4,\">1\",B1:B2)");
    model._set("D3", "=AVERAGEIF(A1:A4,\">1\",B1:B4)");
    model._set("D4", "=AVERAGEIF(A1:A4,\">1\",B2:B3)");
    // The shape comes from the criteria range, also across columns
    model._set("D5", "=AVERAGEIF(A1:A2,\">0\",B1:C4)");
    model._set("D6", "=AVERAGEIF(A1:B2,\">0\",B1)");
    // AVERAGEIFS does not resize
    model._set("D7", "=AVERAGEIFS(B1,A1:A4,\">1\")");
    model.evaluate();

    assert_eq!(model._get_text("D1"), *"30");
    assert_eq!(model._get_text("D2"), *"30");
    assert_eq!(model._get_text("D3"), *"30");
    // B2:B5 against A1:A4: A2..A4 pass, so B3 and B4 are averaged
    assert_eq!(model._get_text("D4"), *"35");
    assert_eq!(model._get_text("D5"), *"15");
    assert_eq!(model._get_text("D6"), *"15");
    assert_eq!(model._get_text("D7"), *"#DIV/0!");
}
