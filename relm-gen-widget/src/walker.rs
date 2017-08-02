/*
 * Copyright (c) 2017 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

//! Visitor to get all the model attribute used in an expression.

use syn;
use syn::{Expr, ExprField, ExprPath, Ident};
use syn::delimited::Element;
use syn::ExprKind::{Field, Path};
use syn::visit::{Visitor, walk_expr};

pub struct ModelVariableVisitor {
    pub idents: Vec<Ident>,
}

impl ModelVariableVisitor {
    pub fn new() -> Self {
        ModelVariableVisitor {
            idents: vec![],
        }
    }
}

impl Visitor for ModelVariableVisitor {
    fn visit_expr(&mut self, expr: &Expr) {
        if let Field(ExprField { ref expr, ref field, .. }) = expr.node {
            // TODO: check if the right fields are chosen on the next line.
            if let Field(ExprField { ref expr, field: ref model_field, .. }) = expr.node {
                if let Expr { node: Path(ExprPath { path: syn::Path { ref segments, .. }, .. }), .. } = **expr {
                    if field.sym.as_str() == "model" {
                        if let Element::End(segment) = segments.get(0) {
                            if segment.map(|segment| &segment.ident).sym.as_str() == "self" {
                                self.idents.push(field.clone());
                            }
                        }
                    }
                }
            }
        }
        walk_expr(self, expr);
    }
}
