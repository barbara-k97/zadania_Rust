// Zad 342.Power of Four
// https://leetcode.com/problems/power-of-four/description/
impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        let mut liczba = n;
       // 4 do potęgi 0 to 1 , dla 1 to prawda
        if n==1 {
           return true;
        }
        if n<4 {
            return false;
        }
        //potęgi 4 to 4,16,64,256,...
        // liczba jest potęgą 4 gdy reszta z dzielenia tej liczby przez 4 jest równa 0
      while liczba>1{
            if liczba%4 != 0{
                return false;
            }
            liczba = liczba/4;
        }
        return true;
    }
}