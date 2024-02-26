// class Solution {
// public:
//     int maxPower(string s) {
//         int len = s.length();
// 
//         //early exit
//         if (len == 1){
//             return 1;
//         }
// 
//         //define rest of vars
//         int i = 0;
//         int maxSubArr = 1;
//         int currSubArr = 1;
//         char lastChar = '_';
// 
//         //recurring
//         return findMSR(
//             s, 
//             maxSubArr, 
//             currSubArr, 
//             lastChar,
//             i,
//             len
//         );
//     }
//     int findMSR(
//         const string& s, 
//         int maxSubArr, 
//         int currSubArr, 
//         int lastChar,
//         int i,
//         int len
//     ){
//         //if can iderate
//         if (i < len){
//             // check for increment
//             if (lastChar == s[i]){
//                 currSubArr += 1; 
//                 if (currSubArr > maxSubArr) {
//                     maxSubArr += 1;
//                 }
//             }
//             //if not, them prime for next loop
//             else {
//                 lastChar = s[i];
//                 currSubArr = 1;
//             }
//             //once logic is done, repeat
//             findMSR(
//                 s, 
//                 maxSubArr, 
//                 currSubArr, 
//                 lastChar,
//                 i,
//                 len
//             );
//         }
// 
//         //return if invalid
//         return maxSubArr;
//     }
// };

class Solution {
public:
    int maxPower(string s) {
        int len = s.length();

        //early exit
        if (len == 1){
            return 1;
        }

        //define rest of vars
        int i = 0;
        int maxSubArr = 1;
        int currSubArr = 1;
        char lastChar = '_';

        //recurring
        return findMSR(
            s, 
            maxSubArr, 
            currSubArr, 
            lastChar,
            i,
            len
        );
    }

    int findMSR(
        const string& s, 
        int maxSubArr, 
        int currSubArr, 
        int lastChar,
        int i,
        int len
    ){
        //if can iterate
        if (i < len){
            // check for increment
            if (lastChar == s[i]){
                currSubArr += 1; 
                if (currSubArr > maxSubArr) {
                    maxSubArr = currSubArr;
                }
            }
            //if not, then prime for the next loop
            else {
                lastChar = s[i];
                currSubArr = 1;
            }
            //once logic is done, repeat and return the result
            return findMSR(
                s, 
                maxSubArr, 
                currSubArr, 
                lastChar,
                i + 1,  // Increment i to make progress in the string
                len
            );
        }

        //return the result if invalid
        return maxSubArr;
    }
};

