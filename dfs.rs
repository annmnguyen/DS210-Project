use std::collections::VecDeque;

use super::{Graph, Vertex};

pub type Component = i128;

pub fn mark_component_dfs(vertex:Vertex, graph:&Graph, component:&mut Vec<Option<Component>>, component_no:Component) {
    component[vertex as usize] = Some(component_no);
    for w in graph.outedges[vertex as usize].iter() {
        if let None = component[*w as usize] {
            mark_component_dfs(*w,graph,component,component_no);
        }        
    }
}
pub fn dfs_collect_stack(v:Vertex, graph:&Graph, stack:&mut Vec<Vertex>, visited:&mut Vec<bool>) {
    if !visited[v as usize] {
        visited[v as usize] = true;
        for w in graph.outedges[v as usize].iter() {
            dfs_collect_stack(*w, graph, stack, visited);
        }
        stack.push(v);
    }
}

pub fn topological_sort(graph: &Graph) {
    let mut new = VecDeque::new();
    let mut component_count = 0;
    let mut component: Vec<Option<Component>> = vec![None;graph.n as usize];
    for v in 0..graph.n {
        if let None = component[v as usize] {
             component_count += 1;
             new.push_front(mark_component_dfs(v,&graph,&mut component,component_count));
         }
     };
    

}