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

//! Transformer to transform the self.model by the actual model identifier.

use proc_macro2::Term;
use syn;
use syn::{
    Expr,
    ExprField,
    ExprPath,
    Ident,
    PathParameters,
    PathSegment,
    parse_path,
};
use syn::delimited::Element;
use syn::fold::{Folder, noop_fold_expr};
use syn::ExprKind::{Field, Path};

pub struct Transformer {
    model_ident: String,
}

impl Transformer {
    pub fn new(model_ident: &str) -> Self {
        Transformer {
            model_ident: model_ident.to_string(),
        }
    }
}

impl Folder for Transformer {
    fn fold_expr(&mut self, expr: Expr) -> Expr {
        if let Field(ExprField { expr: ref field_expr, ref field, .. }) = expr.node {
            if field.sym.as_str() == "model" {
                if let Path(ExprPath { path: syn::Path { ref segments, .. }, .. }) = field_expr.node {
                    if let Element::End(segment) = segments.get(0) {
                        if segment.ident.sym.as_str() == "self" {
                            let model_ident = Ident::new(Term::intern(&self.model_ident), field.span);
                            let node = PathSegment {
                                ident: model_ident,
                                parameters: PathParameters::None,
                            };
                            let node: syn::Path = node.into();
                            return Expr {
                                node,
                                attrs: vec![],
                            };
                        }
                    }
                }
            }
        }
        noop_fold_expr(self, expr)
    }
}
