# Notes

## Atlas Index: Converting between flat index and (row, column)

The sprite sheet is a grid, but the computer stores it as a **flat numbered list** starting at 0:

```
         col0  col1  col2  col3  col4  col5  col6  col7  col8
row 0:  [  0,    1,    2,    3,    4,    5,    6,    7,    8 ]
row 1:  [  9,   10,   11,   12,   13,   14,   15,   16,   17 ]
row 2:  [ 18,   19,   20,   21,   22,   23,   24,   25,   26 ]
row 3:  [ 27,   28,   29,   30,   31,   32,   33,   34,   35 ]
...
```

`atlas.index` is just one number from this flat list — say **20**.

You need to figure out **which row and column** 20 sits in. That's where `%` and `/` come in.

### To get the column — use `%` (remainder):

`20 % 9 = 2` — because 20 ÷ 9 = 2 with **remainder 2**. Look at the grid: index 20 is in **column 2**. The remainder tells you "how far along the row are we?"

### To get the row — use `/` (integer division):

`20 / 9 = 2` — because 9 fits into 20 exactly **2** whole times (with leftovers ignored). That means we've completed 2 full rows, so we're on **row 2**.

### Another example — index **14**:

- `14 % 9 = 5` → column 5 (because 14 = 1×9 + **5**)
- `14 / 9 = 1` → row 1 (9 fits into 14 **once**)

Check the grid: row 1, col 5 → that's index 14. Correct.

### Why do we need this?

Because the animation logic needs to know:
- Which **row** we're on → to check if the player changed direction (each direction has its own row)
- Which **column** we're on → to advance to the next frame: `(current_col + 1) % 9` cycles through 0,1,2,...8,0,1,2,... looping the walk animation
