// Zad 191 . Liczba 1 bitów
// https://leetcode.com/problems/number-of-1-bits/
impl Solution {
    pub fn hammingWeight (n: u32) -> i32 {
        // wbudowana metoda 
        // https://doc.rust-lang.org/std/primitive.u32.html#method.count_ones   -Zwraca liczbę jedynek w binarnej reprezentacji self.
     n.count_ones() as i32
    }
}