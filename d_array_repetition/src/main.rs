//{"name":"D. Array Repetition","group":"Codeforces - Codeforces Round 919 (Div. 2)","url":"https://codeforces.com/contest/1920/problem/D","interactive":false,"timeLimit":4000,"tests":[{"input":"4\n5 10\n1 1\n1 2\n2 1\n1 3\n2 3\n1 2 3 4 5 6 14 15 16 20\n10 10\n1 3\n1 8\n2 15\n1 6\n1 9\n1 1\n2 6\n1 1\n2 12\n2 10\n32752 25178 3198 3199 2460 2461 31450 33260 9016 4996\n12 5\n1 6\n1 11\n2 392130334\n1 4\n2 744811750\n1 10\n1 5\n2 209373780\n2 178928984\n1 3\n2 658326464\n2 1000000000\n914576963034536490 640707385283752918 636773368365261971 584126563607944922 1000000000000000000\n2 2\n1 1\n1 2\n1 2\n","output":"1 2 1 2 3 1 2 3 1 3\n9 8 1 3 1 3 6 3 8 8\n11 11 11 10 11\n1 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DArrayRepetition"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let q = input.read_size();

    // kind, val
    let mut ops: Vec<(usize, usize)> = vec![];

    for i in 0..n {
        let b = input.read_size();
        let x = input.read_size();
        ops.push((b, x));
    }

    // we can sort queires and answer from small to large etc.

    // query, pos
    let mut queries: Vec<(usize, usize)> = vec![];

    for i in 0..q {
        let k = input.read_size();
        queries.push((k - 1, i));
        // print kth element (k-1)
    }
    queries.sort();

    let mut ans = vec![0; q];

    while !queries.is_empty() {
        let mut ops_ptr = 0;
        let mut len_before = 0;
        let mut array_len: u128 = 0;

        let mut last_op = 1;
        let mut last_x = 1;

        let mut new_queries = vec![];
        for (q, pos) in queries {
            let q = q as u128;
            while q >= array_len {
                // out.print_line((q, array_len));
                let (b, x) = ops[ops_ptr];
                ops_ptr += 1;

                if b == 1 {
                    array_len += 1;
                    last_op = 1;
                    // add integer to end
                } else {
                    len_before = array_len;
                    if array_len > u64::MAX as u128 {
                        array_len = u128::MAX;
                    } else {
                        array_len *= (x + 1) as u128;
                    }
                    last_op = 2;
                    // copy x times
                }
                last_x = x;
            }
            if last_op == 1 {
                ans[pos] = last_x;
            } else {
                new_queries.push(((q % len_before) as usize, pos));
            }
        }
        queries = new_queries;
        // out.print_line("NEW Q");
        // for q in queries.iter() {
        //     out.print_line(q);
        // }
        queries.sort();
    }

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
