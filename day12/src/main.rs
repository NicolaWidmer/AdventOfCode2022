use std::collections::HashSet;
use std::collections::VecDeque;

type Point = (i64,i64);
type Grid =  Vec<Vec<char>>;

fn main() {
    let mut grid : Grid = include_str!("in.txt").split('\n')
    .map(|x| x.chars().collect())
    .collect();

    let n = grid.len();
    let m = grid[0].len();

    let mut start:Point=(0,0);
    let mut end:Point=(0,0);


    for i in 0..n{
        for j in 0..m{
            if grid[i][j]=='E'{
                end=(i as i64,j as i64);
                grid[i][j]='z';
            }
            else if grid[i][j]=='S'{
                start=(i as i64,j as i64);
                grid[i][j]='a';
            }
        }
    }

    bfs(start,end,&grid,n as i64,m as i64);

}

fn bfs(start:Point,end:Point,grid:&Grid,n:i64,m:i64){

    let mut vis:HashSet<Point> = HashSet::new();
    vis.insert(end);

    let mut deq:VecDeque<(Point,i64)> = VecDeque::new();
    deq.push_front((end,0));

    let mut found_part2 = false;

    loop{

        let (curp,dis) = deq.pop_back().unwrap();

        if !found_part2 && grid[curp.0 as usize][curp.1 as usize] == 'a'{
           println!("The ans for part 2 is {}",dis);
            found_part2 = true;
        }
        
        if curp == start{
            println!("The ans for part 1 is {}",dis);
            return
        }

        for p in possible_next(curp.0,curp.1,&grid,&vis,n as i64,m as i64){
            vis.insert(p);
            deq.push_front((p,dis+1));
        }
    }
}

fn possible_next(x:i64,y:i64,grid:&Grid,vis:&HashSet<Point>,n:i64,m:i64)->Vec<Point>{
    let dirs:Vec<Point> = vec![(1,0),(-1,0),(0,1),(0,-1)];
    let mut ans:Vec<Point> = Vec::new();

    for (dx,dy) in dirs{
        if !vis.contains(&(x+dx,y+dy)) && is_in_range(x+dx,n) && is_in_range(y+dy,m) && ((grid[x as usize][y as usize] as i64)-grid[(x+dx) as usize][(y+dy) as usize] as i64 <=1){
            ans.push((x+dx,y+dy));
        }
    }

    return ans;
}

fn is_in_range(cord:i64,len:i64)->bool{
    cord>=0&&cord<len
}