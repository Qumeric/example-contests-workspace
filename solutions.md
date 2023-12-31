# Solutions

Here are solutions for some tasks in text. The reason it exists because sometimes I am lazy to code.

It's grouped by rating

## 1800

### Little Elephant and Array
https://codeforces.com/problemset/problem/220/B

It screams "SQRT DECOMPOSITION"

We calculate number of occurences for each element in v.

Then for each query we check all number from 1 to sqrt(n) and all numbers that have >= sqrt(n) occurences (there are at most sqrt(n)) of them.

We can check using sorted index arrays (Map<usize, Vec<_>>). Complexity will be O(m*sqrt(n)*log(n)).

### Beautiful Numbers
https://codeforces.com/problemset/problem/300/C?locale=en

This is combinatorics problem. Each number corresponds to a bitmask. Consider equivalence class where masks are split by number of ones. For each class we know if it's excellent or not. Now we just need to calculat sum of amounts of masks of excellent clesses. There is a combinatorics formula for it. TODO: how to calculate this exactly? It's easy and known but how?

## 1900

### Salary Changing
https://codeforces.com/problemset/problem/1251/D

Binary search answer. 

On every step:
1. Sort by l, to ones where l_i > x pay l_i.
2. Sort by r. Take ones with r >= x.
3. Sort taken by l. Untake ones with smallest l_i as much as possible.
4. Pay taken x. Pay remaining l.
5. Compare if we paid more than s or not.

### Print a 1337-string...
https://codeforces.com/problemset/problem/1202/D

First add 3s in format 13...37 until we can't add more.

Then add ones so string will be 13..31..1337. Each one will add 1 to answer and there will be enough space.

### Same Sum Blocks (Hard)
https://codeforces.com/problemset/problem/1141/F2

Calculate map<sum, [(l, r)]> for all possible blocks.

Now for each sum sort blocks by r. Now take lazily (it's always beneficial to take block with smallest r because it leaves largest amount of space).

### Array Stabilization (GCD version)
https://codeforces.com/problemset/problem/1547/F

Use "gcd on subsegments" trick. Then binary search answer.

TODO ???

### MinOr Tree
https://codeforces.com/problemset/problem/1624/G

Start with bitmask of ones. Try to set each bit to 0 from right to left. Check connectivity (e.g DSU)

### Guessing the Greatest (hard version)
???

### Education
https://codeforces.com/problemset/problem/1512/F

There is some position where we stop earning to educate and start earning to win. Just iterate over this position, keep time taken to get there and amount of tugriks and update and with minim to win from there.

Main insight: it's always worth it to educate as soon as possible in position i unless we are not going to educate ever.


### Old Floppy Drive
https://codeforces.com/contest/1490/problem/G

Calculate amount of full circular spin F.

Now convert a to map<remainder F, div F>

Now for every x we need to find element in map. It should have the same remainder F and maximum div <= x div F. It can be done with binsearch or maybe even linearly if we orgainze smart.

Be careful about negative F.

### Moving Points
https://codeforces.com/contest/1311/problem/F

sort points by x
initialize set S
initialize answer as 0

for each point pi with speed vi:
    add (vi) to set S
    count = number of elements in S > vi
    answer += count

then do the same but in reverse. Either go from left or just reverse array and negate values.

