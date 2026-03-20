# The Diom Programming language

This is mostly a language written because:

1. I haven't written a compiler in a while
2. I wanted to give it a go in rust :P
3. I wanted to give a language with minimal syntax a go
4. I like expression-driven languages but don't want to be stuck with FP
5. I've gotten interested in Algebraic effects

## Todo

- [ ] Tidy up all imports to use `super -> crate -> ...`

### Syntax

- [ ] Prevent multiple `Rest` patterns from being used in `Tuple`s and `Struct`s\
      This can be acheived by, instead of allowing any item to be `Rest` pattern\
      only allowing the last item to be one and using an `Option<Rest>` property\
      within the `Struct` and `Tuple`.
- [ ] Merge the `Array` and `Tuple` patterns (they're the same at this point...)
- [ ] We currently only use `Path` nodes for pattern matching, why?
- [ ] Convert monad nodes and parsing to use Algebraic effects

### Parsing

- [ ] f-string support
  - I'll need to update how strings are parsed...
  - maybe consider having f-strings create templates not strings\
    if we support string conversion on these, we can solve most\
    of the standard string injection attacks by not converting to\
    strings until the last possible second.
- [x] property testing on AST -> code -> AST round trip
  - [x] implement `Display` for AST
  - [x] implement generators for ASTs
  - [x] add property testing
- [x] use `nom-language`'s `precedence` parser

### Formatting

- [ ] add a custom formatting for creating boxes
- [ ] pretty printing ASTs (formatting)
- [x] add a custom formatter for indented structures
- [x] printing ASTs as code

### Interpreter

- [ ] add function calls

### Documentation

- [ ] Ensure docstrings are present on all syntax nodes
- [ ] Add simple docstrings to parsers that mention what they parse
