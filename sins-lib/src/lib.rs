mod clippy;
pub trait Rule<'ast> {
  fn apply_item_fn(
    &mut self,
    _context: &RuleContext,
    _node: &'ast syn::ItemFn,
  ) {
  }

  fn apply_fn_arg(&mut self, _context: &RuleContext, _node: &'ast syn::FnArg) {}

  fn apply_expr_method_call(
    &mut self,
    _context: &RuleContext,
    _node: &'ast syn::ExprMethodCall,
  ) {
  }

  fn apply_expr_binary(
    &mut self,
    _context: &RuleContext,
    _node: &'ast syn::ExprBinary,
  ) {
  }
}

pub struct RuleContext {
  pub ancestors: Vec<Box<dyn std::any::Any>>,
}

impl RuleContext {
  pub fn emit_report(&self, report: Report) {
    let start = report.node.span().start();
    println!(
      "[foo.rs:{}:{}] {}",
      start.line, start.column, report.message
    )
  }
}

struct RuleEngine<'a> {
  rules: Vec<Box<dyn Rule<'a>>>,
  context: RuleContext,
}

pub struct Report<'a> {
  pub message: String,
  pub node: &'a dyn syn::spanned::Spanned,
}

impl<'ast> syn::visit::Visit<'ast> for RuleEngine<'ast> {
  fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
    self.apply_item_fn_rules(node);
    self.context.ancestors.push(Box::new(node.clone())); // todo: is this clone needed?
    syn::visit::visit_item_fn(self, node);
    self.context.ancestors.pop();
  }

  fn visit_bare_fn_arg(&mut self, node: &'ast syn::BareFnArg) {
    // self.apply_expr_method_call(node);
    self.context.ancestors.push(Box::new(node.clone()));
    syn::visit::visit_bare_fn_arg(self, node);
    self.context.ancestors.pop();
  }

  fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
    self.apply_expr_method_call(node);
    self.context.ancestors.push(Box::new(node.clone()));
    syn::visit::visit_expr_method_call(self, node);
    self.context.ancestors.pop();
  }

  fn visit_fn_arg(&mut self, node: &'ast syn::FnArg) {
    self.apply_fn_arg_rules(node);
    syn::visit::visit_fn_arg(self, node);
  }

  fn visit_abi(&mut self, i: &'ast syn::Abi) {
    syn::visit::visit_abi(self, i);
  }

  fn visit_angle_bracketed_generic_arguments(
    &mut self,
    i: &'ast syn::AngleBracketedGenericArguments,
  ) {
    syn::visit::visit_angle_bracketed_generic_arguments(self, i);
  }

  fn visit_arm(&mut self, i: &'ast syn::Arm) {
    syn::visit::visit_arm(self, i);
  }

  fn visit_attr_style(&mut self, i: &'ast syn::AttrStyle) {
    syn::visit::visit_attr_style(self, i);
  }

  fn visit_attribute(&mut self, i: &'ast syn::Attribute) {
    syn::visit::visit_attribute(self, i);
  }

  fn visit_bin_op(&mut self, i: &'ast syn::BinOp) {
    syn::visit::visit_bin_op(self, i);
  }

  fn visit_binding(&mut self, i: &'ast syn::Binding) {
    syn::visit::visit_binding(self, i);
  }

  fn visit_block(&mut self, i: &'ast syn::Block) {
    syn::visit::visit_block(self, i);
  }

  fn visit_bound_lifetimes(&mut self, i: &'ast syn::BoundLifetimes) {
    syn::visit::visit_bound_lifetimes(self, i);
  }

  fn visit_const_param(&mut self, i: &'ast syn::ConstParam) {
    syn::visit::visit_const_param(self, i);
  }

  fn visit_constraint(&mut self, i: &'ast syn::Constraint) {
    syn::visit::visit_constraint(self, i);
  }

  fn visit_data(&mut self, i: &'ast syn::Data) {
    syn::visit::visit_data(self, i);
  }

  fn visit_data_enum(&mut self, i: &'ast syn::DataEnum) {
    syn::visit::visit_data_enum(self, i);
  }

  fn visit_data_struct(&mut self, i: &'ast syn::DataStruct) {
    syn::visit::visit_data_struct(self, i);
  }

  fn visit_data_union(&mut self, i: &'ast syn::DataUnion) {
    syn::visit::visit_data_union(self, i);
  }

  fn visit_derive_input(&mut self, i: &'ast syn::DeriveInput) {
    syn::visit::visit_derive_input(self, i);
  }

  fn visit_expr(&mut self, i: &'ast syn::Expr) {
    syn::visit::visit_expr(self, i);
  }

  fn visit_expr_array(&mut self, i: &'ast syn::ExprArray) {
    syn::visit::visit_expr_array(self, i);
  }

  fn visit_expr_assign(&mut self, i: &'ast syn::ExprAssign) {
    syn::visit::visit_expr_assign(self, i);
  }

  fn visit_expr_assign_op(&mut self, i: &'ast syn::ExprAssignOp) {
    syn::visit::visit_expr_assign_op(self, i);
  }

  fn visit_expr_async(&mut self, i: &'ast syn::ExprAsync) {
    syn::visit::visit_expr_async(self, i);
  }

  fn visit_expr_await(&mut self, i: &'ast syn::ExprAwait) {
    syn::visit::visit_expr_await(self, i);
  }

  fn visit_expr_binary(&mut self, node: &'ast syn::ExprBinary) {
    self.apply_expr_binary(node);
    self.context.ancestors.push(Box::new(node.clone()));
    syn::visit::visit_expr_binary(self, node);
    self.context.ancestors.pop();
  }

  fn visit_expr_block(&mut self, i: &'ast syn::ExprBlock) {
    syn::visit::visit_expr_block(self, i);
  }

  fn visit_expr_box(&mut self, i: &'ast syn::ExprBox) {
    syn::visit::visit_expr_box(self, i);
  }

  fn visit_expr_break(&mut self, i: &'ast syn::ExprBreak) {
    syn::visit::visit_expr_break(self, i);
  }

  fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
    syn::visit::visit_expr_call(self, i);
  }

  fn visit_expr_cast(&mut self, i: &'ast syn::ExprCast) {
    syn::visit::visit_expr_cast(self, i);
  }

  fn visit_expr_closure(&mut self, i: &'ast syn::ExprClosure) {
    syn::visit::visit_expr_closure(self, i);
  }

  fn visit_expr_continue(&mut self, i: &'ast syn::ExprContinue) {
    syn::visit::visit_expr_continue(self, i);
  }

  fn visit_expr_field(&mut self, i: &'ast syn::ExprField) {
    syn::visit::visit_expr_field(self, i);
  }

  fn visit_expr_for_loop(&mut self, i: &'ast syn::ExprForLoop) {
    syn::visit::visit_expr_for_loop(self, i);
  }

  fn visit_expr_group(&mut self, i: &'ast syn::ExprGroup) {
    syn::visit::visit_expr_group(self, i);
  }

  fn visit_expr_if(&mut self, i: &'ast syn::ExprIf) {
    syn::visit::visit_expr_if(self, i);
  }

  fn visit_expr_index(&mut self, i: &'ast syn::ExprIndex) {
    syn::visit::visit_expr_index(self, i);
  }

  fn visit_expr_let(&mut self, i: &'ast syn::ExprLet) {
    syn::visit::visit_expr_let(self, i);
  }

  fn visit_expr_lit(&mut self, i: &'ast syn::ExprLit) {
    syn::visit::visit_expr_lit(self, i);
  }

  fn visit_expr_loop(&mut self, i: &'ast syn::ExprLoop) {
    syn::visit::visit_expr_loop(self, i);
  }

  fn visit_expr_macro(&mut self, i: &'ast syn::ExprMacro) {
    syn::visit::visit_expr_macro(self, i);
  }

  fn visit_expr_match(&mut self, i: &'ast syn::ExprMatch) {
    syn::visit::visit_expr_match(self, i);
  }

  fn visit_expr_paren(&mut self, i: &'ast syn::ExprParen) {
    syn::visit::visit_expr_paren(self, i);
  }

  fn visit_expr_path(&mut self, i: &'ast syn::ExprPath) {
    syn::visit::visit_expr_path(self, i);
  }

  fn visit_expr_range(&mut self, i: &'ast syn::ExprRange) {
    syn::visit::visit_expr_range(self, i);
  }

  fn visit_expr_reference(&mut self, i: &'ast syn::ExprReference) {
    syn::visit::visit_expr_reference(self, i);
  }

  fn visit_expr_repeat(&mut self, i: &'ast syn::ExprRepeat) {
    syn::visit::visit_expr_repeat(self, i);
  }

  fn visit_expr_return(&mut self, i: &'ast syn::ExprReturn) {
    syn::visit::visit_expr_return(self, i);
  }

  fn visit_expr_struct(&mut self, i: &'ast syn::ExprStruct) {
    syn::visit::visit_expr_struct(self, i);
  }

  fn visit_expr_try(&mut self, i: &'ast syn::ExprTry) {
    syn::visit::visit_expr_try(self, i);
  }

  fn visit_expr_try_block(&mut self, i: &'ast syn::ExprTryBlock) {
    syn::visit::visit_expr_try_block(self, i);
  }

  fn visit_expr_tuple(&mut self, i: &'ast syn::ExprTuple) {
    syn::visit::visit_expr_tuple(self, i);
  }

  fn visit_expr_type(&mut self, i: &'ast syn::ExprType) {
    syn::visit::visit_expr_type(self, i);
  }

  fn visit_expr_unary(&mut self, i: &'ast syn::ExprUnary) {
    syn::visit::visit_expr_unary(self, i);
  }

  fn visit_expr_unsafe(&mut self, i: &'ast syn::ExprUnsafe) {
    syn::visit::visit_expr_unsafe(self, i);
  }

  fn visit_expr_while(&mut self, i: &'ast syn::ExprWhile) {
    syn::visit::visit_expr_while(self, i);
  }

  fn visit_expr_yield(&mut self, i: &'ast syn::ExprYield) {
    syn::visit::visit_expr_yield(self, i);
  }

  fn visit_field(&mut self, i: &'ast syn::Field) {
    syn::visit::visit_field(self, i);
  }

  fn visit_field_pat(&mut self, i: &'ast syn::FieldPat) {
    syn::visit::visit_field_pat(self, i);
  }

  fn visit_field_value(&mut self, i: &'ast syn::FieldValue) {
    syn::visit::visit_field_value(self, i);
  }

  fn visit_fields(&mut self, i: &'ast syn::Fields) {
    syn::visit::visit_fields(self, i);
  }

  fn visit_fields_named(&mut self, i: &'ast syn::FieldsNamed) {
    syn::visit::visit_fields_named(self, i);
  }

  fn visit_fields_unnamed(&mut self, i: &'ast syn::FieldsUnnamed) {
    syn::visit::visit_fields_unnamed(self, i);
  }

  fn visit_file(&mut self, i: &'ast syn::File) {
    syn::visit::visit_file(self, i);
  }

  fn visit_foreign_item(&mut self, i: &'ast syn::ForeignItem) {
    syn::visit::visit_foreign_item(self, i);
  }

  fn visit_foreign_item_fn(&mut self, i: &'ast syn::ForeignItemFn) {
    syn::visit::visit_foreign_item_fn(self, i);
  }

  fn visit_foreign_item_macro(&mut self, i: &'ast syn::ForeignItemMacro) {
    syn::visit::visit_foreign_item_macro(self, i);
  }

  fn visit_foreign_item_static(&mut self, i: &'ast syn::ForeignItemStatic) {
    syn::visit::visit_foreign_item_static(self, i);
  }

  fn visit_foreign_item_type(&mut self, i: &'ast syn::ForeignItemType) {
    syn::visit::visit_foreign_item_type(self, i);
  }

  fn visit_generic_argument(&mut self, i: &'ast syn::GenericArgument) {
    syn::visit::visit_generic_argument(self, i);
  }

  fn visit_generic_method_argument(
    &mut self,
    i: &'ast syn::GenericMethodArgument,
  ) {
    syn::visit::visit_generic_method_argument(self, i);
  }

  fn visit_generic_param(&mut self, i: &'ast syn::GenericParam) {
    syn::visit::visit_generic_param(self, i);
  }

  fn visit_generics(&mut self, i: &'ast syn::Generics) {
    syn::visit::visit_generics(self, i);
  }

  fn visit_ident(&mut self, i: &'ast proc_macro2::Ident) {
    syn::visit::visit_ident(self, i);
  }

  fn visit_impl_item(&mut self, i: &'ast syn::ImplItem) {
    syn::visit::visit_impl_item(self, i);
  }

  fn visit_impl_item_const(&mut self, i: &'ast syn::ImplItemConst) {
    syn::visit::visit_impl_item_const(self, i);
  }

  fn visit_impl_item_macro(&mut self, i: &'ast syn::ImplItemMacro) {
    syn::visit::visit_impl_item_macro(self, i);
  }

  fn visit_impl_item_method(&mut self, i: &'ast syn::ImplItemMethod) {
    syn::visit::visit_impl_item_method(self, i);
  }

  fn visit_impl_item_type(&mut self, i: &'ast syn::ImplItemType) {
    syn::visit::visit_impl_item_type(self, i);
  }

  fn visit_index(&mut self, i: &'ast syn::Index) {
    syn::visit::visit_index(self, i);
  }

  fn visit_item(&mut self, i: &'ast syn::Item) {
    syn::visit::visit_item(self, i);
  }

  fn visit_item_const(&mut self, i: &'ast syn::ItemConst) {
    syn::visit::visit_item_const(self, i);
  }

  fn visit_item_enum(&mut self, i: &'ast syn::ItemEnum) {
    syn::visit::visit_item_enum(self, i);
  }

  fn visit_item_extern_crate(&mut self, i: &'ast syn::ItemExternCrate) {
    syn::visit::visit_item_extern_crate(self, i);
  }

  fn visit_item_foreign_mod(&mut self, i: &'ast syn::ItemForeignMod) {
    syn::visit::visit_item_foreign_mod(self, i);
  }

  fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
    syn::visit::visit_item_impl(self, i);
  }

  fn visit_item_macro(&mut self, i: &'ast syn::ItemMacro) {
    syn::visit::visit_item_macro(self, i);
  }

  fn visit_item_macro2(&mut self, i: &'ast syn::ItemMacro2) {
    syn::visit::visit_item_macro2(self, i);
  }

  fn visit_item_mod(&mut self, i: &'ast syn::ItemMod) {
    syn::visit::visit_item_mod(self, i);
  }

  fn visit_item_static(&mut self, i: &'ast syn::ItemStatic) {
    syn::visit::visit_item_static(self, i);
  }

  fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
    syn::visit::visit_item_struct(self, i);
  }

  fn visit_item_trait(&mut self, i: &'ast syn::ItemTrait) {
    syn::visit::visit_item_trait(self, i);
  }

  fn visit_item_trait_alias(&mut self, i: &'ast syn::ItemTraitAlias) {
    syn::visit::visit_item_trait_alias(self, i);
  }

  fn visit_item_type(&mut self, i: &'ast syn::ItemType) {
    syn::visit::visit_item_type(self, i);
  }

  fn visit_item_union(&mut self, i: &'ast syn::ItemUnion) {
    syn::visit::visit_item_union(self, i);
  }

  fn visit_item_use(&mut self, i: &'ast syn::ItemUse) {
    syn::visit::visit_item_use(self, i);
  }

  fn visit_label(&mut self, i: &'ast syn::Label) {
    syn::visit::visit_label(self, i);
  }

  fn visit_lifetime(&mut self, i: &'ast syn::Lifetime) {
    syn::visit::visit_lifetime(self, i);
  }

  fn visit_lifetime_def(&mut self, i: &'ast syn::LifetimeDef) {
    syn::visit::visit_lifetime_def(self, i);
  }

  fn visit_lit(&mut self, i: &'ast syn::Lit) {
    syn::visit::visit_lit(self, i);
  }

  fn visit_lit_bool(&mut self, i: &'ast syn::LitBool) {
    syn::visit::visit_lit_bool(self, i);
  }

  fn visit_lit_byte(&mut self, i: &'ast syn::LitByte) {
    syn::visit::visit_lit_byte(self, i);
  }

  fn visit_lit_byte_str(&mut self, i: &'ast syn::LitByteStr) {
    syn::visit::visit_lit_byte_str(self, i);
  }

  fn visit_lit_char(&mut self, i: &'ast syn::LitChar) {
    syn::visit::visit_lit_char(self, i);
  }

  fn visit_lit_float(&mut self, i: &'ast syn::LitFloat) {
    syn::visit::visit_lit_float(self, i);
  }

  fn visit_lit_int(&mut self, i: &'ast syn::LitInt) {
    syn::visit::visit_lit_int(self, i);
  }

  fn visit_lit_str(&mut self, i: &'ast syn::LitStr) {
    syn::visit::visit_lit_str(self, i);
  }

  fn visit_local(&mut self, i: &'ast syn::Local) {
    syn::visit::visit_local(self, i);
  }

  fn visit_macro(&mut self, i: &'ast syn::Macro) {
    syn::visit::visit_macro(self, i);
  }

  fn visit_macro_delimiter(&mut self, i: &'ast syn::MacroDelimiter) {
    syn::visit::visit_macro_delimiter(self, i);
  }

  fn visit_member(&mut self, i: &'ast syn::Member) {
    syn::visit::visit_member(self, i);
  }

  fn visit_meta(&mut self, i: &'ast syn::Meta) {
    syn::visit::visit_meta(self, i);
  }

  fn visit_meta_list(&mut self, i: &'ast syn::MetaList) {
    syn::visit::visit_meta_list(self, i);
  }

  fn visit_meta_name_value(&mut self, i: &'ast syn::MetaNameValue) {
    syn::visit::visit_meta_name_value(self, i);
  }

  fn visit_method_turbofish(&mut self, i: &'ast syn::MethodTurbofish) {
    syn::visit::visit_method_turbofish(self, i);
  }

  fn visit_nested_meta(&mut self, i: &'ast syn::NestedMeta) {
    syn::visit::visit_nested_meta(self, i);
  }

  fn visit_parenthesized_generic_arguments(
    &mut self,
    i: &'ast syn::ParenthesizedGenericArguments,
  ) {
    syn::visit::visit_parenthesized_generic_arguments(self, i);
  }

  fn visit_pat(&mut self, i: &'ast syn::Pat) {
    syn::visit::visit_pat(self, i);
  }

  fn visit_pat_box(&mut self, i: &'ast syn::PatBox) {
    syn::visit::visit_pat_box(self, i);
  }

  fn visit_pat_ident(&mut self, i: &'ast syn::PatIdent) {
    syn::visit::visit_pat_ident(self, i);
  }

  fn visit_pat_lit(&mut self, i: &'ast syn::PatLit) {
    syn::visit::visit_pat_lit(self, i);
  }

  fn visit_pat_macro(&mut self, i: &'ast syn::PatMacro) {
    syn::visit::visit_pat_macro(self, i);
  }

  fn visit_pat_or(&mut self, i: &'ast syn::PatOr) {
    syn::visit::visit_pat_or(self, i);
  }

  fn visit_pat_path(&mut self, i: &'ast syn::PatPath) {
    syn::visit::visit_pat_path(self, i);
  }

  fn visit_pat_range(&mut self, i: &'ast syn::PatRange) {
    syn::visit::visit_pat_range(self, i);
  }

  fn visit_pat_reference(&mut self, i: &'ast syn::PatReference) {
    syn::visit::visit_pat_reference(self, i);
  }

  fn visit_pat_rest(&mut self, i: &'ast syn::PatRest) {
    syn::visit::visit_pat_rest(self, i);
  }

  fn visit_pat_slice(&mut self, i: &'ast syn::PatSlice) {
    syn::visit::visit_pat_slice(self, i);
  }

  fn visit_pat_struct(&mut self, i: &'ast syn::PatStruct) {
    syn::visit::visit_pat_struct(self, i);
  }

  fn visit_pat_tuple(&mut self, i: &'ast syn::PatTuple) {
    syn::visit::visit_pat_tuple(self, i);
  }

  fn visit_pat_tuple_struct(&mut self, i: &'ast syn::PatTupleStruct) {
    syn::visit::visit_pat_tuple_struct(self, i);
  }

  fn visit_pat_type(&mut self, i: &'ast syn::PatType) {
    syn::visit::visit_pat_type(self, i);
  }

  fn visit_pat_wild(&mut self, i: &'ast syn::PatWild) {
    syn::visit::visit_pat_wild(self, i);
  }

  fn visit_path(&mut self, i: &'ast syn::Path) {
    syn::visit::visit_path(self, i);
  }

  fn visit_path_arguments(&mut self, i: &'ast syn::PathArguments) {
    syn::visit::visit_path_arguments(self, i);
  }

  fn visit_path_segment(&mut self, i: &'ast syn::PathSegment) {
    syn::visit::visit_path_segment(self, i);
  }

  fn visit_predicate_eq(&mut self, i: &'ast syn::PredicateEq) {
    syn::visit::visit_predicate_eq(self, i);
  }

  fn visit_predicate_lifetime(&mut self, i: &'ast syn::PredicateLifetime) {
    syn::visit::visit_predicate_lifetime(self, i);
  }

  fn visit_predicate_type(&mut self, i: &'ast syn::PredicateType) {
    syn::visit::visit_predicate_type(self, i);
  }

  fn visit_qself(&mut self, i: &'ast syn::QSelf) {
    syn::visit::visit_qself(self, i);
  }

  fn visit_range_limits(&mut self, i: &'ast syn::RangeLimits) {
    syn::visit::visit_range_limits(self, i);
  }

  fn visit_receiver(&mut self, i: &'ast syn::Receiver) {
    syn::visit::visit_receiver(self, i);
  }

  fn visit_return_type(&mut self, i: &'ast syn::ReturnType) {
    syn::visit::visit_return_type(self, i);
  }

  fn visit_signature(&mut self, i: &'ast syn::Signature) {
    syn::visit::visit_signature(self, i);
  }

  fn visit_span(&mut self, i: &proc_macro2::Span) {
    syn::visit::visit_span(self, i);
  }

  fn visit_stmt(&mut self, i: &'ast syn::Stmt) {
    syn::visit::visit_stmt(self, i);
  }

  fn visit_trait_bound(&mut self, i: &'ast syn::TraitBound) {
    syn::visit::visit_trait_bound(self, i);
  }

  fn visit_trait_bound_modifier(&mut self, i: &'ast syn::TraitBoundModifier) {
    syn::visit::visit_trait_bound_modifier(self, i);
  }

  fn visit_trait_item(&mut self, i: &'ast syn::TraitItem) {
    syn::visit::visit_trait_item(self, i);
  }

  fn visit_trait_item_const(&mut self, i: &'ast syn::TraitItemConst) {
    syn::visit::visit_trait_item_const(self, i);
  }

  fn visit_trait_item_macro(&mut self, i: &'ast syn::TraitItemMacro) {
    syn::visit::visit_trait_item_macro(self, i);
  }

  fn visit_trait_item_method(&mut self, i: &'ast syn::TraitItemMethod) {
    syn::visit::visit_trait_item_method(self, i);
  }

  fn visit_trait_item_type(&mut self, i: &'ast syn::TraitItemType) {
    syn::visit::visit_trait_item_type(self, i);
  }

  fn visit_type(&mut self, i: &'ast syn::Type) {
    syn::visit::visit_type(self, i);
  }

  fn visit_type_array(&mut self, i: &'ast syn::TypeArray) {
    syn::visit::visit_type_array(self, i);
  }

  fn visit_type_bare_fn(&mut self, i: &'ast syn::TypeBareFn) {
    syn::visit::visit_type_bare_fn(self, i);
  }

  fn visit_type_group(&mut self, i: &'ast syn::TypeGroup) {
    syn::visit::visit_type_group(self, i);
  }

  fn visit_type_impl_trait(&mut self, i: &'ast syn::TypeImplTrait) {
    syn::visit::visit_type_impl_trait(self, i);
  }

  fn visit_type_infer(&mut self, i: &'ast syn::TypeInfer) {
    syn::visit::visit_type_infer(self, i);
  }

  fn visit_type_macro(&mut self, i: &'ast syn::TypeMacro) {
    syn::visit::visit_type_macro(self, i);
  }

  fn visit_type_never(&mut self, i: &'ast syn::TypeNever) {
    syn::visit::visit_type_never(self, i);
  }

  fn visit_type_param(&mut self, i: &'ast syn::TypeParam) {
    syn::visit::visit_type_param(self, i);
  }

  fn visit_type_param_bound(&mut self, i: &'ast syn::TypeParamBound) {
    syn::visit::visit_type_param_bound(self, i);
  }

  fn visit_type_paren(&mut self, i: &'ast syn::TypeParen) {
    syn::visit::visit_type_paren(self, i);
  }

  fn visit_type_path(&mut self, i: &'ast syn::TypePath) {
    syn::visit::visit_type_path(self, i);
  }

  fn visit_type_ptr(&mut self, i: &'ast syn::TypePtr) {
    syn::visit::visit_type_ptr(self, i);
  }

  fn visit_type_reference(&mut self, i: &'ast syn::TypeReference) {
    syn::visit::visit_type_reference(self, i);
  }

  fn visit_type_slice(&mut self, i: &'ast syn::TypeSlice) {
    syn::visit::visit_type_slice(self, i);
  }

  fn visit_type_trait_object(&mut self, i: &'ast syn::TypeTraitObject) {
    syn::visit::visit_type_trait_object(self, i);
  }

  fn visit_type_tuple(&mut self, i: &'ast syn::TypeTuple) {
    syn::visit::visit_type_tuple(self, i);
  }

  fn visit_un_op(&mut self, i: &'ast syn::UnOp) {
    syn::visit::visit_un_op(self, i);
  }

  fn visit_use_glob(&mut self, i: &'ast syn::UseGlob) {
    syn::visit::visit_use_glob(self, i);
  }

  fn visit_use_group(&mut self, i: &'ast syn::UseGroup) {
    syn::visit::visit_use_group(self, i);
  }

  fn visit_use_name(&mut self, i: &'ast syn::UseName) {
    syn::visit::visit_use_name(self, i);
  }

  fn visit_use_path(&mut self, i: &'ast syn::UsePath) {
    syn::visit::visit_use_path(self, i);
  }

  fn visit_use_rename(&mut self, i: &'ast syn::UseRename) {
    syn::visit::visit_use_rename(self, i);
  }

  fn visit_use_tree(&mut self, i: &'ast syn::UseTree) {
    syn::visit::visit_use_tree(self, i);
  }

  fn visit_variadic(&mut self, i: &'ast syn::Variadic) {
    syn::visit::visit_variadic(self, i);
  }

  fn visit_variant(&mut self, i: &'ast syn::Variant) {
    syn::visit::visit_variant(self, i);
  }

  fn visit_vis_crate(&mut self, i: &'ast syn::VisCrate) {
    syn::visit::visit_vis_crate(self, i);
  }

  fn visit_vis_public(&mut self, i: &'ast syn::VisPublic) {
    syn::visit::visit_vis_public(self, i);
  }

  fn visit_vis_restricted(&mut self, i: &'ast syn::VisRestricted) {
    syn::visit::visit_vis_restricted(self, i);
  }

  fn visit_visibility(&mut self, i: &'ast syn::Visibility) {
    syn::visit::visit_visibility(self, i);
  }

  fn visit_where_clause(&mut self, i: &'ast syn::WhereClause) {
    syn::visit::visit_where_clause(self, i);
  }

  fn visit_where_predicate(&mut self, i: &'ast syn::WherePredicate) {
    syn::visit::visit_where_predicate(self, i);
  }
}

