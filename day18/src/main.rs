use std::collections::HashSet;
use std::collections::VecDeque;

type Point = (i64,i64,i64);

fn main() {
    let vec : Vec<Point> = include_str!("in.txt").split('\n')
    .map(|x| {
        let mut it =x.split(',')
        .map(str::parse::<i64>)
        .map(Result::unwrap);
        (it.next().unwrap(),it.next().unwrap(),it.next().unwrap())
    })
    .collect();

    let points:HashSet<Point> = HashSet::from_iter(vec.iter().cloned());

    let dirs = vec![-1,1];
    
    
    let mut ans1 : i64 = 0;

    for (x,y,z) in vec{
        for d in &dirs{
            if !points.contains(&(x+d,y,z)){
                ans1+=1;
            }
            if !points.contains(&(x,y+d,z)){
                ans1+=1;
            }
            if !points.contains(&(x,y,z+d)){
                ans1+=1;
            }
        }
    }

    println!("The ans for part 1 is {}",ans1);


    let mut mincord = 0;
    let mut maxcord = 0;

    for (x,y,z) in &points{
        mincord = min(mincord,*x,*y,*z);
        maxcord = max(maxcord,*x,*y,*z);
    }
    mincord-=1;
    maxcord+=1;

    let mut ans2 : i64 = 0;

    let start = (mincord,mincord,mincord);
    let mut vis:HashSet<Point> = HashSet::new();
    let mut deq:VecDeque<Point> = VecDeque::new();

    deq.push_front(start);
    loop{
        let (x,y,z):Point = deq.pop_back().unwrap();
        for d in &dirs{
            bfs_step(&mut ans2,&mut vis,&points,&mut deq,(x+d,y,z),maxcord,mincord);
            bfs_step(&mut ans2,&mut vis,&points,&mut deq,(x,y+d,z),maxcord,mincord);
            bfs_step(&mut ans2,&mut vis,&points,&mut deq,(x,y,z+d),maxcord,mincord);
        }
        if deq.len()==0{
            break;
        } 
    }
    println!("The ans for part 2 is {}",ans2);
}

fn bfs_step(ans:&mut i64,vis:&mut HashSet<Point>, points:&HashSet<Point>,deq:&mut VecDeque<Point>,p:Point,maxcord:i64,mincord:i64){
    if vis.contains(&p)||p.0<mincord||p.0>maxcord||p.1<mincord||p.1>maxcord||p.2<mincord||p.2>maxcord{
        return;
    }
    if points.contains(&p){
        *ans=*ans+1;
        return;
    }
    deq.push_front(p);
    vis.insert(p);
}

fn min(a:i64,b:i64,c:i64,d:i64)->i64{
    std::cmp::min(std::cmp::min(a,b),std::cmp::min(c,d))
}
fn max(a:i64,b:i64,c:i64,d:i64)->i64{
    std::cmp::max(std::cmp::max(a,b),std::cmp::max(c,d))
}

