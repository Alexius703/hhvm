// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.

use anyhow::bail;
use anyhow::Result;
use assemble_opcode_macro::assemble_imm_for_enum;
use bumpalo::Bump;
use ffi::Str;
use ffi::Vector;
use hhbc::StringId;

use crate::assemble;
use crate::assemble::DeclMap;
use crate::lexer::Lexer;
use crate::token::Token;

assemble_imm_for_enum!(
    hhbc::BareThisOp,
    [
        BareThisOp::NeverNull,
        BareThisOp::NoNotice,
        BareThisOp::Notice
    ]
);

assemble_imm_for_enum!(
    hhbc::ClassGetCMode,
    [ClassGetCMode::Normal, ClassGetCMode::ExplicitConversion,]
);

assemble_imm_for_enum!(
    hhbc::CollectionType,
    [
        CollectionType::ImmMap,
        CollectionType::ImmSet,
        CollectionType::ImmVector,
        CollectionType::Map,
        CollectionType::Pair,
        CollectionType::Set,
        CollectionType::Vector,
    ]
);

assemble_imm_for_enum!(
    hhbc::FatalOp,
    [FatalOp::Parse, FatalOp::Runtime, FatalOp::RuntimeOmitFrame,]
);

assemble_imm_for_enum!(
    hhbc::IncDecOp,
    [
        IncDecOp::PostDec,
        IncDecOp::PostInc,
        IncDecOp::PreDec,
        IncDecOp::PreInc,
    ]
);

assemble_imm_for_enum!(
    hhbc::InitPropOp,
    [InitPropOp::NonStatic, InitPropOp::Static]
);

assemble_imm_for_enum!(
    hhbc::IsLogAsDynamicCallOp,
    [
        IsLogAsDynamicCallOp::DontLogAsDynamicCall,
        IsLogAsDynamicCallOp::LogAsDynamicCall,
    ]
);

assemble_imm_for_enum!(
    hhbc::IsTypeOp,
    [
        IsTypeOp::ArrLike,
        IsTypeOp::Bool,
        IsTypeOp::Class,
        IsTypeOp::ClsMeth,
        IsTypeOp::Dbl,
        IsTypeOp::Dict,
        IsTypeOp::Func,
        IsTypeOp::Int,
        IsTypeOp::Keyset,
        IsTypeOp::LegacyArrLike,
        IsTypeOp::Null,
        IsTypeOp::Obj,
        IsTypeOp::Res,
        IsTypeOp::Scalar,
        IsTypeOp::Str,
        IsTypeOp::Vec,
    ]
);

assemble_imm_for_enum!(
    hhbc::MOpMode,
    [
        MOpMode::Define,
        MOpMode::InOut,
        MOpMode::None,
        MOpMode::Unset,
        MOpMode::Warn,
    ]
);

assemble_imm_for_enum!(
    hhbc::ObjMethodOp,
    [ObjMethodOp::NullSafe, ObjMethodOp::NullThrows,]
);

assemble_imm_for_enum!(
    hhbc::OODeclExistsOp,
    [
        OODeclExistsOp::Class,
        OODeclExistsOp::Interface,
        OODeclExistsOp::Trait,
    ]
);

assemble_imm_for_enum!(
    hhbc::QueryMOp,
    [
        QueryMOp::CGet,
        QueryMOp::CGetQuiet,
        QueryMOp::InOut,
        QueryMOp::Isset,
    ]
);

assemble_imm_for_enum!(
    hhbc::ReadonlyOp,
    [
        ReadonlyOp::Any,
        ReadonlyOp::CheckMutROCOW,
        ReadonlyOp::CheckROCOW,
        ReadonlyOp::Mutable,
        ReadonlyOp::Readonly,
    ]
);

assemble_imm_for_enum!(
    hhbc::SetOpOp,
    [
        SetOpOp::AndEqual,
        SetOpOp::ConcatEqual,
        SetOpOp::DivEqual,
        SetOpOp::MinusEqual,
        SetOpOp::ModEqual,
        SetOpOp::MulEqual,
        SetOpOp::OrEqual,
        SetOpOp::PlusEqual,
        SetOpOp::PowEqual,
        SetOpOp::SlEqual,
        SetOpOp::SrEqual,
        SetOpOp::XorEqual,
    ]
);

assemble_imm_for_enum!(hhbc::SetRangeOp, [SetRangeOp::Forward, SetRangeOp::Reverse]);

assemble_imm_for_enum!(hhbc::SilenceOp, [SilenceOp::End, SilenceOp::Start]);