impl<'ast> RuleEngine<'ast> {
  pub fn new() -> Self {
    Self {
      rules: vec![],
      context: RuleContext { ancestors: vec![] },
    }
  }

  pub fn add_rule(&mut self, rule: Box<dyn Rule<'ast>>) {
    self.rules.push(rule);
  }

  pub fn apply_rules(&mut self, syntax_tree: &'ast syn::File) {
    use syn::visit::Visit;
    self.visit_file(&syntax_tree);
  }

  fn apply_item_fn_rules(&mut self, node: &'ast syn::ItemFn) {
    self
      .rules
      .iter_mut()
      .for_each(|rule| rule.apply_item_fn(&self.context, node))
  }

  fn apply_fn_arg_rules(&mut self, node: &'ast syn::FnArg) {
    self
      .rules
      .iter_mut()
      .for_each(|rule| rule.apply_fn_arg(&self.context, node))
  }

  fn apply_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
    self
      .rules
      .iter_mut()
      .for_each(|rule| rule.apply_expr_method_call(&self.context, node))
  }

  fn apply_expr_binary(&mut self, node: &'ast syn::ExprBinary) {
    self
      .rules
      .iter_mut()
      .for_each(|rule| rule.apply_expr_binary(&self.context, node))
  }
}

#[cfg(test)]
mod tests {
  use syn::visit::Visit;

  use super::*;

  #[test]
  fn ok() {
    let mut file = std::fs::File::open(
      "/Users/paul.sastrasinh/repos/sins/sins-lib/src/lib.rs",
    )
    .unwrap();
    let mut content = String::new();
    std::io::Read::read_to_string(&mut file, &mut content).unwrap();
    let syntax_tree = syn::parse_file(&content).unwrap();

    struct FooFunctionNameRule;
    impl<'ast> Rule<'ast> for FooFunctionNameRule {
      fn apply_expr_method_call(
        &mut self,
        context: &RuleContext,
        node: &'ast syn::ExprMethodCall,
      ) {
        if node.method == "emit_report" {
          context.emit_report(Report {
            message: "bad usage".into(),
            node,
          })
        }
      }
    }
    let foo_rule = FooFunctionNameRule {};

    let mut rule_engine = RuleEngine::new();
    rule_engine.add_rule(Box::new(foo_rule));
    rule_engine.apply_rules(&syntax_tree);
  }
}
