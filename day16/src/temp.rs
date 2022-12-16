use std::collections::HashMap;

type Node = (char,char);

fn main() {
    let vec = include_str!("test.txt").split('\n');
    let n = vec.clone().count();
    let rounds = 30;

    let mut flows: HashMap<Node,i64> = HashMap::new();
    
    let mut graph: HashMap<Node,Vec<Node>> = HashMap::new();

    let mut nodes_with_flow:Vec<Node> = Vec::new();

    let mut all_nodes:Vec<Node> = Vec::new(); 

    for s in vec{
        let s_prime = s.replace("Valve ","")
        .replace("has flow rate=","")
        .replace(",","")
        .replace("; tunnels lead to valves","")
        .replace("; tunnel leads to valve","");
        let mut it = s_prime.split(" ");
        let curnode = str_to_node(it.next().unwrap());
        let flow:i64 = str::parse::<i64>(it.next().unwrap()).unwrap();
        let next_nodes:Vec<Node> = it.map(str_to_node).collect();
        flows.insert(curnode,flow);
        graph.insert(curnode,next_nodes);
        all_nodes.push(curnode);
        if flow != 0{
            nodes_with_flow.push(curnode);
        }
    }
    let m = nodes_with_flow.len();

    let mut node_to_ind:HashMap<Node,usize> = HashMap::new();

    for i in 0..m{
        node_to_ind.insert(nodes_with_flow[i],i);
    }

    let mut dp:HashMap<Node,Vec<i64>> = HashMap::new();

    for node in all_nodes.iter(){
        let two:usize=2;
        dp.insert(*node,vec![0;two.pow(m as u32)]);
    }

    let mut dp_prev: HashMap<Node,Vec<i64>> = dp.clone();

    let mut dp_prev_prev: HashMap<Node,Vec<i64>> = dp.clone();

    for i in 1..rounds{
        dp_prev_prev=dp_prev;
        dp_prev = dp;
        dp = HashMap::new();
        for cur in all_nodes.iter(){
            let two:usize=2;
            let mut cur_vals=vec![0;two.pow(m as u32)];

            //visited_nodes node with index i has open vent if bit i of visited_nodes is set
            for visited_nodes in 0..two_to_the_power(m){
                let mut ans:i64 = 0;
                for next in graph.get(cur).unwrap().iter(){
                    ans = std::cmp::max(dp_prev.get(next).unwrap()[visited_nodes],ans);
                }
                match node_to_ind.get(cur){
                    None=> (),
                    Some(ind) =>{
                        if visited_nodes&(1<<ind)==0{
                            let uunvis = visited_nodes|(1<<ind);
                            for next in graph.get(cur).unwrap().iter(){
                                ans = std::cmp::max(dp_prev_prev.get(&next).unwrap()[uunvis]+i*flows[cur],ans);
                            }
                        }
                    }
                }
                cur_vals[visited_nodes]=ans;
            }
            dp.insert(*cur,cur_vals);
        }
    }
    


    println!("ans for part 1 {}",dp.get(&('A','A')).unwrap()[0]);

}


fn str_to_node(s:&str)->Node{
    let mut it = s.chars();
    (it.next().unwrap(),it.next().unwrap())
}

