fn main() {
    let input = include_str!("input1.txt");
    
    let mut parsed = input.split("\n\n").map(|x| {
        return x.lines().flat_map(str::parse::<u32>).sum::<u32>();
    }).collect::<Vec<u32>>();

    parsed.sort_by(|a, b| b.cmp(a));

    println!("1: {0}", parsed[0]);
    print!("2: {0}", parsed.iter().take(3).sum::<u32>());

} 
