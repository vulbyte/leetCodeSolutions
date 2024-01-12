/**
 * @param {string[]} strs
 * @return {string}
 */
var longestCommonPrefix = function(strs) {
    let i = 1;
    let prf = strs[0];

    while (i < strs.length) {
        if (!strs[i].startsWith(prf)) {
            prf = prf.slice(0, -1)
        }
        else {
            i++
        }
    }

    return prf;
};

// ORIGINAL IN THE LOWER 5%
// /**
//  * @param {string[]} strs
//  * @return {string}
//  */
// var longestCommonPrefix = function(strs) {
//     let currentChar;
//     let sum = "";
//     //console.log(sum); //\U0001fab5

//     let shortestStr = strs[0].length;
//     // console.log(shortestStr.length); //\U0001fab5

//     // based on the shortestStr, see which how many align
//     for (i=0; i<strs[0].length; i++){
//         currentChar = strs[0][i];
//         //console.log(strs[0][i]); //\U0001fab5
//         for (j=0; j<strs.length; j++) {
//             console.log("evaluating str: " + strs[i])
//             if (strs[j][i] == currentChar) {
//                 //console.log(strs[i][j]); //\U0001fab5
//                 continue;
//             }
//             else {
//                 //console.log("no match returning"); //\U0001fab5
//                 return (sum)
//             }
//         }
//         sum += currentChar;
//     }

//     //console.log('shortestStr: ' + shortestStr );
//     //console.log(sum); //\U0001fab5
//     return(sum);
// };
