/*
Given an integer n, return the smallest prime palindrome greater than or equal to n.

An integer is prime if it has exactly two divisors: 1 and itself. Note that 1 is not a prime number.

For example, 2, 3, 5, 7, 11, and 13 are all primes.
An integer is a palindrome if it reads the same from left to right as it does from right to left.

For example, 101 and 12321 are palindromes.
The test cases are generated so that the answer always exists and is in the range [2, 2 * 108].
    */

var isPrime = (n) => {
    if (n < 2) return false
    if (n === 2) return true
    if (n % 2 === 0) return false
    for (let i = 3; i * i <= n; i += 2) {
        if (n % i === 0) {
            return false
        }
    }
    return true
}

function primePalindrome(n) {
    if (n >= 8 && n <= 11) return 11
    const numOfDigits = ('' + n).length
    for (let i = numOfDigits; i < 10 ** numOfDigits; i++) {
        const str = '' + i
        let strReverse = str.split('').reverse().join('')
        const palindromeNum = parseInt(str + strReverse.slice(1))
        if (palindromeNum >= n) {
            if (isPrime(palindromeNum)) {
                return palindromeNum
            }
        }
    }
}
