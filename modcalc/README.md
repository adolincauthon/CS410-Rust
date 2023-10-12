# Modulus Calculator

Adam Hiatt
10-6-2023

### Overview

This program calculates x^y mod m. Restrictions on variables are:

- 0 <= x < 2^64
- 0 <= y < 2^64
- 0 < m < 2^64

If an incorrect value is entered the the usage will be printed.

### Testing

I used the testing function provided in the assignment and added some additional tests. I added a test with x and y being the max u64 integer and selected an arbitrary m. I calculated the value using wolfram alpha in order to verify it was the correct number. I also added tests for some other arbirtrary cases. For invalid values, such as negative numbers or numbers greater than 2^64-1, I manually tested. I did not write a function to parse the CLI arguments and just parsed them in main.

### How'd it go?

This went pretty well, it was a good assignment to git used to the Rust language and ecosystem. I didn't experience to many difficults.

### Sources

Both of the class textbooks and the wikipedia article linked within the assignment for the algorithm. No other outside sources.
