/*
Constraints:

2 <= nums.length <= 100
nums.length is even.
0 <= nums[i] <= 100
*/

/* MY ATTEMPT
class Solution {
public:
    int distinctAverages(vector<int>& nums) {
        float size = nums.size() - 1;
        float returnFloat = 0.0;

        for (int i = size; i >= 0; i--){
            returnFloat = ((returnFloat + nums[i]) /2.0);
            printf("%f\n", returnFloat);
            nums.pop_back();
        }
        return returnFloat;
    };
};
*/

class Solution {
    //given solution

    // public methods
    public:
        // int fn that takes in a reference to a vector of ints
        int distinctAverages(vector<int>& nums) {
            //this sort is needed somewhere for some reason, but i have 0 idea why
            sort(nums.begin(),nums.end());
            //create a std::set<double> which is a set that is always arranged in ascending order and only can contain "double"s
            set<double> my;
            //starting at i, while i < the size of nums/2, per iteration i+1;
            for(int i=0;i<nums.size()/2;i++)
            //add value of nums[i]+nums[nums.size()-i-1])/2);
            my.insert((double)(nums[i]+nums[nums.size()-i-1])/2);
            //return set.size
            return my.size(); 
        }
};
