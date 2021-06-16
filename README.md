# Summer of Bitcoin
This is the Solution for challenge of Summer of Bitcoin in Rust.

### Challenge
Maximize the Fee while minimizing the weight (<span>&#8804;</span> 4000000)

### Intuition
The problem is very similar to 0-1 Knapsack in a way.
We need to maximize the fee while constraining the weight to HIGHEST_WT i.e. minimizing the weight.
So to order the records we can sort it according to the ratio of fee/wt.
Also if there is an unknown parent transaction we simply reject it.


