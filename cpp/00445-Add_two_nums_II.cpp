/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */

 // i have had no experiance with walking linked lists at the time of "solving, so below is my reasoning, then in the soluition i'll walk through it attempting to understand"

// my solution to this:
    // compare l1[i] and l2[i]
    // if the next element == null
        // push value to next in the list then
        // return for each l3[i] == null until item is last in the list
    // if the next element != null is last in the list
        // return the sum in l3[i]
class Solution {
public:
    //take in 2 vars, l1 and l2 which are both lists
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        //declare vars a and b which are strings
        string a, b;
        //set the result to be a null ptr 
        ListNode *result = nullptr;
        //while l1 != NULL add new element to end of vector =  l1 -> val+"0"
        //then set l1 to prt.member(next)
        while(l1) { a.push_back(l1->val+'0'); l1 = l1->next;}
        //above but with l2
        while(l2) { b.push_back(l2->val+'0'); l2 = l2->next;}
        // init 3 vars, l, r, and carry to the following values
        int l = a.size()-1, r = b.size()-1, carry = 0;
        //read it
        while(l >= 0 || r >= 0 || carry == 1) {
            //this is a mess i can work through but i really wish were seperated oml 
            //and this is where i take my leave
            int c = (l >= 0 ? a[l--]-'0' : 0) + ( r >= 0 ? b[r--]-'0' : 0) + carry;
            ListNode *temp = new ListNode(c%10);
            temp->next = result;
            result = temp;
            carry = c/10;
        }        
        return result;
    }
};
