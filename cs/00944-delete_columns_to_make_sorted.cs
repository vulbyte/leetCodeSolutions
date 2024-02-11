// public class Solution
// {
//     public int MinDeletionSize(string[] strs)
//     {
//         //init return value
//         int res = 0;
// 
//         //for the len of strs[0]
//         for (int i = 0; i < strs[0].Length; i++)
//         {
//             //select ea in s and return s[i] for a quick iterator loop
//             var col = strs.Select(s => s[i]);
//             //if sequence matches then remove and res++
//             if (!col.OrderBy(c => c).SequenceEqual(col)) res++;
//         }
// 
//         //return res
//         return res;
//     }
// }

// public class Solution {
//     public int MinDeletionSize(string[] strs) {
//         int columnDel = 0;
//         for(int i = 0; i < strs[0].Length; i++)
//         {
//             for(int j = 0; j < strs.Length-1; j++)
//             {
//                 if(strs[j][i] > strs[j+1][i])
//                 {
//                     columnDel++;
//                     break;
//                 }
//             }
//         }
//         return columnDel;
//     }
// }

public class Solution
{
    public int MinDeletionSize(string[] strs)
    {
        int rowCount = strs.Length;
        int colCount = strs[0].Length;
        int deletedColumns = 0;

        for (int col = 0; col < colCount; col++)
        {
            for (int row = 1; row < rowCount; row++)
            {
                // Check if characters in the current column are not sorted
                if (strs[row][col] < strs[row - 1][col])
                {
                    deletedColumns++;
                    break; // Move to the next column
                }
            }
        }

        return deletedColumns;
    }
}
