use std::collections::{HashMap,HashSet};

#[derive(Debug)]
enum Appe<'a>{
    Num(i64),
    Op(Vec<&'a str>),
}

fn main() {
    let mut map:HashMap<&str,Appe> = HashMap::new();
    
    for s in include_str!("in.txt").split('\n'){
        let mut it = s.split(": ");
        let name = it.next().unwrap();
        let vec:Vec<&str> = it.next().unwrap().split(" ").collect();
        if vec.len()==1{
            map.insert(name,Appe::Num(str::parse::<i64>(vec[0]).unwrap()));
        }
        else{
            map.insert(name,Appe::Op(vec));
        }
    }
    println!("The ans for part 1 is {}",eval("root",&map));


    let mut contains_unknown: HashSet<&str> = HashSet::new();

    contains_x("root",&map,&mut contains_unknown);

    if let Some(Appe::Op(vec))= map.get("root"){
        let lhs = vec[0];
        let rhs = vec[2];
        if contains_unknown.contains(lhs){
            println!("The ans for part 2 is {}",solve(eval(rhs,&map),lhs,&map,&contains_unknown));
        }
        if contains_unknown.contains(rhs){
            println!("The ans for part 2 is {}",solve(eval(lhs,&map),rhs,&map,&contains_unknown));
        }
    }
    
}

fn contains_x<'a>(s:&'a str,map:&HashMap<&str,Appe<'a>>,set:&mut HashSet<&'a str>){
    if s == "humn"{
        set.insert(s);
        return;
    }
    match map.get(s){
        Some(Appe::Num(n)) => (),
        None => (),
        Some(Appe::Op(vec)) =>{
            let lhs = vec[0];
            let rhs = vec[2];
            contains_x(lhs,map,set);
            contains_x(rhs,map,set);
            if set.contains(lhs) && set.contains(rhs){
                println!("{} and {} contain unknown",lhs,rhs);
            }
            if set.contains(lhs) || set.contains(rhs){
                set.insert(s);
            }
        }
    }
}

fn solve(n:i64,s:&str,map:&HashMap<&str,Appe>,set:&HashSet<&str>) -> i64{
    if s == "humn"{
        return n;
    }
    if let Some(Appe::Op(vec))=map.get(s){
        let lhs = vec[0];
        let rhs = vec[2];
        if set.contains(lhs){
            let val_rhs = eval(rhs,map);
            let next = match vec[1]{
                "+" => n-val_rhs,
                "-" => n+val_rhs,
                "*" => n/val_rhs,
                "/" => n*val_rhs,
                _ => panic!("{} is invalid operator",vec[1]),
            };
            return solve(next,lhs,map,set);
        }
        else{
            let val_lhs = eval(lhs,map);
            let next = match vec[1]{
                "+" => n-val_lhs,
                "-" => val_lhs-n,
                "*" => n/val_lhs,
                "/" => val_lhs/n,
                _ => panic!("{} is invalid operator",vec[1]),
            };
            return solve(next,rhs,map,set);
        }
    };
    panic!("error in solve with string {}",s);
}

fn eval(s:&str,map:&HashMap<&str,Appe>)->i64{
    match map.get(s){
        Some(Appe::Num(n)) => *n,
        Some(Appe::Op(vec)) =>{
            let lhs = eval(vec[0],map);
            let rhs = eval(vec[2],map);
            match vec[1]{
                "+" => lhs+rhs,
                "-" => lhs-rhs,
                "*" => lhs*rhs,
                "/" => lhs/rhs,
                _ => panic!("{} is invalid operator",vec[1]),
            }
        },
        None => panic!("appe {} not in map",s),
    }
}

