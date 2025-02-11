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

use std::mem::swap;

/// Returns the Levenshtein distance (`u32`) between two strings (`&str`), `a` and `b`.
/// The `ascii` flag indicates whether the strings can be treated as ASCII-only.
#[must_use]
pub fn edit_distance(a: &str, b: &str, ascii: bool) -> u32 {
    // Handle edge cases as early as possible
    if a == b {
        return 0;
    }

    if a.is_empty() {
        if ascii {
            return b.len() as u32;
        }
        return b.chars().count() as u32;
    }

    if b.is_empty() {
        if ascii {
            return a.len() as u32;
        }
        return a.chars().count() as u32;
    }

    if ascii {
        min_distance(a.as_bytes(), b.as_bytes())
    } else {
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();
        min_distance(&a_chars, &b_chars)
    }
}

fn min_distance<T: PartialEq>(a: &[T], b: &[T]) -> u32 {
    // We already know: strings are not equal; neither string is empty
    let m = a.len();

    // "Previous row" is initialized with the base case:
    // the distance from an empty string to each prefix of `a`.
    let mut dp_prev: Vec<u32> = (0..=m as u32).collect();
    let mut dp_curr: Vec<u32> = vec![0; m + 1];

    for (i, b_char) in b.iter().enumerate() {
        // i.e., cost of deleting all chars from `b` up to this point
        dp_curr[0] = i as u32 + 1;

        for j in 1..=m {
            if a[j - 1] == *b_char {
                dp_curr[j] = dp_prev[j - 1];
                continue;
            }

            let insert = dp_curr[j - 1] + 1;
            let delete = dp_prev[j] + 1;
            let substitute = dp_prev[j - 1] + 1;

            dp_curr[j] = insert.min(delete).min(substitute);
        }

        // `curr` becomes `prev` for next iteration
        swap(&mut dp_prev, &mut dp_curr);
    }

    dp_prev[m]
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

    #[test]
    fn empty_ascii() {
        let a = "levenshtein";
        let b = "";
        assert_eq!(edit_distance(a, b, true), 11);
    }

    #[test]
    fn empty_unicode() {
        let a = "maḥmūd";
        let b = "";
        assert_eq!(edit_distance(a, b, false), 6);
    }

    #[test]
    fn equal_regardless() {
        let a = "Ghiyāth al-Dīn";
        let b = "Ghiyāth al-Dīn";
        assert_eq!(edit_distance(a, b, true), 0);
    }
}
