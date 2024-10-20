/*
    图
    graph
    This problem requires you to implement a basic graph function
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
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        // 添加节点
        self.add_node(edge.0);
        self.add_node(edge.1);

        self.adjacency_table_mutable()
            .get_mut(edge.0) // 获取该节点的可变引用
            .unwrap()
            .push((edge.1.to_string(), edge.2)); // 添加边
                                                 // 无向图，添加两条边
        self.adjacency_table_mutable()
            .get_mut(edge.1)
            .unwrap()
            .push((edge.0.to_string(), edge.2));
    }
}
pub trait Graph {
    fn new() -> Self;
    // 返回可变邻接表
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    // 返回不可变邻接表
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    // 添加一个节点
    fn add_node(&mut self, node: &str) -> bool {
        // // 看一下该节点是否已经存在
        // if self.adjacency_table_mutable().contains_key(node) {
        //     false
        // } else {
        //     self.adjacency_table_mutable()
        //         .insert(node.to_string(), Vec::new());
        //     true
        // }
        // entry 写法
        self.adjacency_table_mutable()
            .entry(node.to_owned()) // 如果节点不存在，则插入 Vec::new
            .or_insert_with(Vec::new) // 返回了该节点的 Vector
            .is_empty() // 如果节点是刚刚插入的，则返回的 Vector 是空的，
                        // 返回 true，如果节点之前就有，返回的 Vector 不是空的，返回 false
                        // 但是本题没有用到这个返回值。
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        self.adjacency_table_mutable()
            .get_mut(edge.0) // 获取该节点的可变引用
            .unwrap()
            .push((edge.1.to_string(), edge.2)); // 添加边
    }
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}

fn main() {}

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
            assert_eq!(graph.edges().contains(edge), true);
        }
    }
}
