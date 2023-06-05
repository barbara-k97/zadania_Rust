// Zad 509. Fibonacci Number
// https://leetcode.com/problems/fibonacci-number/
//self jako bieżący odbiornik dla metody pozwala w większości przypadków pominąć typ parametru. Z wyjątkiem tej szczególnej cechy,  (https://doc.rust-lang.org/std/keyword.self.html)

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n < 2 {
            n
        } else {
             Self::fib(n-2)  +  Self::fib(n-1) 
        }
    }
}