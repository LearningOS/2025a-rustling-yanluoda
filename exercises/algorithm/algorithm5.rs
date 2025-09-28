/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/

use std::collections::VecDeque;

// 定义图结构
struct Graph {
    adj: Vec<Vec<usize>>, // 邻接表表示图
}

impl Graph {
    // 创建包含n个顶点的新图
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n], // 初始化邻接表
        }
    }

    // 添加无向边（双向连接）
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }

    // 实现BFS算法，返回节点访问顺序
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        // 处理空图情况
        if self.adj.is_empty() {
            return vec![];
        }
        
        let mut visited = vec![false; self.adj.len()]; // 访问标记数组
        let mut queue = VecDeque::new(); // BFS队列
        let mut visit_order = Vec::new(); // 访问顺序记录
        
        // 初始化起始节点
        visited[start] = true;
        queue.push_back(start);
        
        while let Some(current) = queue.pop_front() {
            visit_order.push(current); // 记录当前节点访问顺序
            
            // 遍历当前节点的所有邻居
            for &neighbor in &self.adj[current] {
                if !visited[neighbor] {
                    visited[neighbor] = true; // 标记为已访问
                    queue.push_back(neighbor); // 加入队列
                }
            }
        }
        
        visit_order // 返回访问顺序
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}
