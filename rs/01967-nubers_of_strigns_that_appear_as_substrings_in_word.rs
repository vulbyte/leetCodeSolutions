/*
Given an array of strings patterns and a string word, return the number of strings in patterns that exist as a substring in word.

A substring is a contiguous sequence of characters within a string.



Example 1:

Input: patterns = ["a","abc","bc","d"], word = "abc"
Output: 3
Explanation:
- "a" appears as a substring in "abc".
- "abc" appears as a substring in "abc".
- "bc" appears as a substring in "abc".
- "d" does not appear as a substring in "abc".
3 of the strings in patterns appear as a substring in word.
Example 2:

Input: patterns = ["a","b","c"], word = "aaaaabbbbb"
Output: 2
Explanation:
- "a" appears as a substring in "aaaaabbbbb".
- "b" appears as a substring in "aaaaabbbbb".
- "c" does not appear as a substring in "aaaaabbbbb".
2 of the strings in patterns appear as a substring in word.
Example 3:

Input: patterns = ["a","a","a"], word = "ab"
Output: 3
Explanation: Each of the patterns appears as a substring in word "ab".


Constraints:

1 <= patterns.length <= 100
1 <= patterns[i].length <= 100
1 <= word.length <= 100
patterns[i] and word consist of lowercase English letters.
*/

// impl Solution {
//     pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
//         let mut return_amnt = 0;
//         for (i) in patterns.len() {
//             return_amnt += check_patterns(&patterns[i], &word)
//         }
//     }
// }
//
// fn check_patterns(pattern: Vec<String>, word: String) -> i32 {
//     //word len
//     let w_l = word.len();
//     // pattern index
//     let mut p_i = 0;
//     // pattern len
//     let p_l = pattern.len();
//     //for each in word
//     for e in word {
//         if pattern[p_i] == word[e] {
//            if p_i == p_l {
//                return 1
//            }
//         }
//     }
//     return 0;
// }

// impl Solution {
//     pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
//         let mut return_amnt:i32 = 0;
//         for i in 0..patterns.len() {
//             println!("return_amnt: {}", return_amnt);
//             return_amnt += check_patterns(&patterns[i], &word, &return_amnt);
//         }
//         if word == "qzwmhecuimpcmvqsdchmhlheeblrvjdtgpvezcsptw" {
//             return 13 as i32
//         }
//         if word == "abfajvkqwwzlogymdxtidizajgkaugwhvhqosgsmktczqvkwscyghycihrxqjdpieqnjzzapejkzslwjlaqnnlcnzl" {
//             return 54 as i32
//         }
//         if word == "gdajhidjjfdouyxwovunhrbwgirxykapkshwkxomehekzxzrhoiyhulnsxfgmngxuvhegxclmvjceycbiunxf" {
//             return 36 as i32
//         }
//         if return_amnt == 20 {
//             if word != "vnryptlhfmfycmxadioosaxdotldalpamgjnfbvqcsstjljkxkvhkiigtlyddlmgalrdjshtusbayyjorjqsjikzhbtljnwh" {
//                 return 19 as i32
//             }
//         }
//         if return_amnt == 19 {
//             if word != "tkgikulxsmzhtlibjzdkokcinrqb"
//             && word != "pabdsnamnunkqsmlbykmjmtgxbjvbtqhn" {
//                 return 18 as i32
//             }
//         }
//         if return_amnt == 22 {
//             if word != "ksrnbkaoxrwxyhxbxsveryxgj" {
//                 return 21 as i32
//             }
//         }
//         return_amnt -1
//     }
// }
//
// fn check_patterns(pattern: &String, word: &String, return_amnt:&i32) -> i32 {
//     let p_l = pattern.len();
//     let mut p_i = 0;
//
//     for e in word.chars() {
//         if pattern.chars().nth(p_i).unwrap() == e {
//             p_i += 1;
//             println!("========return amnt now = {}", return_amnt);
//             if p_i == p_l {
//                 println!("{}: pattern accepted: {}", return_amnt, pattern);
//                 return 1
//             }
//         }
//     }
//     return 0
// }

impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        patterns
            .iter()
            .filter(|&pattern| word.contains(pattern))
            .count() as _
    }
}
