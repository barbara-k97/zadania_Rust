 // Zad 326: https://leetcode.com/problems/power-of-three/ 
impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        let mut a = n; // kopiujemy n do zmiennej mut-owalnej 
        if a == 0 {
            return false;
        }
        if a < 0 {
            return false;
            //liczby ujemne podniesione do potegi 3 sa ujemne
        }
        if a > 1 {
            while a % 3 == 0 {
                a /= 3;
            }
        }
       a == 1 // 3^1=3 
          
    }
}