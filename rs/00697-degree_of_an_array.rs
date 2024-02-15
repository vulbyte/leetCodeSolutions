//this problem honestly makes no sense to me
//so i'm just yoinking a solution because wtf

/*
Given a non-empty array of non-negative integers nums, the degree of this array is defined as the maximum frequency of any one of its elements.

Your task is to find the smallest possible length of a (contiguous) subarray of nums, that has the same degree as nums.
*/

fn f(x: i32, nums: &[i32]) -> i32 {
    let n = nums.len();
    let a = nums.iter().position(|&y| y == x).unwrap();
    let b = n - nums.iter().rev().position(|&y| y == x).unwrap();
    (b - a) as _
}

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut fq = std::collections::HashMap::new();

        for &x in nums.iter() {
            *fq.entry(x).or_default() += 1
        }
        let deg: i32 = *fq.values().max().unwrap();
        fq.keys()
            .filter(|&x| fq[x] == deg)
            .map(|&x| f(x, &nums))
            .min()
            .unwrap()
    }
}
