# Finding counter examples to the Collatz Conjecture

This program goes through all the numbers (up to a limit) to check if that number is a counter example to the Collatz Conjecture. Of course it is highly unlikely to find one (nobody has so far), so this project is more of a practice for writing this kind of program.

To go through the numbers faster this program uses the optimizations mentioned on [Wikipedia](https://en.wikipedia.org/wiki/Collatz_conjecture#Optimizations):
1. It jumps k steps at a time, which is possible thanks to a time-space tradeoff based on the idea of a parity sequence
2. It only checks numbers that are candidates for being a counter example, which is done using modular restrictions in rings of the powers of 2
