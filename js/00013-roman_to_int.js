// /** SKIP TO BOTTOM FOR ANSWER
//  * @param {string} s
//  * @return {number}
//  */
// var romanToInt = function(s) {    
//     const values = {
//         I: 1,
//         V: 5,
//         X: 10,
//         L: 50,
//         C: 100,
//         D: 500,
//         M: 1000,
//     };
//     const conditionals = {
//         I: 1,
//         X: 10,
//         C: 100,
//     };

//     sum = 0;

//     for (i = 1; i<= s.length; i++){
//         char=s[i-1];
//         if (i != s.length) {
//             nextChar=s[i];
//         }

//         if (conditionals.hasOwnProperty(char) && char < nextChar ) {
//             sum -= values[char];
//             console.log(sum);
//             continue;
//         }
//         else {
//             sum += values[char];
//             console.log(sum);
//             continue;
//         }
//     }
//     return sum;
// }




// THIS SOLUTION IS TOO SLOW AND TAKES TOO MUCH MEMORY
// /**
//  * @param {string} s
//  * @return {number}
//  */
// var romanToInt = function(s) {
//   const values = {
//     I: 1,
//     V: 5,
//     X: 10,
//     L: 50,
//     C: 100,
//     D: 500,
//     M: 1000,
//   };
//   const conditionals = {
//     I: 1,
//     X: 10,
//     C: 100,
//   };
//   const asterisks = {
//     V: 5,
//     X: 10,
//     L: 50,
//     C: 100,
//     D: 500,
//     M: 1000,
//   };
//   let sum = 0;

//   for (let i = 0; i < s.length; i++) {
//     const char = s[i];
//     try {
//         if (
//             conditionals.hasOwnProperty(char) == true && 
//             asterisks.hasOwnProperty(s[i+1]) &&
//             values[char] < values[s[i+1]]
//         ) {
//             sum -= values[char];
//             console.log(sum + ' subing ' + values[char] + s[i] + " + " + s[i+1]);
//             continue;
//         }
//         else {
//             sum += values[char];
//             console.log(sum + ' adding ' + values[char] + s[i] + " + " + s[i+1]);
//             continue;
//         }
//     }
//     catch {
//         sum += values[char];
//         console.log('CATCH CASE' + sum + ' adding ' + values[char] + s[i] + " + " + s[i+1]);
//         continue;
//     }
//   }

//   return sum;
// };


// THIS SOLUTION IS TOO SLOW AND TAKES TOO MUCH MEMORY
/**
 * @param {string} s
 * @return {number}
 */
var romanToInt = function(s) {
    const values = {
        I: 1,
        V: 5,
        X: 10,
        L: 50,
        C: 100,
        D: 500,
        M: 1000,
    };
    const conditionals = {
        I: 1,
        X: 10,
        C: 100,
    };
    const asterisks = {
        V: 5,
        X: 10,
        L: 50,
        C: 100,
        D: 500,
        M: 1000,
    };
    let sum = 0;

    for (let i = 0; i < s.length; i++) {
        const char = s[i];
        if (
            conditionals.hasOwnProperty(char) == true &&
            asterisks.hasOwnProperty(s[i + 1]) &&
            values[char] < values[s[i + 1]]
        ) {
            sum -= values[char];
            // console.log(sum + ' subing ' + values[char] + s[i] + " + " + s[i+1]);
            continue;
        }
        else {
            sum += values[char];
            // console.log(sum + ' adding ' + values[char] + s[i] + " + " + s[i+1]);
            continue;
        }
    }

    return sum;
};


