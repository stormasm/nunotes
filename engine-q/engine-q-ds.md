
### Engine-q data structures

##### nu-protocol

```rust
pub struct Block {
    pub signature: Box<Signature>,
    pub stmts: Vec<Statement>,
    pub exports: Vec<(Vec<u8>, DeclId)>, // Assuming just defs for now
}

pub enum Statement {
    Declaration(DeclId),
    Pipeline(Pipeline),
}

pub struct Pipeline {
    pub expressions: Vec<Expression>,
}

pub struct Expression {
    pub expr: Expr,
    pub span: Span,
    pub ty: Type,
    pub custom_completion: Option<String>,
}

pub enum Expr {
    Bool(bool),
    Int(i64),
    Float(f64),
    Range(
        Option<Box<Expression>>, // from
        Option<Box<Expression>>, // next value after "from"
        Option<Box<Expression>>, // to
        RangeOperator,
    ),
    Var(VarId),
    Call(Box<Call>),
    ExternalCall(String, Span, Vec<Expression>),
    Operator(Operator),
    RowCondition(VarId, Box<Expression>),
    BinaryOp(Box<Expression>, Box<Expression>, Box<Expression>), //lhs, op, rhs
    Subexpression(BlockId),
    Block(BlockId),
    List(Vec<Expression>),
    Table(Vec<Expression>, Vec<Vec<Expression>>),
    Keyword(Vec<u8>, Span, Box<Expression>),
    ValueWithUnit(Box<Expression>, Spanned<Unit>),
    Filepath(String),
    GlobPattern(String),
    String(String),
    CellPath(CellPath),
    FullCellPath(Box<FullCellPath>),
    Signature(Box<Signature>),
    Garbage,
}

pub struct Span {
    pub start: usize,
    pub end: usize,
}

pub enum Type {
    Int,
    Float,
    Range,
    Bool,
    String,
    Block,
    CellPath,
    Duration,
    Date,
    Filesize,
    List(Box<Type>),
    Number,
    Nothing,
    Record(Vec<String>, Vec<Type>),
    Table,
    ValueStream,
    Unknown,
    Error,
    Binary,
}

pub type BlockId = usize;
pub type DeclId = usize;
pub type VarId = usize;

pub struct Signature {
    pub name: String,
    pub usage: String,
    pub extra_usage: String,
    pub required_positional: Vec<PositionalArg>,
    pub optional_positional: Vec<PositionalArg>,
    pub rest_positional: Option<PositionalArg>,
    pub named: Vec<Flag>,
    pub is_filter: bool,
    pub creates_scope: bool,
}

pub struct Flag {
    pub long: String,
    pub short: Option<char>,
    pub arg: Option<SyntaxShape>,
    pub required: bool,
    pub desc: String,
    // For custom commands
    pub var_id: Option<VarId>,
}

pub struct PositionalArg {
    pub name: String,
    pub desc: String,
    pub shape: SyntaxShape,
    // For custom commands
    pub var_id: Option<VarId>,
}

pub enum SyntaxShape {
    /// A specific match to a word or symbol
    Keyword(Vec<u8>, Box<SyntaxShape>),

    /// Any syntactic form is allowed
    Any,

    /// Strings and string-like bare words are allowed
    String,

    /// A dotted path to navigate the table
    CellPath,

    /// A dotted path to navigate the table (including variable)
    FullCellPath,

    /// Only a numeric (integer or decimal) value is allowed
    Number,

    /// A range is allowed (eg, `1..3`)
    Range,

    /// Only an integer value is allowed
    Int,

    /// A filepath is allowed
    Filepath,

    /// A glob pattern is allowed, eg `foo*`
    GlobPattern,

    /// A module path pattern used for imports
    ImportPattern,

    /// A block is allowed, eg `{start this thing}`
    Block(Option<Vec<SyntaxShape>>),

    /// A table is allowed, eg `[[first, second]; [1, 2]]`
    Table,

    /// A table is allowed, eg `[first second]`
    List(Box<SyntaxShape>),

    /// A filesize value is allowed, eg `10kb`
    Filesize,

    /// A duration value is allowed, eg `19day`
    Duration,

    /// An operator
    Operator,

    /// A math expression which expands shorthand forms on the lefthand side, eg `foo > 1`
    /// The shorthand allows us to more easily reach columns inside of the row being passed in
    RowCondition,

    /// A general math expression, eg `1 + 2`
    MathExpression,

    /// A variable name
    Variable,

    /// A variable with optional type, `x` or `x: int`
    VarWithOptType,

    /// A signature for a definition, `[x:int, --foo]`
    Signature,

    /// A general expression, eg `1 + 2` or `foo --bar`
    Expression,

    /// A boolean value
    Boolean,

    /// A custom shape with custom completion logic
    Custom(Box<SyntaxShape>, String),
}

pub enum Operator {
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Contains,
    NotContains,
    Plus,
    Minus,
    Multiply,
    Divide,
    In,
    NotIn,
    Modulo,
    And,
    Or,
    Pow,
}

pub struct Call {
    /// identifier of the declaration to call
    pub decl_id: DeclId,
    pub head: Span,
    pub positional: Vec<Expression>,
    pub named: Vec<(Spanned<String>, Option<Expression>)>,
}

pub enum Value {
    Bool {
        val: bool,
        span: Span,
    },
    Int {
        val: i64,
        span: Span,
    },
    Filesize {
        val: i64,
        span: Span,
    },
    Duration {
        val: i64,
        span: Span,
    },
    Date {
        val: DateTime<FixedOffset>,
        span: Span,
    },
    Range {
        val: Box<Range>,
        span: Span,
    },
    Float {
        val: f64,
        span: Span,
    },
    String {
        val: String,
        span: Span,
    },
    Record {
        cols: Vec<String>,
        vals: Vec<Value>,
        span: Span,
    },
    Stream {
        stream: ValueStream,
        span: Span,
    },
    List {
        vals: Vec<Value>,
        span: Span,
    },
    Block {
        val: BlockId,
        span: Span,
    },
    Nothing {
        span: Span,
    },
    Error {
        error: ShellError,
    },
    Binary {
        val: Vec<u8>,
        span: Span,
    },
    CellPath {
        val: CellPath,
        span: Span,
    },
}
```

