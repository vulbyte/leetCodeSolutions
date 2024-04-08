class Solution {
public:
    int minOperations(vector<string>& logs) {
        int depth = 0; //declare var as an i32, as we always start in same index of 0, we can predefine this.
        for (int i = 0; i < logs.size(); ++i) { // n is a placeholder for each var in an array, in this case is logs
            if (logs[i] == "../"){ // if we're going to a parent directory
                if (depth >= 1) { // and the operation is possible
                    depth -= 1; // subtract 1
                }
            }
            else if (logs[i] == "./"){ // "./" effectively means "do nothing", so that's what we'll do, and use this to skip the next statement
                //no op
            }
            else { // if the other 2 conditions aren't meant, increment
                depth += 1;
            }
        }

        
        return depth; // return the counted value
    }
}
