/*
Given an array of strings words, return the first palindromic string in the array. If there is no such string, return an empty string "".

A string is palindromic if it reads the same forward and backward.



Example 1:

Input: words = ["abc","car","ada","racecar","cool"]
Output: "ada"
Explanation: The first string that is palindromic is "ada".
Note that "racecar" is also palindromic, but it is not the first.
Example 2:

Input: words = ["notapalindrome","racecar"]
Output: "racecar"
Explanation: The first and only string that is palindromic is "racecar".
Example 3:

Input: words = ["def","ghi"]
Output: ""
Explanation: There are no palindromic strings, so the empty string is returned.


Constraints:

1 <= words.length <= 100
1 <= words[i].length <= 100
words[i] consists only of lowercase English letters.
*/

impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        // return words.char()
        //     .enumerate()
        //     .filter(|&(w)|{
        //         if w == w.rev() {
        //             return
        //         }
        //     })

        for w in words.iter() {
            if w == &w.chars().rev().collect::<String>() {
                return w.clone();
            }
        }
        return "".to_string();
    }
}
