# Research Report: Improved Binary Search Experiment #
## Abstract ##

This research investigates an enhancement to the traditional binary search algorithm. By dynamically adjusting the search range based on the target value's relative position within the data, this "Improved Binary Search" aims to reduce the average number of iterations needed to locate a value. </br>

## Introduction ##
Binary search is a well-known efficient algorithm for finding items in a sorted array with a runtime complexity of O(log n). However, this study introduces a variation that attempts to further reduce the number of iterations by leveraging the target's probable location derived from its value relative to the range's bounds.

## Experiment Description ##

The experiment was conducted using arrays ranging from size 10 to 100,000,000. Two versions of binary search were compared:

    Traditional Binary Search - which splits the array into halves.
    Improved Binary Search - which initially narrows down the search space based on the proximity of the target to the minimum or maximum values within the array.

The hypothesis posits that the Improved Binary Search can outperform traditional binary search in scenarios where the target's proximity to either end of the array significantly reduces the initial search space.

## Methodology ##
Arrays of various sizes were generated, and both search methods were applied to locate randomly chosen targets. The number of iterations and the time taken for each search were recorded. The experiment was performed on different hardware to assess performance across various system configurations.

## Results ##
Initial results showed that while the Improved Binary Search often performed fewer iterations in the first few rounds, the overall benefit was counteracted by the overhead of calculating the minimum and maximum distances. In many cases, the traditional binary search proved more efficient in terms of time, even though the iteration counts were similar.

## Discussion ##
The efficiency of the Improved Binary Search depends heavily on the distribution of the target values and the array's size. The added computational overhead of repeatedly calculating distances can negate the gains from reduced iterations unless the search space is significantly narrowed down in the first pass.

## Conclusion ##
The Improved Binary Search offers a theoretical improvement over traditional binary search by reducing the search space more effectively under ideal conditions. However, practical implementations need to carefully balance the overhead of initial calculations against the benefits of fewer iterations. Future work will focus on optimizing the transition point between the improved and traditional binary search phases and exploring adaptive techniques based on data distribution.

## Acknowledgments ##
This research was conducted using MacBook Pro 2020 and MacBook Air M2, with varying system specifications which contributed to performance differences observed.

## References ##
Performance data and additional details can be accessed in the ongoing experiment notes and logs, as documented in the research files and online repositories that will be linked in this report.
