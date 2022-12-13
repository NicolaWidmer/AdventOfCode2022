extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::iterators::Pair;
use pest::Parser;

use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;

use core::cmp::Ordering;

#[derive(Parser)]
#[grammar = "grammer.pest"]
pub struct ElemParser;

#[derive(Debug,Clone,Eq,PartialEq)]
enum Elem{
    Int(i64),
    List(Vec<Elem>),
}

impl Ord for Elem{
    fn cmp(&self, other: &Self) -> Ordering {
        match is_ordered(self,other){
            None => Ordering::Equal,
            Some(true) => Ordering::Less,
            Some(false) => Ordering::Greater,
        }
    }
}

impl PartialOrd for Elem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_elem(pair:Pair<Rule>)->Elem{
    match pair.as_rule(){
        Rule::int => Elem::Int(pair.as_str().parse::<i64>().unwrap()),
        Rule::list => Elem::List(pair.into_inner().map(parse_elem).collect()),
        _=>panic!("invalid rule")
    }

}
fn pares_line(input:&str)->Elem{
    parse_elem(ElemParser::parse(Rule::elem, input).unwrap().next().unwrap())
}

fn is_ordered(el1:&Elem,el2:&Elem)->Option<bool>{
    match (el1,el2){
        (Elem::Int(i1),Elem::Int(i2)) => 
            if i1<i2 {
                Some(true)} 
            else if i1>i2 {
                Some(false)} 
            else {None},

        (Elem::List(l1),Elem::List(l2)) => {
            l1.iter().zip_longest(l2.iter()).fold(
                None,|acc,val| {
                    if acc==None{
                        match val{
                            Both(e1, e2) => is_ordered(e1,e2),
                            Left(_) => Some(false),
                            Right(_) => Some(true),
                        }
                    }
                    else{
                        acc
                    }
                }
            )
        },

        (Elem::Int(i1),Elem::List(l2)) => is_ordered(&Elem::List(vec![Elem::Int(*i1)]),&Elem::List(l2.to_vec())),

        (Elem::List(l1),Elem::Int(i2)) => is_ordered(&Elem::List(l1.to_vec()),&Elem::List(vec![Elem::Int(*i2)])),
    }
}

fn main() {
    
    let vec : Vec<(Elem,Elem)> = include_str!("in.txt").split("\n\n")
    .map(|x| { 
        let mut it = x.split('\n');
        (pares_line(it.next().unwrap()),pares_line(it.next().unwrap()))
    })
    .collect();

    let mut ans1:usize = 0;

    let mut packets:Vec<Elem> = Vec::new();

    for (i,(e1,e2)) in vec.iter().enumerate(){
        packets.push(e1.clone());
        packets.push(e2.clone());
        if is_ordered(e1,e2)==Some(true){
            ans1+=i+1;
        }
    }

    let p1 = pares_line(&"[[2]]");
    let p2 = pares_line(&"[[6]]");
    packets.push(p1.clone());
    packets.push(p2.clone());

    packets.sort();

    let mut ans2 = 1;
    for (i,e) in packets.iter().enumerate(){
        if *e == p1 || *e == p2{
            ans2 *= i+1;
        }
    }

    println!("The ans for part 1 is {}",ans1);
    println!("The ans for part 2 is {}",ans2);
}

