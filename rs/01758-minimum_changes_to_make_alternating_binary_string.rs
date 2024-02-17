/*
You are given a string s consisting only of the characters '0' and '1'. In one operation, you can change any '0' to '1' or vice versa.

The string is called alternating if no two adjacent characters are equal. For example, the string "010" is alternating, while the string "0100" is not.

Return the minimum number of operations needed to make s alternating.



Example 1:

Input: s = "0100"
Output: 1
Explanation: If you change the last character to '1', s will be "0101", which is alternating.
Example 2:

Input: s = "10"
Output: 0
Explanation: s is already alternating.
Example 3:

Input: s = "1111"
Output: 2
Explanation: You need two operations to reach "0101" or "1010".


Constraints:

1 <= s.length <= 104
s[i] is either '0' or '1'.
*/

impl Solution {
    // FIRST ATTEMPT
    // pub fn min_operations(s: String) -> i32 {
    //     let mut needed_changes:i32 = 0;
    //
    //     fn toggle(c:char) -> char {
    //         if c == '1' {'0'}
    //         else {'1'}
    //     };
    //
    //     let mut exp_val:char = toggle(
    //         s.chars().next().unwrap()
    //     );
    //
    //
    //     //let filtered_chars: Vec<_> =
    //     println!("running filter");
    //     s.chars()
    //     .enumerate()
    //     .rev()
    //     .filter(|x| {
    //         if x.1 == exp_val {
    //             println!("CHANGE NEEDED: exp_val {}, entered val: {}", exp_val, x.1);
    //             exp_val = toggle(exp_val);
    //             needed_changes += 1;
    //
    //             true
    //         }
    //         else {
    //             println!("change not needed: exp_val {}, entered val: {}", exp_val, x.1);
    //             exp_val = toggle(exp_val);
    //
    //             false
    //         }
    //     })
    //     .map(|x| x.1)
    //     .collect::<Vec<_>>();
    //
    //     return needed_changes
    // }

    //Attempt 2
    // pub fn min_operations(s: String) -> i32 {
    //     let split = s.chars().enumerate().filter(|c| c == '1').map(|x| x.1).collect();
    //
    //     if split == s.len() /2 {
    //         return split
    //     }
    //
    //     return s.len()-split;
    // }

    // stolen valor
    pub fn min_operations(s: String) -> i32 {
        //let chars: Vec<char> = s.chars().collect();

        //prints each char as an arr, ie 0100 will return: ['0'. '1', '0', '0']
        let k = s
            .chars()
            //enumerate through list
            .enumerate()
            //take in the index and char,
            // take in 2 arguements (c=='1') && (i%2==1)
            // then preform a ^ (XOR) operation
            .filter(|&(i, c)| (c == '1') ^ (i % 2 == 1))
            //if XOR returns true, count++
            .count();
        //
        k.min(s.len() - k) as _
    }
}
