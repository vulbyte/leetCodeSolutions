// got the top 100% with this B)

class Solution {
public:
    int timeRequiredToBuy(vector<int>& tickets, int k) {
        int len = tickets.size();
        int seconds = 0;

        loop:
            for (int i = 0; i < len; i++) {
                if (tickets[k] != 0) { 
                    if (tickets[i] <= 0){ 
                       continue; 
                    }
                    else{
                        tickets[i] -= 1;
                        seconds += 1; 
                    }
                }
                else {
                    return seconds;
                }
            }
        goto loop;    
    }
};
