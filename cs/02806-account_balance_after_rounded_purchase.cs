/*
 Initially, you have a bank account balance of 100 dollars.

You are given an integer purchaseAmount representing the amount you will spend on a purchase in dollars.

At the store where you will make the purchase, the purchase amount is rounded to the nearest multiple of 10. In other words, you pay a non-negative amount, roundedAmount, such that roundedAmount is a multiple of 10 and abs(roundedAmount - purchaseAmount) is minimized.

If there is more than one nearest multiple of 10, the largest multiple is chosen.

Return an integer denoting your account balance after making a purchase worth purchaseAmount dollars from the store.

Note: 0 is considered to be a multiple of 10 in this problem.
 */


// public class Solution {
//     public int AccountBalanceAfterPurchase(int purchaseAmount) {
//         if (purchaseAmount%10 >= 5) {
//             purchaseAmount = (int)Math.Round(purchaseAmount);
//         }
//         else {
//             purchaseAmount = (int)Math.Floor(purchaseAmount);
//         }
// 
//         return (100 - purchaseAmount);
//     }
// }

// public class Solution {
//     public int AccountBalanceAfterPurchase(int purchaseAmount) {
//         int caldValue = 0;
//         if (purchaseAmount % 10 >= 5) {
//             caldValue = (int)Math.Round(purchaseAmount);
//         }
//         else {
//             caldValue = (int)Math.Floor(purchaseAmount);
//         }
// 
//         return (100 - caldValue); 
//     }
// }

// public class Solution {
//     public int AccountBalanceAfterPurchase(int purchaseAmount) {
//         int roundedValue = 0;
// 
//         if (purchaseAmount <= 9 && purchaseAmount >= 5) {
//             Console.WriteLine("returning early: " + purchaseAmount);
//             return (90);
//         }
// 
//         if (purchaseAmount % 10 >= 5 && purchaseAmount % 10 <=) {
//             roundedValue = (int)Math.Round(purchaseAmount / 10.0) * 10;
//             Console.WriteLine("roundedValue early: " + roundedValue);
//         }
//         // else {
//         //     roundedValue = (int)Math.Floor(purchaseAmount / 10.0) * 10;
//         //     Console.WriteLine("roundedValue early: " + roundedValue);
//         // }
// 
// 
//         return (100 - roundedValue);
//     }
// }

public class Solution {
    public int AccountBalanceAfterPurchase(int purchaseAmount) {
        return (int)(100.0 - Math.Round(purchaseAmount/10.0, MidpointRounding.AwayFromZero) * 10);
    }
}
