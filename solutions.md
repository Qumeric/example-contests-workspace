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

### [TODO] Palindrome Game (hard version)
https://codeforces.com/problemset/problem/1527/B2

State: number of 00, number of 01 (or 10), if there is 0 in the middle, if last turn was reverse, (scoreA, scoreB) 

if there are no 01, we can skip turn. We want to skip as much as possible, if we calc max number of skips we can restore answer easily.

If there are only 01s then we know the result, nobody will flip.

If there are only 00s. Then we can flip and then always keep at least 1 so enemy can't flip.
A                B       A         B       A       B       A       B
0000 -> [skip a] 0000 -> 1000   -> 1100 -> 1110 -> 1111 -> ...
                                        -> 2100 -> 2110 -> 2210 -> 2211 -> ...

Need now consider both and also this 0 in the middle... Maybe 0 in the middle can be actually checked by just letting first play it or second play it? Idk though.

### [EASY IMPL] Guessing the Greatest (hard version)
https://codeforces.com/problemset/problem/1486/C2

First ask all. Now ask right to the found (including found) what is 2nd. If the same max is on right, else it on left.

18 questions left. We now just have to binary search.

### [DO NOT SOLVE] Replace by MEX
https://codeforces.com/contest/1375

It's in Global Round 9 which I did not try.

### [TODO] Need for Pink Slips
https://codeforces.com/problemset/problem/1543/C

This is almost solved, some bug in solution.

### Array Stabilization (GCD version)
https://codeforces.com/problemset/problem/1547/F

Use "gcd on subsegments" trick. Then binary search answer.

TODO ???

### Genius's Gambit [EASY IMPL]
https://codeforces.com/problemset/problem/1492/D

We can easily do
1 1111 1000
1 1111 0001

if we need 3 ones

or second can be e.g 

0010
0001

if we need just single one. But what to do with ones? insight: setting two ones on the same pos is same as setting as two zero

so if we need x ones
we ones in first position (if b == 1 then answer is Yes iff k == 0)

then we set a single one on position k+1 on top
and set a single one one on position 1 on the bottom
now we have some other positions but it doesn't matter if they are 1 or 0, they just must be the same

### [TODO] MEX Sequences
https://codeforces.com/problemset/problem/1613/D

Key insight: Numbers can never go down or up much.

### [EASY IMPL] 1D Sokoban
https://codeforces.com/problemset/problem/1494/C

First solve it for negative and positive and add answers. Solving negative can be mirrored so we only need to solve positive.

Just move pointers. One pointer is a special position of last box in a line of boxes .

### K-periodic Garland [EASY IMPL]
https://codeforces.com/problemset/problem/1353/E

Need to reach state where distance between all ones is k (1001 is k = 3) with flipping bits.

The problem is that we can have zero prefix and zero suffix.

If we write for i % k positions in sequence.

Then we will need to solve the following task: have a set of elements. Can remove or insert elements. Need to find minimum amount to make elements be a segment (can be empty).

There will be k such task each with at most n/k elements

To solve this task we keep balances. Go from start, if balance goes below 0 then it's better to remove ones. Do the same from the end

### Inconvenient Pairs
https://codeforces.com/contest/1569/problem/D

If something is on crossroad then we can exclude it.

For each point inconvenient will be ones which are between (exclusive) left and right roads (for somebody on horizontal road, WLOG) but not on the same horizonal road.

/ 2 in the end. Be careful in general.

### [TODO] Asterism (Easy Version)
https://codeforces.com/problemset/problem/1371/E1

Given array. How many orderings such that If x >= ai + i (0 indexed)?

Okay so for each number we know on which positions we can set it.

f(x) number of valid permutations for x. Given prime number p <= n. Find all x such that f(x) doesn't divide p.

Seems like I can just iterate over all x up to 4k? And solve this thing in n or n log(n).


### Skyscrapers (hard version)
https://codeforces.com/problemset/problem/1313/C2

Need segment tree which finds minimum element and position.

Then do divide and conquer. For each segment answer is max(min_elem*(left_len+1) + right_seg, left_seg + min_elem(right_len+1)). That's it?

For some reason WA... Maybe bug idk? Try stress or come up with same but simpler solution

### [TODO] Bandit in a City
https://codeforces.com/problemset/problem/1436/D

Seems easy.


### [TODO] Painting the Array I
https://codeforces.com/problemset/problem/1480/D1

Iterate.

If there are more than 2 numbers in a row it's the same as 2 numbers.

Maybe now stupidiest greedy? Not sure

### Cow and Fields
https://codeforces.com/contest/1307/problem/D

Just need to calc dist_from dist_to with 2 bfses

Then we iterate over special field 1 and try to find special field 2 which will result in longest path
(dist_from[sf1] + 1 + query). Where query is max on subsegment before and subsegment after in segment tree where values are 0 if not special or dist_to[i].

