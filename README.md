# sins

Playground for a rust linter based off of the `syn` crate.

## Why?

- Clippy is great, but is complex and requires `rustc` knowledge to develop a new lint
- Maybe something based on `syn` and just AST without other parts of of `rustc` could be faster and more convenient to write custom rules at the cost of analysis detail
  - Doesn't need the whole program to compile, just enough to get a working AST (maybe partial AST is better? tree-sitter might work well)
  - This might allow faster IDE feedback
- Or maybe not!

## What?

- Playground to try this out!

```rust
  // Sample lint to test for disallowed function name

  // 1. Read and parse the file
  let mut file = std::fs::File::open("test.rs").unwrap();
  let mut content = String::new();
  std::io::Read::read_to_string(&mut file, &mut content).unwrap();
  let syntax_tree = syn::parse_file(&content).unwrap();

  // 2. Define lint
  struct FooFunctionNameRule;
  impl<'ast> Rule<'ast> for FooFunctionNameRule {
    fn apply_expr_method_call(
      &mut self,
      context: &RuleContext,
      node: &'ast syn::ExprMethodCall,
    ) {
      if node.method == "foo" {
        context.emit_report(Report {
          message: "foo method disallowed".into(),
          node,
        })
      }
    }
  }
  let foo_rule = FooFunctionNameRule {};

  // 3. Run lint(s)
  let mut rule_engine = RuleEngine::new();
  rule_engine.add_rule(Box::new(foo_rule));
  rule_engine.apply_rules(&syntax_tree);
```
