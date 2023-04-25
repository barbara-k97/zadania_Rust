// Zad 231 : https://leetcode.com/problems/power-of-two/ 
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        let mut a = n; // kopiujemy n do zmiennej mut-owalnej 
        if a == 0 {
            return false;
        }
        if a > 1 {
            while a % 2 == 0 {
                a /= 2;
            }
        }
       a == 1 // 2^1=2 
    }
    
}