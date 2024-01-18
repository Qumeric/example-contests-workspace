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

### Shichikuji and Power Grid
https://codeforces.com/problemset/problem/1245/D

This is just MST problem. Just add "POWER" node and make edges to it. All other nodes will have edges as described (variant of manhattan distance).


### 1D Sokoban
https://codeforces.com/problemset/problem/1494/C

First solve it for negative and positive and add answers. Solving negative can be mirrored so we only need to solve positive.

Just move pointers. One pointer is a special position of last box in a line of boxes .

TODO but this is basically implementation problem?

### 0-1 MST [EASY IMPL]
https://codeforces.com/problemset/problem/1242/B

If we find connected components by 0 edges then the answer will be number of connected components -1.

We can use DSU but how to merge? 

If graph has <= 10000 nodes then we have 1e8 edges and can do properly.

Otherwise take 100 verticies with most 1 edges. Do properly for them. Remaining verticies can't have more than 100 1-edges. Seems like all of them should be connected?

Actually just take as much largest nodes as possible. We are going to do n operations for each node so we can take something like max(n, 1e8/n). Maybe can try something smaller like 5e7/n to be safer in terms of time.

## Tree with Maximum Cost
https://codeforces.com/problemset/problem/1092/F

Given tree. Find max sum dist (i, v) * w_v

### Koa and the Beach (Easy) [EASY IMPL]
https://codeforces.com/contest/1384/problem/B1

Lol this is div2 B1...

Just brute force.

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

### Shichikuji and Power Grid
https://codeforces.com/contest/1245/problem/D

### K-periodic Garland [EASY IMPL]
https://codeforces.com/problemset/problem/1353/E

Need to reach state where distance between all ones is k (1001 is k = 3) with flipping bits.

The problem is that we can have zero prefix and zero suffix.

If we write for i % k positions in sequence.

Then we will need to solve the following task: have a set of elements. Can remove or insert elements. Need to find minimum amount to make elements be a segment (can be empty).

There will be k such task each with at most n/k elements

To solve this task we keep balances. Go from start, if balance goes below 0 then it's better to remove ones. Do the same from the end

### Salary Changing
https://codeforces.com/problemset/problem/1251/D

Binary search answer. 

On every step:
1. Sort by l, to ones where l_i > x pay l_i.
2. Sort by r. Take ones with r >= x.
3. Sort taken by l. Untake ones with smallest l_i as much as possible.
4. Pay taken x. Pay remaining l.
5. Compare if we paid more than s or not.

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
https://codeforces.com/problemset/problem/1486/C2

First ask all. Now ask right to the found (including found) what is 2nd. If the same max is on right, else it on left.

18 questions left. We now just have to binary search.

### Old Floppy Drive
https://codeforces.com/contest/1490/problem/G

Calculate amount of full circular spin F.

Now convert a to map<remainder F, div F>

Now for every x we need to find element in map. It should have the same remainder F and maximum div <= x div F. It can be done with binsearch or maybe even linearly if we orgainze smart.

Be careful about negative F.

### Equalize the Remainders
https://codeforces.com/problemset/problem/999/D

Ones with cr <= n/m just stay. Others move to the closest cr which is lower. Just store crs which are low in set, find and update. 

Just go iteratively.

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

 ### Pairs
 TODO but it feels like probably valid xs are a single subsegment so we only need to find first and last.
 This is the same, just in mirror.
 To do it we need to iterate, get element i as minimum and smallest larger such that it's not in b  > v_i as maximum

 ### Fish
 https://codeforces.com/problemset/problem/16/E

 DP. State: [alive fish, fishes left]
There is <= 18 * 2^18 = 4718592 ~ 5e6 states
Step: all masks with +1 fish (18) with the same alive fish. so 1e8
Answer is dp_i[0]
Basis dp_i[((1 << n) - 1) ^ (1 << i)] = 1

Seems like need to calculate for all fishes at each amount of alive fishes from large to small?

### Cow and Fields
https://codeforces.com/contest/1307/problem/D

Just need to calc dist_from dist_to with 2 bfses

Then we iterate over special field 1 and try to find special field 2 which will result in longest path
(dist_from[sf1] + 1 + query). Where query is max on subsegment before and subsegment after in segment tree where values are 0 if not special or dist_to[i].

Take min(ans, distance_from_to) because if ans is large enough it may be larger than original distance.

TODO: not 100% sure partly cuz chatgpt says it's wrong but I think it's correct.

### Skyscrapers (hard version)
https://codeforces.com/problemset/problem/1313/C2

Need segment tree which finds minimum element and position.

Then do divide and conquer. For each segment answer is max(min_elem*(left_len+1) + right_seg, left_seg + min_elem(right_len+1)). That's it?

For some reason WA... Maybe bug idk? Try stress or come up with same but simpler solution

### String Reversal
https://codeforces.com/problemset/problem/1430/E

Just match first a in original with first a in reversed etc. Now we just know where each symbol should move. Now need to calculate how long it will take. It's segment tree maybe?

For each position write how many it should move (+ if backward or - if forward). Now take one which needs to go to 0. Add value to answer, add +1 to all on left. Now take one which needs to go to 1...

