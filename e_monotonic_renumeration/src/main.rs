//{"name":"E. Monotonic Renumeration","group":"Codeforces - Codeforces Round 531 (Div. 3)","url":"https://codeforces.com/contest/1102/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1 2 1 2 3\n","output":"2\n"},{"input":"2\n100 1\n","output":"2\n"},{"input":"4\n1 3 3 7\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMonotonicRenumeration"}}}

use std::cmp::max;
use std::collections::BTreeMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut indexes = BTreeMap::<usize, Vec<usize>>::new();

    for i in 0..n {
        indexes
            .entry(a[i])
            .and_modify(|v| v.push(i))
            .or_insert(vec![i]);
    }

    let mut segs = vec![];

    for (k, v) in indexes {
        segs.push((*v.first().unwrap(), *v.last().unwrap()));
    }
    segs.sort();
    // for seg in &segs {
    //     out.print_line(seg);
    // }

    let mut merged_segs = vec![];

    let mut i = 1;
    let mut seg = segs[0].clone();

    while i < segs.len() {
        if segs[i].0 < seg.1 {
            seg.1 = max(seg.1, segs[i].1);
        } else {
            merged_segs.push(seg);
            seg = segs[i];
        }
        i += 1;
    }

    merged_segs.push(seg);

    // out.print_line("mg");
    // for seg in &merged_segs {
    //     out.print_line(seg);
    // }

    let ans = ModIntF::new(2).power(merged_segs.len() - 1);

    out.print_line(ans);
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
