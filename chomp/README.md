### Overview

These crates are for playing a game of Chomp. The game is played on an n x m board. When you chomp a block on the board,
all of the blocks to the right and below are also chomped. The person to chomp the last piece loses.

### How it went

It went pretty well. I got a lot more experience working with Result and Option types which was vary useful. I ran
into a little difficulty with the algorithm since the construction and printing of the board was incorrect at first.
It took me a while to figure that out but once it was resolved the rest was straight forward. I provided MAX_COLUMNS
and MAX_ROWS values but did not enforce this in the crate in case a user wanted to play on a larger instance (which would be very slow)

### Testing

I tested the lib.rs crate thoroughly. I wrote unit tests for creating the board, finding a winning move, chomping valid
and invalid blocks, and finding the smallest possible chomp. I did not write tests in the main.rs file since it was all processing
user input and feeding to the already tested lib.rs file.