aaaza to azaaa
0 -1 0 2 0

ans +2
x 0 0 x 0

### Ehab and the Expected XOR Problem
https://codeforces.com/problemset/problem/1174/D

TODO

### AB-string
https://codeforces.com/problemset/problem/1238/D

Key insight: consider strings only of length 2 and 3.

TODO

### Flood Fill

It seems that greedy here is just selecting 2 closest elements of the same color? Obviously at first replace all 3 3 3 .. to just 3.

### Inconvenient Pairs
https://codeforces.com/contest/1569/problem/D

If something is on crossroad then we can exclude it.

For each point inconvenient will be ones which are between (exclusive) left and right roads (for somebody on horizontal road, WLOG) but not on the same horizonal road.

/ 2 in the end. Be careful in general.

## 2000

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

### Masha-Forgetful
https://codeforces.com/problemset/problem/1624/E
Just split on segments of len 2 and 3. Then DP. 

### Directing Edges
https://codeforces.com/problemset/problem/1385/E

First leave only directed edges. Find connected components, check them for cycles in proccess (print NO if any).
Find topsort for each component. Now iterate over undirected edges. If (u, v) in the same connected component, orient using topsort order.

Now the task converged to orient all edges of bidirected graph. I think it can be done with dfs where you orient depening on what you found in the end. If you found leaf, orient forward, otherwise orient backwards.

TODO: how to implement in the simplest possible way?

### Ice Cave
https://codeforces.com/problemset/problem/540/C

If there is no path:
   NO
Otherwise:
   If you are already staying on proper cell:
      if you can't move:
         NO
      otherwise:
         YES # go one step and back
   Otherwise:
      if end point have at least 2 adjacent empty squares:
         YES
      otherwise:
         NO
   
### Fixed Points
https://codeforces.com/problemset/problem/1551/E

Simple DP. I just need to store for each length maximum amount of sequence and add iteratively

basis: 0
step: dp[i][len+1] = dp[i-1][len] + (a[i] == len+1)

### Jzzhu and Cities
https://codeforces.com/problemset/problem/449/B

It seems that (not a figure of speach, I am not completely sure) that we can just do dijkstra from multiple points: root and all points with roads.

The only potential problem is that it's not very clear what to select if distance is equal. However, if we loose closely we can notice that if it's equal between path from root and bridge then we take only root and if it's equal from two bridges, we take both bridges.

### Number of Simple Paths
https://codeforces.com/problemset/problem/1454/E

We have one circle and some trees growing on it (cacti?) the  answer is number of paths multiplied by two minus number of paths in each growing tree.

The only question how to implement most elegantly.

### Shortest Path
https://codeforces.com/problemset/problem/59/E

Triples are hard but pairs are easier. It's pair of edges. Do modified BFS where we kind of go over edges (or store prev and current vertex). There are < n^2 edges so it will be fast enough.

### Orac and Game of Life
https://codeforces.com/problemset/problem/1349/C

We can observe that each cell either flickers or stays the same. And amount of points which stay the same goes down each step, at most 2000 (+ eps?) steps. 

For each point we need to find where is stops to flicker and what color it had. This can be done for each "shape" with bfs from multisource. Actually seems like we can just bfs from all points that change color? If there are no such points then things are constant.

### Square Subsets
https://codeforces.com/problemset/problem/895/C

Something something bitmask TODO.

### Triangular Paths
https://codeforces.com/problemset/problem/1506/F

Need to sort points by layer first and traverse in this order.

Now between two points we consider evennes of start and end and amount to go.

We can have function solve (isStartEven, stepsDown, stepsDownRight). 

TODO

### Propagating Tree
https://codeforces.com/problemset/problem/383/C

Here we just need to build segment tree for each depth level.

### Choosing Flowers
Main insight: answer looks like many flowers of some kind and at most 1 flower of each other kind.

Has set sorted by a.
Then iterate over flower which we take >= 1 times. Remove from set (and return after handling this iter). Find all flowers with a_i > b_x. Take them (or as much as possible). If space remained fill wiht b_x.

### Prefixes and Suffixes
https://codeforces.com/problemset/problem/432/D

I can just calculate Z-function.
And now I know which prefixes match suffixes.

Save those matching things.

Now I calculate prefix-function. And now for each suffix as substring I know in how many prefixes it includes.

Just add to segment tree or something?

TODO: not quite right, for example if AB matches A matches as well and I will add A two times but it should be added only once

but if AA matches, A should be added twice! So maybe we need to somehow call something for matching prefix also to do 5+


## 2100
### How Many Paths?
https://codeforces.com/problemset/problem/1547/G

First find 0, remove them.

Now find strongly connected components. Now mark nodes with loops. Now find SCC. Mark nodes in SCC with size at least 2. Now do bfs from all marked nodes. Mark everything reached with -1, remove.

Now we only need to differ between 1 and 2, this is easy. Can be done with bfs prob?

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


# List to Solve
Ok I solved or had ideas or banned a lot of problems. Let's make a list of what to solve

### Palindrome Game (hard)
https://codeforces.com/contest/1527/problem/B2

TODO

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
