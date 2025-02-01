//! This library provides a common algorithm for calculating the Levenshtein distance
//! between two strings, i.e., the minimum number of single-character edits (insertions,
//! deletions, or substitutions) required to change one string into the other. There is
//! a single public function, `edit_distance`, which takes two string references
//! (`&str`) and a `bool` flag indicating whether the strings can be treated as
//! ASCII-only. If the flag is set to false—the safer option—the strings will operated
//! on as sequences of `char`s, i.e., 32-bit Unicode scalar values. This does involve
//! more allocation and probably a longer running time than the ASCII case. The return
//! value of `edit_distance`, in any event, is the Levenshtein distance as `u32`.

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::cast_possible_truncation)]

/// Returns the Levenshtein distance (`u32`) between two strings (`&str`), `a` and `b`.
/// The `ascii` flag indicates whether the strings can be treated as ASCII-only.
#[must_use]
pub fn edit_distance(a: &str, b: &str, ascii: bool) -> u32 {
    if a == b {
        0
    } else if ascii {
        min_distance(a.as_bytes(), b.as_bytes())
    } else {
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();
        min_distance(&a_chars, &b_chars)
    }
}

fn min_distance<T: PartialEq>(a: &[T], b: &[T]) -> u32 {
    let m = a.len();
    let n = b.len();

    if m == 0 {
        return n as u32;
    }
    if n == 0 {
        return m as u32;
    }

    let mut dp: Vec<u32> = (1..).take(m).collect();

    for (row, char_b) in b.iter().enumerate() {
        let mut left = row as u32;
        let mut diag = row as u32;

        for (col, char_a) in a.iter().enumerate() {
            let insert = left + 1;
            let delete = dp[col] + 1;
            let subst = if char_a == char_b { diag } else { diag + 1 };

            let min_cost = insert.min(delete).min(subst);

            diag = dp[col]; // Save for next iteration of inner loop
            left = min_cost;
            dp[col] = min_cost;
        }
    }

    // Return final value from dp array
    dp[m - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sitting_kitten() {
        assert_eq!(edit_distance("sitting", "kitten", true), 3);
    }

    #[test]
    fn ascii_no_difference() {
        let a = "If I were a wise man";
        let b = "I would do my part";
        let ascii_dist = edit_distance(a, b, true);
        let unicode_dist = edit_distance(a, b, false);
        assert_eq!(ascii_dist, unicode_dist);
    }

    #[test]
    fn accents_difference() {
        let a = "ʿAlī ibn Abī Ṭālib";
        let b = "ʿUthmān ibn ʿAffān";
        let ascii_dist = edit_distance(a, b, true);
        let unicode_dist = edit_distance(a, b, false);
        assert_ne!(ascii_dist, unicode_dist);
    }

    #[test]
    fn shahnama_unicode() {
        let a = "شاهنامه";
        let b = "شهنامه";
        assert_eq!(edit_distance(a, b, false), 1);
    }

    #[test]
    fn shahnama_ascii() {
        let a = "شاهنامه";
        let b = "شهنامه";
        assert_eq!(edit_distance(a, b, true), 2);
    }
}
