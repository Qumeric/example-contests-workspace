use super::{edges::edge_trait::EdgeTrait, graph::Graph};

const IN_DECOMPOSITION: u64 = 1 << 63;

/// Centroid Decomposition for a tree.
///
/// Given a tree, it can be recursively decomposed into centroids. Then the
/// parent of a centroid `c` is the previous centroid that splitted its connected
/// component into two or more components. It can be shown that in such
/// decomposition, for each path `p` with starting and ending vertices `u`, `v`,
/// the lowest common ancestor of `u` and `v` in centroid tree is a vertex of `p`.
///
/// ATTENTION:
/// The input tree should have its vertices numbered from 1 to n, and
/// `graph_enumeration.rs` may help to convert other representations.
pub struct CD {
    /// The root of the centroid tree, should _not_ be set by the user
    pub root: usize,
    /// The result. `decomposition[v]` is the parent of `v` in centroid tree.
    /// `decomposition[root]` is 0
    pub decomposition: Vec<usize>,
    /// Used internally to save the big_child of a vertex, and whether it has
    /// been added to the centroid tree.
    vert_state: Vec<u64>,
    /// Used internally to save the subtree size of a vertex
    vert_size: Vec<usize>,
}

impl CD {
    pub fn new(mut num_vertices: usize) -> Self {
        num_vertices += 1;
        CD {
            root: 0,
            decomposition: vec![0; num_vertices],
            vert_state: vec![0; num_vertices],
            vert_size: vec![0; num_vertices],
        }
    }
    #[inline]
    fn put_in_decomposition(&mut self, v: usize, parent: usize) {
        self.decomposition[v] = parent;
        self.vert_state[v] |= IN_DECOMPOSITION;
    }
    #[inline]
    fn is_in_decomposition(&self, v: usize) -> bool {
        (self.vert_state[v] & IN_DECOMPOSITION) != 0
    }
    fn dfs_size(&mut self, v: usize, parent: usize, adj: &Vec<Vec<impl EdgeTrait>>) -> usize {
        self.vert_size[v] = 1;
        let mut big_child = 0_usize;
        let mut bc_size = 0_usize; // big child size
        for edge in adj[v].iter() {
            let u = edge.to();
            if u == parent || self.is_in_decomposition(u) {
                continue;
            }
            let u_size = self.dfs_size(u, v, adj);
            self.vert_size[v] += u_size;
            if u_size > bc_size {
                big_child = u;
                bc_size = u_size;
            }
        }
        self.vert_state[v] = big_child as u64;
        self.vert_size[v]
    }
    fn dfs_centroid(&self, v: usize, size_thr: usize) -> usize {
        // recurse until big child's size is <= `size_thr`
        match self.vert_state[v] as usize {
            u if self.vert_size[u] <= size_thr => v,
            u => self.dfs_centroid(u, size_thr),
        }
    }
    fn decompose_subtree(
        &mut self,
        v: usize,
        centroid_parent: usize,
        calculate_vert_size: bool,
        adj: &Vec<Vec<impl EdgeTrait>>,
    ) -> usize {
        // `calculate_vert_size` determines if it is necessary to recalculate
        // `self.vert_size`
        if calculate_vert_size {
            self.dfs_size(v, centroid_parent, adj);
        }
        let v_size = self.vert_size[v];
        let centroid = self.dfs_centroid(v, v_size >> 1);
        self.put_in_decomposition(centroid, centroid_parent);
        for edge in adj[centroid].iter() {
            let u = edge.to();
            if self.is_in_decomposition(u) {
                continue;
            }
            self.decompose_subtree(
                u,
                centroid,
                self.vert_size[u] > self.vert_size[centroid],
                adj,
            );
        }
        centroid
    }
    pub fn decompose_tree(&mut self, adj: &Vec<Vec<impl EdgeTrait>>) {
        self.decompose_subtree(1, 0, true, adj);
    }
}

pub trait CentroidDecomposition {
    fn centroid_decomposition_with_root(&self, root: usize) -> CD;

    fn centroid_decomposition(&self) -> CD {
        self.centroid_decomposition_with_root(1)
    }
}

// TODO: it works but it's not well compatibale with how Graph works in algo-lib
// Graph uses 0-indexing and centroid decomposition uses 1-indexing
// Therefore it's not very convenient and e.g. graph.is_tree() will return false
impl<E: EdgeTrait> CentroidDecomposition for Graph<E> {
    fn centroid_decomposition_with_root(&self, root: usize) -> CD {
        let mut cd = CD::new(self.vertex_count() - 1);

        cd.decompose_subtree(root, 0, true, &self.edges);

        cd
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::{centroid_decomposition::CentroidDecomposition, graph::Graph};

    #[test]
    fn single_path() {
        let len = 16;
        let mut edges = vec![(1, 2), (15, 14)];

        for i in 2..15 {
            edges.push((i, i + 1));
            edges.push((i, i - 1));
        }
        let tree = Graph::from_edges(len, &edges);
        // for (i, v) in tree.edges.enumerate() {
        //     for e in v {
        //         println!("edge {}-{}", i, e.to());
        //     }
        // }
        let cd = tree.centroid_decomposition();
        // We should get a complete binary tree
        assert_eq!(
            cd.decomposition,
            vec![0, 2, 4, 2, 8, 6, 4, 6, 0, 10, 12, 10, 8, 14, 12, 14]
        );
    }
}
