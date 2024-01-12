//js doc
/**
 * @param {string} s
 * @return {number}
 */
var lengthOfLongestSubstring = function(s) {
    //create set
    let set = new Set();
    // position in the index, will jump if pattern broke
    let left = 0;
    let maxSize = 0;

    //this saved needing to loop through the code if == 0 or 1
    if (s.length <= 1) { return s.length }

    for (let i = 0; i < s.length; i++) {
        //while index is less than s.length On^2 can be optimized out
        while (set.has(s[i])) {
            //this clears an existing char from the set, to prevent douplicaitons
            set.delete(s[left]);
            left++;
        }
        //idk man lol
        set.add(s[i]);
        maxSize = Math.max(maxSize, i - left + 1);
    }

    return maxSize;
};