Take min(ans, distance_from_to) because if ans is large enough it may be larger than original distance.

TODO: not 100% sure partly cuz chatgpt says it's wrong but I think it's correct.


### [DO NOT SOLVE] Bouncing Boomerangs
https://codeforces.com/contest/1428/problem/D


### Pairs
TODO but it feels like probably valid xs are a single subsegment so we only need to find first and last.
This is the same, just in mirror.
To do it we need to iterate, get element i as minimum and smallest larger such that it's not in b  > v_i as maximum

### [TODO] Strange Definition
https://codeforces.com/problemset/problem/1471/D

Uncommon primes all has even degree -- adjacent

max number of adjacent -- beauty

things only get merged, so maybe DSU

is this relation transitive?

### Koa and the Beach (Easy) [EASY IMPL]
https://codeforces.com/contest/1384/problem/B1

Lol this is div2 B1...

Just brute force.

## [DO NOT SOLVE] Tree with Maximum Cost
https://codeforces.com/problemset/problem/1092/F

Given tree. Find max sum dist (i, v) * w_v

### GameGame

TODO. I did not read this problem but seems there is an interesting discussion here: https://codeforces.com/blog/entry/80422#comment-668160

consider element with rightmost 1

there are 4 cases:
odd this, odd other 

goes to:
* even this, odd other we get here but with having 1. and player 2 starts. we will end. so player 2 always leaves the last this to us so we lose. unless this is 0



* odd this, even other. -- LOSE so we don't go here

odd this, even other -- WIN. take this and then mirror every turn


even this, odd other
even this, even other

in both last first bit doesn't matter because it will be always the same for both. so we just solve the same without first bit. It nicesly generalizes because 0 is even.

So iterating left to right over bits.
If there is just 1 bit WIN.
If number is odd then skip
Otherwise WIN if len is odd, otherwise LOSE (this one I am not 100% sure)
If skipped all then DRAW

### [TODO] Frog Traveler
https://codeforces.com/problemset/problem/1602/D

### [TODO] Divide and Sum
https://codeforces.com/contest/1445/problem/D

given array 2n. split on 2 subsequences of len n each. sort p in non-decreasing, sort q in non-increasing. cost sum of abs diff.

find sum over all correct partitions mod F.

### Longest Regular Bracket Sequence
https://codeforces.com/problemset/problem/5/C

Calculate balances.
Binary search answer. Balance in the beginning should be balance at the end + 1 and there should be no balance less than beginning - 1 in between (use segtree). 

### [TODO] Zuma
https://codeforces.com/problemset/problem/607/B


### [TODO] AB-string
https://codeforces.com/problemset/problem/1238/D

Fuck it's crucial that it's AB string. Maybe can only treat segments of len 2 and 3 due to it?

### Salary Changing
https://codeforces.com/problemset/problem/1251/D

Binary search answer. 

On every step:
1. Sort by l, to ones where l_i > x pay l_i.
2. Sort by r. Take ones with r >= x.
3. Sort taken by l. Untake ones with smallest l_i as much as possible.
4. Pay taken x. Pay remaining l.
5. Compare if we paid more than s or not.

### Old Floppy Drive
https://codeforces.com/contest/1490/problem/G

Calculate amount of full circular spin F.

Now convert a to map<remainder F, div F>

Now for every x we need to find element in map. It should have the same remainder F and maximum div <= x div F. It can be done with binsearch or maybe even linearly if we orgainze smart.

Be careful about negative F.

### Gargari and Permutations
https://codeforces.com/problemset/problem/463/D

Each integer represents a point in 5d space (or 4d, 3d, 2d but 5d WLOG). Edge u-v exists iff all coordinates of v are larger than corresponding coordinates of u.

Now we just need to find longest path on DAG.

Would be nice to solve and add longest path, topsort etc. to the lib.

### Beautiful Array
We can remove 0s first. 

Then seems like I have to split array on pos - neg parts (each pos starts with neg and end with neg). then just convert each such part to sum. 

when! {
   x == 0 => output max
   x > 0 => output (max * x)
   x < 0 => output max over (pos_i + -neg_(i+1) + pos_(i+2))
 }

 TODO: not completely sure but sounds right?


### [TODO] Nezzar and Binary String
https://codeforces.com/problemset/problem/1477/B

queries on string. we can change less than half characters in segment *after* inspect. need to say if we can always have 0 or 1 segments and also make string to be matching desirsed at the end. 

seems like we want to number each introspection and at each change we want
1. change stuff so it matches f if there will be no more introspections
2. change stuff so it matches later introspections?

idk.

### Johnny and Grandmaster
https://codeforces.com/problemset/problem/1361/B

There are degrees of p. Need to split in two sets such that difference is minimal. Print % MOD

handle p = 1 seaparately, just out n % 2

