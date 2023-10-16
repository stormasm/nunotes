
### Parser mantras

The *parse()* method returns a compiler
from which you can display the state of AstNode's.

The AstNode is an enum.

### The parse() method

```rust
pub fn parse(mut self) -> Compiler {
     self.block(BlockContext::Bare);
     self.compiler
}
```

The *parse()* method starts with block so lets start there and dive down...

The main driving function in block is *has_tokens()*

#### has_tokens() is what drives the lexer

the lexer is happening in concert with the parsing

The main concepts that are getting lexed and parsed are

* let_statement
* while_statement
* for_statement
* expression_or_assignment

Besides the *block()* method in *parse()* it also shows up in

* if_expression
* while_statement
* for_statement
* record_or_closure

### Lexing

##### Tokens are returned by the following methods

* lex_quoted_string
* lex_number
* lex_name
* lex_symbol
* newline
* peek
* next
---

the next() method returns the next *Token*; it is called 57 times

The *next()* methods job is to return  
* lex_number
* lex_name
* lex_symbol

---



* lots of methods return the *NodeId*
