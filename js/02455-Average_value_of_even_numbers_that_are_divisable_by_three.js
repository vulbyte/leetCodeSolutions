// /**
//  * @param {number[]} nums
//  * @return {number}
//  */
// var averageValue = function(nums) {
//     let numbers = [];
//     numbers = FilterNumbers(nums, numbers);
// 
//     console.log("numbers: ", numbers)
// 
//     if (numbers.length === 0) {
//         console.log("numbers.length = :", numbers.length);
//         return 0;
//     }
//     return AverageNums(numbers);
// };
// 
// function FilterNumbers(nums) {
//     console.log("filteringNums", nums);
//     let returnArray = [];
//     let i;
// 
//     for (i = nums.length - 1; i >= 0; i--) {
//         if (nums[i] % 3 == 0 && nums[i] %2 == 0) {
//             returnArray.push(nums[i]);
//         }
//     }
// 
//     console.log("returning: ", returnArray);
//     return returnArray;
// }
// 
// function AverageNums(inputArr) {
//     console.log("averaging numbers");
//     var i = inputArr.length;
//     var returnVal = 0;
// 
//     for (i; i > 0; i--) {
//         returnVal += inputArr[i - 1];
//     }
// 
//     console.log("final value: ", returnVal / inputArr.length);
// 
//     return (returnVal / inputArr.length);
// }

var averageValue = function(nums) {
    let sum = 0;
    let count = 0;
    for (let n of nums) {
        if (n % 6 === 0) {
            sum += n;
            count++;
        }
    }
    return sum === 0 ? sum : Math.floor(sum / count);
};
