// KNEE JERK SOLUTION, TOO SLOW BECAUSE O(n)
// class Solution {
// public:
//     int maxLengthBetweenEqualCharacters(string s) {
//         int maxGap = -1;
//         int gap = -1;
// 
//         for (int i = 0; i < s.size(); i++) {
//             gap = findGap(s, s[i], i);
//             if (gap > maxGap){
//                 maxGap = gap;
//             }
//         }
// 
//         if (maxGap == -1) {
//             return -1;
//         }
// 
//         return maxGap-1;
//     }
//     int findGap(string s, char c, int i){
//         for (int j = i+1; j < s.size(); j++) {
//             if (s[j] == c){
//                 cout << "char gap = " << abs(i-j) << endl;
//                 return abs(i-j);
//             }
//         }
//         cout << "char gap = {-1}" << endl;
//         return -1;
//     }
// };
// 
// "mgntdygtxrvxjnwksqhxuxtrv"
//     ^                  ^

// GIVEN SOLUTION, TESTED TO COMPARE SPEED, DIFFERENCE WAS NEGLIGABLE
// class Solution {
// public:
//     int maxLengthBetweenEqualCharacters(string s) {
//         //character an unordered map named first index
//         unordered_map<char, int> firstIndex;
//         //init ans
//         int ans = -1;
//         
//         // until i == s.size();
//         for (int i = 0; i < s.size(); i++) {
//             //if first and last index of char
//             if (firstIndex.find(s[i]) != firstIndex.end()) {
//                 //if found, return 
//                 ans = max(ans, i - firstIndex[s[i]] - 1);
//             } else {
//                 // increment the index
//                 firstIndex[s[i]] = i;
//             }
//         }
// 
//         // return
//         return ans;
//     }
// };

// ATTEMPTED ANOTHER "FASTER" SOLUTION, WAS NEGLIGABLE FROM THE LAST
// class Solution {
// public:
//     int maxLengthBetweenEqualCharacters(string s) {
//         unordered_map<char, vector<int>> mp;
//         for(int i = 0; i < s.size(); i++){
//             mp[s[i]].emplace_back(i);
//         }
//         int max = -1;
//         int n = mp.size();
//         for(int i = 0; i< n; i++){
//             if(mp[s[i]].size() > 1 && max < mp[s[i]][mp[s[i]].size() - 1] - mp[s[i]][0]){
//                 max = mp[s[i]][mp[s[i]].size() - 1] - mp[s[i]][0] - 1;
//             }
//         }
//         return max;
//     }
// };

// TESTED THE "LEAST MEMORY" SOLUTION, AND THIS WAS THE FASTEST RUNTIME.
// HONESTLY PROBABLY A GLITCH BUT YEH
class Solution {
public:
    int maxLengthBetweenEqualCharacters(string s) {
        int n = s.size();

        int res = -1;

        for (int l = 0; l < n - 1; ++l){
            int r = n - 1;
            while (l < r){
                if (s[l] == s[r]) res = max(res, r - l - 1);
                --r;
            }
        }
        return res;
    }
};
