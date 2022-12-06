use std::collections::HashSet;

fn main() {
    let s = include_str!("in.txt");

    for i in 4..(s.len()){
        let set:HashSet<char> = HashSet::from_iter(s[i-4..i].chars());
        if set.len() == 4{
            println!("The ans for part 1 is {}",i);
            break;
        }
    }
    
    for i in 14..(s.len()){
        let set:HashSet<char> = HashSet::from_iter(s[i-14..i].chars());
        if set.len() == 14{
            println!("The ans for part 2 is {}",i);
            break;
        }
    }
    
}