assemble_imm_for_enum!(
    hhbc::SpecialClsRef,
    [
        SpecialClsRef::LateBoundCls,
        SpecialClsRef::ParentCls,
        SpecialClsRef::SelfCls,
    ]
);

assemble_imm_for_enum!(
    hhbc::TypeStructResolveOp,
    [
        TypeStructResolveOp::DontResolve,
        TypeStructResolveOp::Resolve,
    ]
);
assemble_imm_for_enum!(
    hhbc::TypeStructEnforceKind,
    [TypeStructEnforceKind::Deep, TypeStructEnforceKind::Shallow]
);

assemble_imm_for_enum!(
    hhbc::AsTypeStructExceptionKind,
    [
        AsTypeStructExceptionKind::Error,
        AsTypeStructExceptionKind::Typehint
    ]
);

assemble_imm_for_enum!(
    hhbc::ContCheckOp,
    [ContCheckOp::IgnoreStarted, ContCheckOp::CheckStarted,]
);

pub(crate) trait AssembleImm<'arena, T> {
    fn assemble_imm(&mut self, alloc: &'arena Bump, decl_map: &DeclMap) -> Result<T>;
}

impl AssembleImm<'_, i64> for Lexer<'_> {
    fn assemble_imm(&mut self, _: &'_ Bump, _: &DeclMap) -> Result<i64> {
        self.expect_and_get_number()
    }
}

impl<'arena> AssembleImm<'arena, hhbc::AdataId> for Lexer<'_> {
    fn assemble_imm(&mut self, _: &'arena Bump, _: &DeclMap) -> Result<hhbc::AdataId> {
        let adata_id = self.expect_with(Token::into_global)?;
        debug_assert!(adata_id[0] == b'@');
        Ok(hhbc::AdataId::parse(std::str::from_utf8(&adata_id[1..])?)?)
    }
}

impl<'arena> AssembleImm<'arena, hhbc::ClassName> for Lexer<'_> {
    fn assemble_imm(&mut self, _: &'arena Bump, _: &DeclMap) -> Result<hhbc::ClassName> {
        Ok(hhbc::ClassName::new(
            assemble::assemble_unescaped_unquoted_intern_str(self)?,
        ))
    }
}

impl<'arena> AssembleImm<'arena, hhbc::ConstName> for Lexer<'_> {
    fn assemble_imm(&mut self, _: &'arena Bump, _: &DeclMap) -> Result<hhbc::ConstName> {
        Ok(hhbc::ConstName::new(
            assemble::assemble_unescaped_unquoted_intern_str(self)?,
        ))
    }
}

impl<'arena> AssembleImm<'arena, hhbc::FCallArgs> for Lexer<'_> {
    fn assemble_imm(&mut self, _: &'arena Bump, _: &DeclMap) -> Result<hhbc::FCallArgs> {
        // <(fcargflags)*> numargs numrets inouts readonly async_eager_target context
        let fcargflags = assemble::assemble_fcallargsflags(self)?;
        let num_args = self.expect_and_get_number()?;
        let num_rets = self.expect_and_get_number()?;
        let inouts = assemble::assemble_inouts_or_readonly(self)?;
        let readonly = assemble::assemble_inouts_or_readonly(self)?;
        let async_eager_target = assemble::assemble_async_eager_target(self)?;
        let context = assemble::assemble_fcall_context(self)?;
        let fcargs = hhbc::FCallArgs::new(
            fcargflags,
            num_rets,
            num_args,
            inouts,
            readonly,
            async_eager_target,
            None,
        );
        Ok(hhbc::FCallArgs { context, ..fcargs })
    }
}

impl AssembleImm<'_, hhbc::FloatBits> for Lexer<'_> {
    fn assemble_imm(&mut self, _: &'_ Bump, _: &DeclMap) -> Result<hhbc::FloatBits> {
        Ok(hhbc::FloatBits(self.expect_and_get_number()?))
    }
}

impl<'arena> AssembleImm<'arena, hhbc::FunctionName> for Lexer<'_> {
    fn assemble_imm(&mut self, _: &'arena Bump, _: &DeclMap) -> Result<hhbc::FunctionName> {
        Ok(hhbc::FunctionName::new(
            assemble::assemble_unescaped_unquoted_intern_str(self)?,
        ))
    }
}

