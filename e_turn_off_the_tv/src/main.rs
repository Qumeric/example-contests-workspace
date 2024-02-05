//{"name":"E. Turn Off The TV","group":"Codeforces - Educational Codeforces Round 29","url":"https://codeforces.com/contest/863/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 3\n4 6\n1 7\n","output":"1\n"},{"input":"2\n0 10\n0 10\n","output":"1\n"},{"input":"3\n1 2\n3 4\n6 8\n","output":"-1\n"},{"input":"3\n1 2\n2 3\n3 4\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ETurnOffTheTV"}}}

use std::collections::HashMap;

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::specs::{ArqSpec, AssignMax, AssignMin};
use algo_lib::collections::static_arq::StaticArq;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();

    let mut coords = vec![];
    let mut segs = vec![];

    for i in 0..n {
        let l = input.read_size() + 1;
        let r = input.read_size() + 1;
        segs.push((l, r));
        coords.push(l);
        coords.push(r);
        coords.push(r + 1);
    }
    coords.sort();
    coords.dedup();
    // out.print_line(&coords);
    let coords: HashMap<_, _> = coords.enumerate().map(|(i, &val)| (val, i)).collect();

    segs = segs
        .map(|(l, r)| (*coords.get(l).unwrap(), *coords.get(r).unwrap()))
        .collect_vec();

    for seg in &segs {
        out.print_line(seg);
    }

    // need to find segment such that removal will wont increase number of 1s

    let mut tree1 = StaticArq::<AssignMin>::new(&vec![i64::MAX; coords.len()]);
    let mut tree2 = StaticArq::<AssignMax>::new(&vec![i64::MIN; coords.len()]);

    for i in 0..n {
        let (l, r) = segs[i];
        let i = i as i64;
        tree1.update(l, r, &(i + 1));
        tree2.update(l, r, &(i + 1));
    }

    for i in 0..n {
        let (l, r) = segs[i];
        let min = tree1.query(l, r);
        let max = tree1.query(l, r);

        out.print_line(("QUERY", l, r));
        out.print_line((_ones, twos));

        if twos as usize == r - l + 1 {
            out.print_line(i + 1);
            return;
        }
    }
    out.print_line(-1);
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
