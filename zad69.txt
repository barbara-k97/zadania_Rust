// Zad 69 : https://leetcode.com/problems/sqrtx/
// Wykorzystane materiały : https://doc.rust-lang.org/std/primitive.f64.html#method.sqrt 
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        //f63 - 64-bitowy typ zmiennoprzecinkowy  
        //i32 - 32-bitowego typu liczby całkowitej ze znakiem
        let y = (x as f64);
        y.sqrt() as i32
    }
}