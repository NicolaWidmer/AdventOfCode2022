use std::collections::VecDeque;

fn main() {
    let vec : Vec<&str> = include_str!("in.txt").split('\n').collect();
    solve1(&vec);
    solve2(&vec);
}

fn solve2(vec:&Vec<&str>){
    let mut stacks:Vec<VecDeque<char>> = Vec::new();
    for _i in 0..10{
        stacks.push(VecDeque::new());
    }
    for s in &vec[0..8]{
        for j in 0..(s.len()/4+1){
            let c:char = s.chars().nth(j*4+1).unwrap();
            if c !=' '{
                stacks[j+1].push_front(c);
            }
        } 
    }

    for s in &vec[10..]{
        let mov: Vec<&str> = s.split(' ').collect();

        let times = str::parse::<i8>(mov[1]).unwrap();
        let from = str::parse::<usize>(mov[3]).unwrap();
        let to = str::parse::<usize>(mov[5]).unwrap();

        let mut curstack : VecDeque<char> = VecDeque::new();

        for _i in 0..times{
            curstack.push_back(stacks[from].pop_back().unwrap());
        }
        for _i in 0..times{
            stacks[to].push_back(curstack.pop_back().unwrap());
        }
    }

    print!("ans2 = ");
    for i in 1..10{
        print!("{}",stacks[i].pop_back().unwrap());
    }
    println!("");
}

fn solve1(vec:&Vec<&str>){
    let mut stacks:Vec<VecDeque<char>> = Vec::new();
    for _i in 0..10{
        stacks.push(VecDeque::new());
    }
    for s in &vec[0..8]{
        for j in 0..(s.len()/4+1){
            let c:char = s.chars().nth(j*4+1).unwrap();
            if c !=' '{
                stacks[j+1].push_front(c);
            }
        } 
    }

    for s in &vec[10..]{
        let mov: Vec<&str> = s.split(' ').collect();

        let times = str::parse::<i8>(mov[1]).unwrap();
        let from = str::parse::<usize>(mov[3]).unwrap();
        let to = str::parse::<usize>(mov[5]).unwrap();

        for _i in 0..times{
            let val = stacks[from].pop_back().unwrap();
            stacks[to].push_back(val);
        }
    }

    print!("ans1 = ");
    for i in 1..10{
        print!("{}",stacks[i].pop_back().unwrap());
    }
    println!("");
}