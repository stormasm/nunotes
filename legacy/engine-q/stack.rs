// Example 1
// This comes from the the println below in the eval.rs eval_block method...

for stmt in &block.stmts {
    println!("\n{:?}",stmt);

if $true { 10 } else { 20 }

Expression(Expression {
    expr: Call(Call { decl_id: 1,
                      head: Span { start: 0, end: 2 },
                      positional:
                        [
                        Expression {
                                    expr: Bool(true),
                                    span: Span { start: 3, end: 8 },
                                    ty: Bool },
                        Expression {
                                    expr: Block(0),
                                    span: Span { start: 10, end: 14 },
                                    ty: Block },
                        Expression {
                                    expr: Keyword([101, 108, 115, 101],
                                          Span { start: 16, end: 20 },
                                          Expression { expr: Block(1),
                                                       span: Span { start: 22, end: 26 },
                                                       ty: Block }),
                                    span: Span { start: 16, end: 20 },
                                    ty: Block }
                       ],
                      named: [] }),
    span: Span { start: 0, end: 27 },
      ty: Unknown })

Expression(Expression { expr: Int(10), span: Span { start: 11, end: 13 }, ty: Int })
