type Monkey<'a> =(Vec<i64>,(Option<i64>,char,Option<i64>),(i64,usize,usize));

fn main() {
    let mut monkeys : Vec<Monkey> = include_str!("in.txt").split("\n\n")
    .map(parse_monkey)
    .collect();

    let stressdividor = 1; /* change to 3 for part 1 */
    let turns = 10000; /* change to 20 for part 1 */

    let modulo = monkeys.iter().fold(1,|acc, m| acc*m.2.0 );

    let size = monkeys.len(); 

    let mut counts:Vec<i64> = vec![0;monkeys.len()];

    for _ in 0..turns{
        for i in 0..size{
            let mut v1:Vec<i64> = Vec::new();
            let mut v2:Vec<i64> = Vec::new();
            let mo1:usize;
            let mo2:usize;
            {
                let mut m : &mut Monkey = &mut monkeys[i];
                let (div,m1,m2)=m.2;
                mo1=m1;
                mo2=m2;
                let (op1,op,op2) = m.1;
                for n in &m.0{
                    let new = operation(op1,op,op2,*n)/stressdividor;
                    if new%div==0{
                       v1.push(new%modulo);
                    }
                    else{
                        v2.push(new%modulo);
                    }
                    counts[i]+=1;
                }
                m.0=Vec::new();
            }
            monkeys[mo1].0.append(&mut v1);
            monkeys[mo2].0.append(&mut v2);
        }
    }
    counts.sort();
    

    println!("The ans for part 2 is {}",counts[size-2]*counts[size-1]);
}

fn operation(op1:Option<i64>,op:char,op2:Option<i64>,n:i64)->i64{
    let o1 = op1.unwrap_or(n);
    let o2 = op2.unwrap_or(n);
    match op{
        '*' => o1*o2,
        '+' => o1+o2,
        _ => panic!("{} unexpected operand",op),
    }
}


fn parse_monkey<'a>(s:&'a str)->Monkey<'a>{
    let mut it = s.split('\n');
    it.next();
    let items : Vec<i64> = it.next().unwrap()
        .replace("  Starting items: ","").split(", ")
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .collect();
    
    let binding = it.next().unwrap()
    .replace("  Operation: new = ","");
    let mut ops_it = binding.split(" ");

    let op1 = str::parse::<i64>(ops_it.next().unwrap()).ok();
    let operator = ops_it.next().unwrap().chars().next().unwrap();
    let op2 = str::parse::<i64>(ops_it.next().unwrap()).ok();

    let div = str::parse::<i64>(&it.next()
    .unwrap().replace("  Test: divisible by ", "")[..])
    .unwrap();
    let m1 = str::parse::<usize>(&it.next()
    .unwrap().replace("    If true: throw to monkey ", "")[..])
    .unwrap();
    let m2 = str::parse::<usize>(&it.next()
    .unwrap().replace("    If false: throw to monkey ", "")[..])
    .unwrap();

    (items,(op1,operator,op2),(div,m1,m2))
}

