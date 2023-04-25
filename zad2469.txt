// Zad 2469: https://leetcode.com/problems/convert-the-temperature/
impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        //Kelvin = Celsius + 273.15
        let k = celsius + 273.15;
        //Fahrenheit = Celsius * 1.80 + 32.00
        let f = (celsius*1.80) + 32.00;
        vec![k,f]
    }
}