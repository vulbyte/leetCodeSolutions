// Given an integer array nums that does not contain any zeros, find the largest positive integer k such that -k also exists in the array.
//
// Return the positive integer k. If there is no such integer, return -1.
//
//
//
// Example 1:
//
// Input: nums = [-1,2,-3,3]
// Output: 3
// Explanation: 3 is the only valid k we can find in the array.
// Example 2:
//
// Input: nums = [-1,10,6,7,-7,1]
// Output: 7
// Explanation: Both 1 and 7 have their corresponding negative values in the array. 7 has a larger value.
// Example 3:
//
// Input: nums = [-10,8,6,7,-2,-3]
// Output: -1
// Explanation: There is no a single valid k, we return -1.
//
//
// Constraints:
//
// 1 <= nums.length <= 1000
// -1000 <= nums[i] <= 1000
// nums[i] != 0

/* solution published to leetcode:
if you find this helpful, please consider giving this non-ai write up an upvote

Intuition
this solution is based on not needing to do any complex searching, and instead relys on logic

Approach
assuming the list is going to be quicker to sort then it will be to binary search though, sort.
after sorting, think of "burning the candle from both ends" until we can find a value that matches.
because we're only ever searching for the largest value, unless there's no pairs this is relitively quick.

Complexity
Time complexity:
complexity of O(n), where n is the length of the array

Space complexity:
additional space used is O(n+1) because the space being used is = to the array with the addition of 1 varable (that isn't really needed, but i believe speeds up calc time by preventing realocated space)
*/

// Code
impl Solution {
    pub fn find_max_k(mut nums: Vec<i32>) -> i32 {
        //clone the list so we can edit it
        let mut sorted_nums = nums.clone();
        //sort the list
        sorted_nums.sort();

        // add the inverse of n as a value to cache the space in memory and make the code a touch more readable
        let mut inverse: i32 = 0;

        //for (reference to)n, iterate over the list, in reverse
        for &n in sorted_nums.iter().rev() {
            //find the inverse value of n
            inverse = -n;

            //if the sorted arra contains the number, that is the largest number, so return
            if sorted_nums.contains(&inverse) {
                return n;
            }
        }
        //if no other values can be found, exit with err value
        return -1;
    }
}
// that's it, enjoy!

// vulbyte 2024/02/06 ymd
