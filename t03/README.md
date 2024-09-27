### Task

Sort lines in a file similar to the console utility sort (`man sort` - see the description and main parameters): a file with unsorted lines is fed at the input, a file with sorted lines is fed at the output. There is no need to support sorting large files on the disk, it is enough to sort in memory.

Implement the utility support for the following keys:

    -k - specify a column for sorting (words in a line can act as columns, the default separator is a space)

    -n - sort by numeric value

    -r - sort in reverse order

    -u - do not output duplicate lines

#### Additional

Implement the utility support for the following keys:

    -M - sort by month name

    -b - ignore trailing spaces

    -c - check if the data is sorted

    -h - sort by numeric value taking into account suffixes
