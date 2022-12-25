fn main() {
    let vec : Vec<i64> = include_str!("in.txt").split('\n')
    .map(SNAFU_to_i64)
    .collect();
    
    let ans1 : i64 = vec.iter().sum();

    println!("The ans for part 1 is {}",i64_to_SNAFU(ans1));
    
    let ans2 : i64 = vec.iter().sum();

    println!("The ans for part 2 is {}",ans2);
}

fn SNAFU_to_i64(s:&str)->i64{
    let mut ans = 0;
    for c in s.chars(){
        ans*=5;
        ans += match c {
            '-' => -1,
            '=' => -2,
            _ => c as i64 - '0' as i64,
        }
    }
    ans
}

fn i64_to_SNAFU(n:i64)->String{
    let mut ans:String = String::new();
    let mut cur = n;
    loop {
        if cur==0{
            break;
        }
        if cur%5==0{
            ans.push('0');
            cur = cur/5;
        }
        else if cur%5==1{
            ans.push('1');
            cur = cur/5;
        }
        else if cur%5==2{
            ans.push('2');
            cur = cur/5;
        }
        else if cur%5==3{
            ans.push('=');
            cur = cur/5+1;
        }

        else if cur%5==4{
            ans.push('-');
            cur = cur/5+1;
        }
    }
    ans = ans.chars().rev().collect::<String>();

    ans
}
