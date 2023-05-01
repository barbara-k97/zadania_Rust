// Zad 2660 : https://leetcode.com/problems/determine-the-winner-of-a-bowling-game/
impl Solution {
    pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
        let mut wynik1 = 0;
        let mut wynik2 = 0;
        let mut dl1 = player1.len();
        let mut dl2 = player2.len();
        // wynik gracz nr 1
        for a in 0..dl1{
            if (a>1 && player1[a-2]==10 ) || ( a>0 && player1[a-1]==10 ){
                wynik1 += (2*player1[a]);
                }
            else 
                {wynik1 += player1[a];}
        }
        // wynik gracz nr 2
        for b in 0..dl2 {
              if (b>1 && player2[b-2]==10 ) || ( b>0 && player2[b-1]==10 ){
                wynik2 += (2*player2[b]);
                }
            else 
                {wynik2 += player2[b];}
        }
    
        println!("{}", wynik1);
        println!("{}", wynik2);
        if wynik1 == wynik2 {
            return 0;
        }
        if wynik1 > wynik2 {
            return 1;
        }
        return 2;

    }
}