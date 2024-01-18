/*Given two integer arrays nums1 and nums2, return an array of their intersection. Each element in the result must be unique and you may return the result in any order.



Example 1:

Input: nums1 = [1,2,2,1], nums2 = [2,2]
Output: [2]
Example 2:

Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
Output: [9,4]
Explanation: [4,9] is also accepted.


Constraints:

1 <= nums1.length, nums2.length <= 1000
0 <= nums1[i], nums2[i] <= 1000
*/

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        //to my understanding, i need to iderate over 2 arrays
        //then teturn what values are shared by the 2 arrays
        //ie, if inputs are:
        // [1, 2, 3, 4] && [2, 4, 6, 8]
        // the return should be:
        // [2, 4]
        MergeAndCull(nums1, nums2)
    }
}

fn MergeAndCull(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    //let mut returnVec:Vec<i32> = vec![];
    // THIS WAS MY VERSION
    //for n in &nums1 {
    //    for m in &nums2 {
    //        if (n == m) {
    //            if returnVec.contains(n) {
    //                break;
    //            } else {
    //                returnVec.push(*n);
    //            }
    //        }
    //    }
    //}
    //returnVec

    //THIS IS THE "fast" solution
    //init a vec of 1001 elements, with the first value being "false"
    let mut seen = vec![false; 1001];

    // for each value in nums1, flip the value in seen to true
    for n in nums1 {
        seen[n as usize] = true;
    }

    //init return vector
    let mut result = vec![];
    //for each number in nums2,
    for num in nums2 {
        // if seen[num] == true (not exactly this, but communicates the idea)
        if seen[num as usize] {
            //add to result
            result.push(num);
            seen[num as usize] = false;
        }
    }
    result
}
