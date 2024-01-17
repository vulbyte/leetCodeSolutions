// struct Solution;
//
// impl Solution {
//     pub fn average_value(nums: Vec<i32>) -> i32 {
//         let mut sum = 0;
//         let mut count = 0;
//         for n in nums.iter() {
//             if n % 6 == 0 {
//                 sum = sum + n;
//                 count = count + 1;
//             };
//         }
//         return sum / count;
//     }
// }
//
// fn main() {
//     let test1: [i32; 6] = [1, 3, 6, 10, 12, 15];
//     let test2: [i32; 5] = [1, 2, 4, 7, 10];
//
//     let mut result = Solution::average_value(test1);
//
//     println!("average value: {}", Solution::average_value(test1));
//
//     println!("average value: {}", Solution::average_value(test2));
// }



struct Solution;

impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut count = 0;
        for n in nums.iter() {
            if n % 6 == 0 {
                sum = sum + n;
                count = count + 1;
            };
        }
        if count == 0 {
            // Avoid division by zero
            return 0.0 as i32;
        }
        return (sum as f64 / count as f64) as i32;
    }
}

fn main() {
    let test1: [i32; 6] = [1, 3, 6, 10, 12, 15];
    let test2: [i32; 5] = [1, 2, 4, 7, 10];

    println!("average value: {}", Solution::average_value(test1.to_vec()));

    println!("average value: {}", Solution::average_value(test2.to_vec()));
}

