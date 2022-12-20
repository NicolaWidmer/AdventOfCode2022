use scanf::sscanf;

use std::collections::HashMap;

type State = (i64,i64,i64,i64,i64,i64,i64);

type Blueprint = (i64,i64,(i64,i64),(i64,i64));

fn main() {
    let vec : Vec<Blueprint> = include_str!("in.txt").split('\n')
    .map(|s|{
        let mut n:i64=0;
        let mut ore_ore:i64=0;
        let mut ore_clay:i64=0;
        let mut ore_obs:i64=0;
        let mut clay_obs:i64=0;
        let mut ore_geod:i64=0;
        let mut obs_geod:i64=0;
        if sscanf!(s,
            "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.",
            n,
            ore_ore,
            ore_clay,
            ore_obs,
            clay_obs,
            ore_geod,
            obs_geod,
        ).is_ok(){
            (ore_ore,ore_clay,(ore_obs,clay_obs),(ore_geod,obs_geod))
        }
        else{
            panic!("scanf unsuccessful");
        }
    })
    .collect();
    
    let mins1 = 24;
    let mins2 = 32;

    let mut ans1 = 0;
    let mut ans2 = 1;

    for (i,b) in vec.iter().enumerate(){
        let mut dp:HashMap<State,i64> = HashMap::new();
        ans1+=((i+1) as i64)*rec(mins1,0,0,0,1,0,0,*b,&mut dp);
        /* works but takes ca. 10 min to run*/
        if i<3{
            ans2*=rec(mins2,0,0,0,1,0,0,*b,&mut dp);
        }
    }
    println!("The ans for part 1 is {}",ans1);
    println!("The ans for part 2 is {}",ans2);


}

fn rec(days:i64,ore:i64,clay:i64,obs:i64,orebot:i64,claybot:i64,obsbot:i64,blueprint:Blueprint,dp:&mut HashMap<State,i64>)->i64{
    if days==0{
        return 0;
    }
    let state =  (days,ore,clay,obs,orebot,claybot,obsbot);
    match dp.get(&state){
        None=>(),
        Some(ans)=>return *ans,
    }

    let mut ans=rec(days-1,ore+orebot,clay+claybot,obs+obsbot,orebot,claybot,obsbot,blueprint,dp);

    if orebot<max(vec![blueprint.0,blueprint.1,blueprint.2.0,blueprint.3.0]) && ore>=blueprint.0{
        ans = std::cmp::max(ans,rec(days-1,ore-blueprint.0+orebot,clay+claybot,obs+obsbot,orebot+1,claybot,obsbot,blueprint,dp));
    }
    if claybot<blueprint.2.1&&ore>=blueprint.1{
        ans = std::cmp::max(ans,rec(days-1,ore+orebot-blueprint.1,clay+claybot,obs+obsbot,orebot,claybot+1,obsbot,blueprint,dp));
    }
    if obsbot<blueprint.3.1&&ore>=blueprint.2.0&&clay>=blueprint.2.1{
        ans = std::cmp::max(ans,rec(days-1,ore+orebot-blueprint.2.0,clay+claybot-blueprint.2.1,obs+obsbot,orebot,claybot,obsbot+1,blueprint,dp));
    }
    if ore>=blueprint.3.0&&obs>=blueprint.3.1{
        ans = std::cmp::max(ans,rec(days-1,ore+orebot-blueprint.3.0,clay+claybot,obs+obsbot-blueprint.3.1,orebot,claybot,obsbot,blueprint,dp)+days-1);
    }

    dp.insert(state,ans);
    return ans;
}

fn max(vec:Vec<i64>)->i64{
    *vec.iter().max().unwrap()
}

