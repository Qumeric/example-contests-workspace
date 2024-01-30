//{"name":"F. Destroy it!","group":"Codeforces - Codeforces Round 565 (Div. 3)","url":"https://codeforces.com/contest/1176/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3\n1 6\n1 7\n1 5\n2\n1 4\n1 3\n3\n1 10\n3 5\n2 3\n3\n1 15\n2 4\n1 10\n1\n1 100\n","output":"263\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FDestroyIt"}}}

use std::cmp::max;

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

// This is completely correct idea but subtle bug
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();

    let mut turns = vec![];
    let mut maxx = vec![];

    for i in 0..n {
        let k = input.read_size();
        let mut cards = vec![vec![]; 4]; // 1-indexed

        let mut one_card = 0;

        for j in 0..k {
            let c = input.read_size();
            let d = input.read_size();
            one_card.maxim(d);
            cards[c].push(d);
        }
        for j in 0..=3 {
            cards[j].sort();
            cards[j].reverse();
        }
        let mut vec = vec![];
        let mut vec_maxx = vec![];
        vec.push(0i64);
        vec_maxx.push(0i64);
        vec.push(one_card as i64);
        vec_maxx.push(one_card as i64);
        let mut two_cards = 0;
        let mut two_cards_maxx = 0;
        if !cards[1].is_empty() && !cards[2].is_empty() {
            if cards[1][0] + cards[2][0] > two_cards {
                two_cards = cards[1][0] + cards[2][0];
                two_cards_maxx = max(cards[1][0], cards[2][0]);
            }
        }
        if cards[1].len() >= 2 {
            if cards[1][0] + cards[1][1] > two_cards {
                two_cards = cards[1][0] + cards[1][1];
                two_cards_maxx = max(cards[1][0], cards[1][1]);
            };
        }
        if two_cards == 0 {
            vec.push(i64::MIN);
        } else {
            vec.push(two_cards as i64);
        }
        vec_maxx.push(two_cards_maxx as i64);
        if cards[1].len() >= 3 {
            vec.push((cards[1][0] + cards[1][1] + cards[1][2]) as i64);
            vec_maxx.push(*cards[1].iter().max().unwrap() as i64);
        } else {
            vec.push(i64::MIN);
            vec_maxx.push(0);
        }
        turns.push(vec);
        maxx.push(vec_maxx);
    }

    // each turn at most 3 cost

    // for each turn I get best1, best2, best3
    // now it's dp [turn][cards_played % 10] = damage

    let mut dp = Arr2d::new(n, 10, i64::MIN);

    // check for underflow
    dp[0][0] = turns[0][0];
    dp[0][1] = turns[0][1];
    dp[0][2] = turns[0][2];
    dp[0][3] = turns[0][3];

    for i in 1..n {
        for before_cards in 0..10 {
            let prev_val = dp[i - 1][before_cards];
            if prev_val == i64::MIN {
                continue;
            }
            for now_cards in 0..=3 {
                if turns[i][now_cards] == i64::MIN {
                    continue;
                }
                let total_cards = before_cards + now_cards;
                dp[i][total_cards % 10].maxim(prev_val + turns[i][now_cards]);

                if total_cards >= 10 {
                    // DOUBLE DAMAGE
                    // out.print_line(("DOUBLE DAMAGE", maxx[i][now_cards]));
                    dp[i][total_cards % 10] += maxx[i][now_cards];
                }
            }
        }
    }

    let mut ans = 0;
    for cards in 0..10 {
        ans.maxim(dp[n - 1][cards]);
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
