type Point = (i64,i64);
type Interval = (i64,i64);

use itertools::{max,min};
use std::collections::HashSet;

fn main() {

    
    let vec : Vec<(Point,Point)> = include_str!("in.txt").split('\n')
    .map(|s| s.replace(" x=","").replace(" y=","").replace("Sensor at",""))
    .map(|s| {  let it = s.split(": closest beacon is at");
                let mut iter = it.map(|x| {
                    let mut it = x.split(",")
                    .map(str::parse::<i64>)
                    .map(Result::unwrap);
                    (it.next().unwrap(),it.next().unwrap())
                });
                (iter.next().unwrap(),iter.next().unwrap())
    })
    .collect();


    let y_part1 = 2000000;

    let dist = vec.iter().map(|(s,b)| manhattan_dist(s,b));

    let xcords = vec.iter().map(|(s,_)| s.0);

    let maxdist = max(dist).unwrap();

    let minx = min(xcords.clone()).unwrap();
    let maxx = max(xcords).unwrap();

    let mut inters = vec![(minx-maxdist-1,maxx+maxdist+1)];
    let mut nextinters: Vec<Interval> = Vec::new();

    for (s,b) in &vec{
        match not_pos(y_part1,s,b){
            None => continue,
            Some(no_s) => {
                for interval in inters{
                    nextinters.append(&mut split_interval(interval,no_s));
                }
            inters = nextinters;
            nextinters = Vec::new();
            },
        }
    }

    let beacon_on_y:HashSet<Point> = HashSet::from_iter(
        vec.iter()
        .filter_map( |(_,b)| if b.1==y_part1 {Some(*b)} else {None})
    );
    let mut ans1:i64 = -((beacon_on_y).len() as i64);

    for i in 0..inters.len()-1{
        ans1 += inters[i+1].0-inters[i].1-1
    }

    println!("The ans for part 1 is {}",ans1);

    let cords:i64 = 4000000;
    for y in 0..cords{
        let mut poscords: Vec<Interval> = vec![(0,cords)];
        let mut nexcords: Vec<Interval> = Vec::new();
        for (s,b) in &vec{
            match not_pos(y,s,b){
                None => continue,
                Some(no_s) => {
                    for interval in poscords{
                        nexcords.append(&mut split_interval(interval,no_s));
                    }
                poscords = nexcords;
                nexcords = Vec::new();
                },
            }
        }
        if poscords.len()!=0{
            println!("The ans for part 2 is {}",poscords[0].0*4000000+y);
            break;
        }
    }
}

fn manhattan_dist(p1:&Point,p2:&Point)->i64{
    (p1.0-p2.0).abs()+(p1.1-p2.1).abs()
}

fn not_pos(y:i64,s:&Point,b:&Point)->Option<Interval>{
    let dis = manhattan_dist(s,b);
    let absy = (y-s.1).abs();
    if dis < absy{
        return None;
    }
    return Some(((absy-dis+s.0),(dis-absy+s.0)))

}

fn split_interval(pos_s:Interval,no_s:Interval) -> Vec<Interval>{
    if no_s.0<=pos_s.0 && no_s.1>=pos_s.1{
        return Vec::new();
    }
    else if no_s.0<=pos_s.0 && no_s.1<pos_s.1{
        return vec![(std::cmp::max(pos_s.0,no_s.1+1),pos_s.1)];
    }
    else if no_s.0>pos_s.0 && no_s.1>=pos_s.1{
        return vec![(pos_s.0,std::cmp::min(pos_s.1,no_s.0-1))];
    }
    else{
        return vec![(pos_s.0,no_s.0-1),(no_s.1+1,pos_s.1)];
    }
}
