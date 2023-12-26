use crate::graph::edges::edge_trait::BidirectionalEdgeTrait;
use crate::graph::graph::Graph;

/// Returns two vectors representing the DFS order of the graph.
///
/// The first vector contains the discovery times (position of first visit),
/// and the second vector contains the finish times (position after visiting all descendants)
/// for each vertex in the DFS traversal.
pub trait DFSOrder {
    /// Computes the DFS order starting from a given root vertex.
    ///
    /// Returns two vectors: the first for discovery times, and the second for finish times.
    fn dfs_order_with_root(&self, root: usize) -> (Vec<usize>, Vec<usize>);

    /// Computes the DFS order starting from the first vertex (vertex 0).
    ///
    /// Returns two vectors: the first for discovery times, and the second for finish times.
    fn dfs_order(&self) -> (Vec<usize>, Vec<usize>) {
        self.dfs_order_with_root(0)
    }
}

impl<E: BidirectionalEdgeTrait> DFSOrder for Graph<E> {
    fn dfs_order_with_root(&self, root: usize) -> (Vec<usize>, Vec<usize>) {
        debug_assert!(self.is_tree());
        let count = self.vertex_count();
        let mut position = vec![0; count];
        let mut end = vec![0; count];
        let mut edge = vec![0u32; count];
        let mut stack = vec![0u32; count];
        let mut last = vec![0u32; count];
        let mut size = 1usize;
        last[root] = root as u32;
        stack[0] = root as u32;
        position[root] = 0;
        let mut index = 0usize;
        while size > 0 {
            let current = stack[size - 1] as usize;
            let c_edge = &mut edge[current];
            if *c_edge == self[current].len() as u32 {
                end[current] = index + 1;
                size -= 1;
            } else {
                let next = self[current][*c_edge as usize].to();
                *c_edge += 1;
                if next == (last[current] as usize) {
                    continue;
                }
                index += 1;
                position[next] = index;
                last[next] = current as u32;
                stack[size] = next as u32;
                size += 1;
            }
        }
        (position, end)
    }
}
