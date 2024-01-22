/*
Write a function expect that helps developers test their code. It should take in any value val and return an object with the following two functions.

toBe(val) accepts another value and returns true if the two values === each other. If they are not equal, it should throw an error "Not Equal".
notToBe(val) accepts another value and returns true if the two values !== each other. If they are equal, it should throw an error "Equal".
*/

/**
 * @param {string} val
 * @return {Object}
 */
var expect = function(val) {
    return {
        toBe: function(input) {
            if (input === val) {
                return true;
            } else {
                throw new Error("Not Equal");
            }
        },
        notToBe: function(input) {
            if (input !== val) {
                return true;
            } else {
                throw new Error("Equal");
            }
        }
    };
}/*
Write a function expect that helps developers test their code. It should take in any value val and return an object with the following two functions.

toBe(val) accepts another value and returns true if the two values === each other. If they are not equal, it should throw an error "Not Equal".
notToBe(val) accepts another value and returns true if the two values !== each other. If they are equal, it should throw an error "Equal".
*/

/**
 * @param {string} val
 * @return {Object}
 */
var expect = function(val) {
    return {
        toBe: function(input) {
            if (input === val) {
                return true;
            } else {
                throw new Error("Not Equal");
            }
        },
        notToBe: function(input) {
            if (input !== val) {
                return true;
            } else {
                throw new Error("Equal");
            }
        }
    };
};;
