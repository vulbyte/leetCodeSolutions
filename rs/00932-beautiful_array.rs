/*
    this quesiton  makes no sense to me and doesn't make sense. so for this one i had to use a provided solution because the question doesn't make much if any sense to me

An array nums of length n is beautiful if:

nums is a permutation of the integers in the range [1, n].
For every 0 <= i < j < n, there is no index k with i < k < j where 2 * nums[k] == nums[i] + nums[j].
Given the integer n, return any beautiful array nums of length n. There will be at least one valid answer for the given n.



Example 1:

Input: n = 4
Output: [2,1,4,3]
Example 2:

Input: n = 5
Output: [3,1,2,5,4]
*/

impl Solution {
    pub fn beautiful_array(n: i32) -> Vec<i32> {
        let mut answer = Vec::with_capacity(n as usize);
        answer.push(1);
        while answer.len() < n as usize {
            answer = answer
                .iter()
                .map(|m| m * 2 - 1)
                .chain(answer.iter().map(|m| m * 2))
                .filter(|&m| m <= n)
                .collect();
        }
        answer
    }
}
