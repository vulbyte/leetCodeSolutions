// impl Solution {
//     pub fn remove_duplicates(s: String) -> String {
//         let result: String = s
//             .chars()
//             .enumerate()
//             .filter(|&(index, x)| {
//                 x != s.chars().nth(index+1).unwrap_or('\0')
//             })
//             .map(|(_, x)| x)
//             .collect();
//
//             return result
//     }
// }

// impl Solution {
//     pub fn remove_duplicates(s: String) -> String {
//         let mut result = Vec::new();
//
//         for c in s.chars() {
//             if let Some(last) = result.pop() {
//                 if last != c {
//                     // If the last character is not equal to the current one, push them both back.
//                     result.push(last);
//                     result.push(c);
//                 }
//                 // If they are equal, we've removed the duplicate.
//             } else {
//                 // If the result is empty, push the first character.
//                 result.push(c);
//             }
//         }
//
//         result.into_iter().collect()
//     }
// }

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        s.chars()
            .fold(Vec::new(), |mut result, c| {
                if let Some(last) = result.pop() {
                    if last != c {
                        result.push(last);
                        result.push(c);
                    }
                } else {
                    result.push(c);
                }
                result
            })
            .into_iter()
            .collect()
    }
}
