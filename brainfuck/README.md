## Brainfuck interpreter

[Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) is a very simple esoteric programming language.
It has eight commands, each of which consists of exactly one character: `>`, `<`, `+`, `-`, `.`, `,`, `[`, and `]`.

These commands are defined on the Wikipedia page linked above.

I've included a couple of simple brainfuck programs.

## hello_world.bf
The classic.  Running is as follows:

```
rust-toys\brainfuck> brainfuck hello_world
Hello World!

```

## reverse_stdin.bf
An even simpler program, which reverses the input string provided as a second argument to the interpreter:

```
rust-toys\brainfuck> main reverse_stdin.bf "The quick brown fox jumps over the lazy dog."
.god yzal eht revo spmuj xof nworb kciuq ehT
```
