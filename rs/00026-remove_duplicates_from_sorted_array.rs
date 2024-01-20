/*
Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once. The relative order of the elements should be kept the same. Then return the number of unique elements in nums.

Consider the number of unique elements of nums to be k, to get accepted, you need to do the following things:

Change the array nums such that the first k elements of nums contain the unique elements in the order they were present in nums initially. The remaining elements of nums are not important as well as the size of nums.
Return k.
Custom Judge:

The judge will test your solution with the following code:

int[] nums = [...]; // Input array
int[] expectedNums = [...]; // The expected answer with correct length

int k = removeDuplicates(nums); // Calls your implementation

assert k == expectedNums.length;
for (int i = 0; i < k; i++) {
    assert nums[i] == expectedNums[i];
}
If all assertions pass, then your solution will be accepted.
 */

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
