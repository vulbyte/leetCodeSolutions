/* 744. Find Smallest Letter Greater Than Target Solved
Easy
Topics
Companies
Hint
You are given an array of characters letters that is sorted in non-decreasing order, and a character target. There are at least two different characters in letters.

Return the smallest character in letters that is lexicographically greater than target. If such a character does not exist, return the first character in letters.

 

Example 1:

Input: letters = ["c","f","j"], target = "a"
Output: "c"
Explanation: The smallest character that is lexicographically greater than 'a' in letters is 'c'.
Example 2:

Input: letters = ["c","f","j"], target = "c"
Output: "f"
Explanation: The smallest character that is lexicographically greater than 'c' in letters is 'f'.
Example 3:

Input: letters = ["x","x","y","y"], target = "z"
Output: "x"
Explanation: There are no characters in letters that is lexicographically greater than 'z' so we return letters[0].
 */
#include <algorithm>
#include <vector>
#include <iostream>

struct Tests {
    int testNum;
    std::vector<char> chars;
    char target;
    char expectedOutput;
};

class Solution {
public:
    char nextGreatestLetter(std::vector<char> &letters, char target) {
        //presorting hurts memeory usage a lot
        //std::sort(std::begin(letters), std::end(letters));
        letters.erase( std::unique(std::begin(letters), std::end(letters)), std::end(letters));

        //O(n) threw the list finding smallest
        for(int i = 0; i < letters.size(); ++i){
            if(letters[i] > target){
                return letters[i];
            }
        };

        return(letters[0]);
    };
};

void StdTest(Tests &test){
    Solution sol;

    char output = sol.nextGreatestLetter(test.chars, test.target);

    if((char)output == test.expectedOutput){
        std::cout << "✅ PASS" << std::endl;
    }
    else {
        std::cout << "❌ FAIL with test: " << test.testNum << std::endl << "expected: " << test.expectedOutput << " " << "output: "<< output << std::endl;
    }

    return;
}

int main(){
    Tests test1 = {
        1,
        {'c', 'f', 'j'},
        'a',
        'c',
    }; 
    StdTest(test1);

    Tests test2 = {
        2,
        {'c', 'f', 'j'},
        'c',
        'f',
    };
    StdTest(test2);

    Tests test3 = {
        3,
        {'x','x','y','y'},
        'z',
        'x'
    };
    StdTest(test3);
};


