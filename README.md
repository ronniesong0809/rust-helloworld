# rust-helloworld

[![Build Status](https://travis-ci.com/ronniesong0809/rust-helloworld.svg?branch=master)](https://travis-ci.com/ronniesong0809/rust-helloworld)

Copyright (c) 2020 Ronnie Song

This is a Rust based command-line calculator that computes *gcd()*.

<br>

*gcd()* is calculate the greatest common divisor, for example:
```
gcd(54, 24) = 6
```

The prime factorizations of the two numbers:
```
54 = 2 * 3 * 3 * 3
24 = 2 * 3 * 2 * 2
```
They share in common is `2` and `3`
Greatest common divisor: `2 * 3 = 6`

## Run

```
$ cargo run 54 24
   Compiling...
    Finished...
     Running...
The gcd of [54, 24] is 6
```

## License

This program is licensed under the "MIT License".  Please
see the file `LICENSE` in the source distribution of this
software for license terms.