impl AssembleImm<'_, hhbc::IterArgs> for Lexer<'_> {
    fn assemble_imm(&mut self, alloc: &'_ Bump, decl_map: &DeclMap) -> Result<hhbc::IterArgs> {
        // IterArg { iter_id: IterId (~u32), key_id: Local, val_id: Local}
        // Ex: 0 NK V:$v
        let idx: u32 = self.expect_and_get_number()?;
        let tok = self.expect_token()?;
        let key_id: hhbc::Local = match tok.into_identifier()? {
            b"NK" => hhbc::Local::INVALID,
            b"K" => {
                self.expect(Token::is_colon)?;
                self.assemble_imm(alloc, decl_map)?
            }
            _ => return Err(tok.error("Invalid key_id given as iter args to IterArg")),
        };
        self.expect_str(Token::is_identifier, "V")?;
        self.expect(Token::is_colon)?;
        let iter_id = hhbc::IterId { idx };
        let val_id = self.assemble_imm(alloc, decl_map)?;
        Ok(hhbc::IterArgs {
            iter_id,
            key_id,
            val_id,
        })
    }
}

impl AssembleImm<'_, hhbc::IterId> for Lexer<'_> {
    fn assemble_imm(&mut self, _: &'_ Bump, _: &DeclMap) -> Result<hhbc::IterId> {
        Ok(hhbc::IterId {
            idx: self.expect_and_get_number()?,
        })
    }
}

impl AssembleImm<'_, hhbc::Label> for Lexer<'_> {
    fn assemble_imm(&mut self, _: &'_ Bump, _: &DeclMap) -> Result<hhbc::Label> {
        assemble::assemble_label(self)
    }
}

impl AssembleImm<'_, hhbc::Local> for Lexer<'_> {
    fn assemble_imm(&mut self, _: &'_ Bump, decl_map: &DeclMap) -> Result<hhbc::Local> {
        // Returns the local (u32 idx) a var or unnamed corresponds to.
        // This information is based on the position of the var in parameters of a function/.declvars
        // or, if an unnamed, just the idx referenced (_1 -> idx 1)
        // $a -> idx where $a is stored in hcu body
        // _3 -> 3
        match self.next() {
            Some(Token::Variable(v, p)) => {
                let v = hhbc::intern(std::str::from_utf8(v)?);
                if let Some(idx) = decl_map.get(&v) {
                    Ok(hhbc::Local { idx: *idx })
                } else {
                    bail!("Unknown local var: {:?} at {:?}", v, p);
                }
            }
            Some(Token::Identifier(i, _)) => {
                debug_assert!(i[0] == b'_');
                Ok(hhbc::Local {
                    idx: std::str::from_utf8(&i[1..i.len()])?.parse()?,
                })
            }
            Some(tok) => Err(tok.error("Unknown local")),
            None => Err(self.error("Expected local")),
        }
    }
}

impl AssembleImm<'_, hhbc::LocalRange> for Lexer<'_> {
    fn assemble_imm(&mut self, _: &'_ Bump, _: &DeclMap) -> Result<hhbc::LocalRange> {
        self.expect_str(Token::is_identifier, "L")?;
        self.expect(Token::is_colon)?;
        let start = hhbc::Local {
            idx: self.expect_and_get_number()?,
        };
        //self.expect(Token::is_plus)?; // Not sure if this exists yet
        let len = self.expect_and_get_number()?;
        Ok(hhbc::LocalRange { start, len })
    }
}

impl<'arena> AssembleImm<'arena, hhbc::MemberKey> for Lexer<'_> {
    fn assemble_imm(&mut self, alloc: &'arena Bump, decl_map: &DeclMap) -> Result<hhbc::MemberKey> {
        // EC: stackIndex readOnlyOp | EL: local readOnlyOp | ET: string readOnlyOp | EI: int readOnlyOp
        // PC: stackIndex readOnlyOp | PL: local readOnlyOp | PT: propName readOnlyOp | QT: propName readOnlyOp
        let tok = self.expect_token()?;
        match tok.into_identifier()? {
            b"EC" => {
                self.expect(Token::is_colon)?;
                Ok(hhbc::MemberKey::EC(
                    self.assemble_imm(alloc, decl_map)?,
                    self.assemble_imm(alloc, decl_map)?,
                ))
            }
            b"EL" => {
                self.expect(Token::is_colon)?;
                Ok(hhbc::MemberKey::EL(
                    self.assemble_imm(alloc, decl_map)?,
                    self.assemble_imm(alloc, decl_map)?,
                ))
            }
            b"ET" => {
                self.expect(Token::is_colon)?;
                Ok(hhbc::MemberKey::ET(
                    hhbc::intern_bytes(escaper::unescape_literal_bytes_into_vec_bytes(
                        // In bp, print_quoted_str also escapes the string
                        escaper::unquote_slice(self.expect_with(Token::into_str_literal)?),
                    )?),
                    self.assemble_imm(alloc, decl_map)?,
                ))
            }
            b"EI" => {
                self.expect(Token::is_colon)?;
                Ok(hhbc::MemberKey::EI(
                    self.expect_and_get_number()?,
                    self.assemble_imm(alloc, decl_map)?,
                ))
            }
            b"PC" => {
                self.expect(Token::is_colon)?;
                Ok(hhbc::MemberKey::PC(
                    self.assemble_imm(alloc, decl_map)?,
                    self.assemble_imm(alloc, decl_map)?,
                ))
            }
            b"PL" => {
                self.expect(Token::is_colon)?;
                Ok(hhbc::MemberKey::PL(
                    self.assemble_imm(alloc, decl_map)?,
                    self.assemble_imm(alloc, decl_map)?,
                ))
            }
            b"PT" => {
                self.expect(Token::is_colon)?;
                Ok(hhbc::MemberKey::PT(
                    assemble::assemble_prop_name_from_str(self)?,
                    self.assemble_imm(alloc, decl_map)?,
                ))
            }
            b"QT" => {
                self.expect(Token::is_colon)?;
                Ok(hhbc::MemberKey::QT(
                    assemble::assemble_prop_name_from_str(self)?,
                    self.assemble_imm(alloc, decl_map)?,
                ))
            }
            b"W" => Ok(hhbc::MemberKey::W),
            _ => Err(tok.error("Expected a MemberKey")),
        }
    }
}

