impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        // return words.char()
        //     .enumerate()
        //     .filter(|&(w)|{
        //         if w == w.rev() {
        //             return
        //         }
        //     })

        for w in words.iter() {
            if w == &w.chars().rev().collect::<String>() {
                return w.clone();
            }
        }
        return "".to_string();
    }
}
