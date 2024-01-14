//this is my solution, which ended up falling short. i attempted to do it in a systematic way incase things could be added or removed systematically.
//i was too stupid for this approach
/**
* @param {number} mainTank
* @param {number} additionalTank
* @return {number}
*/
/*
var distanceTraveled = function(mainTank, additionalTank) {
let distance = 0 * 1;
let additionalDistance = 0 * 1;

while (mainTank > 1) {
    console.log("main tank, additional tank: ", mainTank, ", ", additionalTank);

    if (distance % 5 == 0) {
        additionalTank = additionalTank - 1;
        additionalDistance = additionalDistance + 1;
        mainTank = mainTank - 1;
        distance = distance + 1;
    }
    else {
        mainTank = mainTank - 1;
        distance = distance + 1;
    }
    console.log("distance = ", distance);
}

distance++;

console.log("final distance = ", (distance + additionalDistance) * 10);
return (distance + additionalDistance) * 10;
}
*/

//this was an example solution given, it does what the obvious thing in a decently obvious way i didn't think of
/**
 * @param {number} mainTank
 * @param {number} additionalTank
 * @return {number}
 */
var distanceTraveled = function(mainTank, additionalTank) {
    let distance = 0;

    if (mainTank < 5) {
        let remainingFuel = mainTank % 5;
        return remainingFuel * 10;
    }

    while (mainTank > 0 || additionalTank > 0) {
        if (mainTank >= 5) {
            distance += 50;
            mainTank -= 5;

            if (additionalTank > 0) {
                mainTank += 1;
                additionalTank -= 1;
            }
        } else {
            let remainingFuel = mainTank % 5;
            distance += remainingFuel * 10;
            break;
        }
    }

    return distance;
};
