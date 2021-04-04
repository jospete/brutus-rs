# brutus-rs

Rust implementation of a string brute-force iterator with multithreading.

Example Usage:

```
cargo run --release -- --charset-pattern [a|A|1|!] --target [string] --swarm [number]
```

Options:

- ```charset-pattern``` - flag pattern of character subsets to use (recommended)
- ```target``` - target string to search for (recommended)
- ```swarm``` - number of threads to use for brute force (recommended)
- ```charset``` - explicit set of characters to use in the combinator output
- ```length``` - explicit length of the pattern being searched for
- ```count``` - max iterations before bombing out

Charset Pattern Flags:

- ```a``` - all lowercase characters
- ```A``` - all uppercase characters
- ```1``` - all numbers
- ```!``` - all special characters