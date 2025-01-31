#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::cast_possible_truncation, clippy::needless_range_loop)]

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
