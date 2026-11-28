use crate::{calc_result::Range, expressions::types::CellReferenceIndex};

/// It returns the closest cell from cell_reference to range in the same column/row
/// Examples
///  * i_i(B5, A2:A9) -> B5
///  * i_i(B5, A7:A9) -> None
///  * i_i(B5, A2:D2) -> B2
///  * i_i(B2, A2:D2) -> B2 (level with a one-row range)
///  * i_i(B2, A1:D3) -> None (a block, not a row or a column)
///
/// Excel-parity, cross-sheet:
///  * a 1x1 range always dereferences, regardless of the consuming cell's
///    position or sheet (Excel: `=@Sheet2!D5` and OFFSET/INDIRECT results in
///    scalar context return the single cell's value);
///  * row/column-aligned intersection works across sheets — the intersection
///    happens in the range's own coordinate space (Excel: `=Data!D1:D5`
///    entered on another sheet's C3 returns Data!D3). The resulting reference
///    lives on the RANGE's sheet.
pub(crate) fn implicit_intersection(
    cell_reference: &CellReferenceIndex,
    range: &Range,
) -> Option<CellReferenceIndex> {
    let left = &range.left;
    let right = &range.right;
    // A range spanning two sheets never intersects.
    if left.sheet != right.sheet {
        return None;
    }
    let sheet = left.sheet;
    // Single-cell range: always dereference (position- and sheet-independent).
    if left.row == right.row && left.column == right.column {
        return Some(CellReferenceIndex {
            sheet,
            row: left.row,
            column: left.column,
        });
    }
    let row = cell_reference.row;
    let column = cell_reference.column;
    // A one-column range intersects by row and a one-row range by column. Checking which way the range runs first matters when
    // the cell is level with a one-row range: `=Data!B6:J6` entered in B6 of another sheet is Data!B6, not an error.
    if left.column == right.column && row >= left.row && row <= right.row {
        return Some(CellReferenceIndex {
            sheet,
            row,
            column: left.column,
        });
    }
    if left.row == right.row && column >= left.column && column <= right.column {
        return Some(CellReferenceIndex {
            sheet,
            row: left.row,
            column,
        });
    }
    None
}
