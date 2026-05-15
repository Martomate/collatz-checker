# Finding counter examples to the Collatz Conjecture

This program goes through all the numbers (up to a limit) to check if that number is a counter example to the Collatz Conjecture. Of course it is highly unlikely to find one (nobody has so far), so this project is more of a practice for writing this kind of program.

To go through the numbers faster this program uses the optimizations mentioned on [Wikipedia](https://en.wikipedia.org/wiki/Collatz_conjecture#Optimizations):
1. It jumps k steps at a time, which is possible thanks to a time-space tradeoff based on the idea of a parity sequence
2. It only checks numbers that are candidates for being a counter example, which is done using modular restrictions in rings of the powers of 2

The program can currently check the first trillion numbers in less than 1 minute, which is pretty fast but probably not fast enough to find a counter example. It would take about 4000 years (single threaded, on my computer) to even get to the numbers we have not checked yet. One could of course use more computers, but it would still take years. If we want to speed this up we would need yet another way to reduce the search space. Alternatively we might find that there is an equivalent problem that is easier to prove and as a result also prove this one. My guess is that we will eventually prove it by showing that a counter example would have to satisfy certain properties, but we would then find that no number could fulfill all of those properties. That would be a proof by contradiction. Some of those properties would be the modular restrictions from above, but we would need more. Perhaps we could use more primes than just 2? We'll see.
