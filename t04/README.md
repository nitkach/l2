### Task

Write a function to search for all sets of anagrams in a dictionary.

For example:

    'pyatak', 'pyatka' and 'tyapka' belong to one set,

    'listok', 'slitok' and 'stolik' belong to another.

#### Requirements:

1) Input data for the function: a reference to an array, each element of which is a word in russian in UTF-8 encoding.

2) Output data: a map of anagram sets

3) The key is the first word from the set that appears in the dictionary. The value is a reference to an array, each element of which is a word from the set.

4) The array must be sorted in ascending order.

5) Sets of one element must not be included in the result.

6) All words must be converted to lower case.

7) As a result, each word must appear only once.
