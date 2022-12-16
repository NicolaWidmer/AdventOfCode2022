use std::collections::HashMap;
use std::collections::VecDeque;

type Node = (char,char);

fn main() {
    let vec = include_str!("in.txt").split('\n');
    let rounds = 30;

    let mut flows: Vec<usize> = Vec::new();
    
    let mut graph: HashMap<Node,Vec<Node>> = HashMap::new();

    let mut nodes_with_flow:Vec<Node> = Vec::new();

    for s in vec{
        let s_prime = s.replace("Valve ","")
        .replace("has flow rate=","")
        .replace(",","")
        .replace("; tunnels lead to valves","")
        .replace("; tunnel leads to valve","");

        let mut it = s_prime.split(" ");

        let curnode = str_to_node(it.next().unwrap());
        let flow:usize = str::parse::<usize>(it.next().unwrap()).unwrap();
        let next_nodes:Vec<Node> = it.map(str_to_node).collect();
        graph.insert(curnode,next_nodes);

        if flow != 0 || curnode==('A','A'){
            flows.push(flow);
            nodes_with_flow.push(curnode);
        }
    }

    let m = nodes_with_flow.len();

    let mut compresed_graph: Vec<Vec<(usize,usize)>> = vec![Vec::new();m];

    let mut start:usize=0;

    for i in 0..m{
        if nodes_with_flow[i]==('A','A'){
            start=i;
        }
        for j in 0..m{
            if j==i{
                continue;
            }
            compresed_graph[i].push((j,shortest_path(nodes_with_flow[i],nodes_with_flow[j],&graph)));
        }
    }
    println!("{start}");

    let mut dp:Vec<Vec<Vec<usize>>> = vec![vec![vec![usize::MAX;1<<m];m];rounds+1];

    println!("ans for part 1 {}",rec(30,start,0,&mut dp,&compresed_graph,&flows));

    let mut ans2:usize=0;

    for vis in 0..(1<<(m-1)){
        let notvis = !vis & ((1<<m)-1);
        ans2 = std::cmp::max(ans2,rec(26,start,vis,&mut dp,&compresed_graph,&flows)+rec(26,start,notvis,&mut dp,&compresed_graph,&flows));
    }

    println!("ans for part 1 {}",ans2);

}

fn rec(rounds:usize,pos:usize,vis:usize,dp:& mut Vec<Vec<Vec<usize>>>, graph: &Vec<Vec<(usize,usize)>>,flows:&Vec<usize>)->usize{
    if rounds<=0{
        return 0;
    }
    if dp[rounds][pos][vis]!=usize::MAX{
        return dp[rounds][pos][vis];
    }
    let mut ans:usize=0;
    if vis&(1<<pos)==0{
        ans = rec(rounds-1,pos,vis|(1<<pos),dp,graph,flows)+(rounds-1)*flows[pos];
    }
    for (next,dis) in &graph[pos]{
        if rounds>(*dis){
            ans = std::cmp::max(ans,rec(rounds-(*dis),*next,vis,dp,graph,flows))
        }
    }
    dp[rounds][pos][vis]=ans;
    return ans;
}


fn str_to_node(s:&str)->Node{
    let mut it = s.chars();
    (it.next().unwrap(),it.next().unwrap())
}

fn shortest_path(s:Node,t:Node,graph:&HashMap<Node,Vec<Node>>)->usize{
    let mut deq:VecDeque<(Node,usize)> = VecDeque::new();
    deq.push_front((s,0));
    loop {
        let (cur,dis) = deq.pop_back().unwrap();
        if cur==t{
            return dis;
        }
        for next in graph.get(&cur).unwrap(){
            deq.push_front((*next,dis+1));
        }
    }

}

