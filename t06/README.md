### Task

Implement a utility similar to the console command `cut` (`man cut`). The utility should accept lines via `stdin`, split them into columns by a separator (`TAB`), and output the requested ones.

Implement the utility's support for the following switches:

    -f — "fields" - select fields (columns)

    -d — "delimiter" - use a different separator

    -s — "separated" - only lines with a separator
