use std::cmp::{max, min};
use std::collections::BTreeSet;

use crate::collections::bit_set::BitSet;
use crate::collections::dsu::DSU;
use crate::collections::min_max::MinimMaxim;
use crate::graph::graph::Graph;
use crate::misc::recursive_function::{Callable2, RecursiveFunction2};

use super::edges::bi_edge::BiEdge;
use super::edges::bi_edge_trait::BiEdgeTrait;

pub trait BridgeSearch {
    fn bridges(&self) -> Vec<(usize, usize)>;
    fn biconnected_tree(&self) -> Graph<BiEdge<()>>;
}

impl<E: BiEdgeTrait> BridgeSearch for Graph<E> {
    fn bridges(&self) -> Vec<(usize, usize)> {
        assert!(E::REVERSABLE);
        let n = self.vertex_count();
        let mut timer = 0;
        let mut tin = vec![0; n];
        let mut fup = vec![0; n];
        let mut used = BitSet::new(n);
        let mut ans = Vec::new();
        for i in 0..n {
            if !used[i] {
                let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
                    used.set(vert);
                    tin[vert] = timer;
                    fup[vert] = timer;
                    timer += 1;
                    for e in &self[vert] {
                        if e.to() == prev {
                            continue;
                        }
                        let to = e.to();
                        if used[to] {
                            fup[vert].minim(tin[to]);
                        } else {
                            f.call(to, vert);
                            let cand = fup[to];
                            fup[vert].minim(cand);
                            if fup[to] > tin[vert] {
                                ans.push((vert, to));
                            }
                        }
                    }
                });
                dfs.call(i, i);
            }
        }
        ans
    }

    fn biconnected_tree(&self) -> Graph<BiEdge<()>> {
        let bridges: BTreeSet<(usize, usize)> = self
            .bridges()
            .into_iter()
            .map(|(a, b)| (min(a, b), max(a, b)))
            .collect();

        let n = self.vertex_count();

        let mut dsu = DSU::new(n);

        for from in 0..n {
            for edge in self.edges[from].iter() {
                let p = (min(from, edge.to()), max(from, edge.to()));
                if !bridges.contains(&p) {
                    dsu.join(from, edge.to());
                }
            }
        }

        let mut colors = vec![0; n];
        let mut color = 0;

        for v in 0..n {
            let comp = dsu.get(v);
            if colors[comp] == 0 {
                color += 1;
                colors[comp] = color;
            }
        }

        let mut g = Graph::<BiEdge<()>>::new(color);

        for (u, v) in bridges {
            let a = colors[dsu.get(u)] - 1;
            let b = colors[dsu.get(v)] - 1;

            g.add_edge(BiEdge::new(a, b));
        }

        g
    }
}
