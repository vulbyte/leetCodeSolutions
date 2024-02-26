// impl solution {
//     pub fn maximum_element_after_decrementing_and_rearranging(arr: vec<i32>) -> i32 {
//         // sort array
//         let mut sorted_arr: vec<i32> = arr.iter().cloned().collect();
//         sorted_arr.sort();
//
//         // filter and clone
//         let new_arr: vec<i32> = sorted_arr
//             .iter()
//             .enumerate()
//             .filter(|&(index, &n)| *n >= 1 && *n >= index as i32)
//             .map(|(_, &n)| n)
//             .collect();
//
//         new_arr.len() as i32
//     }
// }

// impl Solution {
//     pub fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32 {
//         // Sort array
//         let mut sorted_arr: Vec<i32> = arr.iter().cloned().collect();
//         sorted_arr.sort();
//
//         // Filter and clone
//         let new_arr: Vec<i32> = sorted_arr
//             .iter()
//             .enumerate()
//             .filter(|&(index, &n)| n >= 1 && n >= index+1 as i32)
//             .map(|(_, &n)| n)
//             .collect();
//
//         new_arr.len() as i32
//     }
// }

impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32 {
        //copy n_arr
        let mut n_arr = arr;
        //alternative sort algorithm
        n_arr.sort_unstable();
        //cut out elements below the index,
        n_arr.into_iter().fold(0, |acc, x| {
            //if x > acc, return
            match x > acc {
                true => acc + 1,
                false => acc,
            }
        })
    }
}
