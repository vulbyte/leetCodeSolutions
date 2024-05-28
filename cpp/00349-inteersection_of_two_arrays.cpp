class Solution {
public:
std::vector<int> intersection(std::vector<int>& nums1, std::vector<int>& nums2) {
    std::vector<int> result;

    // Determine the smaller and larger vector for efficiency
    std::vector<int>& smaller = (nums1.size() < nums2.size()) ? nums1 : nums2;
    std::vector<int>& larger = (nums1.size() < nums2.size()) ? nums2 : nums1;

    // Sort the smaller vector for efficient searching
    std::sort(smaller.begin(), smaller.end());

    // Copy elements from the larger vector that are in the smaller vector
    std::copy_if(larger.begin(), larger.end(), std::back_inserter(result), [&smaller](int value) {
        return std::binary_search(smaller.begin(), smaller.end(), value);
    });

    // Remove duplicates from the result
    std::sort(result.begin(), result.end());
    result.erase(std::unique(result.begin(), result.end()), result.end());

    return result;
};
//source.erase(
//        std::remove_if(source.begin(), source.end(), [&target](int value) {
//            return std::find(target.begin(), target.end(), value) == target.end();
//        }),
//        source.end()
//    );;

//private:
//    vector<int> filterLargeVecFromSmall(vector<int> largeArr, vector<int> smallArr){
//        int smallindex = 0;
//        for(int i = 0; i < largeArr.size(); ++i){
//            for(int j = 0; i < smallArr.size(); ++i){
//                if(smallArr[j] == largeArr[i]){
//
//                    break;
//                }
//            }
//        }
//    };
}
