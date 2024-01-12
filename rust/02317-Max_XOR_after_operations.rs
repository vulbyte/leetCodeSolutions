/*
You are given a 0-indexed integer array nums. In one operation, select any non-negative integer x and an index i, then update nums[i] to be equal to nums[i] AND (nums[i] XOR x).

Note that AND is the bitwise AND operation and XOR is the bitwise XOR operation.

Return the maximum possible bitwise XOR of all elements of nums after applying the operation any number of times.



Example 1:

Input: nums = [3,2,4,6]
Output: 7
Explanation: Apply the operation with x = 4 and i = 3, num[3] = 6 AND (6 XOR 4) = 6 AND 2 = 2.
Now, nums = [3, 2, 4, 2] and the bitwise XOR of all the elements = 3 XOR 2 XOR 4 XOR 2 = 7.
It can be shown that 7 is the maximum possible bitwise XOR.
Note that other operations may be used to achieve a bitwise XOR of 7.
Example 2:

Input: nums = [1,2,3,9,2]
Output: 11
Explanation: Apply the operation zero times.
The bitwise XOR of all the elements = 1 XOR 2 XOR 3 XOR 9 XOR 2 = 11.
It can be shown that 11 is the maximum possible bitwise XOR.


Constraints:

1 <= nums.length <= 105
0 <= nums[i] <= 108
*/

// i had no idea what this one was, so the comments aren't me explaining it, but me attempting to understand what happened

//impl is basically a rust method, in this case named "solution"
impl Solution {
    //public (global) function
    //named maximum_xor
    //takes in var nums
    //which is a vec made up of i32's
    //returns a i32
    pub fn maximum_xor(nums: Vec<i32>) -> i32 {
        //on nums, implement the
        //into_iter fn, which creates a non-consuming iter, which means per each item
        nums.into_iter()
            // the fold arguement takes in 2 args
            // the first being teh initial value
            // the 2nd being a closure with 2 arguments.
            // one being a "accumulator"
            // the other being an element.
            // the closure returns the value that the accumulator shoudl have for the next iteration.
            .fold(
                //starting at a value of 0
                0,
                //for each element, preforma  bitwise | operation with ACCumulator,
                |acc, x| acc | x,
            )
    }
}

//the breakdown is, with the input values of:
// starting with 0 or bitwise: 0000
// [3,2,4,6] or bitwise: [ 0011, 0010, 0100, 0110 ]

// 0 | 3 = 3 because 0000 | 0011 = 0011
// then
// 3 | 2 = 3 because 0011 | 0010 = 0011
// then
// 3 | 4 = 7 because 0011 | 0100 = 0111
// then
// 7 | 6 = 7 because 0111 | 0110 = 0111

// this makes sense now but holy shiza
