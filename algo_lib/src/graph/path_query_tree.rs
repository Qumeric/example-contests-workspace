use crate::collections::specs::ArqSpec;
use crate::collections::static_arq::StaticArq;
use crate::graph::graph::Graph;
use crate::graph::hl_decomposition::HLDecomposition;
use crate::graph::lca::LCATrait;

use super::edges::bi_edge::BiEdge;
use super::hl_decomposition::HLD;
use super::lca::LCA;

// TODO:
// It would be nice to make query kind generic to avoid asserts
// but rust doesn't support specification by enum variant as of 1.75
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum QueryKind {
    Vertex,
    Edge,
}

// HLD-based tree which supports operations on paths and subtrees
pub struct PathQueryTree<Spec: ArqSpec> {
    pub query_kind: QueryKind,
    lca: LCA,
    hld: HLD,
    seg_trees: Vec<StaticArq<Spec>>,
    n: usize,
}

impl<Spec: ArqSpec> PathQueryTree<Spec> {
    pub fn new_with_root<E: Clone>(
        tree: &Graph<BiEdge<E>>,
        query_kind: QueryKind,
        root: usize,
    ) -> Self {
        assert!(tree.is_tree());

        let hld = tree.hl_decomposition_with_root(root);
        let seg_trees: Vec<_> = hld
            .paths
            .iter()
            .map(|p| StaticArq::<Spec>::new(&vec![Default::default(); p.len()]))
            .collect();

        Self {
            query_kind,
            lca: tree.lca_with_root(root),
            hld,
            seg_trees,
            n: tree.vertex_count(),
        }
    }

    pub fn new<E: Clone>(tree: &Graph<BiEdge<E>>, query_kind: QueryKind) -> Self {
        PathQueryTree::new_with_root(tree, query_kind, 0)
    }

    pub fn update(&mut self, mut u: usize, mut v: usize, val: &Spec::F) {
        let cur_lca = self.lca.lca(u, v);
        let hld = &mut self.hld;
        let seg_trees = &mut self.seg_trees;

        let mut update_path = |x: &mut usize| {
            while hld.id[*x] != hld.id[cur_lca] {
                let path_id = hld.id[*x];
                let cur_pos = hld.pos[*x];

                let path = &mut seg_trees[path_id];
                path.update(0, cur_pos, &val);
                // out.print_line(("updating", path_id, "to", cur_pos));

                *x = self.lca.parent(hld.paths[path_id][0]).unwrap();
            }
            if *x != cur_lca {
                seg_trees[hld.id[*x]].update(hld.pos[cur_lca] + 1, hld.pos[*x], val);
            }
        };

        update_path(&mut u);
        update_path(&mut v);
        // It would be slighly easier to include LCA in `update_path` and then subtract here
        // But I don't want to require support of the minus operator from `Spec`
        if self.query_kind == QueryKind::Vertex {
            seg_trees[hld.id[cur_lca]].update_point(hld.pos[cur_lca], val);
        }
    }

    pub fn get_vertexes(&mut self) -> Vec<Spec::S> {
        self.get_vertexes_internal(self.query_kind)
    }

    fn get_vertexes_internal(&mut self, query_kind: QueryKind) -> Vec<Spec::S> {
        assert_eq!(query_kind, QueryKind::Vertex);

        (0..self.n)
            .map(|i| {
                let path_id = self.hld.id[i];
                let pos = self.hld.pos[i];
                self.seg_trees[path_id].query_point(pos)
            })
            .collect()
    }

    pub fn get_edges(&mut self, edges: &[(usize, usize)]) -> Vec<Spec::S> {
        assert_eq!(self.query_kind, QueryKind::Edge);

        let mut vertex_ans = self.get_vertexes_internal(QueryKind::Vertex);

        edges
            .into_iter()
            .map(|&(u, v)| {
                let child = if self.lca.level(u) > self.lca.level(v) {
                    u
                } else {
                    v
                };
                let mut value: Spec::S = Default::default();

                // Guarantee: each child will be taken at most once
                std::mem::swap(&mut vertex_ans[child], &mut value);
                value
            })
            .collect()
    }
}
