use crate::graph::edges::edge_trait::BidirectionalEdgeTrait;
use crate::graph::graph::Graph;
use crate::misc::recursive_function::{Callable2, RecursiveFunction2};

pub struct HLD {
    pub paths: Vec<Vec<usize>>,
    pub id: Vec<usize>,
    pub pos: Vec<usize>,
}

pub trait HLDecomposition {
    fn hl_decomposition_with_root(&self, root: usize) -> HLD;

    fn hl_decomposition(&self) -> HLD {
        self.hl_decomposition_with_root(0)
    }
}

/// Performs a Heavy-Light Decomposition of a tree graph.
///
/// This decomposition allows us to efficiently process queries and updates on paths
/// in a tree by splitting the tree into a set of paths with certain properties.
///
/// # Returns
///
/// A struct HLD containing:
/// - `paths`: A `Vec<Vec<usize>>` where each inner vector represents a heavy path in the tree.
/// - `id`: A `Vec<usize>` mapping each node to its heavy path identifier.
/// - `pos`: A `Vec<usize>` mapping each node to its position within its heavy path.
///
/// # Examples
///
/// ```
/// use algo_lib::graph::graph::Graph;
/// use algo_lib::graph::edges::edge_trait::BidirectionalEdgeTrait;
/// use algo_lib::graph::hl_decomposition::HLDecomposition;
///
/// let mut graph = Graph::from_biedges(
///     5,
///     &vec![(0, 1), (1, 2), (1, 3), (3, 4)]
/// );
///
/// let HLD {paths, id, pos} = graph.hl_decomposition();
///
/// // The tree is decomposed into heavy paths, and each node is assigned to a path
/// // with a unique identifier and a position within that path.
/// ```
///
/// This decomposition is useful for performing operations on paths of the tree,
/// such as querying for the maximum or minimum value on a path, or updating all
/// nodes along a path. By decomposing the tree into heavy paths, we can reduce
/// the complexity of such operations.

impl<E: BidirectionalEdgeTrait> HLDecomposition for Graph<E> {
    fn hl_decomposition_with_root(&self, root: usize) -> HLD {
        debug_assert!(self.is_tree());
        let n = self.vertex_count();
        let mut paths = Vec::new();
        let mut id = vec![0; n];
        let mut pos = vec![0; n];
        let mut size = vec![0u32; n];
        let mut calc_size = RecursiveFunction2::new(|f, vert, last| {
            size[vert] = 1;
            for e in self[vert].iter() {
                let next = e.to();
                if next == last {
                    continue;
                }
                size[vert] += f.call(next, vert);
            }
            size[vert]
        });
        calc_size.call(root, root);
        paths.push(vec![root]);
        let mut build = RecursiveFunction2::new(|f, vert: usize, last| {
            if vert != root {
                if 2 * size[vert] >= size[last] {
                    id[vert] = id[last];
                    pos[vert] = pos[last] + 1;
                    paths[id[vert]].push(vert);
                } else {
                    id[vert] = paths.len();
                    paths.push(vec![vert]);
                }
            }
            for e in self[vert].iter() {
                let next = e.to();
                if next == last {
                    continue;
                }
                f.call(next, vert);
            }
        });
        build.call(root, root);
        HLD { paths, id, pos }
    }
}
