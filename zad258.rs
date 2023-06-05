// Zad 258. Add Digits 
// https://leetcode.com/problems/add-digits/

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let mut suma = 0;
        let mut liczba = num;
        while liczba / 10 > 0 {
            while liczba > 0 {
                suma += liczba%10;
                liczba /= 10;
            }
            liczba = suma;
            suma = 0;
            println!("{}",liczba);
        }
        return liczba;
    }
}