impl<'arena> AssembleImm<'arena, hhbc::MethodName> for Lexer<'_> {
    fn assemble_imm(&mut self, _: &'arena Bump, _: &DeclMap) -> Result<hhbc::MethodName> {
        Ok(hhbc::MethodName::new(
            assemble::assemble_unescaped_unquoted_intern_str(self)?,
        ))
    }
}

impl<'arena> AssembleImm<'arena, hhbc::PropName> for Lexer<'_> {
    fn assemble_imm(&mut self, _: &'arena Bump, _: &DeclMap) -> Result<hhbc::PropName> {
        Ok(hhbc::PropName::new(
            assemble::assemble_unescaped_unquoted_intern_str(self)?,
        ))
    }
}

impl<'arena> AssembleImm<'arena, Vector<hhbc::Label>> for Lexer<'_> {
    fn assemble_imm(&mut self, _: &'arena Bump, _: &DeclMap) -> Result<Vector<hhbc::Label>> {
        let mut labels = Vec::new();
        self.expect(Token::is_lt)?;
        while !self.peek_is(Token::is_gt) {
            labels.push(assemble::assemble_label(self)?)
        }
        self.expect(Token::is_gt)?;
        Ok(labels.into())
    }
}

impl<'arena> AssembleImm<'arena, Vector<Str<'arena>>> for Lexer<'_> {
    fn assemble_imm(&mut self, alloc: &'arena Bump, _: &DeclMap) -> Result<Vector<Str<'arena>>> {
        self.expect(Token::is_lt)?;
        let mut d = Vec::new();
        while !self.peek_is(Token::is_gt) {
            d.push(assemble::assemble_unescaped_unquoted_str(alloc, self)?);
        }
        self.expect(Token::is_gt)?;
        Ok(d.into())
    }
}

impl AssembleImm<'_, hhbc::StackIndex> for Lexer<'_> {
    fn assemble_imm(&mut self, _: &'_ Bump, _: &DeclMap) -> Result<hhbc::StackIndex> {
        // StackIndex : u32
        self.expect_and_get_number()
    }
}

impl<'arena> AssembleImm<'arena, Str<'arena>> for Lexer<'_> {
    fn assemble_imm(&mut self, alloc: &'arena Bump, _: &DeclMap) -> Result<Str<'arena>> {
        assemble::assemble_unescaped_unquoted_str(alloc, self)
    }
}

impl<'arena> AssembleImm<'arena, StringId> for Lexer<'_> {
    fn assemble_imm(&mut self, _: &'arena Bump, _: &DeclMap) -> Result<StringId> {
        assemble::assemble_unescaped_unquoted_intern_str(self)
    }
}

impl AssembleImm<'_, hhbc::SwitchKind> for Lexer<'_> {
    fn assemble_imm(&mut self, _: &'_ Bump, _: &DeclMap) -> Result<hhbc::SwitchKind> {
        let tok = self.expect_token()?;
        match tok.into_identifier()? {
            b"Unbounded" => Ok(hhbc::SwitchKind::Unbounded),
            b"Bounded" => Ok(hhbc::SwitchKind::Bounded),
            _ => Err(tok.error("Unknown switch kind")),
        }
    }
}
