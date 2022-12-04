fn main() {
    let vec:Vec<(Vec<i64>,Vec<i64>)>  = include_str!("in.txt").split('\n')
    .map(|x| {
        let mut iter = 
        x.split(",")
        .map(|y| 
                y.split("-")
                .map(str::parse::<i64>)
                .map(Result::unwrap)
                .collect()
        );
        (iter.next().unwrap(),iter.next().unwrap())
    })
    .collect();
    
    let ans1 : usize = vec.iter().filter(|(r1,r2)| 
       (r1[0]<=r2[0]&&r1[1]>=r2[1]) || (r2[0]<=r1[0]&&r2[1]>=r1[1])
    ).count();

    let ans2 : usize = vec.iter().filter(|(r1,r2)|
        std::cmp::max(r1[0],r2[0])<=std::cmp::min(r1[1],r2[1])
    ).count();
    

    println!("The ans for part 1 is {}",ans1);
    println!("The ans for part 2 is {}",ans2);
}


