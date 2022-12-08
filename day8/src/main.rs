fn main() {
    let vec : Vec<Vec<i64>> = include_str!("in.txt").split('\n')
    .map(|x| {
        x.chars().map(|c| { c as i64 - '0' as i64})
        .collect()
    }).collect();

    let n:usize = vec.len();

    let mut isvis: Vec<Vec<i64>> = vec![ vec![ 0; n]; n];
    
    for i in 0..n{
        let mut max:Vec<i64> =  vec![ -1; 4];
        for j in 0..n{
            if vec[i][j]>max[0]{
                isvis[i][j]=1;
                max[0] = vec[i][j];
            }
            if vec[i][n-j-1]>max[1]{
                isvis[i][n-j-1]=1;
                max[1] = vec[i][n-j-1];
            }
            if vec[j][i]>max[2]{
                isvis[j][i]=1;
                max[2] = vec[j][i];
            }
            if vec[n-j-1][i]>max[3]{
                isvis[n-j-1][i]=1;
                max[3] = vec[n-j-1][i];
            }
            
        }
    }

    let ans1 : i64 = isvis.iter().map(|x| x.iter().sum::<i64>()).sum();
    println!("The ans for part 1 is {}",ans1);
    
    let mut ans2 : i64 = 0;
    for i in 0..n{
        for j in 0..n{
            let mut dirs: Vec<i64> = vec![0;4];
            let curh = vec[i][j];
            for len in 1..n{
                if i+len<n && dirs[0]==0{
                    if vec[i+len][j]>=curh || i+len+1==n{
                        dirs[0]=len as i64;
                    }
                }
                if j+len<n && dirs[1]==0{
                    if vec[i][j+len]>=curh || j+len+1==n{
                        dirs[1]=len as i64;
                    }
                }
                if i>=len && dirs[2]==0{
                    if vec[i-len][j]>=curh || i-len==0{
                        dirs[2]=len as i64;
                    }
                }
                if j>=len && dirs[3]==0{
                    if vec[i][j-len]>=curh || j-len==0{
                        dirs[3]=len as i64;
                    }
                }
            }
            ans2 = std::cmp::max(ans2,dirs[0]*dirs[1]*dirs[2]*dirs[3]);
        }
    }
    
    println!("The ans for part 2 is {}",ans2);
}

