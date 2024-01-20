impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // THE OG TRY
        // let mut amntOfUniqueNums:i32 = 0;
        // let mut lastnum:i32 = -100; // set to this for last constraint
        // for n in nums {
        //     if (*n == lastnum) {
        //         nums.remove((*n).try_into().unwrap());
        //     }
        //     else {
        //         lastnum = *n;
        //         amntOfUniqueNums += 1;
        //     }
        // }
        // amntOfUniqueNums

        // og try with gpt help
        // note, this was done with the mentality of doing so in place instead of being speedy
        //early escape
        if nums.is_empty() {
            return 0;
        }

        // var decloration
        let mut amount_of_unique_nums = 1;
        let mut last_num = nums[0];
        let mut len = nums.len();
        let mut i = 1;

        // iderate over array
        while i < len {
            if nums[i] != last_num {
                amount_of_unique_nums += 1;
                last_num = nums[i];
                i += 1;
            } else {
                //remove doublicates from nums from a vector where all values are from i to i
                nums.drain(i..=i);
                len -= 1;
            }
        }

        amount_of_unique_nums
    }
}
