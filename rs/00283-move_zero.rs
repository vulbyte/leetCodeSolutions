/*
Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.

Note that you must do this in-place without making a copy of the array.

Example 1:

Input: nums = [0,1,0,3,12]
Output: [1,3,12,0,0]
Example 2:

Input: nums = [0]
Output: [0]


Constraints:

1 <= nums.length <= 10^(4)
-2^(31) <= nums[i] <= 2^(31) - 1
*/

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let zero: i32 = 0;
        let mut amnt_of_0s = 0;
        nums.retain(|&x| {
            if x == zero {
                amnt_of_0s += 1;
                //return false for elements that are null
                return false;
            } else {
                //keep the element that is not zero
                true
            }
        });

        //add the zeros bak to the end of the vector
        nums.extend(std::iter::repeat(zero).take(amnt_of_0s));
        //end fn
        return;
    }
}
