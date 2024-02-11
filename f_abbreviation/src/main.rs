//{"name":"F. Abbreviation","group":"Codeforces - Codeforces Round 494 (Div. 3)","url":"https://codeforces.com/contest/1003/problem/F","interactive":false,"timeLimit":1000,"tests":[{"input":"6\nto be or not to be\n","output":"12\n"},{"input":"10\na ab a a b ab a a b c\n","output":"13\n"},{"input":"6\naa bb aa aa bb bb\n","output":"11\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FAbbreviation"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;
use algo_lib::string::string_algorithms::z_algorithm::ZAlgorithm;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let w = input.read_str_vec(n);

    let mut vocab = w.clone();
    vocab.sort();
    vocab.dedup();

    let init_len: usize = w.iter().map(|w| w.len()).sum::<usize>() + n - 1;

    let mut encoded = vec![];

    for i in 0..n {
        let mut pos = n;
        for j in 0..vocab.len() {
            if vocab[j] == w[i] {
                pos = j;
                break;
            }
        }
        encoded.push((pos, w[i].len()));
    }

    // for (a, b) in &encoded {
    //     out.print_line((a, b));
    // }

    let mut best_shortening = 0;
    for i in 0..n {
        let mut word_len = 0;
        for j in i..n {
            // [i, j]
            let abbreavtion_len = j - i + 1;
            word_len += encoded[j].1;
            let single_delta = word_len - 1;
            let mut str = vec![];
            for k in i..=j {
                str.push(encoded[k].0);
            }
            for k in 0..n {
                str.push(encoded[k].0);
            }
            let zf = str.z_algorithm();
            let mut last = 0;
            let mut changes = 0;
            for i in abbreavtion_len..zf.len() {
                if zf[i] >= abbreavtion_len && i >= last + abbreavtion_len {
                    last = i;
                    changes += 1;
                }
            }
            // out.print_line((i, j, changes, changes * single_delta));
            // out.print_line(str);
            // out.print_line(zf);
            if changes > 1 {
                best_shortening.maxim(changes * single_delta);
            }
        }
    }
    // out.print_line(&init_len);
    // out.print_line(&best_shortening);
    out.print_line(init_len - best_shortening);
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
