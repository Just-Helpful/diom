# The Diom Programming language

This is mostly a language written because:

1. I haven't written a compiler in a while
2. I wanted to give it a go in rust :P
3. I wanted to give a language with minimal syntax a go
4. I like expression-driven languages but don't want to be stuck with FP
5. I've gotten interested in Algebraic effects

## Todo

- [ ] Convert monad nodes and parsing to use Algebraic effects

### Syntax

- [ ] Prevent multiple `Rest` patterns from being used in `Tuple`s and `Struct`s\
      This can be acheived by, instead of allowing any item to be `Rest` pattern\
      only allowing the last item to be one and using an `Option<Rest>` property\
      within the `Struct` and `Tuple`.
- [ ] Merge the `Array` and `Tuple` patterns (they're the same at this point...)
- [ ] We currently only use `Path` nodes for pattern matching, why?

### Parsing

- [ ] property testing on AST -> code -> AST round trip
  - [x] implement `Display` for AST
  - [ ] implement generators for ASTs
  - [ ] add property testing
- [ ] f-string support
  - I'll need to update how strings are parsed...
  - maybe consider having f-strings create templates not strings\
    if we support string conversion on these, we can solve most\
    of the standard string injection attacks by not converting to\
    strings until the last possible second.
- [x] use `nom-language`'s `precedence` parser

### Formatting

- [ ] add a custom formatting for creating boxes
- [x] add a custom formatter for indented structures
- [x] printing ASTs as code

### Interpreter

- [ ] add function calls
