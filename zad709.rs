// Zad 709 : https://leetcode.com/problems/to-lower-case/
// https://doc.rust-lang.org/std/primitive.str.html#method.to_ascii_lowercase
impl Solution {
    pub fn to_lower_case(s: String) -> String {
        // wykorzystuje wbudowaną metodę
        let mut string =s;
        let mut x= string.to_ascii_lowercase();
        return x;
    }
}