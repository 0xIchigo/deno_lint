// Copyright 2020 the Deno authors. All rights reserved. MIT license.
use super::Context;
use super::LintRule;
use swc_ecma_ast::VarDecl;
use swc_ecma_visit::Node;
use swc_ecma_visit::Visit;

pub struct SingleVarDeclarator;

impl LintRule for SingleVarDeclarator {
  fn new() -> Box<Self> {
    Box::new(SingleVarDeclarator)
  }

  fn lint_module(&self, context: Context, module: swc_ecma_ast::Module) {
    let mut visitor = SingleVarDeclaratorVisitor::new(context);
    visitor.visit_module(&module, &module);
  }
}

pub struct SingleVarDeclaratorVisitor {
  context: Context,
}

impl SingleVarDeclaratorVisitor {
  pub fn new(context: Context) -> Self {
    Self { context }
  }
}

impl Visit for SingleVarDeclaratorVisitor {
  fn visit_var_decl(&mut self, var_decl: &VarDecl, _parent: &dyn Node) {
    if var_decl.decls.len() > 1 {
      self.context.add_diagnostic(
        var_decl.span,
        "singleVarDeclarator",
        "Multiple variable declarators are not allowed",
      );
    }
  }
}