
### Nushell data structures

This document just covers the core data structures of nu-protocol

##### crate: nu-protocol

```rust
pub struct Block {
    pub signature: Box<Signature>,
    pub pipelines: Vec<Pipeline>,
    pub captures: Vec<VarId>,
    pub redirect_env: bool,
    pub span: Option<Span>, // None option encodes no span to avoid using test_span()
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
    VarDecl(VarId),
    Call(Box<Call>),
    ExternalCall(Box<Expression>, Vec<Expression>),
    Operator(Operator),
    RowCondition(BlockId),
    BinaryOp(Box<Expression>, Box<Expression>, Box<Expression>), //lhs, op, rhs
    Subexpression(BlockId),
    Block(BlockId),
    List(Vec<Expression>),
    Table(Vec<Expression>, Vec<Vec<Expression>>),
    Record(Vec<(Expression, Expression)>),
    Keyword(Vec<u8>, Span, Box<Expression>),
    ValueWithUnit(Box<Expression>, Spanned<Unit>),
    Filepath(String),
    GlobPattern(String),
    String(String),
    CellPath(CellPath),
    FullCellPath(Box<FullCellPath>),
    ImportPattern(ImportPattern),
    Signature(Box<Signature>),
    StringInterpolation(Vec<Expression>),
    Nothing,
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
    Record(Vec<(String, Type)>),
    Table,
    ValueStream,
    Unknown,
    Error,
    Binary,
    Custom,
    Signature,
}

pub type VarId = usize;
pub type DeclId = usize;
pub type AliasId = usize;
pub type BlockId = usize;
pub type OverlayId = usize;

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
    // Signature category used to classify commands stored in the list of declarations
    pub category: Category,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PositionalArg {
    pub name: String,
    pub desc: String,
    pub shape: SyntaxShape,
    // For custom commands
    pub var_id: Option<VarId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Category {
    Default,
    Conversions,
    Core,
    Date,
    Env,
    Experimental,
    FileSystem,
    Filters,
    Formats,
    Math,
    Network,
    Random,
    Platform,
    Shells,
    Strings,
    System,
    Viewers,
    Hash,
    Generators,
    Custom(String),
    Deprecated,
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

    /// A record value
    Record,

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
    List {
        vals: Vec<Value>,
        span: Span,
    },
    Block {
        val: BlockId,
        captures: HashMap<VarId, Value>,
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
    CustomValue {
        val: Box<dyn CustomValue>,
        span: Span,
    },
}

pub enum PathMember {
    String { val: String, span: Span },
    Int { val: usize, span: Span },
}

pub struct CellPath {
    pub members: Vec<PathMember>,
}
```

##### nu-protocol: engine_state.rs

```rust
pub struct EngineState {
    files: im::Vector<(String, usize, usize)>,
    file_contents: im::Vector<(Vec<u8>, usize, usize)>,
    vars: im::Vector<Type>,
    decls: im::Vector<Box<dyn Command + 'static>>,
    aliases: im::Vector<Vec<Span>>,
    blocks: im::Vector<Block>,
    overlays: im::Vector<Overlay>,
    pub scope: im::Vector<ScopeFrame>,
    pub ctrlc: Option<Arc<AtomicBool>>,
    pub env_vars: im::HashMap<String, Value>,
    #[cfg(feature = "plugin")]
    pub plugin_signatures: Option<PathBuf>,
}

pub const NU_VARIABLE_ID: usize = 0;
pub const SCOPE_VARIABLE_ID: usize = 1;
pub const IN_VARIABLE_ID: usize = 2;
pub const CONFIG_VARIABLE_ID: usize = 3;
pub const ENV_VARIABLE_ID: usize = 4;

pub struct ScopeFrame {
    pub vars: HashMap<Vec<u8>, VarId>,
    predecls: HashMap<Vec<u8>, DeclId>, // temporary storage for predeclarations
    pub decls: HashMap<Vec<u8>, DeclId>,
    pub aliases: HashMap<Vec<u8>, AliasId>,
    pub env_vars: HashMap<Vec<u8>, BlockId>,
    pub overlays: HashMap<Vec<u8>, OverlayId>,
    visibility: Visibility,
}

pub struct StateWorkingSet<'a> {
    pub permanent_state: &'a EngineState,
    pub delta: StateDelta,
}

pub struct StateDelta {
    files: Vec<(String, usize, usize)>,
    pub(crate) file_contents: Vec<(Vec<u8>, usize, usize)>,
    vars: Vec<Type>,              // indexed by VarId
    decls: Vec<Box<dyn Command>>, // indexed by DeclId
    aliases: Vec<Vec<Span>>,      // indexed by AliasId
    pub blocks: Vec<Block>,       // indexed by BlockId
    overlays: Vec<Overlay>,       // indexed by OverlayId
    pub scope: Vec<ScopeFrame>,
    #[cfg(feature = "plugin")]
    plugins_changed: bool, // marks whether plugin file should be updated
}

pub struct Stack {
    /// Variables
    pub vars: HashMap<VarId, Value>,
    /// Environment variables arranged as a stack to be able to recover values from parent scopes
    pub env_vars: Vec<HashMap<String, Value>>,
    /// Tells which environment variables from engine state are hidden. We don't need to track the
    /// env vars in the stack since we can just delete them.
    pub env_hidden: HashSet<String>,
}
```

### References

##### usize

The pointer-sized unsigned integer type.

The size of this primitive is how many bytes it takes to reference any location in memory. For example, on a 32 bit target, this is 4 bytes and on a 64 bit target, this is 8 bytes.

##### Last Updated: February 15, 2022
