fn main() {
    let vec : Vec<Vec<i64>> = include_str!("in.txt").split('\n')
    .map(|x| {x.split(" ")
             .map(|y| {
                if y=="A"||y=="X"{
                    return 1
                }
                else if y=="B"||y=="Y"{
                    return 2
                }
                else if y=="C"||y=="Z" {
                    return 3
                }
                panic!("not a valid char");
             })
             }.collect())
    .collect();
    let ans1 : i64 = vec.iter().map(|x| {((x[1]-x[0]+4)%3)*3+x[1]}).sum();
    let ans2 : i64 = vec.iter().map(|x| {((x[0]+x[1])%3+1)+(x[1]-1)*3}).sum();

    println!("The ans for part 1 is {}",ans1);
    println!("The ans for part 2 is {}",ans2);
}

