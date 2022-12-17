use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Eq,PartialEq,Clone,Copy)]
enum Direction{
    Left,
    Right,
}

type Point = (i64,i64);
type HeightMap = (i64,i64,i64,i64,i64,i64,i64);

type State = (HeightMap,usize,usize);

type Block = Vec<Point>;

fn main() {
    let dirs : Vec<Direction> = include_str!("in.txt").chars().map(char_to_dir).collect();
    
    let blocks:Vec<Block> = vec![
        vec![(0,0),(1,0),(2,0),(3,0)],
        vec![(1,0),(0,1),(1,1),(2,1),(1,2)],
        vec![(0,0),(1,0),(2,0),(2,1),(2,2)],
        vec![(0,0),(0,1),(0,2),(0,3)],
        vec![(0,0),(1,0),(0,1),(1,1)]];

    let mut rocks: HashSet<Point> = HashSet::new();
    let mut maxhs:Vec<i64> = vec![-1;7];

    let _m = dirs.len();
    let n = blocks.len();

    let mut dir_ind = 0;

    let rounds1 = 2022;
    let rounds2 = 1_000_000_000_000;

    let mut i=0;
    let mut states:HashMap<State,(usize,i64)> = HashMap::new();

    let mut ans1 = -1;
    let mut ans2 = -1;

    states.insert((heights_to_heightmap(&maxhs),dir_ind,i%n),(0,-1));

    loop{

        simulate_round(&mut maxhs,&blocks,i%n,&mut rocks,&dirs,&mut dir_ind);

        i+=1;

        if i == rounds1{
            ans1 = maxhs.iter().max().unwrap()+1;
        }

        if ans2==-1{

            let state = (heights_to_heightmap(&maxhs),dir_ind,i%n);

            match states.get(&state){
                None=> (),
                Some((round,max))=>{
                    let max2:i64 = *maxhs.iter().max().unwrap();
                    let maxdif = max2-max;
                    let rounddif = i-round;
                    let numrounds = rounds2 - round;
                    ans2 = max+((numrounds/rounddif) as i64)*maxdif;
    
                    for j in 0..numrounds%rounddif{
                        if i+j == rounds1{
                            ans1 = maxhs.iter().max().unwrap()+1;
                        }
                        simulate_round(&mut maxhs,&blocks,(i+j)%n,&mut rocks,&dirs,&mut dir_ind);
                    }

                    ans2+= maxhs.iter().max().unwrap()-max2+1;

                    if ans1!=-1{
                        break;
                    }
                }
            }

            states.insert(state,(i,*maxhs.iter().max().unwrap()));

            if ans1!=-1&&ans2!=-1{
                break;
            }
        }
        
    }
    println!("The ans for part 1 is {}",ans1);

    println!("The ans for part 2 is {}",ans2);

}





fn simulate_round(maxhs:&mut Vec<i64>,blocks:&Vec<Block>,block_ind:usize,rocks:&mut HashSet<Point>,dirs:&Vec<Direction>,dir_ind:&mut usize){
    let maxh = maxhs.iter().max().unwrap();
    let mut cur_block: Block = blocks[block_ind].iter().map(|(x,y)| (x+2,y+4+*maxh)).collect();
        loop {
            let mut next_block=move_block_dir(cur_block,dirs[*dir_ind],&rocks);
            let cur = (*dir_ind+1)%dirs.len();
            *dir_ind=cur;
            cur_block=next_block;
            next_block = move_block_down(cur_block.clone(),&rocks);
            if next_block==cur_block{
                for r in &cur_block{
                    for (x,y) in &cur_block{
                        if maxhs[*x as usize]<*y{
                            maxhs[*x as usize]=*y;
                        }
                    }
                    rocks.insert(*r);
                }
                break;
            }
            cur_block=next_block
        }

}

fn heights_to_heightmap(heights:&Vec<i64>)->HeightMap{
    let maxh = heights.iter().max().unwrap();
    (heights[0]-maxh,heights[1]-maxh,heights[2]-maxh,heights[3]-maxh,heights[4]-maxh,heights[5]-maxh,heights[6]-maxh)
}


fn move_block_down(block:Block,rocks:&HashSet<Point>)->Block{
    let ans:Block = block.iter().map(|(x,y)| (*x,*y-1)).collect();
    if ans.iter().all(|(x,y)| *y>=0&&!rocks.contains(&(*x,*y)) ){
        return ans;
    }
    block
}

fn move_block_dir(block:Block,dir:Direction,rocks:&HashSet<Point>)->Block{
    let ans:Block = block.iter().map(|(x,y)| 
    {
        if dir==Direction::Left{
            (*x-1,*y)
        }
        else{
            (*x+1,*y)
        }
    }
    ).collect();
    if ans.iter().all(|(x,y)| 0<=*x&&*x<7&&!rocks.contains(&(*x,*y)) ){
        return ans;
    }
    block
}

fn char_to_dir(c:char)->Direction{
    if c == '<' {Direction::Left} else {Direction::Right}
}

fn print_rocks(rocks:&HashSet<Point>,h:i64){
    for y in (0..h+1).rev(){
        print!("|");
        for x in 0..7{
            print!("{}",if rocks.contains(&(x,y)){'#'} else {'.'});
        }
        println!("|");
    }
    println!("+-------+");

}

