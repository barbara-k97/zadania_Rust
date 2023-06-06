// Zad : 977. Squares of a Sorted Array
// https://leetcode.com/problems/squares-of-a-sorted-array/
// Bibliografia : https://doc.rust-lang.org/std/iter/struct.Map.html 
// https://doc.rust-lang.org/std/primitive.slice.html#method.sort


impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        // wykorzystanie mapowania do potęgowania
        let mut v: Vec<i32> = nums.into_iter().map(|x| x*x).rev().collect(); 
        v.sort();  //sortowanie 
        return v ;
    }
}