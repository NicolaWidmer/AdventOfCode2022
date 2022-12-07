fn main() {
    let vec : Vec<i64> = include_str!("in.txt").split('\n')
    .map(str::parse::<i64>)
    .map(Result::unwrap)
    .collect();
    
    let ans1 : i64 = vec.iter().sum();

    println!("The ans for part 1 is {}",ans1);
    
    let ans2 : i64 = vec.iter().sum();

    println!("The ans for part 2 is {}",ans2);
}

