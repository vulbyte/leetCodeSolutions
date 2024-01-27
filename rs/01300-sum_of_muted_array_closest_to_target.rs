/*
 Given an integer array arr and a target value target, return the integer value such that when we change all the integers larger than value in the given array to be equal to value, the sum of the array gets as close as possible (in absolute difference) to target.

In case of a tie, return the minimum such integer.

Notice that the answer is not neccesarilly a number from arr.



Example 1:

Input: arr = [4,9,3], target = 10
Output: 3
Explanation: When using 3 arr converts to [3, 3, 3] which sums 9 and that's the optimal answer.
Example 2:

Input: arr = [2,3,5], target = 10
Output: 5
Example 3:

Input: arr = [60864,25176,27249,21296,20204], target = 56803
Output: 11361


Constraints:

1 <= arr.length <= 104
1 <= arr[i], target <= 105
   */

fn helper(arr: &Vec<i32>, target: i32, mid: i32) -> i32 {
    let mut temp = 0;
    for &v in arr {
        temp += v.min(mid);
    }
    (target - temp).abs()
}

impl Solution {
    pub fn find_best_value(arr: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = 100010;
        while left + 2 < right {
            let lr1 = (left * 2 + right) / 3;
            let lr2 = (left + right * 2) / 3;

            let lr1v = helper(&arr, target, lr1);
            let lr2v = helper(&arr, target, lr2);

            if lr1v > lr2v {
                left = lr1;
            } else {
                right = lr2;
            }
        }

        let mut min = i32::max_value();
        let mut result = 0;
        for mid in (left..=right).rev() {
            let v = helper(&arr, target, mid);
            if v <= min {
                min = v;
                result = mid;
            }
        }

        result
    }
}
