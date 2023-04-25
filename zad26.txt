// Zad 26 : https://leetcode.com/problems/remove-duplicates-from-sorted-array/
// Bibliografia : 
// https://doc.rust-lang.org/std/vec/struct.Vec.html#method.dedup
// https://doc.rust-lang.org/std/primitive.slice.html#method.sort
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // sort - sortuje elementy
        nums.sort();
        // - usuwa powtarzające się elementy
        nums.dedup();
        let n = nums.len() as i32;
        return n;
    }
}