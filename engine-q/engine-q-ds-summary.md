
##### nu-protocol pub enum

```rust
ty.rs  
pub enum Type

ast/cell_path.rs  
pub enum PathMember

ast/import_pattern.rs  
pub enum ImportPatternMember

ast/expr.rs  
pub enum Expr

ast/statement.rs  
pub enum Statement

ast/operator.rs  
pub enum Operator  
pub enum RangeInclusion

syntax_shape.rs  
pub enum SyntaxShape  

shell_error.rs  
pub enum ShellError  

value/unit.rs  
pub enum Unit  

value/mod.rs  
pub enum Value  
```

##### nu-protocol pub struct

```rust
example.rs  
pub struct Example

engine/engine_state.rs  
pub struct EngineState  
pub struct ScopeFrame  
pub struct StateWorkingSet<'a>  
pub struct StateDelta

engine/call_info.rs  
pub struct UnevaluatedCallInfo

engine/evaluation_context.rs  
pub struct EvaluationContext  
pub struct StackFrame  
pub struct Stack(Rc<RefCell<StackFrame>>);

ast/cell_path.rs  
pub struct CellPath  
pub struct FullCellPath

ast/import_pattern.rs  
pub struct ImportPattern

ast/pipeline.rs  
pub struct Pipeline

ast/block.rs  
pub struct Block

signature.rs  
pub struct Flag  
pub struct PositionalArg  
pub struct Signature

ast/call.rs
pub struct Call

ast/expression.rs
pub struct Expression

ast/operator.rs
pub struct RangeOperator

span.rs
pub struct Spanned<T>
pub struct Span

value/stream.rs  
pub struct ValueStream(pub Rc<RefCell<dyn Iterator<Item = Value>>>);

value/row.rs
pub struct RowStream(Rc<RefCell<dyn Iterator<Item = Vec<Value>>>>);

value/range.rs  
pub struct Range  
pub struct RangeIterator
```