##### nu-protocol: engine_state.rs

```rust
pub struct EngineState {
    files: Vec<(String, usize, usize)>,
    file_contents: Vec<u8>,
    vars: Vec<Type>,
    decls: Vec<Box<dyn Command>>,
    blocks: Vec<Block>,
    pub scope: Vec<ScopeFrame>,
}

pub struct ScopeFrame {
    pub vars: HashMap<Vec<u8>, VarId>,
    predecls: HashMap<Vec<u8>, DeclId>, // temporary storage for predeclarations
    decls: HashMap<Vec<u8>, DeclId>,
    aliases: HashMap<Vec<u8>, Vec<Span>>,
    modules: HashMap<Vec<u8>, BlockId>,
    visibility: Visibility,
}

pub struct StateWorkingSet<'a> {
    pub permanent_state: &'a EngineState,
    pub delta: StateDelta,
}

pub struct StateDelta {
    files: Vec<(String, usize, usize)>,
    pub(crate) file_contents: Vec<u8>,
    vars: Vec<Type>,              // indexed by VarId
    decls: Vec<Box<dyn Command>>, // indexed by DeclId
    blocks: Vec<Block>,           // indexed by BlockId
    pub scope: Vec<ScopeFrame>,
}
```

##### engine/evaluation_context.rs

```rust
pub struct EvaluationContext {
    pub engine_state: Rc<RefCell<EngineState>>,
    pub stack: Stack,
}

pub struct StackFrame {
    pub vars: HashMap<VarId, Value>,
    pub env_vars: HashMap<String, String>,
    pub parent: Option<Stack>,
}

pub struct Stack(Rc<RefCell<StackFrame>>);
```

##### nu-parser parser.rs

```rust
pub enum Import {}

pub struct VarDecl {
    var_id: VarId,
    expression: Expression,
}
```

### References

##### usize

The pointer-sized unsigned integer type.

The size of this primitive is how many bytes it takes to reference any location in memory. For example, on a 32 bit target, this is 4 bytes and on a 64 bit target, this is 8 bytes.

##### Update: October 14, 2021
