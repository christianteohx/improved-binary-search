# IDEA <br />
Let's say we have an array that goes from 1 to 20 and we want to look for 4.<br />
------v-----------------------------------------<br />
|1|2|3|4|5|6|7|8|9|10|11|12|13|14|15|16|17|18|19|20|<br />
<br />
## Binary Search <br />
With binary search, our search range goes from <br />
1 -> 20, 1 -> 10, 1 -> 5, 2 -> 5, found 4. <br />
This takes 3 iterations.<br />
<br />
But what if we can reduce the range using the number being searched?<br />
Since we are looking for 4, we can calculate the maximum distance between the number being searched and the smallest/largest number in the array.<br />
<br />
## Improved? Binary Search <br />
For example, min = 1 and max = 20.<br />
If we assume the smallest difference between any number is 1, the maximum distance between 4 and 1 is 3, while the maximum number between 20 and 4 is 16.<br />
Just by taking the range with the smallest difference, our search scope will be reduced in the following way:<br />
1 -> 20, 1 -> 4, found 4.<br />
This takes only 1 iteration.<br />
\* We switch to binary search once the newly reduced range does not eliminate more than half of the previous range since binary search would be more efficient then (eliminates half the previous range).<br />
\* Also, this improved binary search is only meant to reduce the range used in the first few rounds.<br /><br />
>[!NOTE]
>This works theoretically (at least that's what my barely functioning brains thinks) but for some reason unbeknowest to all sapiens that are still able to convert oxygen into carbon dioxide, my test results still shows that binary search uses less iteration???????? and that the improved binary takes more time on average to find the key. (This might have something to do with computing the min/max and differences every time, but I don't see how it makes such a difference since the difference in average iteration used is not that much)

>[!NOTE]
>The extra iterations and time taken might just be the way I'm calculating the iterations or it could also be the redundant binary search while switching from the improved binary search to regular binary search. NEED TO FIND A BETTER WAY TO SWITCH ONCE IMPROVED BINARY SEARCH BECOMES LESS EFFICIENT!!!! 

> [!IMPORTANT]
> Most recent results as of 4/8/2024 2:59AM on device_1 <br />
> Array Size: 100,000,000 &nbsp; Test count: 100 &nbsp; Search count: 1000
> |          |Average Time     | Average iterations|
> | -------- | --------------- |------------------ |
> | Improved | 0.000007823006  | 26.69221          |
> | Normal   | 0.0000036157471 | 26.32271          |
> 
> Regular binary search uses less iterations than improved binary search 41850 times. 

> [!IMPORTANT]
> | device_name | Device           | RAM  | Processor                       |
> | ----------- | ---------------- | ---- | ------------------------------- |
> | device_1    | Macbook Pro 2020 | 16GB | 1.4 GHz Quad-Core Intel Core i5 |
> | device_2    | Macbook Air M2   | 8GB  | M2 Chip                         |
