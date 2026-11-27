use std::cmp::Ordering;

use crate::{calc_result::CalcResult, expressions::types::CellReferenceIndex, model::Model};

use super::util::compare_values;

// NOTE: We don't know how Excel exactly implements binary search internally.
// This means that if the values on the lookup range are not in order our results and Excel's will differ

// Assumes values are in ascending order, returns matching index or the largest value smaller than target.
// Returns None if target is smaller than the smaller value.
pub(crate) fn binary_search_or_smaller<T: Ord>(target: &T, array: &[T]) -> Option<i32> {
    // We apply binary search leftmost for value in the range
    let n = array.len();
    if n == 0 {
        return None;
    }
    let mut l = 0;
    let mut r = n;
    while l < r {
        let m = (l + r) / 2;
        if &array[m] < target {
            l = m + 1;
        } else {
            r = m;
        }
    }
    if l == n {
        return Some((l - 1) as i32);
    }
    // Now l points to the leftmost element
    if &array[l] == target {
        return Some(l as i32);
    }
    // If target is less than the minimum return None
    if l == 0 {
        return None;
    }
    Some((l - 1) as i32)
}

// Assumes values are in ascending order, returns matching index or the smaller value larger than target.
// Returns None if target is smaller than the smaller value.
pub(crate) fn binary_search_or_greater<T: Ord>(target: &T, array: &[T]) -> Option<i32> {
    let mut l = 0;
    let mut r = array.len();
    while l < r {
        let mut m = (l + r) / 2;
        match &array[m].cmp(target) {
            Ordering::Less => {
                l = m + 1;
            }
            Ordering::Greater => {
                r = m;
            }
            Ordering::Equal => {
                while m > 1 {
                    if &array[m - 1] == target {
                        m -= 1;
                    } else {
                        break;
                    }
                }
                return Some(m as i32);
            }
        }
    }
    // If target is larger than the maximum return None
    if r == array.len() {
        return None;
    }
    // Now r points to the rightmost element
    Some(r as i32)
}

// Assumes values are in descending order
pub(crate) fn binary_search_descending_or_smaller<T: Ord>(target: &T, array: &[T]) -> Option<i32> {
    let n = array.len();
    let mut l = 0;
    let mut r = n;
    while l < r {
        let m = (l + r) / 2;
        let mut index = n - m - 1;
        match &array[index].cmp(target) {
            Ordering::Less => {
                l = m + 1;
            }
            Ordering::Greater => {
                r = m;
            }
            Ordering::Equal => {
                while index < n - 1 {
                    if &array[index + 1] == target {
                        index += 1;
                    } else {
                        break;
                    }
                }
                return Some(index as i32);
            }
        }
    }
    if l == 0 {
        return None;
    }
    Some((n - l) as i32)
}

// Assumes values are in descending order, returns matching index or the smaller value larger than target.
// Returns None if target is smaller than the smaller value.
pub(crate) fn binary_search_descending_or_greater<T: Ord>(target: &T, array: &[T]) -> Option<i32> {
    let n = array.len();
    let mut l = 0;
    let mut r = n;
    while l < r {
        let m = (l + r) / 2;
        let mut index = n - m - 1;
        match &array[index].cmp(target) {
            Ordering::Less => {
                l = m + 1;
            }
            Ordering::Greater => {
                r = m;
            }
            Ordering::Equal => {
                while index < n - 1 {
                    if &array[index + 1] == target {
                        index += 1;
                    } else {
                        break;
                    }
                }
                return Some(index as i32);
            }
        }
    }
    if r == n {
        return None;
    }
    Some((n - r - 1) as i32)
}

impl<'a> Model<'a> {
    /// Returns an array with the list of cell values in the range
    pub(crate) fn prepare_array(
        &mut self,
        left: &CellReferenceIndex,
        right: &CellReferenceIndex,
        is_row_vector: bool,
    ) -> Vec<CalcResult> {
        let n = if is_row_vector {
            right.row - left.row
        } else {
            right.column - left.column
        } + 1;
        let mut result = vec![];
        for index in 0..n {
            let row;
            let column;
            if is_row_vector {
                row = left.row + index;
                column = left.column;
            } else {
                column = left.column + index;
                row = left.row;
            }
            let value = self.evaluate_cell(CellReferenceIndex {
                sheet: left.sheet,
                row,
                column,
            });
            result.push(value);
        }
        result
    }
}

fn is_blank(value: &CalcResult) -> bool {
    matches!(value, CalcResult::EmptyCell | CalcResult::EmptyArg)
}

/// Whether a lookup value is an empty cell. Excel returns #N/A when an
/// approximate (or binary) search is asked for one.
pub(crate) fn is_blank_target(target: &CalcResult) -> bool {
    is_blank(target)
}

/// Runs `search` on `array` ignoring empty cells, like Excel does for approximate
/// matches (a range larger than its data, with trailing blank rows, is common).
/// The index returned by `search` is mapped back to a position in `array`.
/// Returns `None` if every cell is empty.
pub(crate) fn ignoring_blanks(
    array: &[CalcResult],
    search: impl Fn(&[CalcResult]) -> Option<i32>,
) -> Option<i32> {
    // Skip the leading and trailing blanks without copying anything
    let start = array.iter().position(|v| !is_blank(v))?;
    let end = array.iter().rposition(|v| !is_blank(v))? + 1;
    let trimmed = &array[start..end];
    if !trimmed.iter().any(is_blank) {
        return search(trimmed).map(|l| l + start as i32);
    }
    // Empty cells in the middle: search only the cells that have a value
    let (positions, values): (Vec<usize>, Vec<CalcResult>) = trimmed
        .iter()
        .enumerate()
        .filter(|(_, v)| !is_blank(v))
        .map(|(i, v)| (start + i, v.clone()))
        .unzip();
    search(&values).map(|l| positions[l as usize] as i32)
}

/// Old style binary search over an already materialized vector of values.
/// Returns the index of the matching element or the largest element smaller
/// than `target`. Returns `-2` if `target` is smaller than every element.
/// Empty cells are ignored, and an empty `target` is never found.
pub(crate) fn binary_search_on_array(target: &CalcResult, array: &[CalcResult]) -> i32 {
    if is_blank(target) {
        return -2;
    }
    ignoring_blanks(array, |array| {
        // We apply binary search leftmost for value in the array
        let mut l = 0;
        let mut r = array.len();
        while l < r {
            let m = (l + r) / 2;
            match compare_values(&array[m], target) {
                -1 => {
                    l = m + 1;
                }
                1 => {
                    r = m;
                }
                _ => {
                    return Some(m as i32);
                }
            }
        }
        // If target is less than the minimum return #N/A
        if l == 0 {
            return None;
        }
        // Now l points to the leftmost element
        Some((l - 1) as i32)
    })
    .unwrap_or(-2)
}
