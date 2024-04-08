**IDEA** <br />
Let's say we have an array that goes from 1 to 20 and we want to look for 4.<br />
-------v--------------------------------------------<br />
|1|2|3|4|5|6|7|8|9|10|11|12|13|14|15|16|17|18|19|20|<br />
<br />
**Binary Search** <br />
With binary search, our search range goes from <br />
1 -> 20, 1 -> 10, 1 -> 5, 2 -> 5, found 4. <br />
This takes 3 iterations.<br />
<br />
But what if we can reduce the range using the number being searched?<br />
Since we are looking for 4, we can calculate the maximum distance between the number being searched and the smallest/largest number in the array.<br />
<br />
**Improved Binary Search?** <br />
For example, min = 1 and max = 20.<br />
If we assume the smallest difference between any number is 1, the maximum distance between 4 and 1 is 3, while the maximum number between 20 and 4 is 16.<br />
Just by taking the range with the smallest difference, our search scope will be reduced in the following way:<br />
1 -> 20, 1 -> 4, found 4.<br />
This takes only 1 iteration.<br />
