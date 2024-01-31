# List of reason why I get WA:
* Found answer with binary search but predicate is not monotonic (1)
* When checking divisors up to sqrt(n) only taking div but forgetting about n/div or vice versa (1) 
* Used usize when meant to use i64 (1)
* Use mod9 when mod7 was asked for (1)
* TLE: O(n^3) when n = 1000 and t = 1 (1)
* TLE: O(n^2) when n = 1000 but t is 50000 and there is no sum(n) <= 1000 constraint (1)
* Asked GPT to write small simple piece, it misunderstood me (subarray instead of subsequence) and I didn't check (1)
* Almost correct greedy solution but wrong initialization (1)
* Typo: similar things (a vs A, min vs max) (3)
* Misread problem -- definition was not standard (1)
* Wrong solution -- strategy is not optimal (1)
* Return in lambda returns not from outer func but wrom lambda (1)
* Wrong binsearch/iter bounds (used n instead of m, off by 1 etc.) (3)
* Forgot to initializes sum dp state such as (0, 1) (1)
* Did not handle special case when something is 1 or 0 (1).
* When doing bit operations wanted to check for existance of 1 bit and compared it with 1: mask & (1 << x) == 1 (1)
* Traversing grid and adding edges twice even though we do biedges graph (1)
* Did heavy precalc in each test case (1)


# 1919
Extremely bad result :(

Main reason: I wrote basically correct solution but initialized in a not quite right way.

I did empty arrays and had special case for handling empty arrays which was wrong. Instead I should have initialize simply wiht one INFINITE element. 

LESSON: CHECK YOUR CODE FROM THE START! INCLUDING INPUT READING AND INITIALIZATION.
LESSON: TRY TO GET RID OF SPECIAL CASES IF THERE ARE NO MORE IDEAS

Also I think I could write stress test and probably find this issue. I considered it but was naaaah I am going to solve it now. This is fucking wrong.

LESSON: FUCKING WRITE FUCKING STRESS TEST

Also I was too much on "intuition" side. And intuition was correct but I got WA and thought oh I guess it's not correct I will try something else (I did not find something else although it existed).

LESSON: TRY TO PROVE WHY YOUR SOLUTION WHICH FEELS CORRECT IS INCORRECT

TODO: Implement some stress testing approach so it would be easier to do psychologically during contest.

Again I was in situation when something went wrong and I can't find it and I don't know what to do. It's because my training is not correct. I do not look up editorials (almost completely) but I always look up tests. So I have no practice of writing stress and no practice of undrestanding what's going on.

LESSON: DO NOT LOOK UP TESTS WHILE PRACTICING. AT LEAST NO IMMEDIATELY! TAKE AT LEAST 15-20 MINUTES AND TRY TO UNDERSTAND WHAT'S WRONG! IF STRESS IS FEASIBLE DON'T LOOK BEFORE DOING STRESS

Also don't use fucking Cursor. Writing super standard stuff is a part of contest too. It good to be able to do it quickly. Also it's not fair play. Previous reasons are enough but it's also simply not beneficial, more trouble than benefit.
