// You are given a string s, which contains stars *.

// In one operation, you can:
// 
// Choose a star in s.
// Remove the closest non-star character to its left, as well as remove the star itself.
// Return the string after all stars have been removed.
// 
// Note:
// 
// The input will be generated such that the operation is always possible.
// It can be shown that the resulting string will always be unique.
//  
// 
// Example 1:
// 
// Input: s = "leet**cod*e"
// Output: "lecoe"
// Explanation: Performing the removals from left to right:
// - The closest character to the 1st star is 't' in "leet**cod*e". s becomes "lee*cod*e".
// - The closest character to the 2nd star is 'e' in "lee*cod*e". s becomes "lecod*e".
// - The closest character to the 3rd star is 'd' in "lecod*e". s becomes "lecoe".
// There are no more stars, so we return "lecoe".
// Example 2:
// 
// Input: s = "erase*****"
// Output: ""
// Explanation: The entire string is removed, so we return an empty string.


// i attempted to do this solution in place without creating a new string because i thought it would be faster
// it was not faster but it was more memory efficent then ~90% of other solutions so that's a nice consolation prize 
class Solution {
public:
    string removeStars(string s) {
        int cToRem = 0; //chars to remove

        for (int i = s.length() -1; i >= 0; i--) {
            if (s[i] == '*') {
                cToRem += 1;
                s.erase(i, 1); 
            }
            else if (cToRem > 0) {
                cToRem -= 1;
                s.erase(i, 1); 
            } 
        }
        return s;
    }
}
