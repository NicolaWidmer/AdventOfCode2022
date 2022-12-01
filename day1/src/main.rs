fn main() {
    let vec :Vec<&str> = include_str!("day1.txt").split('\n').collect();
    
    let mut ans:Vec<i64>=Vec::new();
    let mut cur:i64=0;

    for s in vec{
        if s==""{
            ans.push(cur);
            cur=0;
        }
        else{
            cur = cur + s.parse::<i64>().unwrap()
        }
    }

    ans.sort();

    println!("The ans for part 1 is {}",ans[ans.len()-1]);
    println!("The ans for part 2 is {}",ans[ans.len()-1]+ans[ans.len()-2]+ans[ans.len()-3]);
}

