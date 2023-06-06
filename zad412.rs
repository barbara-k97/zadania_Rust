// Zad. 412  Fizz Buzz
// https://leetcode.com/problems/fizz-buzz/ 
// Bibliografia :
//  https://doc.rust-lang.org/std/string/struct.String.html#method.push
// https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push 
// https://doc.rust-lang.org/alloc/vec/struct.Vec.html

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut wynik = vec![]; // wektor do przechopwywania 
        // petla sprawdzająca wszystkie liczby od 1 do n
        for a in 1..=n {
            if a%3 == 0 &&  a%5 == 0 {
                 wynik.push("FizzBuzz".to_string());
              }
            else if a % 3 == 0{
                  wynik.push("Fizz".to_string());
              }
             else if a % 5 == 0 {
                  wynik.push("Buzz".to_string());
              }
            else {
                wynik.push(a.to_string());
              }
        }

        wynik
    
    }
}


