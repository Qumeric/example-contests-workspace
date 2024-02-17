//{"name":"A. Literature Lesson","group":"Codeforces - Codeforces Beta Round 99 (Div. 1)","url":"https://codeforces.com/contest/138/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"1 1\nday\nmay\nsun\nfun\n","output":"aabb\n"},{"input":"1 1\nday\nmay\ngray\nway\n","output":"aaaa\n"},{"input":"2 1\na\na\na\na\na\na\ne\ne\n","output":"aabb\n"},{"input":"2 1\nday\nmay\nsun\nfun\ntest\nhill\nfest\nthrill\n","output":"NO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ALiteratureLesson"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();

    let mut poem: Vec<(Str<'_>, Str<'_>, Str<'_>, Str<'_>)> = vec![];
    for i in 0..n {
        let a = input.read_str();
        let b = input.read_str();
        let c = input.read_str();
        let d = input.read_str();
        poem.push((a, b, c, d));
    }

    let mut a = vec![false; n];
    let mut b = vec![false; n];
    let mut c = vec![false; n];

    let rhyme = |aa: &Str<'_>, bb: &Str<'_>| {
        let mut a = aa.clone();
        let mut b = bb.clone();
        let mut k1 = k;
        let mut k2 = k;
        let mut suf_a = vec![];
        let mut suf_b = vec![];

        while k1 > 0 {
            if let Some(last_char) = a.pop() {
                if matches!(last_char as char, 'a' | 'e' | 'i' | 'o' | 'u') {
                    k1 -= 1;
                }
                suf_a.push(last_char);
            } else {
                return false;
            }
        }

        while k2 > 0 {
            if let Some(last_char) = b.pop() {
                if matches!(last_char as char, 'a' | 'e' | 'i' | 'o' | 'u') {
                    k2 -= 1;
                }
                suf_b.push(last_char);
            } else {
                return false;
            }
        }

        suf_a == suf_b
    };

    let check_a = |i| {
        let q: &(Str<'_>, Str<'_>, Str<'_>, Str<'_>) = &poem[i];
        rhyme(&q.0, &q.1) && rhyme(&q.2, &q.3)
    };

    let check_b = |i| {
        let q: &(Str<'_>, Str<'_>, Str<'_>, Str<'_>) = &poem[i];
        rhyme(&q.0, &q.2) && rhyme(&q.1, &q.3)
    };

    let check_c = |i| {
        let q: &(Str<'_>, Str<'_>, Str<'_>, Str<'_>) = &poem[i];
        rhyme(&q.0, &q.3) && rhyme(&q.1, &q.2)
    };

    for i in 0..n {
        a[i] = check_a(i);
        b[i] = check_b(i);
        c[i] = check_c(i);
    }

    let res_a = a.into_iter().all(|x| x);
    let res_b = b.into_iter().all(|x| x);
    let res_c = c.into_iter().all(|x| x);

    if res_a && res_b && res_c {
        out.print_line("aaaa");
        return;
    }
    if res_a {
        out.print_line("aabb");
    } else if res_b {
        out.print_line("abab")
    } else if res_c {
        out.print_line("abba");
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
