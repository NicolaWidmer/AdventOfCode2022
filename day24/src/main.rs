fn main() {
    let vec : Vec<Vec<char>> = include_str!("in.txt").split('\n')
    .map(|s| s.chars().collect())
    .collect();

    let n = vec.len()-2;
    let m = vec[0].len()-2;


    let mut up:Vec<Vec<bool>> = vec![vec![false;m];n]; 
    let mut down:Vec<Vec<bool>>= vec![vec![false;m];n]; 
    let mut right:Vec<Vec<bool>> = vec![vec![false;m];n]; 
    let mut left:Vec<Vec<bool>> = vec![vec![false;m];n]; 

    
    for i in 0..n{
        for j in 0..m{
            if vec[i+1][j+1]=='^'{
                up[i][j]=true;
            }
            if vec[i+1][j+1]=='v'{
                down[i][j]=true;
            }
            if vec[i+1][j+1]=='>'{
                right[i][j]=true;
            }
            if vec[i+1][j+1]=='<'{
                left[i][j]=true;
            }
        }
    }

    let mut ans = cross(0,0,n-1,m-1,&mut up,&mut down,&mut right,&mut left);
    println!("The ans to part 1 is {}",ans);
    
    ans += cross(n-1,m-1,0,0,&mut up,&mut down,&mut right,&mut left)-1;
    ans += cross(0,0,n-1,m-1,&mut up,&mut down,&mut right,&mut left)-1;
    println!("The ans to part 2 is {}",ans);
    

}

fn cross(startx:usize,starty:usize,endx:usize,endy:usize,up:&mut Vec<Vec<bool>>,down:&mut Vec<Vec<bool>>,right:&mut Vec<Vec<bool>>,left:&mut Vec<Vec<bool>>)->usize{
    let mut day = 0;
    let n = up.len();
    let m = up[0].len();
    let mut vis:Vec<Vec<bool>> = vec![vec![false;m];n]; 
    loop{
        
        day+=1;
        
        if vis[endx][endy]{
            return day;
        }

        let mut next_vis:Vec<Vec<bool>> = vec![vec![false;m];n]; 
        let mut next_up:Vec<Vec<bool>> = vec![vec![false;m];n]; 
        let mut next_down:Vec<Vec<bool>> = vec![vec![false;m];n]; 
        let mut next_right:Vec<Vec<bool>> = vec![vec![false;m];n]; 
        let mut next_left:Vec<Vec<bool>> = vec![vec![false;m];n];

        for i in 0..n{
            for j in 0..m{
                if up[i][j]{
                    next_up[(i+n-1)%n][j]=true;
                }
                if down[i][j]{
                    next_down[(i+1)%n][j]=true;
                }
                if right[i][j]{
                    next_right[i][(j+1)%m]=true;
                }
                if left[i][j]{
                    next_left[i][(j+m-1)%m]=true;
                }
            }
        }
        *up = next_up;
        *down = next_down;
        *right = next_right;
        *left = next_left;


        if no_bliz(startx,starty,up,down,right,left){
            next_vis[startx][starty]=true;
        }

        for i in 0..n{
            for j in 0..m{
                if no_bliz(i,j,up,down,right,left)&&neighb_is_vis(i,j,n,m,&vis){
                    next_vis[i][j]=true;
                }
            }
        }
        vis = next_vis;

    }
}


fn no_bliz(i:usize,j:usize,v1:&Vec<Vec<bool>>,v2:&Vec<Vec<bool>>,v3:&Vec<Vec<bool>>,v4:&Vec<Vec<bool>>)->bool{
    !v1[i][j]&&!v2[i][j]&&!v3[i][j]&&!v4[i][j]
}

fn neighb_is_vis(i:usize,j:usize,n:usize,m:usize,vis:&Vec<Vec<bool>>)->bool{
    vis[i][j]||(i>0&&vis[i-1][j])||(j>0&&vis[i][j-1])|| (i+1<n && vis[i+1][j]) || (j+1<m && vis[i][j+1])
}

