fn main() {
    let vec : Vec<&str> = include_str!("in.txt").split('\n')
    .collect();

    
    let mut ans1:i32 = 0;
    let mut X:i32 = 1;
    let mut cyc:i32 = 0; //for part 1 its 1;
    let mut ans2:Vec<char> = vec!['.';240];
    for s in &vec{
        let mut it =s.split(" ");
        if it.next().unwrap()=="noop"{
            (cyc,ans1) = update_cycle(X, cyc, ans1, &mut ans2);
        }
        else{
            (cyc,ans1) = update_cycle(X, cyc, ans1, &mut ans2);
            (cyc,ans1) = update_cycle(X, cyc, ans1, &mut ans2);
            let add = str::parse::<i32>(it.next().unwrap()).unwrap();
            X+=add;
        }
    }
    println!("The ans for part 1 is {}",ans1);
    println!("The ans for part 2 is",);
    for i in 0..6{
        for j in 0..40{
            print!("{}",ans2[i*40+j])
        }
        println!();
    }
    
}

fn update_cycle(X:i32,cyc:i32,ans1:i32,ans2:&mut Vec<char>) -> (i32,i32){
    if (cyc%40-X).abs()<=1{
        ans2[cyc as usize]='#';
    }
    let cyc=cyc+1;
    if cyc%40==20{
        return (cyc,ans1+cyc*X);
    }
    return (cyc,ans1);
}
