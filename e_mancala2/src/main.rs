//{"name":"E - Mancala 2","group":"AtCoder - KAJIMA CORPORATION CONTEST 2024（AtCoder Beginner Contest 340）","url":"https://atcoder.jp/contests/abc340/tasks/abc340_e","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3\n1 2 3 4 5\n2 4 0\n","output":"0 4 2 7 2\n"},{"input":"3 10\n1000000000 1000000000 1000000000\n0 1 0 1 0 1 0 1 0 1\n","output":"104320141 45436840 2850243019\n"},{"input":"1 4\n1\n0 0 0 0\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMancala2"}}}

use algo_lib::collections::specs::PlusSum;
use algo_lib::collections::static_arq::StaticArq;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_long_vec(n);
    let b = input.read_size_vec(m);

    let mut tree = StaticArq::<PlusSum>::new(&a);
    for i in 0..m {
        let p = b[i];
        let elem = tree.query_point(p);
        if elem == 0 {
            continue;
        }
        tree.update_point(p, &-elem);

        // [first, last]
        let first = (p + 1) as usize;
        let last = first + elem as usize - 1;

        if last >= n {
            tree.update(first, n - 1, &1);
            let suf_len = n - first;
            let now_need = elem as usize - suf_len;
            let full = (now_need / n) as i64;
            tree.update(0, n - 1, &full);
            let pref_len = now_need % n;
            if pref_len > 0 {
                tree.update(0, pref_len - 1, &1);
            }
        } else {
            tree.update(first, last, &1);
        }
    }
    for i in 0..n {
        out.print(tree.query_point(i));
        out.print(" ");
    }
    out.print_line("");
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
