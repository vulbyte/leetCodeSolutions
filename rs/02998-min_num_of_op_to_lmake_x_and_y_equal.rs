/*
 You are given two positive integers x and y.

In one operation, you can do one of the four following operations:

Divide x by 11 if x is a multiple of 11.
Divide x by 5 if x is a multiple of 5.
Decrement x by 1.
Increment x by 1.
Return the minimum number of operations required to make x and y equal.
 */

//i took this because i have no idea what deapth first search is, so i'll need ot this

impl Solution {
    pub fn minimum_operations_to_make_equal(x: i32, y: i32) -> i32 {
        let (x, y) = (x as usize, y as usize);
        let mut memo = vec![-1; x + 11];
        dp(x, y, &mut memo)
    }
}

fn dp(x: usize, y: usize, memo: &mut Vec<i32>) -> i32 {
    if x <= y { return (y - x) as i32 };
    if memo[x] != -1 { return memo[x] };
    
    memo[x] = 1_000_000_007;

    if x % 11 == 0 {
        memo[x] = memo[x].min(1 + dp(x / 11, y, memo));
    } if x % 5 == 0 {
        memo[x] = memo[x].min(1 + dp(x / 5, y, memo));
    } if x > 0 {
        memo[x] = memo[x].min(1 + dp(x - 1, y, memo));
    } if x < memo.len() - 1 {
        memo[x] = memo[x].min(1 + dp(x + 1, y, memo));
    }

    memo[x]

