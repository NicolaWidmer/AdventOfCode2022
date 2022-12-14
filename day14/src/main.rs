use std::collections::HashSet;

type Point = (usize,usize);

fn insert_line(p1:&Point,p2:&Point,set:&mut HashSet<Point> ){
    for i in std::cmp::min(p1.0,p2.0)..=std::cmp::max(p1.0,p2.0){
        for j in std::cmp::min(p1.1,p2.1)..=std::cmp::max(p1.1,p2.1){
            set.insert((i,j));
        }
    }
}

fn fill(filled:&mut HashSet<Point>,is_round1:bool,maxy:usize){
    loop {
        let mut curx:usize = 500;
        let mut cury:usize = 0;
        if filled.contains(&(curx,cury)) {
            return;
        }
        loop {
            if is_round1&&cury>maxy{
                return;
            }
            else if !is_round1&&cury==maxy+1{
                filled.insert((curx,cury));
                break;
            }
            else if !(filled.contains(&(curx,cury+1))){
                cury+=1;
            }
            else if !(filled.contains(&(curx-1,cury+1))){
                curx-=1;
                cury+=1;
            }
            else if !(filled.contains(&(curx+1,cury+1))){
                curx+=1;
                cury+=1;
            }
            else{
                filled.insert((curx,cury));
                break;
            }
        }
    }
}

fn main() {

    let mut filled:HashSet<Point> = HashSet::new();
    let mut maxy:usize = 0; 

    let lines:Vec<Vec<Point>> = include_str!("in.txt").split('\n')
    .map(|x| x.split(" -> ")
               .map(|y| {
                    let mut it = y.split(',').map(str::parse::<usize>).map(Result::unwrap);
                    (it.next().unwrap(),it.next().unwrap())
            
            }).collect()
        ).collect();


    for v in lines.iter(){
        let mut it = v.iter();
        let mut prev = it.next().unwrap();
        maxy=std::cmp::max(maxy,prev.1);
        for p in v{
            insert_line(prev,p,&mut filled);
            prev=p;
            maxy=std::cmp::max(maxy,prev.1);
        }
    }
    
    let perfil = filled.len();

    fill(&mut filled,true,maxy);

    println!("The ans for part 1 is {}",filled.len()-perfil);

    fill(&mut filled,false,maxy);

    println!("The ans for part 2 is {}",filled.len()-perfil);
}

