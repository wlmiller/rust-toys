## Simple scheme interpreter

This is a very simple interpreter of a "Scheme-like" based primarily on ["Lispy"](http://norvig.com/lispy.html) by Peter Norvig.

It's obviously substantially longer.  I think there are a couple reasons for that:

1. The dyanmic nature of Python really shines.
2. I was fighting the type system and borrow checker the whole way.

Next up is to implement a repl.
My plan is to then move on to ["Lispy 2"](http://norvig.com/lispy2.html).  I'll probably be working back through improving parts of this as I go.

There's a simple test program in [fib.s](fib.s) which prints the first 20 Fibonacci numbers (it's implemented naively and take a few seconds).
```
rust-toys\rscheme> rscheme fib.s
(1 1 2 3 5 8 13 21 34 55 89 144 233 377 610 987 1597 2584 4181 6765)
```
