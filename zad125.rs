//Zad 125 https://leetcode.com/problems/valid-palindrome/
// https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
 
     let s: String = s
            .chars()
            .filter(|x| x.is_alphanumeric()) 
            .map(|x| x.to_ascii_lowercase())
            .collect();
// rev() - odwaca kierunek iteratora , iteruje od prawej do lewej
        println!("{}",s);
        if s.chars().rev().collect::<String>()==s{
            return true;
        }
        else{
            return false;
        }

    }
}
 