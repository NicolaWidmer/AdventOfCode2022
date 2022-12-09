use std::collections::HashSet;

fn main() {
    let vec : Vec<(&str,i64)> = include_str!("in.txt").split('\n')
    .map(|x| {let mut it = x.split(" ");
             (it.next().unwrap(),str::parse::<i64>(it.next().unwrap()).unwrap())})
    .collect();
    println!("The ans for part 2 is {}",solve(&vec,2));
    println!("The ans for part 2 is {}",solve(&vec,10));
}

fn solve(vec:&Vec<(&str,i64)>,size:usize)->usize{
    let mut set:HashSet<(i64,i64)> = HashSet::new();
    let mut rope:Vec<(i64,i64)> = vec![(0,0);size];
    for (s,n) in vec{
        for _ in 0..*n{
            let head = rope[0];
            rope[0]=move_head(head,s);
            for i in 1..size{
                let prev = rope[i-1];
                let cur = rope[i];
                if is_disconnected(prev,cur){
                    rope[i]=move_cur(cur,prev);
                }
            }
            set.insert(rope[size-1]);
        }
    }
    return set.len();
}

fn move_head(head:(i64,i64),s:&str)->(i64,i64){
    match s{
        "L"=>(head.0,head.1-1),
        "R"=>(head.0,head.1+1),
        "D"=>(head.0-1,head.1),
        "U"=>(head.0+1,head.1),
        _ => panic!("illegal direction"),
    }
}

fn is_disconnected(p1:(i64,i64),p2:(i64,i64))->bool{
    (p1.0-p2.0).abs()>1 || (p1.1-p2.1).abs()>1
}

fn move_cur(cur:(i64,i64),prev:(i64,i64))->(i64,i64){
    let mut ans = cur;

    if prev.0-cur.0>0{
        ans.0=cur.0+1;
    }
    else if prev.0-cur.0<0{
        ans.0=cur.0-1;
    } 

    if prev.1-cur.1>0{
        ans.1=cur.1+1;
    }
    else if prev.1-cur.1<0{
        ans.1=cur.1-1;
    }
    ans
}

