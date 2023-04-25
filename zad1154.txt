// Zad 1154 : https://leetcode.com/problems/day-of-the-year/ 
// https://doc.rust-lang.org/std/primitive.str.html#method.split
impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        const dni: [i32;12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        //split - dzieli po okreslonym znaku
        let mut d = date.split("-").map(|item| item.parse().unwrap()).collect::<Vec<i32>>();
        
        let mut rok = d[0];
        let mut miesiac = d[1];
        let mut dzien = d[2];

        let mut dni_miesiaca = 0;
        let mut a =0;
        for a in 0..miesiac-1 {
            dni_miesiaca += dni[a as usize];
        }
        let mut m = dni_miesiaca + dzien;
        //rok przestepny co 4 lata z wyłączeniem stuleci : luty ma 29 dni
        if miesiac>2 && ((rok % 4 == 0 && rok % 100 != 0) || rok % 400 == 0 ){
                m += 1; 
        }

        return m;   
    }
}