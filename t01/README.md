### Task
Create a utility similar to the wc utility that takes a file as input and outputs the result — the number of objects in the file.
The number of characters/words/lines in the file should be counted depending on the parameter:

- `-c` - show the number of characters in the file

- `-l` - display the number of lines in the file

- `-w` - display the number of words in the file

By default, if the parameter is not specified, the number of words is counted.

For example:

    Input: wc report.txt
    Output: 34423

    Input: wc -l report.txt
    Output: 3500
