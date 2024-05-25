// class Solution {
// public:
// 
//     //std::ios_base::sync_with_stdio(false);
//     //cin.tie(nullptr);
// 
//     int minimumOperations(vector<int>& nums) {
//         //early exit
//         if (nums.size() == 1) [[unlikely]] {
//             if (nums[0] == 0){
//                 return 0;
//             }
//             return 1;
//         };
// 
//         int lowest = 101;
//         int count = 0;
// 
//         loopPoint:
//             lowest = findLowestNonZero(nums);
//             nums = subtractFromEach(nums, lowest);
// 
//             if (lowest >= 1 ){
//                 count += 1;
// 
//                 cout << count << ": ";
//                 for(int i = 0; i < nums.size(); ++i){
//                     cout << nums[i] << ", ";
//                 }
//                 cout << endl;
// 
//                 goto loopPoint;
//             };
// 
//         return count;
//     }
// 
//     int findLowestNonZero(vector<int> &arr){
//         int lowest = 1001;
//     
//         for(int i = 0; i < arr.size(); ++i){
//             if (arr[i] > 0 && arr[i] <= lowest){
//                 lowest = arr[i];
//             }
//         }
// 
//         if (lowest == 1001){
//             return 0;
//         }
//     
//         return lowest;
//     }
//     
//     vector<int> subtractFromEach(vector<int> &arr, int lowest){
//         for(int i = 0; i < arr.size(); ++i) {
//             if(arr[i] == 0){
//                 continue;
//             }
//             if(arr[i] - lowest <= 0){
//                 arr[i] = 0;
//             }
//             else {
//                 arr[i] = arr[i] - lowest;
//             }
//         }
//     
//         return arr;
//     }
// };
// 
// 

class Solution {
public:
    int minimumOperations(std::vector<int>& source)
    {
        std::sort(std::begin(source), std::end(source));
        source.erase(std::unique(std::begin(source), std::end(source)), std::end(source));
        return source[0] ? std::size(source) : std::size(source) - 1;
    }
};

