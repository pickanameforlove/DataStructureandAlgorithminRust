use std::cmp::Ordering;

#[derive(Eq)]
pub struct Edge{
    start:u32,
    end:u32,
    weight:u32
}
impl Edge{
    pub fn new(s:u32,e:u32,w:u32)->Self{
        Self{
            start:s,
            end:e,
            weight:w
        }
    }
}
impl PartialOrd for Edge{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Edge{
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}
impl Ord for Edge{
    fn cmp(&self, other: &Self) -> Ordering{
        self.weight.cmp(&other.weight)
    }
}

pub fn kruskal(l:&mut Vec<Edge>,n : u32){
    l.sort_by(|a,b| {a.weight.cmp(&b.weight)});
    let mut index = 0;
    let mut father:Vec<u32> = vec![0;(n+1) as usize];
    let mut totalWeight = 0;
    while index < n-1{
        let edge = l.remove(0);
        let f1 = get_father(&father, edge.start);
        let f2 = get_father(&father, edge.end);
        if f1 != f2{
            father[f1 as usize] = f2;
            index += 1;
            totalWeight += edge.weight;
            println!("{},{},{}",edge.start,edge.end,edge.weight);
        }
        
    }
    println!("min weight is {}",totalWeight);
}
fn get_father(father : &[u32],n:u32) -> u32{
    let mut node = n;
    while father[node as usize] != 0 {
        node = father[node as usize];
    }
    return node;
}

// pub fn dijkstra(l:&Vec<Edge>, n :usize){
//     let mut pathlength = vec![999;n+1];
//     //原点是1
//     let mut mid = 1;
//     pathlength[mid] = 0;
//     let mut index = 0;
//     let number = l.len();
//     while index < number{
//         let edge = l.get(index);
        
//     }
// }