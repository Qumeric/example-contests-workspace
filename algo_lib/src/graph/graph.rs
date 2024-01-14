use crate::collections::dsu::DSU;
use crate::graph::edges::bi_edge::BiEdge;
use crate::graph::edges::bi_weighted_edge::BiWeightedEdge;
use crate::graph::edges::edge::Edge;
use crate::graph::edges::edge_trait::{BidirectionalEdgeTrait, EdgeTrait};
use std::ops::{Index, IndexMut};

pub struct Graph<E: EdgeTrait> {
    pub(super) edges: Vec<Vec<E>>,
    edge_count: usize,
}

impl<E: EdgeTrait> Graph<E> {
    pub fn new(vertex_count: usize) -> Self {
        Self {
            edges: vec![Vec::new(); vertex_count],
            edge_count: 0,
        }
    }

    pub fn add_edge(&mut self, (from, mut edge): (usize, E)) -> usize {
        let to = edge.to();
        assert!(to < self.edges.len());
        let direct_id = self.edges[from].len();
        edge.set_id(self.edge_count);
        self.edges[from].push(edge);
        if E::REVERSABLE {
            let rev_id = self.edges[to].len();
            self.edges[from][direct_id].set_reverse_id(rev_id);
            let mut rev_edge = self.edges[from][direct_id].reverse_edge(from);
            rev_edge.set_id(self.edge_count);
            rev_edge.set_reverse_id(direct_id);
            self.edges[to].push(rev_edge);
        }
        self.edge_count += 1;
        direct_id
    }

    pub fn into_egdes(self) -> Vec<Vec<E>> {
        self.edges
    }

    pub fn add_vertices(&mut self, cnt: usize) {
        self.edges.resize(self.edges.len() + cnt, Vec::new());
    }

    pub fn clear(&mut self) {
        self.edge_count = 0;
        for ve in self.edges.iter_mut() {
            ve.clear();
        }
    }

    pub fn vertex_count(&self) -> usize {
        self.edges.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edge_count
    }

    pub fn degrees(&self) -> Vec<usize> {
        self.edges.iter().map(|v| v.len()).collect()
    }

    /// Perform a depth-first search (DFS) starting from a given vertex.
    ///
    /// This method takes a mutable reference to a function that will be called
    /// for each vertex visited during the DFS. The function receives an `Option<usize>`
    /// representing the parent of the current vertex (or `None` if it's the root of the DFS)
    /// and the index of the current vertex.
    ///
    /// # Arguments
    ///
    /// * `start` - The starting vertex index for the DFS.
    /// * `visit` - A mutable reference to a function that processes each vertex.
    pub fn dfs<F>(&self, start: usize, mut visit: F)
    where
        F: FnMut(Option<usize>, usize),
    {
        let mut stack = vec![(None, start)];
        let mut visited = vec![false; self.vertex_count()];

        while let Some((parent, node)) = stack.pop() {
            if visited[node] {
                continue;
            }
            visited[node] = true;
            visit(parent, node);

            for edge in &self.edges[node] {
                if !visited[edge.to()] {
                    stack.push((Some(node), edge.to()));
                }
            }
        }
    }
}

impl<E: BidirectionalEdgeTrait> Graph<E> {
    pub fn is_tree(&self) -> bool {
        if self.edge_count + 1 != self.vertex_count() {
            false
        } else {
            self.is_connected()
        }
    }

    pub fn is_forest(&self) -> bool {
        let mut dsu = DSU::new(self.vertex_count());
        for i in 0..self.vertex_count() {
            for e in self[i].iter() {
                if i <= e.to() && !dsu.join(i, e.to()) {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_connected(&self) -> bool {
        let mut dsu = DSU::new(self.vertex_count());
        for i in 0..self.vertex_count() {
            for e in self[i].iter() {
                dsu.join(i, e.to());
            }
        }
        dsu.set_count() == 1
    }

    pub fn find_diameter_path(&self) -> Vec<usize> {
        assert!(self.is_tree());
        let mut a_node = 0;
        let mut max_distance = 0;
        let mut distances = vec![0; self.vertex_count()];
        let mut parents = vec![None; self.vertex_count()];
        self.dfs(0, |parent, node| {
            if let Some(parent) = parent {
                distances[node] = distances[parent] + 1;
                parents[node] = Some(parent);
                if distances[node] > max_distance {
                    max_distance = distances[node];
                    a_node = node;
                }
            }
        });
        let mut b_node = 0;
        max_distance = 0;
        distances = vec![0; self.vertex_count()];
        parents = vec![None; self.vertex_count()];
        self.dfs(a_node, |parent, node| {
            if let Some(parent) = parent {
                distances[node] = distances[parent] + 1;
                parents[node] = Some(parent);
                if distances[node] > max_distance {
                    max_distance = distances[node];
                    b_node = node;
                }
            }
        });

        self.find_path_between(a_node, b_node)
    }

    pub fn find_path_between(&self, start: usize, end: usize) -> Vec<usize> {
        let mut path = Vec::new();
        let mut parents = vec![None; self.vertex_count()];
        self.dfs(start, |parent, node| {
            if let Some(parent) = parent {
                parents[node] = Some(parent);
            }
        });
        let mut node = end;
        while let Some(parent) = parents[node] {
            path.push(node);
            node = parent;
        }
        path.push(start);
        path.reverse();
        path
    }
}

impl<E: EdgeTrait> Index<usize> for Graph<E> {
    type Output = [E];

    fn index(&self, index: usize) -> &Self::Output {
        &self.edges[index]
    }
}

impl<E: EdgeTrait> IndexMut<usize> for Graph<E> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.edges[index]
    }
}

impl Graph<Edge<()>> {
    pub fn from_edges(n: usize, edges: &[(usize, usize)]) -> Self {
        let mut graph = Self::new(n);
        for &(from, to) in edges {
            graph.add_edge(Edge::new(from, to));
        }
        graph
    }
}

impl Graph<BiEdge<()>> {
    pub fn from_biedges(n: usize, edges: &[(usize, usize)]) -> Self {
        let mut graph = Self::new(n);
        for &(from, to) in edges {
            graph.add_edge(BiEdge::new(from, to));
        }
        graph
    }
}

impl<W: Copy> Graph<BiWeightedEdge<W, ()>> {
    pub fn from_weighted_biedges(n: usize, edges: &[(usize, usize, W)]) -> Self {
        let mut graph = Self::new(n);
        for &(from, to, w) in edges {
            graph.add_edge(BiWeightedEdge::new(from, to, w));
        }
        graph
    }
}
