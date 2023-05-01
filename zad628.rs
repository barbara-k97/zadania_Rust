// Zad 628 : https://leetcode.com/problems/maximum-product-of-three-numbers/
// https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort
// https://doc.rust-lang.org/std/primitive.f32.html#method.abs 
impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut p =  nums;         
        //badamy dlugosc 
        let mut l = p.len();
        // sortowanie od min do max
        p.sort();
        //gdy same dodatnie : pobieramy 3 ostatnie indeksy i mnozymy
        let mut r = p[l-1]*p[l-2]*p[l-3];
        //gdy ujemne : pobieramy 2 pierwsze i najwieksza dodatnia 
        let mut e = p[0]*p[1]*p[l-1];
        if r < e {
            return e;
        }
        else {
            return r;
        }
        
    }
}