/*
 You are given an array nums of non-negative integers. nums is considered special if there exists a number x such that there are exactly x numbers in nums that are greater than or equal to x.

Notice that x does not have to be an element in nums.

Return x if the array is special, otherwise, return -1. It can be proven that if nums is special, the value for x is unique.
 */

impl Solution {
    pub fn special_array(mut nums: Vec<i32>) -> i32 {
        nums.sort();

        //iterate over each in nums
        // starting at 0, until nums.len
        for i in 0..=nums.len() {
            //create a paramater that filters through nums to see if x >= i
            //return as i32
            let count_greater = nums.iter().filter(|&&x| x >= i as i32).count() as i32;
            //if solution passes, return
            if count_greater == i as i32 {
                return i as i32;
            }
        }
        //no matching condition found
        return -1;
    }
}
