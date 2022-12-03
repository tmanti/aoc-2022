fn main(){
    let rock_score = 1;
    let paper_score = 2;
    let scissor_score = 3;

    let loss = 0;
    let draw = 3;
    let win = 6;

    let input = include_str!("input2.txt");

    let mut score = 0;

    let rounds = input.lines();

    for x in rounds.into_iter() {
        let round = x.split(" ").collect::<Vec<&str>>();
        let strat = round[0];
        let resp = round[1];

        
    }

}