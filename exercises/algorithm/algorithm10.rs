/*
	graph
	This problem requires you to implement a basic graph functio
*/
use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeNotInGraph;
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    
    fn add_node(&mut self, node: &str) -> bool {
        let node_str = node.to_string();
        let table = self.adjacency_table_mutable();
        if table.contains_key(&node_str) {
            false // 节点已存在
        } else {
            table.insert(node_str, Vec::new());
            true // 节点添加成功
        }
    }
    
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (node1, node2, weight) = edge;
        let node1_str = node1.to_string();
        let node2_str = node2.to_string();
        
        // 确保两个节点都存在
        self.add_node(node1);
        self.add_node(node2);
        
        // 获取邻接表的可变引用
        let table = self.adjacency_table_mutable();
        
        // 添加node1->node2的边
        if let Some(neighbors) = table.get_mut(&node1_str) {
            neighbors.push((node2_str.clone(), weight));
        }
        
        // 添加node2->node1的边（无向图）
        if let Some(neighbors) = table.get_mut(&node2_str) {
            neighbors.push((node1_str, weight));
        }
    }
    
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().contains_key(node)
    }
    
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
    
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, neighbors) in self.adjacency_table() {
            for (to_node, weight) in neighbors {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}

pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
    
    // 重写add_edge确保无向图特性
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (node1, node2, weight) = edge;
        let node1_str = node1.to_string();
        let node2_str = node2.to_string();
        
        // 确保两个节点都存在
        self.add_node(node1);
        self.add_node(node2);
        
        // 添加双向边（无向图特性）
        self.adjacency_table_mutable()
            .entry(node1_str.clone())
            .or_insert_with(Vec::new)
            .push((node2_str.clone(), weight));
        
        self.adjacency_table_mutable()
            .entry(node2_str)
            .or_insert_with(Vec::new)
            .push((node1_str, weight));
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;

    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        
        for edge in expected_edges.iter() {
            assert!(graph.edges().contains(edge));
        }
    }
}