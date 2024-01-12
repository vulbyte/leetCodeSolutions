/**
 * @param {number} n
 * @return {number}
 */
var nthUglyNumber = function(n) {
    //escape if <0 or null
    if (!n) {
        return 0;
    }

    let i2 = 0, i3 = 0, i5 = 0; // the 2,3,5 indeces
    let out = [1]; // we will fill this with the ugly multiples
    
    while (!out[n-1]) { 
        let c2 = out[i2] * 2;
        let c3 = out[i3] * 3;
        let c5 = out[i5] * 5;
        // In c2,c3,c5 we now have 3 candidates for the next number. Pick the lowest one, to record in order
        // In the first round that will be 2
        let next = Math.min(Math.min(c2, c3), c5);
        out.push(next);
        
        if (next === c2) {
            // now the 2 index will increase, and next round, c2 will be four, so c3=3 will be lowest
            i2++;
        }
        if (next === c3) {
            i3++;
        }
        if (next === c5) {
            i5++;
        } 
        
    }

    return out[n-1]; 
}
