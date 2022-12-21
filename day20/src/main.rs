

fn main() {
    let vec : Vec<(usize,i64)> = include_str!("in.txt").split('\n')
    .map(str::parse::<i64>)
    .map(Result::unwrap)
    .enumerate()
    .collect();

    println!("The ans for part 1 is {}",solve(1,1,&vec));

    let description_key = 811589153;
    let rounds = 10;

    println!("The ans for part 2 is {}",solve(rounds,description_key,&vec));
   
}

fn solve(rounds:usize,description_key:i64,input:&Vec<(usize,i64)>) -> i64{

    let mut vec: Vec<(usize,i64)> = input.iter()
    .map(|(i,v)| (*i,v*description_key))
    .collect();

    let n = vec.len();

    for _ in 0..rounds{
        for j in 0..vec.len(){

            let (i,val) = vec[j];
            let n_minus_one:i64 = n as i64 -1;
            let inext = (((i as i64 + val)%(n_minus_one)+n_minus_one)%(n_minus_one)) as usize;

            for k in 0..vec.len(){
                if k==j{
                    continue;
                }

                let (i2,v) = vec[k];
                let mut nexti2 = i2;

                if i<i2&&inext>=i2 {
                    nexti2-=1;
                }
                else if i>=i2&&inext<=i2{
                    nexti2+=1;
                }

                vec[k]=(nexti2,v);
            }
            vec[j]=(inext,val);
        }
    }

    let mut zero_ind=0;
    for i in 0..vec.len(){
        if vec[i].1==0{
            zero_ind=vec[i].0;
        }
    }


    let ind_1000 = (zero_ind+1000)%n;
    let ind_2000 = (zero_ind+2000)%n;
    let ind_3000 = (zero_ind+3000)%n;
    let mut ans = 0;

    for e in vec{
        if e.0 == ind_1000 || e.0 == ind_2000 || e.0 == ind_3000{
            ans+= e.1;
        }
       
    }

    ans
}

