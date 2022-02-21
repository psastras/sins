// https://rust-lang.github.io/rust-clippy/v0.0.212/

struct AbsurdExtremeComparisons;
impl<'ast> crate::Rule<'ast> for AbsurdExtremeComparisons {
  fn apply_expr_binary(
    &mut self,
    context: &crate::RuleContext,
    node: &'ast syn::ExprBinary,
  ) {
    // let left = node.left;
    // match node.op {
    //   syn::BinOp::Le(_) => {
    //     node.left
    //   }
    //   _ => {}
    // }
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_absurd_extreme_comparisons() {
    let mut content = "
    fn main() {
      const Z: u32 = 0;
      let u: u32 = 42;
      u <= 0;
      u <= Z;
    }
    ";
    let syntax_tree = syn::parse_file(&content).unwrap();
    println!("{:#?}", syntax_tree);
    let mut rule_engine = crate::RuleEngine::new();
    rule_engine.add_rule(Box::new(AbsurdExtremeComparisons {}));
    rule_engine.apply_rules(&syntax_tree);
  }
}
