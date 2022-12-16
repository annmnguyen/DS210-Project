mod graph;

use graph::dfs::mark_component_dfs;
use graph::dfs::dfs_collect_stack;
use graph::dfs::topological_sort;
use graph::reverse_edges;
use graph::Graph;
use graph::Vertex;
use graph::dfs::Component;
use std::fs::File;
use std::io::prelude::*;

fn read_file(path: &str) -> Vec<(i128, i128)> {
    let mut result: Vec<(i128, i128)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader.skip(1) {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split_whitespace().collect();
        let x = v[0].parse::<i128>().unwrap();
        let y = v[1].parse::<i128>().unwrap();
        result.push((x, y));
    }
    return result;
}
#[allow(dead_code,non_snake_case)]

fn main() {
    let _edges = read_file("SS-Butterfly_labels.tsv");
    let _n: i128 = 832;
    let graph = Graph::create_undirected(_n, &_edges);
    let mut component = vec![None;graph.n as usize];
    let mut component_count = 0;
    
    for v in 0..graph.n {
       if let None = component[v as usize] {
            component_count += 1;
            mark_component_dfs(v,&graph,&mut component,component_count);
        }
    };
   print!("{} components:\n[  ",component_count);
    for v in 0.._n {
        print!("{}:{}  ",v,component[v as usize].unwrap());
    }
    println!("]\n"); 

    let graph_reverse = Graph::create_directed(_n,&reverse_edges(&_edges));
    let mut stack: Vec<Vertex> = Vec::new();
    let mut visited = vec![false;graph.n as usize];
    for v in 0..graph.n {
        dfs_collect_stack(v,&graph,&mut stack,&mut visited);
    };
    let mut component: Vec<Option<Component>> = vec![None;graph.n as usize];
    let mut component_count = 0;

    while let Some(v) = stack.pop() {
        if let None = component[v as usize] {
            component_count += 1;
            mark_component_dfs(v, &graph_reverse, &mut component, component_count);
        }
    };
    println!("{:?}", topological_sort(&graph));
    print!("{} components:\n",component_count);
    for c in 1..=component_count {
        print!("Component {}: ", c);
        for v in 0.._n {
        if component[v as usize].unwrap() == c {
            print!("{} ",v);
        }
    }
    println!();
}
println!();
}

mod test {
    use super::*;
    #[test]
    fn testdfs() {
        let mut graph = vec![(0,1),(0,2),(1,2),(2,4),(2,3),(4,3),(4,5),(5,6),(4,6),(6,8),(6,7),(8,7),(1,9)];
        let graphs = Graph::create_undirected(10, &graph);
        let graph_reverse = Graph::create_directed(10,&reverse_edges(&graph));
        let mut component: Vec<Option<Component>> = vec![None;graphs.n as usize];
        let mut component_count = 0;
        for v in 0..graphs.n {
            if let None = component[v as usize] {
            component_count += 1;
            assert_eq!(mark_component_dfs(10, &graph_reverse, &mut component, component_count));
            }
        };
    }
}



//use std::fs::File;
//use std::io::prelude::*;

//fn read_weights(path: &str) -> Vec<(usize, usize, f64)> {
//   let mut result: Vec<(usize, usize, f64)> = Vec::new();
//   let file = File::open(path).expect("Could not open file");
//   let buf_reader = std::io::BufReader::new(file).lines();
//   for line in buf_reader.skip(1) {
//   let line_str = line.expect("Error reading");
//        let v: Vec<&str> = line_str.trim().split_whitespace().collect();
//        let x = v[0].parse::<usize>().unwrap();
//        let y = v[1].parse::<usize>().unwrap();
//        let z = v[2].parse::<f64>().unwrap();
//        result.push((x, y, z));
//    }
//    return result;
//}
//#[allow(dead_code,non_snake_case)]
//type Vertex = usize;
//type Distance = f64;
//type Edge = (Vertex, Vertex, Distance);
//#[derive(Debug,Copy,Clone)]
//struct Outedge {
//    vertex: Vertex,
//    length: Distance,
//}

//type AdjacencyList = Vec<Outedge>;

////#[derive(Debug)]
//struct Graph {
//   n: usize,
//    outedges: Vec<AdjacencyList>,
//}

//impl Graph {
//    fn create_directed(n:usize,edges:&Vec<Edge>) -> Graph {
//        let mut outedges = vec![vec![];n as usize];
//        for (u, v, length) in edges {
//            outedges[*u as usize].push(Outedge{vertex: *v, length: *length});
//        }
//        Graph{n,outedges}
//    }
//}
//use std::collections::BinaryHeap;

//fn main() {
//    let edges = read_weights("SS-Butterfly_weights.tsv");
//    let n: usize = 832;
//    let graph = Graph::create_directed(n, &edges);
//    println!("{:?}", graph);
//    let start: Vertex = 0;

//    let mut distances: Vec<Option<Distance> > = vec![None; graph.n as usize];
//    distances[start as usize] = Some(0.00);
//    use core::cmp::Reverse;
//    let mut pq = BinaryHeap::new();
//    pq.push(Reverse::<{Distance: usize},Vertex>)>
//    pq.push(Reverse((0,start)));
//    while let Some(Reverse((dist,v))) = pq.pop() {
//        for Outedge{vertex,length} in graph.outedges[v as usize].iter() {
//            let new_dist = dist + length;
//            let update = match distances[*vertex as usize] {
//                None => {true} |
//                Some(d) => {new_dist < d}
//            };
//            if update {
//                distances[*vertex as usize] = Some(new_dist);
//                pq.push(Reverse((new_dist,*vertex)));
//            }
//        }
//    };
//   println!("{:?}", distances)

//}
