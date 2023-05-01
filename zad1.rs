// Zad 1 : https://leetcode.com/problems/two-sum/ 
// http://rust.w8.pl/book/ch04-03-slices.html?highlight=enumerate#wycinki 

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // wykorzystuje podojną pętle i Vec (dostęp do wartosci według indexu)
        // iter().enumerate() - Zwrócony iterator zwraca pary (i, val), gdzie i jest bieżącym  indeksem iteracji i val jest wartością zwróconą przez iterator.

        for (i,j) in nums.iter().enumerate() {
            for (q,w) in nums[i+1..].iter().enumerate(){
                if (j+w == target){
                    return vec![i as i32 , (i+q+1) as i32]
                }
            }
        }
        //makro wskazujące nieosiągalny kod, wykoryztywan przy iteratorach dynamicznie się kończących iteratorów
         unreachable!()
    }
}