use std::collections::HashMap;

use fasthash::metro;

fn main() {
    let mut path : Vec<&str> = Vec::from_iter([""]);
    let mut sizes : HashMap<u64,i64> = HashMap::new();
    

    for s in include_str!("in.txt").split('\n'){

        if s.starts_with("$ ls") || s.starts_with("$ cd /") || s.starts_with("dir"){
            continue;
        }

        if s=="$ cd .."{
            path.pop();
        }
        else {
            let vec:Vec<&str> = s.split(" ").collect();
            if vec.len()==3{
                path.push(vec[2])
            }
            else {
                let size : i64 = str::parse(vec[0]).unwrap();
                let mut curpath: String = "".to_string();
                for d in &path{
                    curpath.push_str("/");
                    curpath.push_str(d);
                    let ha:u64= metro::hash64(&curpath);
                    let newsize = *sizes.get(&ha).unwrap_or(&0)+size;
                    sizes.insert(ha,newsize);
                }
            }
        }
    }

    let ans1 : i64 = sizes.iter().map(|(_,val)| if *val<=100000 {*val} else {0}).sum();

    println!("The ans for part 1 is {}",ans1);

    let space = *sizes.get(&metro::hash64(&"/".to_string())).unwrap_or(&0)-40000000;
    let ans2 : i64 = *sizes.iter().filter_map(|(_,val)|if *val>=space {Some(val)} else {None}).min().unwrap();

    println!("The ans for part 1 is {}",ans2);
}

