use crate::result::Result;
use crate::ty_env::TyEnv;
// use crate::utils::full_type_name;
use wiz_arena::Arena;
use wiz_hir::typed_decl::{TypedDeclKind, TypedTopLevelDecl, TypedVar};
use wiz_hir::typed_expr::{TypedExpr, TypedExprKind};
use wiz_hir::typed_file::TypedSpellBook;
use wiz_hir::typed_type::TypedType;
use wiz_syntax::syntax::WizFile;

pub(crate) fn infer_source_set(
    source_set: WizFile,
    arena: &Arena,
    ty_env: &TyEnv,
) -> Result<TypedSpellBook> {
    Ok(TypedSpellBook {
        name: "".to_string(),
        uses: vec![],
        body: vec![],
    })
    // Ok(match source_set {
    //     TypedSourceSet::File(f) => ty_env.new_scope::<Result<_>, _>(|ty_env| {
    //         Ok(TypedSourceSet::File(infer_file(f, arena, &ty_env)?))
    //     })?,
    //     TypedSourceSet::Dir { name, items } => TypedSourceSet::Dir {
    //         name,
    //         items: ty_env.new_scope(|ty_env| {
    //             items
    //                 .into_iter()
    //                 .map(|s| ty_env.new_scope(|ty_env| infer_source_set(s, arena, &ty_env)))
    //                 .collect::<Result<_>>()
    //         })?,
    //     },
    // })
}

// fn infer_declaration(d: TypedDecl, arena: &Arena, ty_env: &TyEnv) -> Result<TypedDecl> {
//     Ok(TypedDecl {
//         annotations: d.annotations,
//         package: d.package,
//         modifiers: d.modifiers,
//         kind: match d.kind {
//             TypedDeclKind::Var(v) => TypedDeclKind::Var(infer_typed_var(v, arena, ty_env)?),
//             TypedDeclKind::Fun(f) => TypedDeclKind::Fun(f),
//             TypedDeclKind::Struct(s) => TypedDeclKind::Struct(s),
//             TypedDeclKind::Class => TypedDeclKind::Class,
//             TypedDeclKind::Enum => TypedDeclKind::Enum,
//             TypedDeclKind::Protocol(p) => TypedDeclKind::Protocol(p),
//             TypedDeclKind::Extension(e) => TypedDeclKind::Extension(e),
//         },
//     })
// }

// fn infer_typed_var(t: TypedVar, arena: &Arena, ty_env: &TyEnv) -> Result<TypedVar> {
//     let TypedVar {
//         is_mut,
//         name,
//         type_,
//         value,
//     } = t;
//     let value = expr(
//         value,
//         match type_ {
//             Some(type_) => Some(full_type_name(arena, ty_env,&type_)?),
//             None => None,
//         },
//         arena, ty_env
//     )?;
//     let v = TypedVar {
//         is_mut,
//         name,
//         type_: value.ty.clone(),
//         value,
//     };
//     Ok(v)
// }

fn expr(
    e: TypedExpr,
    type_annotation: Option<TypedType>,
    arena: &Arena,
    ty_env: &TyEnv,
) -> Result<TypedExpr> {
    Ok(e)
    // let TypedExpr { kind, ty } = e;
    // Ok(match kind {
    //     TypedExprKind::Name(n) => {
    //         let (kind, ty) = self.typed_name(n, ty, type_annotation)?;
    //         TypedExpr::new(TypedExprKind::Name(kind), ty)
    //     }
    //     TypedExprKind::Literal(l) => {
    //         let (kind, ty) = self.typed_literal(l, ty, type_annotation)?;
    //         TypedExpr::new(TypedExprKind::Literal(kind), ty)
    //     }
    //     TypedExprKind::BinOp(b) => {
    //         let (kind, ty) = self.typed_binop(b)?;
    //         TypedExpr::new(TypedExprKind::BinOp(kind), ty)
    //     }
    //     TypedExprKind::UnaryOp(u) => {
    //         let (kind, ty) = self.typed_unary_op(u)?;
    //         TypedExpr::new(TypedExprKind::UnaryOp(kind), ty)
    //     }
    //     TypedExprKind::Subscript(s) => {
    //         let (kind, ty) = self.typed_subscript(s, ty)?;
    //         TypedExpr::new(TypedExprKind::Subscript(kind), ty)
    //     }
    //     TypedExprKind::Member(m) => {
    //         let (kind, ty) = self.typed_instance_member(m)?;
    //         TypedExpr::new(TypedExprKind::Member(kind), ty)
    //     }
    //     TypedExprKind::Array(a) => {
    //         let (kind, ty) = self.typed_array(a)?;
    //         TypedExpr::new(TypedExprKind::Array(kind), ty)
    //     }
    //     TypedExprKind::Tuple => TypedExpr::new(TypedExprKind::Tuple, None),
    //     TypedExprKind::Dict => TypedExpr::new(TypedExprKind::Dict, None),
    //     TypedExprKind::StringBuilder => TypedExpr::new(TypedExprKind::StringBuilder, None),
    //     TypedExprKind::Call(c) => {
    //         let (kind, ty) = self.typed_call(c)?;
    //         TypedExpr::new(TypedExprKind::Call(kind), ty)
    //     }
    //     TypedExprKind::If(i) => {
    //         let (kind, ty) = self.typed_if(i)?;
    //         TypedExpr::new(TypedExprKind::If(kind), ty)
    //     }
    //     TypedExprKind::When => TypedExpr::new(TypedExprKind::When, None),
    //     TypedExprKind::Lambda(l) => TypedExpr::new(TypedExprKind::Lambda(l), None),
    //     TypedExprKind::Return(r) => {
    //         let (kind, ty) = self.typed_return(r)?;
    //         TypedExpr::new(TypedExprKind::Return(kind), ty)
    //     }
    //     TypedExprKind::TypeCast(t) => {
    //         let (kind, ty) = self.typed_type_cast(t)?;
    //         TypedExpr::new(TypedExprKind::TypeCast(kind), ty)
    //     }
    //     TypedExprKind::SizeOf(size_of) => TypedExpr::new(
    //         TypedExprKind::SizeOf(self.context.full_type_name(&size_of)?),
    //         ty,
    //     ),
    // })
}
