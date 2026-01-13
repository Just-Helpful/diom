# The Diom Programming language

This is mostly a language written because:

1. I haven't written a compiler in a while
2. I wanted to give it a go in rust :P
3. I wanted to give a language with minimal syntax a go
4. I like expression-driven languages but don't want to be stuck with FP
5. I've gotten interested in Algebraic effects

## Todo

- [ ] f-string support
  - I'll need to update how strings are parsed...
  - maybe consider having f-strings create templates not strings\
    if we support string conversion on these, we can solve most\
    of the standard string injection attacks by not converting to\
    strings until the last possible second.

### Lexing

- [x] pretty printing for Tokens

### Parsing

- [x] pretty printing for ASTs
- [ ] printing ASTs as code
- [ ] property testing on AST -> code -> AST round trip
- [ ] use `nom-language`'s `precedence` parser

### Interpreter

- [x] create a basic interpreter backend
- [x] add block / group evaluation
- [x] add variable definitions
- [ ] add function calls
