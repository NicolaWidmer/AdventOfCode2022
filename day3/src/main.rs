use std::collections::{HashSet,HashMap};

fn main() {

    let mut points = HashMap::new();
    for i in 0..26{
        points.insert('a' as i64 + i,i+1);
        points.insert('A' as i64 + i,i+27);
    }

    println!("The ans for part 1 is {}",solve1(&points));
    println!("The ans for part 2 is {}",solve2(&points));
}

fn solve2(points:&HashMap<i64,i64>) -> i64{
    let mut ans:i64 = 0;

    let vec: Vec<&str>  = include_str!("in.txt").split('\n')
    .collect();

    for i in 0..(vec.len()/3){
        
        let s1:HashSet<char> = HashSet::from_iter(vec[3*i].chars());
        let s2:HashSet<char> = HashSet::from_iter(vec[3*i+1].chars());

        for c in vec[3*i+2].chars(){
            if s1.contains(&c)&&s2.contains(&c){
                ans+=points.get(&(c as i64)).unwrap();
                break;
            }
        }

    }

    return ans;
}

fn solve1(points:&HashMap<i64,i64>) -> i64{
    let mut ans:i64 = 0;

    let vec: Vec<(&str,&str)>  = include_str!("in.txt").split('\n')
    .map(|x| {x.split_at(x.len()/2)})
    .collect();

    for (fst,snd) in vec{

        let mut curs=HashSet::new();

        for c in fst.chars(){
            curs.insert(c);
        }

        for c in snd.chars(){
            if curs.contains(&c){
                ans+=points.get(&(c as i64)).unwrap();
                break;
            }
        }

    }

    return ans;
}

