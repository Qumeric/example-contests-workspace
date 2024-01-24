//{"name":"D. Segment Tree","group":"Codeforces - Educational Codeforces Round 78 (Rated for Div. 2)","url":"https://codeforces.com/problemset/problem/1278/D","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n9 12\n2 11\n1 3\n6 10\n5 7\n4 8\n","output":"YES\n"},{"input":"5\n1 3\n2 4\n5 9\n6 8\n7 10\n","output":"NO\n"},{"input":"5\n5 8\n3 6\n2 9\n7 10\n1 4\n","output":"NO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSegmentTree"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::treap_map::TreapSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut segs = (0..n).map(|_| (input.read1(), input.read1())).collect_vec();
    segs.sort();
    // for s in &segs {
    //     out.print_line(s);
    // }

    let mut dsu = DSU::new(n);

    let mut set = TreapSet::<(usize, usize, usize)>::new();

    for i in 0..n {
        let mut to_remove = vec![];
        for cur in set.iter() {
            // ends too late, i.e. cur includes whole i segment.
            // Everything in set start early and everything later in set will end even later
            if cur.0 > segs[i].1 {
                break;
            // ends before start, need to remove for speed
            } else if cur.0 < segs[i].0 {
                to_remove.push(*cur);
            } else {
                // out.print_line(("MERGING", i, cur.2));
                if !dsu.join(i, cur.2) {
                    out.print_line(false);
                    return;
                }
            }
        }
        for r in to_remove {
            // out.print_line(("REMOVE", r.1, r.0, r.2));
            set.remove(&r);
        }
        set.insert((segs[i].1, segs[i].0, i));
        // out.print_line(("INSERT", segs[i].0, segs[i].1, i));
    }

    out.print_line(dsu.set_count() == 1);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // don't forget to set test_type = Single if you do it
    // tester::stress_test(run, tester::check);
}
//END MAIN
