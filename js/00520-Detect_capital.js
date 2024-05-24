/*
520. detect capital
    
We define the usage of capitals in a word to be right when one of the following cases holds:

All letters in this word are capitals, like "USA".
All letters in this word are not capitals, like "leetcode".
Only the first letter in this word is capital, like "Google".
Given a string word, return true if the usage of capitals in it is right
*/

/* first try beating ~75% B) */

/**
 * @param {string} word
 * @return {boolean}
 */
var detectCapitalUse = function(word) {
    if (
        word == word.toUpperCase() ||
        word == word.toLowerCase() ||
        word.slice(1, word.size) == word.slice(1, word.size).toLowerCase()
    ) {
        return true;
    }
    else {
        return false;
    }
}
