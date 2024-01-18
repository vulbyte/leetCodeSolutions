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

1 <= nums.length <= 104
-231 <= nums[i] <= 231 - 1
*/

//EXAMPLE
// Input: nums = [0,1,0,3,12]
// Output: [1,3,12,0,0]

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        //BELOW WAS FOR TESTING
        ZerosToEnd(nums);
        println!("arraged list: {:?}", nums);

        //ZerosToEnd(nums.to_vec());
    }
}

fn ZerosToEnd(input: &mut Vec<i32>) -> &mut Vec<i32> {
    let mut zeros = Vec::new();

    input.retain(|&n| {
        if n == 0 {
            zeros.push(n);
            false
        } else {
            true
        }
    });

    input.append(&mut zeros); //concat vectors

    input //return
}
