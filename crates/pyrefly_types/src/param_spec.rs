/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::fmt;
use std::fmt::Display;

use dupe::Dupe;
use pyrefly_derive::TypeEq;
use pyrefly_python::module::Module;
use pyrefly_util::arc_id::ArcId;
use pyrefly_util::visit::Visit;
use pyrefly_util::visit::VisitMut;
use ruff_python_ast::Identifier;

use crate::equality::TypeEq;
use crate::equality::TypeEqCtx;
use crate::qname::QName;
use crate::types::Type;

/// Used to represent ParamSpec calls. Each ParamSpec is unique, so use the ArcId to separate them.
#[derive(Clone, Dupe, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ParamSpec(ArcId<ParamSpecInner>);

impl Visit<Type> for ParamSpec {
    const RECURSE_CONTAINS: bool = false;
    fn recurse<'a>(&'a self, _: &mut dyn FnMut(&'a Type)) {}
}

impl VisitMut<Type> for ParamSpec {
    const RECURSE_CONTAINS: bool = false;
    fn recurse_mut(&mut self, _: &mut dyn FnMut(&mut Type)) {}
}

impl Display for ParamSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.qname.id())
    }
}

#[derive(Debug, PartialEq, TypeEq, Eq, Ord, PartialOrd)]
struct ParamSpecInner {
    qname: QName,
    default: Option<Type>,
}

impl ParamSpec {
    pub fn new(name: Identifier, module: Module, default: Option<Type>) -> Self {
        Self(ArcId::new(ParamSpecInner {
            qname: QName::new(name, module),
            default,
        }))
    }

    pub fn qname(&self) -> &QName {
        &self.0.qname
    }

    pub fn default(&self) -> Option<&Type> {
        self.0.default.as_ref()
    }

    pub fn to_type(&self) -> Type {
        Type::ParamSpec(self.dupe())
    }

    pub fn type_eq_inner(&self, other: &Self, ctx: &mut TypeEqCtx) -> bool {
        self.0.type_eq(&other.0, ctx)
    }
}
