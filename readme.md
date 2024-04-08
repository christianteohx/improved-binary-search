**IDEA**
Let's say we have an array that goes from 1 to 20 and we want to look for 4.
       v
|1|2|3|4|5|6|7|8|9|10|11|12|13|14|15|16|17|18|19|20|

**Binary Search**
With binary search, our search range goes from 
1 -> 20, 1 -> 10, 1 -> 5, 2 -> 5, found 4. 
This takes 3 iterations.

But what if we can reduce the range using the number being searched?
Since we are looking for 4, we can calculate the maximum distance between the number being searched and the smallest/largest number in the array.

**Improved Binary Search?**
For example, min = 1 and max = 20.
If we assume the smallest difference between any number is 1, the maximum distance between 4 and 1 is 3, while the maximum number between 20 and 4 is 16.
Just by taking the range with the smallest difference, our search scope will be reduced in the following way:
1 -> 20, 1 -> 4, found 4.
This takes only 1 iteration.
