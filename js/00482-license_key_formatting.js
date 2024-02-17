// ATTEMPT 1
// /**
//  * @param {string} s
//  * @param {number} k
//  * @return {string}
//  */
// var licenseKeyFormatting = function(s, k) {
//     //k = target
//     //s = string
// 
//     //convert string to all capitals
//     s = s.toUpperCase();
// 
//     //skip the first group for optimization
//     const startPos = s.indexOf("-");
//     console.log('startPos: ' + startPos)
// 
//     //loop over every char past the first -
//     for (i = startPos+1; i < s.length; i++) {
//         console.log(
//             "evaling char: " +  
//             s[i] + 
//             " @: " + 
//             i
//         );
// 
//         console.log('%' + i%(k+startPos))
// 
//         if (i%k+1 == 0) {
//             if (s[i] != '-') {
//                 s = s.slice(0, i) + '-' + s.slice(i, 0); 
//             }
//         }
//         else {
//             if (s[i] == '-'){
//                 s = s.slice(0, i) + s.slice(i+1);
//             }
//         }
// 
//     }    
//     return s
// };

//ATTEMPT 2
var licenseKeyFormatting = function(s, k) {
    s = s.replace(/-/g, '').toUpperCase();
    const remainder = s.length % k;
    let result = s.slice(0, s.length % k);
    let i = remainder;

    for (i; i < s.length; i += k) {
        if (i != 0) {
            result += '-';
        }
        result += s.slice(i, i + k);
    }

    return result;
};

// // Test
// var input = "5F3Z-2e-9-w";
// var k = 4;
// var output = licenseKeyFormatting(input, k);
// console.log(output);  // Output: "5F3Z-2E9W"

