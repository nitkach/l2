### Task

Create a utility `lf` (letter frequency), which returns information about how often a particular letter occurs in the text from the input file.
Only Latin characters should be counted.

#### Requirements

The output should look like a dictionary, the symbol is the frequency of occurrence. You should also output the amount of time spent counting characters.

The output should be formatted in json format. For simplicity, we will assume that the input file can be completely loaded into memory.

Also, you need to add the ability to use the `-t` switch, which allows you to specify the number of threads that will be used to count the frequency of occurrence of characters. Without this parameter, the program should use only one thread to count the characters encountered.

#### For example:

    Input: lf -t 4 report.txt

    Output: {
        "elapsed": "2.174 s",
        "result": {
            "a": 5423,
            "b": 1342,
            "c": 3572,
            ...
        }
    }
