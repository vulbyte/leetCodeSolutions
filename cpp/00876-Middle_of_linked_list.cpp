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
 
// class of solution
class Solution {
    //on load
public:
    //init this method
    int countNode(ListNode* head)
    {
        //init
        int count = 0;
        //loop over list until until tmp-> returns (i think) null
        for (ListNode* tmp = head; tmp; tmp = tmp->next) count++;
        //return count
        return count;
    }
    //init this method
    ListNode* middleNode(ListNode* head) {
        //call this passing in the head
        int amount = countNode(head);
        // after above, floo division, add 1 cause c++
        int position = amount / 2 + 1;
        //reinit count
        int count = 0;
        // loop over list until tmp-> returns middle value
        for (ListNode* tmp = head; tmp; tmp = tmp->next) 
        {
            count++;
            if (count == position) return tmp;
        }
        //return value
        return head;
    }
};
