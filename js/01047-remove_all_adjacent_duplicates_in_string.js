// /**
//  * @param {string} s = string
//  * @return {string}
//  */
// var removeDuplicates = function(s) {
//     /**
//      * @param {(array | number)} sE = startEnd abbreviated
//      * @param {string} s = string
//      * @return {array} 
//      */
//     function CheckOuterRange(sE, s) {
//         /**
//         * @param {(array | number)} sE = startEnd abbreviated 
//         * @param {string} s = string
//         * @return {array} 
//         */
//         function Method(sE, s){
//             if (!isNaN(sE)) {
//                 if (s[sE] == s[sE + 1]) {
//                     return Method([sE, sE+1], s);
//                 }
//             }
// 
//             try {
//                 if (s[sE[0]-1] == s[sE[1]+1]) {
//                     return Method([sE[0]-1, sE[1]+1]);
//                 }
//             }
//             catch (err) {
//                 return [sE[0], sE[1]];
//             }
//         }
//         return Method(sE, s);
//     }
// 
//     /**
//     * @param {array} range
//     * @param {string} s
//     * @return {string}
//     */
//     function RemoveRange(range, s){
//         try {
//             if (isNaN(range) == false || range.length <= 1) {
//                 return s;
//             }
//             console.log("REMOVING: " + range);
//             return s.substring(0, range[0]) + s.substring(range[1]+1);
//         }
//         catch(err){
//             console.log("err with (range, s): " + range + " " + s);
//         }
//     }
// 
//     try {
//         for(i=0; i < s.length; i++) { 
//             console.log("evaling: ", i);
//             if (s == null || s == undefined){
//                 return "";
//             }
//             //console.log("CONSOLE LOG: " + CheckOuterRange(i,s), s);
//     
//             s = RemoveRange(
//                 CheckOuterRange(i, s), s
//             );
//             console.log("s: " + s);
//         }    
//     
//         return s;
//     }
//     catch(err){
//         return"";
//     }
// };

// var removeDuplicates = function(s) {
//     function CheckRange(sE, s) {
//         if(!isNaN(sE)){ //init loop
//             sE = [sE, ++sE];
//             if (sE[0] == sE[1]) { //test if equal, if true do next loop
//                 CheckRange(sE, s);
//             }
//             else {
//                 return s;
//             }
//         } 
//         if (sE[0] == sE[1]) { //test if equal, if true do next loop
//             CheckRange(sE, s);
//         }
//         else { //if not equal or invalid, pass to RemoveRange()
//             return RemoveRange(sE, s);
//         }
//     }
// 
//     function RemoveRange(sE, s) {
//         console.log("elvaling: sE: " + sE)
//         if (typeof sE[0] === 'number' && typeof sE[1] === 'number') {
//             return s.substring(0, sE[0]) + s.substring(sE[1] + 1);
//         }
//         else {
//             return s;
//         }
//     }
// 
//     let i = 0;
//     function Iter(s) {
//         console.log("iterating through s: \n" + s);
//         if (i >= s.length){ // exit when complete
//             return s;
//         }
//         else { // if not complete, iterate
//             s = CheckRange(i, s);
//             ++i;
//         }
//     }
// 
//     s = Iter(s);
//     return s;
// }


// var removeDuplicates = function(s) {
//     // function remove(string, from, to) {
//     //     return string.substring(0, from) + string.substring(to);
//     // }
// }


//const removeDuplicates = s => {
//   const stack = [];
//   for (const char of s) {
//     stack[stack.length - 1] === char ? stack.pop() : stack.push(char);
//   }
//   return stack.join('');
// };
//solution is iterative of above from "poppinlp"
const removeDuplicates = s => {
    const stack = [];
    for (const char of s) {
        if (stack[stack.length - 1] === char) {
            stack.pop()
        }
        else {
            stack.push(char)
        }
    }
    return stack.join('');
};

