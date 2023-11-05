Nushell uses a type-directed/type-configurable grammar. This means that as the parser parses each expression, if it finds a call to a command, and that command has known shape(s) for its arguments, those shapes direct the parser to parse the arguments.

This allows, for example:
```
where x > 100
```

Where the `where` command says that it takes a single parameter of shape Expression. Expressions are multi-token shapes, so they can consume what otherwise would be treated as multiple arguments as a single argument.

Parsing in Nushell happens in three steps: lexing, lite parsing, and parsing.

## Lexing

Lexing attempts to turn the input into tokens. Tokens have one of the following forms:

- Quote: 
```
"..."
'...'
`...`
```
- List/Table:
```
[...]
```
- Block/Record:
```
{...}
```
- Subexpression:
```
(...)
```
- Bare word:
A bare word in a contiguous token of non-whitespace characters, with the exception that they may contain inside of them the above forms, so long as they don't start with the initial character from the above. eg) `foo` and `foo"bar"` are a bare words, but `"bar"` is not.

## Lite parsing

Lite-parsing groups tokens into pipelines based on `|` and `;`, where the former creates pipelines and the latter optionally ends statements.

## Parsing

Description of grammar:

```
Non-terminal:
  Choice1
  Choice2
```

Choices short-circuit, so the first choice that matches wins and no other choices are tried. Choices are tried top to bottom.

* (Rule)+ -- Rule applied 1 or more times
* (Rule)*` -- Rule applied 0 or more times
* `literal` -- the syntax input code `literal`
* [A-Z] -- character class matching, a la regex

WARNING: the below is currently **incomplete**.

Missing: definitions, modules, exports, imports

```
Block:
  `{` Statement+ `}`

Statement:
  Statement `;` Pipeline
  Statement `\n` Pipeline
  Statement `\r\n` Pipeline
  Definition
  Pipeline

Pipeline:
  Pipeline `|` Expression
  Expression

Expression:
  Math
  Command
  
Math:
  `not` Expression
  MathLeader `**` Power

MathLeader:
  `true`
  `false`
  `null`
  VariablePath
  SubexpressionPath
  RecordBlockPath
  TableListPath
  QuotedString

VariablePath:
  `$` VariableName Path
  
SubexpressionPath:
  `(` Expression `)` Path
  
RecordBlockPath:
  RecordBlock Path

TableListPath:
  TableList Path
  
QuotedString:
  `"` EscapableChar+ `"`
  `'` (Char - `'`)* `'`
  '`' (Char - '`')* '`'

RecordBlock:
  Record
  Block

Record:
  `{` KeyValuePair* `}`
  
KeyValuePair:
  Token `:` Token `,`?

Table:
  `[` ListReq `;` ListReq+ `]`
  
ListReq:
  `[` ListItem+ `]`

List:
  `[` ListItem* `]`

ListItem:
  Token `,`?

EscapableChar:
  `\"`
  `\u` FourDigitUnicode
  ...
  Char - `\` - `"`

Path:
  (`.` (ColumnName | RowNumber))*

ColumnName:
  QuotedString
  BareString

RowNumber:
  Integer
  
Integer:
  HexInt
  OctalInt
  BinaryInt
  CommonInt
  
HexInt:
  `0x` [0-9A-Fa-f]+

OctalInt:
  `0o` [0-7]+
  
BinaryInt:
  `0b` [0-1]+
  
CommonInt:
  [0-9]+
  
BareString:
  BareWord (see: lexing)

Power:
  Power `**` MulDivMod
  MulDivMod

MulDivMod:
  MulDivMod `*` AddSub
  MulDivMod `/` AddSub
  MulDivMod `%` AddSub
  AddSub
  
AddSub:
  AddSub `+` MatchOp
  AddSub `-` MatchOp
  MatchOp
  
MatchOp:
  MatchOp `=~` And
  MatchOp `!~` And
  MatchOp `=^` And
  MatchOp `<` And
  MatchOp `<=` And
  MatchOp `>` And
  MatchOp `>=` And
  MatchOp `==` And
  MatchOp `!=` And
  MatchOp `in` And
  MatchOp `not-in` And
  And

And:
  And `&&` Or
  Or
  
Or:
  Or `||` Token
  Token

Token:
  MathLeader
  BareWord (see: lexing)
  
Command:
  CommandName (SubCommandName)* ShapeMatch
  
CommandName:
  BareWord

SubCommandName:
  ` ` BareWord

ShapeMatch<FullCommandName>:
  <
    for Shape in FullCommandName.args {
      lookup Shape => get number of tokens to consume
      consume tokens, picking rule above to parse token subset
    }
  >
```
  