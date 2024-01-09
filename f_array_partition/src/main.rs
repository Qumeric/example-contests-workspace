//{"name":"F. Array Partition","group":"Codeforces - Codeforces Round 686 (Div. 3)","url":"https://codeforces.com/contest/1454/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n11\n1 2 3 3 3 4 4 3 4 2 1\n8\n2 9 1 7 3 9 4 1\n9\n2 1 4 2 4 3 3 1 2\n7\n4 2 1 1 4 1 4\n5\n1 1 1 1 1\n7\n4 3 4 3 3 3 4\n","output":"YES\n6 1 4\nNO\nYES\n2 5 2\nYES\n4 1 2\nYES\n1 1 3\nYES\n2 1 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FArrayPartition"}}}

use algo_lib::collections::specs::{AssignMax, AssignMin};
use algo_lib::collections::static_arq::StaticArq;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);

    let mut tree_max = StaticArq::<AssignMax>::new(&a);
    let mut tree_min = StaticArq::<AssignMin>::new(&a);
    let mut maxx = 0;
    for i in 0..n {
        maxx = maxx.max(a[i]);
    }

    let mut max_poss = vec![];
    for i in 0..n {
        if a[i] == maxx {
            max_poss.push(i);
        }
    }

    let max_pos = if max_poss.len() == 1 {
        max_poss[0]
    } else {
        max_poss[1]
    };

    let mut l = max_pos;
    let mut r = max_pos;

    // out.print_line(("max pos", l, r));

    while l > 0 && r < n - 2 {
        let left = tree_max.query(0, l - 1);
        let right = tree_max.query(r + 1, n - 1);
        if left < right {
            r += 1;
        } else if left > right {
            l -= 1;
        } else {
            if left == tree_min.query(l, r) {
                break;
            } else {
                let new_left = a[l - 1];
                let new_right = a[r + 1];
                if new_left >= new_right {
                    l -= 1;
                } else {
                    r += 1;
                }
                // r += 1;
                // l -= 1;
            }
        }
        // out.print_line(("step", l, r));
    }

    if l == 0 || r == n - 1 {
        out.print_line("NO");
        return;
    }

    let m = tree_min.query(l, r);
    if tree_max.query(0, l - 1) == m && m == tree_max.query(r + 1, n - 1) {
        out.print_line("YES");
        let a = l;
        let b = r - l + 1;
        let c = n - a - b;
        out.print_line((a, b, c));
    } else {
        out.print_line("NO");
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
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
