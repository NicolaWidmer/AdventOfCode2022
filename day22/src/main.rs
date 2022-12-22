type Inst = (char,usize);
type Grid = Vec<Vec<char>>;
fn main() {
    let mut it = include_str!("in.txt").split("\n\n");
    let grids:Vec<Grid> = make_grids(it.next().unwrap());
    let instrs:Vec<Inst> = make_inst(it.next().unwrap()); 


    let mut dir = 0;
    let mut grid = 0;
    let mut cords = (0,0);

    for (d,l) in instrs{

        for _ in 0..l{
            /*part1 
            let (next_grid,(x,y)) = next_cord_part1(dir,grid,cords,&grids);
            */
            let (next_dir,next_grid,(x,y)) = next_cord_part2(dir,grid,cords,&grids);
            if grids[next_grid][x][y]=='#'{
                break;
            }
            grid=next_grid;
            cords=(x,y);
            dir=next_dir;//only for part 2
        }


        dir = match d{
            '.'=> dir,
            'R' => (dir+1)%4,
            'L' => (dir+3)%4,
            _ => panic!("illegal direction {}",d),
        }
    }
    println!("the ans is {}",sovle(cords,grid,dir));
}

/* hardcoded for my input */
fn sovle(cords:(usize,usize),grid:usize,dir:usize)->usize{
    let mut ans = dir;
    ans+= 1000*(grid*50+cords.0+1);
    ans+=4*cords.1+4;
    if grid==0 || grid== 1{
        ans+=4*50;
    }
    ans
}

fn next_cord_part2(dir:usize,grid:usize,cords:(usize,usize),grids:&Vec<Grid>)->(usize,usize,(usize,usize)){

    let side_length = 50;
    let n = grids[grid].len();
    let m = grids[grid][0].len();
    let dirs = vec![(0,1),(1,0),(0,-1),(-1,0)];
    let (dx,dy) = dirs[dir];

    if (cords.1!=0 || dy!=-1) && (cords.1!=grids[grid][0].len()-1 || dy!=1) && (cords.0!=0 || dx!=-1) && (cords.0!=grids[grid].len()-1 || dx!=1){
        return (dir,grid,next_wrap(cords,dx,dy,n,m));
    }
    /*hardcoded for my input */
    else if dir==1{
        if grid==0 && cords.1<50{
            return (dir,grid+1,(0,cords.1));
        }
        else if grid==1{
            return (dir,grid+1,(0,cords.1+50));
        }
        else if grid==2 && cords.1<50{
            return (dir,grid+1,(0,cords.1));
        }


        else if grid==2 && cords.1>=50{
            return (2,3,(cords.1-50,49));
        }
        else if grid==3 {
            return (1,0,(0,cords.1+50));
        }
        else if grid==0 && cords.1>=50{
            return (2,1,(cords.1-50,49));
        }
    }
    else if dir==3{
         if grid==1{
            return (dir,0,(49,cords.1));
        }
        else if grid==2 && cords.1>=50{
            return (dir,1,(49,cords.1-50));
        }
        else if grid==3 {
            return (dir,2,(49,cords.1));
        }


        else if grid==2 && cords.1<50{
            return (0,1,(cords.1,0));
        }
        else if grid==0 && cords.1<50{
            return (0,3,(cords.1,0));
        }
        else if grid==0 && cords.1>=50{
            return (dir,3,(49,cords.1-50));
        }
    }
    else if dir==0{
        if grid==0{
            return (2,2,(49-cords.0,99));
        }
        else if grid==1{
            return (3,0,(49,cords.0+50));
        }
        else if grid==2{
            return (2,0,(49-cords.0,99));
        }
        else if grid==3{
            return (3,2,(49,cords.0+50));
        }
    }
    else if dir ==2{
        if grid==0{
            return (0,2,(49-cords.0,0));
        }
        else if grid==1{
            return (1,2,(0,cords.0));
        }
        else if grid==2{
            return (0,0,(49-cords.0,0));
        }
        else if grid==3{
            return (1,0,(0,cords.0));
        }
    }
    println!("gird {grid} dir {dir}");
    
    panic!();
}

fn next_cord_part1(dir:usize,grid:usize,cords:(usize,usize),grids:&Vec<Grid>)->(usize,(usize,usize)){

    let side_length = 50;
    let n = grids[grid].len();
    let m = grids[grid][0].len();
    let dirs = vec![(0,1),(1,0),(0,-1),(-1,0)];
    let (dx,dy) = dirs[dir];

    if dir==0 || dir == 2 || ((cords.0!=0 || dx!=-1) && (cords.0!=grids[grid].len()-1 || dx!=1)){
        return (grid,next_wrap(cords,dx,dy,n,m));
    }
    /*hardcoded for my input */
    else if grid==0 && cords.1>=50{
        return (grid,next_wrap(cords,dx,dy,n,m));
    }
    else if dir==1{
        if grid==0 && cords.1<50{
            return (grid+1,(0,cords.1));
        }
        else if grid==1{
            return (grid+1,(0,cords.1+50));
        }
        else if grid==2 && cords.1<50{
            return (grid+1,(0,cords.1));
        }
        else if grid==2 && cords.1>=50{
            return (0,(0,cords.1-50));
        }
        else if grid==3 {
            return (2,(0,cords.1));
        }
    }
    else if dir==3{
        if grid==0 && cords.1<50{
            return (2,(49,cords.1+50));
        }
        else if grid==1{
            return (0,(49,cords.1));
        }
        else if grid==2 && cords.1<50{
            return (3,(49,cords.1));
        }
        else if grid==2 && cords.1>=50{
            return (1,(49,cords.1-50));
        }
        else if grid==3 {
            return (2,(49,cords.1));
        }
    }
    
    panic!();
}

fn next_wrap(cord:(usize,usize),dx:i64,dy:i64,n:usize,m:usize)->(usize,usize){
    (((cord.0 as i64 + dx + n as i64) as usize)%n,((cord.1 as i64 + dy + m as i64) as usize)%m)

} 

fn make_inst(s:&str)->Vec<Inst>{
    let mut ans:Vec<Inst> = Vec::new();
    let chars:Vec<char> = s.chars().collect();
    let mut n=0;
    for c in chars{
        if c =='R' || c=='L'{
            ans.push((c,n));
            n=0;
        }
        else{
            n = n*10+ c as usize-'0' as usize;
        }
    }
    ans.push(('.',n));
    ans
}

fn make_grids(s:&str)->Vec<Grid>{
    let mut ans:Vec<Grid>=Vec::new();
    let mut cur_grid:Grid=Vec::new();
    let mut prevl=usize::MAX;
    let mut prevw=usize::MAX;
    for line in s.split("\n"){
        let chars = line.chars();
        let l = chars.clone().count();
        let w = chars.clone().filter(|c| *c==' ').count();
        if prevl!=usize::MAX && (w!=prevw || l !=prevl){
            ans.push(cur_grid);
            cur_grid=Vec::new();
        }
        cur_grid.push(chars.filter(|c| *c!=' ').collect());
        prevl=l;
        prevw=w;
    }
    ans.push(cur_grid);
    ans
}

