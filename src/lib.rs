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
#![allow(clippy::cast_possible_truncation, clippy::needless_range_loop)]

/// Returns the Levenshtein distance (`u32`) between two strings (`&str`), `a` and `b`.
/// The `ascii` flag indicates whether the strings can be treated as ASCII-only.
#[must_use]
pub fn edit_distance(a: &str, b: &str, ascii: bool) -> u32 {
    if ascii {
        min_distance(a.as_bytes(), b.as_bytes())
    } else {
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();
        min_distance(&a_chars, &b_chars)
    }
}

fn min_distance<T: PartialEq>(a: &[T], b: &[T]) -> u32 {
    if a.is_empty() {
        return b.len() as u32;
    }
    if b.is_empty() {
        return a.len() as u32;
    }

    let m = a.len();
    let n = b.len();

    let mut dp: Vec<Vec<u32>> = vec![vec![0; n + 1]; m + 1];

    // Initialize first row and column
    for i in 1..=m {
        dp[i][0] = i as u32;
    }
    for j in 1..=n {
        dp[0][j] = j as u32;
    }

    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1]; // Match; no increase
            } else {
                let insert = dp[i][j - 1];
                let delete = dp[i - 1][j];
                let replace = dp[i - 1][j - 1];

                // Update based on cheapest operation
                dp[i][j] = 1 + insert.min(delete).min(replace);
            }
        }
    }

    // Return value from bottom right cell
    dp[m][n]
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
