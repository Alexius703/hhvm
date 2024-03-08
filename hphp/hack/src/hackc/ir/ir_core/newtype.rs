// Copyright (c) Facebook, Inc. and its affiliates. All Rights Reserved.

// Re-export some types in from hhbc so users of `ir` don't have to figure out
// which random stuff to get from `ir` and which to get elsewhere.
use bstr::BStr;
use newtype::newtype_int;

use crate::string_intern::StringInterner;
use crate::UnitBytesId;

macro_rules! interned_hhbc_intern_id {
    ($name: ident, $hhbc: ident) => {
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
        pub struct $name {
            pub id: UnitBytesId,
        }

        impl $name {
            pub fn new(id: UnitBytesId) -> Self {
                Self { id }
            }

            pub fn from_hhbc(id: hhbc::$hhbc, strings: &StringInterner) -> Self {
                Self::new(strings.intern_bytes(id.as_bytes()))
            }

            pub fn from_str(name: &str, strings: &StringInterner) -> Self {
                Self::new(strings.intern_str(name))
            }

            pub fn from_bytes(name: &[u8], strings: &StringInterner) -> Self {
                Self::new(strings.intern_bytes(name))
            }

            pub fn as_bytes<'a>(self, strings: &'a StringInterner) -> &'a [u8] {
                strings.lookup_bytes(self.id)
            }

            pub fn as_bstr<'a>(self, strings: &'a StringInterner) -> &'a BStr {
                strings.lookup_bstr(self.id)
            }
        }
    };
}

interned_hhbc_intern_id!(PropId, PropName);

// A BlockId represents a Block within a Func.
newtype_int!(BlockId, u32, BlockIdMap, BlockIdSet);

// A InstrId represents a Instr within a Func.
newtype_int!(InstrId, u32, InstrIdMap, InstrIdSet);
pub type InstrIdIndexSet = indexmap::set::IndexSet<InstrId, newtype::BuildIdHasher<u32>>;

// A ConstantId represents a Constant within a Func.
newtype_int!(ConstantId, u32, ConstantIdMap, ConstantIdSet);

// A LocId represents a SrcLoc interned within a Func.
newtype_int!(LocId, u32, LocIdMap, LocIdSet);

// A VarId represents an internal variable which is removed by the ssa pass.
// They are disjoint from LocalIds.
newtype_int!(VarId, u32, VarIdMap, VarIdSet);

/// An ValueId can be either an InstrId or a ConstantId.
///
/// Note that special care has been taken to make sure this encodes to the same
/// size as a u32:
///   InstrId values are encoded as non-negative values.
///   ConstantId values are encoded as binary negation (so negative values).
///   None is encoded as i32::MIN_INT.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct ValueId {
    raw: i32,
}

impl ValueId {
    const NONE: i32 = i32::MIN;

    pub fn raw(self) -> i32 {
        self.raw
    }

    pub fn from_instr(idx: InstrId) -> Self {
        assert!(idx != InstrId::NONE);
        let InstrId(idx) = idx;
        Self { raw: idx as i32 }
    }

    pub fn from_constant(idx: ConstantId) -> Self {
        assert!(idx != ConstantId::NONE);
        let ConstantId(idx) = idx;
        Self { raw: !(idx as i32) }
    }

    pub fn none() -> Self {
        Self { raw: Self::NONE }
    }

    pub fn expect_instr(self, msg: &str) -> InstrId {
        if self.raw >= 0 {
            InstrId(self.raw as u32)
        } else {
            panic!("{}", msg);
        }
    }

    pub fn full(self) -> FullInstrId {
        if self.raw >= 0 {
            FullInstrId::Instr(InstrId(self.raw as u32))
        } else if self.raw == Self::NONE {
            FullInstrId::None
        } else {
            FullInstrId::Constant(ConstantId((!self.raw) as u32))
        }
    }

    pub fn constant(self) -> Option<ConstantId> {
        if self.raw >= 0 || self.raw == Self::NONE {
            None
        } else {
            Some(ConstantId((!self.raw) as u32))
        }
    }

    pub fn instr(self) -> Option<InstrId> {
        if self.raw >= 0 {
            Some(InstrId(self.raw as u32))
        } else {
            None
        }
    }

    pub fn is_constant(self) -> bool {
        self.raw < 0 && self.raw != Self::NONE
    }

    pub fn is_instr(self) -> bool {
        self.raw >= 0
    }

    pub fn is_none(self) -> bool {
        self.raw == Self::NONE
    }
}

impl From<InstrId> for ValueId {
    fn from(iid: InstrId) -> ValueId {
        Self::from_instr(iid)
    }
}

impl From<ConstantId> for ValueId {
    fn from(cid: ConstantId) -> ValueId {
        Self::from_constant(cid)
    }
}

// A 'FullInstrId' can be used with match but takes more memory than
// ValueId. Note that the Constant and Instr variants will never contain
// ConstantId::NONE or InstrId::NONE.
pub enum FullInstrId {
    Instr(InstrId),
    Constant(ConstantId),
    None,
}

pub type ValueIdMap<V> = std::collections::HashMap<ValueId, V, newtype::BuildIdHasher<u32>>;
pub type ValueIdSet = std::collections::HashSet<ValueId, newtype::BuildIdHasher<u32>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlobalId {
    pub id: UnitBytesId,
}

impl GlobalId {
    pub fn new(id: UnitBytesId) -> Self {
        Self { id }
    }

    pub fn as_bytes<'a>(self, strings: &'a StringInterner) -> &'a [u8] {
        strings.lookup_bytes(self.id)
    }
}
