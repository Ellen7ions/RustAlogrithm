use std::collections::{HashMap, HashSet};

struct DirectGraph {
    adj_table: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for DirectGraph {
    fn new() -> DirectGraph {
        DirectGraph {
            adj_table: HashMap::new(),
        }
    }

    fn adj_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adj_table
    }
    fn adj_table_mut(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adj_table
    }
}

trait Graph {
    fn new() -> Self;
    fn adj_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    fn adj_table_mut(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn add_node(&mut self, node: &str) {
        self.adj_table_mut()
            .entry(String::from(node))
            .or_insert(Vec::new());
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        self.add_node(edge.0);
        self.add_node(edge.1);
        self.adj_table_mut()
            .entry(String::from(edge.0))
            .and_modify(|v| v.push((String::from(edge.1), edge.2)));
    }
    fn get_nodes(&self) -> HashSet<&String> {
        self.adj_table().keys().collect()
    }
    fn get_edges(&self) -> Vec<(&String, &String, i32)> {
        let mut res = Vec::new();
        for (from, value) in self.adj_table() {
            for (to, w) in value {
                res.push((from, to, *w));
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_graph() {
        let mut graph = DirectGraph::new();
        graph.add_node("A");
        graph.add_node("B");
        graph.add_node("C");
        graph.add_edge(("A", "B", 1));
        graph.add_edge(("B", "C", 5));
        println!("{:?}", graph.get_nodes());
        println!("{:?}", graph.get_edges());
    }
}
