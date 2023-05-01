//ZADANIE 13: https://leetcode.com/problems/roman-to-integer/
// Wykorzystane materiały : 
//http://rust.w8.pl/book/ch18-03-pattern-syntax.html 
//https://doc.rust-lang.org/std/string/struct.String.html#method.replace 

// replace - znajduje pasujący ciag i zastępuje go wskazanym 
// idea: by nie odejmować bo IV to 4 ale trzeba by odejmować od V-I, to zamienić wszystkie liczby typu 4,9 itd na odpowiedniki bez odejmowania, IX -> VIIII , i potem zsumować już znaki z odpowiednimi wartościami 
//map - mapuje sprawdzanie na wszystkie elemety, sum - wykorzystane by zsumowac wartosci
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {

        let dlugosc = s.len(); 
        let s_1 = s 
            .replace("IV", "IIII")  //4
            .replace("IX", "VIIII") //9
            .replace("XL", "XXXX") //40
            .replace("XC", "LXXXX") // 90
            .replace("CD", "CCCC") // 400
            .replace("CM", "DCCCC"); //900


        s_1.chars().map(|znak| {
            match znak {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            }
        }).sum()

    }
}