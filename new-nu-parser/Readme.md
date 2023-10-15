
### Parser mantras

The *parse()* method returns a compiler
from which you can display the state of AstNode's.

The AstNode is an enum.

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
