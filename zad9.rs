// Zad 9: https://leetcode.com/problems/palindrome-number/description/ 
// Bibliografia : https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.rev ,https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect 
// rev() - odwaca kierunek iteratora , iteruje od prawej do lewej
// collect() - przekszrałca iterator w kolejkę, String mozna zbudowac z char
// chars() - zwraca iterator zwycinka łańcucha 
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let a:String = x.to_string();
        let slowo:Vec<_> = a.chars().collect();
        let odwroc_slowo:Vec<_> = a.chars().rev().collect();

        if slowo==odwroc_slowo { return true  } 
        else { return false }
    }
}