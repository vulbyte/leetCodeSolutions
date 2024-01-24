/*
You are given an integer array deck where deck[i] represents the number written on the ith card.

Partition the cards into one or more groups such that:

Each group has exactly x cards where x > 1, and
All the cards in one group have the same integer written on them.
Return true if such partition is possible, or false otherwise.

 

Example 1:

Input: deck = [1,2,3,4,4,3,2,1]
Output: true
Explanation: Possible partition [1,1],[2,2],[3,3],[4,4].
Example 2:

Input: deck = [1,1,1,2,2,2,3,3]
Output: false
Explanation: No possible partition.
*/

/**
 * @param {number[]} deck
 * @return {boolean}
 */
var hasGroupsSizeX = function(deck) {
    const rec = new Map()
    deck.forEach(val => {
        rec.set(val, (rec.get(val) ?? 0) + 1)
    })
    const vals = [...rec.values()]
    const largestCommonDivider = (a, b) => {
        let num1 = a, num2 = b
        while (num2 !== 0) {
            const div = num1 % num2
            num1 = num2
            num2 = div
        }
        return num1
    }
    return vals.reduce((cd, val) => largestCommonDivider(cd, val), vals[0]) >= 2;
};
