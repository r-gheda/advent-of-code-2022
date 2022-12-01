## Day 1 Problem 2
By the time you calculate the answer to the Elves' question, they've already realized that the Elf carrying the most Calories of food might eventually run out of snacks.

To avoid this unacceptable situation, the Elves would instead like to know the total Calories carried by the top three Elves carrying the most Calories. That way, even if one of those Elves runs out of snacks, they still have two backups.

In the example above, the top three Elves are the fourth Elf (with ```24000``` Calories), then the third Elf (with ```11000``` Calories), then the fifth Elf (with ```10000``` Calories). The sum of the Calories carried by these three elves is ```45000```.

Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?

The answer to the provided input is ```204639```.

## Solution
The solution exploits a priority queue to collect elves data and retrieve them easily. The total time cost of the solution is $O(n*log_2n + k)$ where $k$ is the number of top Elves we want to consider for our final answer.

Going through the whole dataset multiple times would bring to solve the problem faster for $k \lt log_2n$. 
