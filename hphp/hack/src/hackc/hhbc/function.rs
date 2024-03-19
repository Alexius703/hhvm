// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.

use bitflags::bitflags;
use ffi::Vector;
use hhvm_types_ffi::ffi::Attr;
use serde::Serialize;

use crate::Attribute;
use crate::Body;
use crate::Coeffects;
use crate::FunctionName;
use crate::ParamEntry;

#[derive(Debug, Serialize)]
#[repr(C)]
pub struct Function {
    pub attributes: Vector<Attribute>,
    pub name: FunctionName,
    pub body: Body,

    pub coeffects: Coeffects,
    pub flags: FunctionFlags,
    pub attrs: Attr,
}

bitflags! {
    #[derive(Default, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    #[repr(C)]
    pub struct FunctionFlags: u8 {
        const ASYNC =          1 << 0;
        const GENERATOR =      1 << 1;
        const PAIR_GENERATOR = 1 << 2;
        const MEMOIZE_IMPL =   1 << 3;
    }
}

impl Function {
    pub fn is_async(&self) -> bool {
        self.flags.contains(FunctionFlags::ASYNC)
    }

    pub fn is_generator(&self) -> bool {
        self.flags.contains(FunctionFlags::GENERATOR)
    }

    pub fn is_pair_generator(&self) -> bool {
        self.flags.contains(FunctionFlags::PAIR_GENERATOR)
    }

    pub fn is_memoize_impl(&self) -> bool {
        self.flags.contains(FunctionFlags::MEMOIZE_IMPL)
    }

    pub fn params(&self) -> &[ParamEntry] {
        self.body.params.as_ref()
    }
}
