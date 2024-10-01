### Task

Implement a filtering utility similar to the console utility (man grep - see description and main parameters).

Implement support for the following keys in the utility:

    -A - "after" print +N lines after the match

    -B - "before" print +N lines before the match

    -C - "context" (A+B) print ±N lines around the match

    -c - "count" (number of lines)

    -i - "ignore-case" (ignore case)

    -v - "invert" (instead of matching, exclude)

    -F - "fixed", exact match with the line, not a pattern

    -n - "line num", print the line number
