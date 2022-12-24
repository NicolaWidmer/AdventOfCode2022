use std::collections::HashMap;
use std::collections::HashSet;

type Point = (i64,i64);

fn main() {
    let dirs = vec![vec![(-1,1),(-1,0),(-1,-1)],vec![(1,1),(1,0),(1,-1)],vec![(-1,-1),(0,-1),(1,-1)],vec![(1,1),(0,1),(-1,1)]];
    let mut set:HashSet<Point> = HashSet::new();
    let mut start_dir:usize = 0;

    let vec : Vec<Vec<char>> = include_str!("in.txt").split('\n')
    .map(|s| s.chars().collect())
    .collect();
    for i in 0..vec.len(){
        for j in 0..vec[i].len(){
            if vec[i][j]=='#'{
                set.insert((i as i64,j as i64));
            }
        }
    }

    let n = set.len();
    let mut j = 0;

    loop {
        let mut no_change=true;

        let mut next_pos:HashMap<Point,i64> = HashMap::new();
        let mut next_set:HashSet<Point>  = HashSet::new();
        for key in &set{
            let ind = next_dir(*key,start_dir,&dirs,&set);
            if ind!=4{
                let possible_next =  (key.0+dirs[ind][1].0,key.1+dirs[ind][1].1);
                let cur = next_pos.get(&possible_next).unwrap_or(&0);
                next_pos.insert(possible_next,cur+1);
            }
        }

        for key in &set{
            let ind = next_dir(*key,start_dir,&dirs,&set);
            if ind!=4{
                let possible_next =  (key.0+dirs[ind][1].0,key.1+dirs[ind][1].1);
                let next_key = if *next_pos.get(&possible_next).unwrap()==1{
                    no_change=false;
                    possible_next
                }
                else{
                    *key
                };
                next_set.insert(next_key);
            }
            else {
                next_set.insert(*key);
            }
        }
        set = next_set;
        start_dir=(start_dir+1)%4;
        j+=1;
        if j==10{
            
            let (mut maxx,mut maxy,mut minx,mut miny):(i64,i64,i64,i64) = (i64::MIN,i64::MIN,i64::MAX,i64::MAX);
            for p in &set{
                maxx = max(maxx,p.0);
                minx = min(minx,p.0);
                maxy = max(maxy,p.1);
                miny = min(miny,p.1);
            }
            println!("the ans for part 1 is {}",(maxx-minx+1)*(maxy-miny+1)-n as i64);
        }

        if no_change{
            println!("the ans for part 2 is {j}");
            break; 
        }

    }
}
fn max(a:i64,b:i64)->i64{
    std::cmp::max(a,b)
}
fn min(a:i64,b:i64)->i64{
    std::cmp::min(a,b)
}

fn next_dir(p:Point,start_dir:usize,dirs:&Vec<Vec<Point>> ,set:&HashSet<Point>)->usize{
    let mut oround_all_free = true;
    for i in 0..4{
        for d in &dirs[i]{
            if set.contains(&(p.0+d.0,p.1+d.1)){
                oround_all_free = false;
            }
        }
    }
    if oround_all_free{
        return 4;
    }
    
    for i in 0..4{
        let mut b = true;
        for d in &dirs[(i+start_dir)%4]{
            if set.contains(&(p.0+d.0,p.1+d.1)){
                b = false;
            }
        }
        if b{
            return (i+start_dir)%4;
        }
    }
    return 4;
}
