/*
   The Leetcode file system keeps a log each time some user performs a change folder operation.

   The operations are described below:

   "../" : Move to the parent folder of the current folder. (If you are already in the main folder, remain in the same folder).
   "./" : Remain in the same folder.
   "x/" : Move to the child folder named x (This folder is guaranteed to always exist).
   You are given a list of strings logs where logs[i] is the operation performed by the user at the ith step.

   The file system starts in the main folder, then the operations in logs are performed.

   Return the minimum number of operations needed to go back to the main folder after the change folder operations.
   */

impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut depth:i32 = 0; //declare var as an i32, as we always start in same index of 0, we can predefine this.
        for n in logs { // n is a placeholder for each var in an array, in this case is logs
            if n == "../"{ // if we're going to a parent directory
                if depth >= 1 { // and the operation is possible
                    depth -= 1; // subtract 1
                }
            }
            else if n == "./"{ // "./" effectively means "do nothing", so that's what we'll do, and use this to skip the next statement
                               //no op
            }
            else { // if the other 2 conditions aren't meant, increment
                depth += 1;
            }
        }


        return depth // return the counted value
    }
}