if max degree > next by 20+ then max will be separate (usually it's less than 5 but for p=2)

Write number for each (we don't need map, array is fine.

Iterate from largest to smallest. If even, skip.

Otherwise there 1 on large, try to build. Dumb and [probably] working way is to iterate over size from largest to smallest (20..1) and try to build to match. If builded then remove stuff and continue. If not then put large in one bin, everything else in another.

Actually maybe just try to "squish" on copy as much as possible. Now iterate. If even, skip. If odd, check if it's even in squished. If it is even then skip but remember that we need to collect it, collect while it collects. Then continue.





### [TODO] Ehab and the Expected XOR Problem
https://codeforces.com/problemset/problem/1174/D

honestly idk, seems kind of complicated but prob not, need some key

### [DO NOT SOLVE] Playlist
https://codeforces.com/problemset/problem/1484/D

### Perform Easily
https://codeforces.com/problemset/problem/1413/C

Sort strings and notes.

Binary search answer ans.

There is some string which plays the lowest note. Iterate over it. It will set lowest fret. and range will start from some k

We definitely will have proper because we iterate over all. It will be the lowest fret because there is no 

For each starting string iterate over all other string and try to play each on any string using any fret from k to k + ans.

a = [1, 4]

notes s [1, 3]


frets will be 


### [TODO] Flood Fill
https://codeforces.com/problemset/problem/1114/D

Note that we need to pick starting square and only change this component...


### Equalize the Remainders [EASY IMPL]
https://codeforces.com/problemset/problem/999/D

Ones with cr <= n/m just stay. Others move to the closest cr which is lower. Just store crs which are low in set, find and update. 

Just go iteratively.

### Same Sum Blocks (Hard)
https://codeforces.com/problemset/problem/1141/F2

Calculate map<sum, [(l, r)]> for all possible blocks.

Now for each sum sort blocks by r. Now take lazily (it's always beneficial to take block with smallest r because it leaves largest amount of space).

### Fish
https://codeforces.com/problemset/problem/16/E

DP. State: [alive fish, fishes left]
There is <= 18 * 2^18 = 4718592 ~ 5e6 states
Step: all masks with +1 fish (18) with the same alive fish. so 1e8
Answer is dp_i[0]
Basis dp_i[((1 << n) - 1) ^ (1 << i)] = 1

Seems like need to calculate for all fishes at each amount of alive fishes from large to small?


## 2000

## [TODO] Two Divisors
https://codeforces.com/problemset/problem/1366/D

## [DO NOT SOLVE] Keep the Average High
https://codeforces.com/problemset/problem/1616/D

### New Year Concert
https://codeforces.com/problemset/problem/1632/D

gcd only goes down while length only goes up.

Do gcd trick from right to left. Now iterate. For each try to find matching len (gcd only goes down when len goes up, so at most single matching). If found then kill it (you can remove by infinitely large number which has no common divs) and ans++. Don't forget to include largest killed when checking for matching len in later iterations.

### Nastia and a Hidden Permutation
https://codeforces.com/problemset/problem/1521/C

First, ask 2 (1 2) 1 which will be min(pi, max(2, pj)) [after (1, 2) (3, 4) etc.]

Hopefully we found pair with 1. If no (got 2 at most) we don't know if min is 2 or 1.
ask reversed, if it returns 1 then min is 1. if it returns 2  min is 2.

Now we have pair with 1, need to locate where it is out of two. Many ways to do so.
Now we can just ask 1 (pos1 i) n-1. which will be max(1, min(n, pi)) = pi


### [TODO] Tree Shuffling
https://codeforces.com/problemset/problem/1363/E


### [EASY IMPL] Masha-Forgetful
https://codeforces.com/problemset/problem/1624/E
Just split on segments of len 2 and 3. Then DP. 

### [TODO] Shuffle
https://codeforces.com/problemset/problem/1622/D

Some kind of dp prob.

### [TODO] Expression Evaluation Error
https://codeforces.com/problemset/problem/1567/D

I almost solved it but it's slightly tricky. See code and continue from there, idea is obvious but implementation is not quite.

### The Strongest Build
https://codeforces.com/problemset/problem/1574/D

no more than 10 slots

if we take last 100 for each surely that much can't be banned (for n = 10). Actually even 10 can't be banned.

### GCD and MST
https://codeforces.com/problemset/problem/1513/D

gcd only goes down when extening segment just like min. gcd can't be more than min.

if there is 1 in array we can make just all edges 1

one way to build mst is just adding smallest edges.

we can just use gcd trick right? we will have nlog values

but only n minimums. so we iterate over minimums. now we know for each minimum array of where it can add edges (both right and left, do it twice). So we don't even need DSU, segments of array get connected

### [TODO] Modular Stability
https://codeforces.com/problemset/problem/1359/E

First step: implement bruteforce solver and see what answers in gives for 7 3, 8 3, 7 4, 6 4 etc.

### [TODO] K-beautiful Strings
https://codeforces.com/problemset/problem/1493/C

Find string >= s where num(letter) % k == 0 for all letters

Feels like binary search.


### Directing Edges
https://codeforces.com/problemset/problem/1385/E

First leave only directed edges. Find connected components, check them for cycles in proccess (print NO if any).
Find topsort for each component. Now iterate over undirected edges. If (u, v) in the same connected component, orient using topsort order.

Now the task converged to orient all edges of bidirected graph. I think it can be done with dfs where you orient depening on what you found in the end. If you found leaf, orient forward, otherwise orient backwards.

TODO: how to implement in the simplest possible way?

### [EASY IMPL] Fixed Points
https://codeforces.com/problemset/problem/1551/E

Simple DP. I just need to store for each length maximum amount of sequence and add iteratively

basis: 0
step: dp[i][len+1] = dp[i-1][len] + (a[i] == len+1)

### Jzzhu and Cities
https://codeforces.com/problemset/problem/449/B

It seems that (not a figure of speach, I am not completely sure) that we can just do dijkstra from multiple points: root and all points with roads.

The only potential problem is that it's not very clear what to select if distance is equal. However, if we look closely we can notice that if it's equal between path from root and bridge then we take only root and if it's equal from two bridges, we take both bridges.

### Number of Simple Paths
https://codeforces.com/problemset/problem/1454/E

We have one circle and some trees growing on it (cacti?) the  answer is number of paths multiplied by two minus number of paths in each growing tree.

The only question how to implement most elegantly.

### [TODO] Cut and Stick
https://codeforces.com/problemset/problem/1514/D

Given segment, partition it into minimal number of subsequences such that no value occurs > ceil(len/2) times

Need to just know most popular value? And then do binary search for longest possible from each i? 

### Choosing Flowers
Main insight: answer looks like many flowers of some kind and at most 1 flower of each other kind.

Has set sorted by a.
Then iterate over flower which we take >= 1 times. Remove from set (and return after handling this iter). Find all flowers with a_i > b_x. Take them (or as much as possible). If space remained fill wiht b_x.


### Orac and Game of Life
https://codeforces.com/problemset/problem/1349/C

We can observe that each cell either flickers or stays the same. And amount of points which stay the same goes down each step, at most 2000 (+ eps?) steps. 

For each point we need to find where is stops to flicker and what color it had. This can be done for each "shape" with bfs from multisource. Actually seems like we can just bfs from all points that change color? If there are no such points then things are constant.

### [TODO] Triangular Paths
https://codeforces.com/problemset/problem/1506/F

Need to sort points by layer first and traverse in this order.

Now between two points we consider evennes of start and end and amount to go.

We can have function solve (isStartEven, stepsDown, stepsDownRight). 

### Propagating Tree
https://codeforces.com/problemset/problem/383/C

Here we just need to build segment tree for each depth level.

### Prefixes and Suffixes
https://codeforces.com/problemset/problem/432/D


I can just calculate Z-function.
And now I know which prefixes match suffixes.

Save those matching things.

Now I calculate prefix-function. And now for each suffix as substring I know in how many prefixes it includes.

Just add to segment tree or something?

TODO: not quite right, for example if AB matches A matches as well and I will add A two times but it should be added only once

but if AA matches, A should be added twice! So maybe we need to somehow call something for matching prefix also to do 5+

### [TODO] Messages
https://codeforces.com/problemset/problem/1612/E

Wants student i to read mi

each student will read at most ki

students read random ki <= 20

maximise number of student who read msg


### [EASY IMPL] XOR Gun
https://codeforces.com/problemset/problem/1457/D

If there are 3 elements with the same highest bit, answer is 1. Can check with xoring 2 elements and checking if it's smaller than prev.

Now we have at most 60 elements. Brute (two conseq segments).

### [TODO] XOR Inverse
1416/C

given array, xor each with x such that number of inversions is minimised. 

This sound like divide and conquer (in radix way).

### [TODO] XOR on Segment
242/E

Can solve same problem for 1 bit length but still not very clear?



### [TODO] The Contest
https://codeforces.com/problemset/problem/1257/E

Have permutation. It is was split on 3 parts. 

Op: move one element from one set to another.

Find min amount of ops such that first is prefix, second is mid and last is suffix (can be empty).

First split into two part by min amount of moves.

Now iterate over prefix len. If elem was moved from 1 then ans--, if elem was moved but not from 1 then nothing, if elem was not moved then ans++.

Is it correct? Dunno... Maybe...

### Shortest Path
https://codeforces.com/problemset/problem/59/E

Triples are hard but pairs are easier. It's pair of edges. Do modified BFS where we kind of go over edges (or store prev and current vertex). There are < n^2 edges so it will be fast enough.

### [DO NOT SOLVE] Bottom-Tier Reversals
https://codeforces.com/contest/1561

## 2100

### [TODO] The Winter Hike 
https://codeforces.com/contest/1621/problem/D

Maybe in any solution we should never move in more than 2 different directions? Then 2d dp for each (2 out of 4) = 6

### [TODO] Errich-Tac-Toe (Easy Version)
https://codeforces.com/contest/1450/problem/C1

Maybe just greedily find positions such that they are both in hor and ver and set them to 0? And then everything is either only in hor or only vert, so I can just set 1 per 3.

Or maybe slightly differnt greedy: caluclate amount of hor and ver for each pos, now do in order of priority, don't forget to update all neighbours. There are not a lot of neighbours so it's not costly.

### [TODO DIV3] Gift Set
https://codeforces.com/problemset/problem/1538/G

### Martial Arts Tournament
https://codeforces.com/problemset/problem/1626/D

Given sorted list, need to split it into tree parts such that adding smallest amount would make all of them degrees of 2.

We can iterate over one segment, now we need to iterate over all possible degrees of two and make second segment as long as possible but not more than current degree. If we squit we will notice that this way is correct. To actually check we only need to store boundaries of consequitive same, no need for binsearch even.

### [DIV3] Nearest Beautiful Number (hard version)
https://codeforces.com/problemset/problem/1560/F2

Maybe need precalc for each k.

For k = 10, 9, 8, 7, 6, 5 can do naively

Iterate over prefix. XXY....

If Y gets to Z=Y+1 then try all XXZ0000, XXZ1111 etc. (XX can change as well, doesn't matter)

Also try all XXY0000, XXY1111 (but number of steps can't be negative)

### [DIV3] How Many Paths?
https://codeforces.com/problemset/problem/1547/G

First find 0, remove them.

Now find strongly connected components. Now mark nodes with loops. Now find SCC. Mark nodes in SCC with size at least 2. Now do bfs from all marked nodes. Mark everything reached with -1, remove.

Now we only need to differ between 1 and 2, this is easy. Can be done with bfs prob?

### The Number of Pairs
https://codeforces.com/problemset/problem/1499/D

lcm/gcd is divisors which only one number has

so we can do c * singleDiv(a, b) = x/gcd(a, b) + d

so singleDiv should is (x/gcd(a, b) + d) / c

so gcd(a, b) should be divisor of x

so let's look at all divisors of x. 

then we will have singleDiv = CONST

we just need to calculate amount of possible splits which is just mult of degrees or smth?

TODO: details

### Max Median
https://codeforces.com/contest/1486/problem/D

Binary search answer. For each candidate convert array to binary. Now we need to find any segment of len at least k with at lest k/2 1 elements.

Now we can write O(n^2) solution here. Iterate over start and then start from j=i+k to end and find the best. "Bestness" will be determined by value and position of j so we can use fenwick/segment tree to find it in log time. O(n log^2 n) easily fits in 1e8 (and it's 2 seconds even).

### Omkar and Circle
https://codeforces.com/problemset/problem/1372/D

Given odd number circle. Pick number, replace with sum adjacent and delete adjacent. Find maximum.

Most elements we keep but some delete (in sum).

ttttHHHHH
tttxHHHHH
ttHHHHH
txHHHHH
HHHHH

we can think of it as removing any segment and merging values on sides

maybe like greedy would work if we knew how to implement?

..AxxxB..CxxxD...
..E..F...

abcde
(a+c)de

option 1: (d+e) (removed 1, 2, 3) -> worse than removed (1, 3)
option 2: a+c+e (removed 2, 4)

so basically we need to remove n//2 elements but can't delete 2 in a row. This is dp with 2 different starts. State is toRem, subtracted.

### [TODO] Binary Subsequence Rotation
https://codeforces.com/problemset/problem/1370/E

Given two binary strings, can clokwise rotate subseqence by 1. Minimise number of ops so A=B. Can always do len/2. If number of 1s do not match then can't at all.

### [TODO] Boring Segments
https://codeforces.com/problemset/problem/1555/E

given weighted segments, need to select subset of min weight such that can travel from 1 to m. So basically cover 1-m?

Cost is max value - minimum value!

### [DO NOT SOLVE] Permutation Shift
https://codeforces.com/problemset/problem/1553/E

### Red-Black Number
https://codeforces.com/problemset/problem/1593/F
each digit adds to reminder
there are only 1600 possible pairs of reminders

so from 1600 pairs (some may be present multiple times) we need to fill.

so iterate over x and have dp[i][taken][pair] 

to recalculate we iterate through all pairs and all taken and try to add. there are 40 steps, 20 taken can be at most and 1600 pairs. 

O(n^4) easily fits. Tbh even O(n^5) may fit if we do it carefully)


### Ant colony
https://codeforces.com/contest/474/problem/F

Need to answer queries "how many elements of subsegment divide all other elements"

If it divides all then it divides gcd. But it can't be smaller than gcd so it is gcd. Using adamant's gcd trick we can quickly find gcd for any subsegment. We can also find minimum using segment tree (or whatever). If minimum does not equal to gcd then answer is r-l. If it does equal then answer is r-l-number_of_minimums. Finding number of minimums is a standard execrcies. I added it to the lib.

### Minimum Spanning Tree for Each Edge
https://codeforces.com/contest/609/problem/E

Shall we just build and MST and then for each edge if it's not in MST answer queries "maximum edge on path from u to v in MST"?

MST building is easy and for queries I have HLD implementation.

### Olya and Energy Drinks
https://codeforces.com/problemset/problem/877/D

it seems like there is no problem to traverse everything in bfs because we will go to each square once?

We need to be able to find next wall from each square in each direction. It's easy to do with 2*n + 2*m precalc (one for each direction)

Now we do bfs but want to ensure that we don't go to same place twice. For this we can use sets and binary search. Store row and columns, find reachable cells [max(wall_l+1, pos-k); min(wall_r-1+pos+k)]. Remove them from set and go to those cells (bfs).

### Round Subset
https://codeforces.com/problemset/problem/837/D

roundness only depends on number of 2 divisors and 5 divisors. So we replace each number by pair (d2, d5) where d2 is at most 60 and d5 is at most 30.

So it total 1800 pairs btw

Now we need select k pairs such that min(d2, d5) is maximum

Sounds like dp extremely similar to red-black number problem

### New Year Tree
https://codeforces.com/contest/620/problem/E
Seems like we just need 60 things which handle subtrees requests? Like centroid decomposition.

When query just answer.
When update in one thing we set to 1. In other things we set to 0.

it will be mc log n which is 400_000 * 60 * 20 ~ 500_000 * 1_000 = 5e8

### Valid Sets
https://codeforces.com/problemset/problem/486/D

Iterate from largest value to smallest value (store (value, index) pairs and sort)

Now current vertex will be max. Find all reachable and large enough (but not included already handled verticies), calc tree amount of possible subtrees starting from root in each. Now multiply numbers for each tree (they should include set as well).

Repeat. n^2 only

### Blood Cousins
https://codeforces.com/contest/208/problem/E

Amount of p-cousins is amount of p-children of p-ancestor of v_i. So let's replace it to this.

It's offline so we can handle in any order. we dfs tree and store ancestors at each dp and merge using smallest-to-largest tecnique (aka sack).

### A and B and Lecture Rooms
https://codeforces.com/problemset/problem/519/E

We have tree. Given queries (A, B) need to find amount of nodes with same distance to A and B.

Find LCA. Then answer is sum of trees from the middle of path from A to B. (need to go up half way). Don't include trees of A and trees of B. If LCA itself is in the middle then also include "up tree".

### Guide
https://codeforces.com/problemset/problem/1510/G

We have a tree (at most 100 verticies). Find shortest path from root which visits k different cities.
Stupid way would be 2k lenthg because we never go over edge more than twice. 

Maybe can do divide and conquer? 

TODO

### Zero Remainder Sum
https://codeforces.com/problemset/problem/1433/F

We can do dp for each row to find maximum sum for each reminder. Then we need to combine reminders somehow? TODO: not clear

dp will have rem taken state for each row.

Now do another similar dp by columnt, but here only have rem (we have to take all)


### Ciel the Commander
Isn't it centroid decomposition?

TODO

### Tree Painting
https://codeforces.com/problemset/problem/1187/E

It doesn't matter how you play -- if you selected vertex the game is split on 1+ games on parts. Each part has predetermined start too. 
So it only matter where you start.

Ahh unclear TODO. Kind of reminds heavy light?? Maybe need to think how answer changes when I move start to adjacent? But depth is also important...

### Maximum Value
https://codeforces.com/problemset/problem/484/B

Can remove same values

If we have <1e4 values we can try all. Prob even for <2e4?

So we have a lot of different values. Maybe binary search things like smallest > x/2, smallest > x/3 etc..

Answer have to be quite large right? If we will look up to sqrt it will be n * sqrt(n) * log n. Althought it's a bit much, more than 1e9.

Answer should be pretty large!

### [TODO] Checkpoints
https://codeforces.com/problemset/problem/1453/D



### Cut
https://codeforces.com/contest/1516/problem/D

If two elements have common divisor then it's a no. We can find all divisors for each element in n^1.5

Now for each element we need to find next element which shares prime.
We can do it by saving position of all divisor in map<prime, set<position>>

Now just find next position for each element, it will be n log^2.

Now for each position we know what last element we can take. 

But naive algo would be still too slow? Because there could be tons of segments in each query? Like for 1 1 1 1 1 1 array?

Now it split at bunch of chains it seems (paths staring at 0, path starting from first unreachable from 0 etc.) ? And chains are arrays. We can map each position to (chain, pos in chain) and do binary search.

Woah kind of a lot of stuff. But everything is pretty easy to implement? Prob can simplify somewhere.

### New Years' Puzzle
https://codeforces.com/problemset/problem/1472/F

It's dp. Only last column matters. You just jump through ms keeping what last column you had. What if n is large? Jump through ms when disance between them is say > 5. You will be able to have any column of the same parity.

Notice that you are considering 2x2 square, last column and new column.

### Pictures with Kittens (hard version)
https://codeforces.com/contest/1077/problem/F2

Need to maximise sum of x elements but do not have gaps of size at least k.

It's just dp with segment tree. Need do have segemnt tree for each amount of kitten and go from the end. n^2 log n

Actually it's too slow. Need to somehow remove log TODO

### Tree with Small Distances
https://codeforces.com/problemset/problem/1029/E

Given tree add minimum number of edges so everything is reachable from 1 with path of len <= 2.

So the task is cover tree with minimum amount of stars such that one of stars in in 0.

Key observation: it never makes sense to cover leafs, it's always better to cover parent. So do bfs. Take all parent for each leaf. Remove all leafs and parents.

store number of children for each and a queue for leaves. also save parents.

take leave, get parent. Remove all edges from parent. recalculate amount of children and add new leaves in queue.

### Rarity and New Dress
https://codeforces.com/problemset/problem/1393/D

Need to "rotate" this thing 45 degree. Now we are looking for squares, need find largest for each top left corner.

Finding can be done with binary search. Checking can be done without data structures, just sum and use inclusion-exclusion. Maybe would be convenient to map a - > 1 -> 2, b -> 2 -> 4, c -> 3 -> 8 etc so sum would never be ambiguous.



### Binary Median [EASY IMPL]
https://codeforces.com/contest/1360/problem/H

Remove at most 100 strings from set of all strings of len at most 60

Print element with index (k-1)/2

k is 2^m - n

Binary search answer? For each element position is element - removedSmaller. If binary serach found removed element, then just take first larger not removed.

### Array Partition
https://codeforces.com/problemset/problem/1454/F

TODO

### Number of Components
https://codeforces.com/problemset/problem/1151/E

Note: stress test is easy

We will do stuff in order

there is x y z subsegment. if x > y && y < z then adding y will increase number

if x > y && y > z || x < y && y < z then adding y will not change number

if x < y && y > z then adding y will decrease number.


we can calculate like this and get sum for (l, r) it seems

then move left boundary. Need only recalculate elems adjacent to removed, it's the same as setting removed to INF

TODO: details


### Mahmoud and a xor trip
https://codeforces.com/problemset/problem/766/E

Tree. distance is xor. find sum of all paths.

Calculate for every bit separately. There will be 20 trees. 

Now we can dfs and keep for each subtree amount of 0-paths and amount of 1-paths. Now for current vertex we calc sum of paths going trough this vertex.

How to calc with not n^2? If node is 1 I am interested it ones * zeros. If node is 0 then I match ones between themselves and zeros between themselves. Can be done in linear fashion.

### SUM AND REPLACE
https://codeforces.com/problemset/problem/920/F

Key insight: there are not many steps. So we can store at most 10 values for each starting point.

Now when we REPLACE we just need to have set (maybe treapset, implicit key) and find lower bound, then iterate through whole subarray and replace. Why it is not slow? Because 0 can be removed. Thus each position will be handled at most 10 times (log3?).

It is O(n*sqrt(max(a)) + m*log(n)*log(max(a))) = 3 * 10^8 + 10^8.

Maybe can be faster if we precalc primes and use only them? At least in practice.

### Longest Array Deconstruction
https://codeforces.com/contest/1575/problem/L

Fuck this is not easy? Idea I had:

Need to have segment tree (MaxMax).

If value is a_i and it's <= i. We do tree[a_i] = max(tree[a_i], tree[0..a_1] + 1)

Test could be: 1 0 3 2 3

Doesn't work on: 2 1 4 2 5. Because will asy 3 because before 5 there is 2 but it requires removal. Maybe need assign to tree and have indexes of i and not a_i.


## 2200
### Erase and Extend (Hard Version)
https://codeforces.com/problemset/problem/1537/E2

### [CLASSIC] Powerful array
https://codeforces.com/problemset/problem/86/D

### Powerful Ksenia
https://codeforces.com/problemset/problem/1438/D

### Cleaning
https://codeforces.com/problemset/problem/1474/D

### Non-Decreasing Dilemma
https://codeforces.com/problemset/problem/1567/E

### [DIV 3] Guess the K-th Zero (Hard version)
https://codeforces.com/problemset/problem/1520/F2

### [DO NOT SOLVE] Lexicoraphically Small Enough
https://codeforces.com/problemset/problem/1616/E

### Gold Transfer
https://codeforces.com/problemset/problem/1535/E

### Kill Anton
https://codeforces.com/problemset/problem/1526/D

### [DO NOT SOLVE] Telepanting
https://codeforces.com/problemset/problem/1552/F

### Figure Fixing
https://codeforces.com/problemset/problem/1537/F

This is prob hard.

### Sum of Paths
https://codeforces.com/problemset/problem/1467/D

### Kate and imperfection
https://codeforces.com/problemset/problem/1333/F

### Proirity Queue
https://codeforces.com/problemset/problem/1542/D

Some kind of dp?

### [DO NOT SOLVE] Carrots for Rabbits
https://codeforces.com/contest/1428/problem/E

### Clear the Multiset
https://codeforces.com/problemset/problem/1400/E

### [DO NOT SOLVE] Phoenix and Computers
https://codeforces.com/problemset/problem/1515/E

### Calendar Ambiguite
https://codeforces.com/problemset/problem/1389/E

### Ezzat and Grid
https://codeforces.com/problemset/problem/1557/D

### Discrete Centrifugal Jumps
https://codeforces.com/problemset/problem/1407/D

If we build graph we maybe win? But too many edges maybe?
Can always jump to next.

If we find next >= than current and next <= than current we limit jumps for each category.
But seems we can only jump there? Not necessarily... Could be 4 1 1 1 3 4 5

### RPD and Rap Sheet (Hard Version)
https://codeforces.com/problemset/problem/1543/D2

### Not Escaping
https://codeforces.com/problemset/problem/1627/E

### Sum of Digits
https://codeforces.com/problemset/problem/1373/E

### [DIV 3] Decreasing Heights
https://codeforces.com/problemset/problem/1353/F

### Bananas in a Microwave
https://codeforces.com/problemset/problem/1498/D

### Mocha and Stars
https://codeforces.com/problemset/problem/1559/E

###  Equillibrium
https://codeforces.com/problemset/problem/1556/E

### Paired Payment
https://codeforces.com/problemset/problem/1486/E

### Make It Increasing 
https://codeforces.com/problemset/problem/1437/E

### Two Hundred Twenty One (hard version)
https://codeforces.com/problemset/problem/1562/D2

### [DO NOT SOLVE] X(or)-mas Tree
https://codeforces.com/problemset/problem/1615/D

### [DIV 3] Make k Equal
https://codeforces.com/problemset/problem/1328/F

## Other

### Number of Parallelorgrams
https://codeforces.com/contest/660/problem/D

Need to iterate over pairs. Find angle. For each angle x*(x-1)/2.

### XOR construction
https://codeforces.com/contest/1895/problem/D

Link on discussion: https://codeforces.com/blog/entry/122034?#comment-1083134

### Gadgets for dollars and pounds
https://codeforces.com/contest/609/problem/D

Seems trivial? Price is just min(a*d, b*d). Now binary search answer, for each answer add to arr and sort, take k cheapest, n log^2 n.

### Monitor (1900)
https://codeforces.com/contest/846/problem/D

Sort broken pixel by time.

Now binary search by pixel position (by t).

On each query we need to check if there is a square. There are many ways to do it. But for example just iterate over all pixels and check in 2d fenwick tree.

This will be nm log n log m log (nm) -- should be fine prob, 3e8 worst case.

What is not nice is that we need to restore pixels in binary search but ok

### Imbalanced Array (1900)
https://codeforces.com/contest/817/problem/D

Add to the end one by one.

Find pos of last smaller and last larger. Left is l, right is r. For [r, last] it's clear, just need abs(sum - last * l). Maybe need to assign on segment (i.e. do max).

TODO but seems that it would work, just fill details

### [TODO] Chocolate Bar (2000)
https://codeforces.com/contest/598/problem/E

k is small. Something something bitmask dp? Or maybe we never need more than 3-4 breaks? Then there will be only 60^(3 or 4) ways.

### Credit Card (1900)

First, it only makes sense to deposit before things are going down. It makes sense to deposit as much us possible but without having limit at some point.

So before - transaction or on checking day we see if we need deposit. If yes then we see what is maximum amount we can deposit by storing balances and using segment tree on postfix.

### Pair Of Lines (2000)
https://codeforces.com/contest/961/problem/D

We can try randomly? There is a line where 50%+ lie. So let's take say 1.000.000 random pairs and draw lines. Find the most popular. It will be ours. Now remove points on this line. Check if remaining are also on line.

### [TODO] Xor-sequences (2000)
https://codeforces.com/contest/691/problem/E

### [TODO] Concatenated Multiples
https://codeforces.com/contest/1029/problem/D

