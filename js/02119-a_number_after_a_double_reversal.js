/**
 * @param {number} num
 * @return {boolean}
 */

// var isSameAfterReversals = function(num) {
//     if (num.toString()[num.toString().length-1] == 0 && num !== 0){
//         return false;
//     }
//     return true;
// };

var isSameAfterReversals = function(num) {
    return num % 10 !== 0 || num == 0 ? true : false;
};

