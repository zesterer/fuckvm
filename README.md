# FuckVM

FuckVM is an attempt at writing a compiler backend that can target Brainfuck, similar in principle to LLVM for any other instruction set.

## What can it do?

So far, not much. FuckVM can compile simple stack-driven code. It can handle arbitrary basic blocks, gotos, predicated branching, basic
arithmetic, comparison operations, arbitrary local stack values, and has limited support for arbitrarily structured types. It currently does
not have support for function calling or pointers, although planned support for these things is part of FuckVM's internal architecture.

## Status

FuckVM is not something I'm proud of. It's the product of a sleepless night and an obsessive mind. The code is not in a fit state for release
and has virtually no comments. It's only got a single (if relatively comprehensive) piece of example code and is extremely unfinished.
If you're looking to use it as a learning resource, do so at your own peril.

## License

I really don't care about licensing this work, and nobody is ever going to want to use it in its current state.

*Dons blindfold, throws dart at spinning wheel*

Oh, would you look at that. It seems to have landed on the... 'Do What the Fuck You Want to Public License'.

Fitting, given the profanity-loving name of its latest member.

http://www.wtfpl.net/
