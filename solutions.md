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

### Old Floppy Drive
https://codeforces.com/contest/1490/problem/G

Calculate amount of full circular spin F.

Now convert a to map<remainder F, div F>

Now for every x we need to find element in map. It should have the same remainder F and maximum div <= x div F. It can be done with binsearch or maybe even linearly if we orgainze smart.

Be careful about negative F.

### Nearest Opposite Parity
https://codeforces.com/problemset/problem/1272/E

It's just multisource bfs. Start from all even and find distance to each odd. Same for starting with odd. Not much edges (2n - 2) so it's fine.

### Equalize the Remainders
https://codeforces.com/problemset/problem/999/D

Ones with cr <= n/m just stay. Others move to the closest cr which is lower. Just store crs which are low in set, find and update. 

Just go iteratively.

### Gargari and Permutations
https://codeforces.com/problemset/problem/463/D

Each integer represents a point in 5d space (or 4d, 3d, 2d but 5d WLOG). Edge u-v exists iff all coordinates of v are larger than corresponding coordinates of u.

Now we just need to find longest path on DAG.

Would be nice to solve and add longest path, topsort etc. to the lib.

### Maximal GCD
https://codeforces.com/problemset/problem/803/C

First notice that minimum is 1 2 3 .. k

If this is > n then -1

Otherwise iterate over divisors of n from largest to lowest and try to do (1 2 3 .. k-1 k+x) * div

### Obtain a Permutation
https://codeforces.com/contest/1294/problem/E

We can solve for each column separately.

For each element we need to calc how many cyclic shifts (if any) will put in a proper position. 
We can use binary search to find this.

Create map[num-shifts, cnt-elems] and select the best.

Then sum ans for all columns.

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

## 2000

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

### Square Subsets
https://codeforces.com/problemset/problem/895/C

Something something bitmask TODO.

### Triangular Paths
https://codeforces.com/problemset/problem/1506/F

TODO

### Propagating Tree
https://codeforces.com/problemset/problem/383/C

Here we just need to build segment tree for each depth level.

### How Many Paths?
https://codeforces.com/problemset/problem/1547/G

First find 0, remove them.

Now find strongly connected components. Now mark nodes with loops. Now find SCC. Mark nodes in SCC with size at least 2. Now do bfs from all marked nodes. Mark everything reached with -1, remove.

Now we only need to differ between 1 and 2, this is easy. Can be done with bfs prob?

### Reducing Delivery Cost
https://codeforces.com/problemset/problem/1433/G

It sounds like we have to try for each road? Because if we find shortest path changing one road may completely change it.

Do floyd. Now we know the smallest path.

Now try every road. For each courier answer will be min(initial_ans, min(f[a][v] + f[u][b], f[a][u] + f[v][a])) 
