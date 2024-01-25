/*
 There are n employees in a company, numbered from 0 to n - 1. Each employee i has worked for hours[i] hours in the company.

The company requires each employee to work for at least target hours.

You are given a 0-indexed array of non-negative integers hours of length n and a non-negative integer target.

Return the integer denoting the number of employees who worked at least target hours.

I SMASHED THIS ONE OOOO RAH
 */

impl Solution {
    pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
        let mut val = 0;
        for n in hours {
            if (n >= target) {
                val = val + 1;
            }
        }
        val
    }
}
