# edit-distance

After doing the [LeetCode problem](https://leetcode.com/problems/edit-distance/) on the edit distance of two strings—which in this case refers to the [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance)—I became quite enamored of the algorithm and wanted to write my own implementation in Rust. That's all this is.

I wrote my version such that the function `edit_distance` takes three arguments: two `&str` and a `bool` flag for "ASCII mode." When this flag is set, the algorithm will work directly with the bytes of the strings, rather than building `Vec`s of their 32-bit (i.e., UTF-32) `char` values. This should be faster and avoid some allocation, as far as I understand. I do wonder if there might be a more efficient approach for the Unicode case.

Either way, the return value is the Levenshtein distance as `u32`. It may be worth noting that input strings of length greater than `u32::MAX` will not yield correct results—though I can't imagine how that problem would ever arise in practice.
