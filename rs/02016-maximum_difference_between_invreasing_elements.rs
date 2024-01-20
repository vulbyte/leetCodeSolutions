//Input: nums = [7,1,5,4]
//Output: 4
//Explanation:
//The maximum difference occurs with i = 1 and j = 2, nums[j] - nums[i] = 5 - 1 = 4.
//Note that with i = 1 and j = 0, the difference nums[j] - nums[i] = 7 - 1 = 6, but i > j, so it is not valid.

// THIS NEEDS TO BE REWRITTEN CAUSE YOU DIDN'T READ THE PROMPT PROPERLY YOU DUNCE

impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut returnValue = -1;
        //store min/max values
        let (mut max, mut min) = (nums[0], nums[0]);
        // for values in nums, iterate through
        for &i in nums[1..].iter() {
            //if index < min value
            if i < min {
                //reassign min and max to index
                min = i;
                max = i;
            } else if i > max {
                //if i > max, max = 1
                max = i;
                //reutrn the max value between the 2 values
                returnValue = returnValue.max(max - min);
            }
        }
        //return
        returnValue
    }
}

