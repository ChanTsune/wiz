use crate::high_level_ir::node_id::ModuleId;
use crate::high_level_ir::type_resolver::TypeResolver;
use std::collections::HashMap;
use wiz_arena::{Arena, DeclarationId};
use wiz_hir::typed_annotation::TypedAnnotations;
use wiz_hir::typed_decl::{
    TypedArgDef, TypedComputedProperty, TypedDeclKind, TypedExtension, TypedFun, TypedFunBody,
    TypedProtocol, TypedStoredProperty, TypedStruct, TypedTopLevelDecl, TypedVar,
};
use wiz_hir::typed_expr::{
    TypedArray, TypedBinOp, TypedBinaryOperator, TypedCall, TypedCallArg, TypedExpr, TypedExprKind,
    TypedIf, TypedInstanceMember, TypedLambda, TypedLiteralKind, TypedName, TypedPostfixUnaryOp,
    TypedPostfixUnaryOperator, TypedPrefixUnaryOp, TypedPrefixUnaryOperator, TypedReturn,
    TypedSubscript, TypedTypeCast, TypedUnaryOp,
};
use wiz_hir::typed_file::TypedSpellBook;
use wiz_hir::typed_stmt::{
    TypedAssignment, TypedAssignmentAndOperation, TypedAssignmentAndOperator, TypedAssignmentStmt,
    TypedBlock, TypedForStmt, TypedLoopStmt, TypedStmt, TypedWhileLoopStmt,
};
use wiz_hir::typed_type::{
    Package, TypedNamedValueType, TypedPackage, TypedType, TypedTypeParam, TypedValueType,
};
use wiz_hir::typed_type_constraint::TypedTypeConstraint;
use wiz_hir::typed_use::TypedUse;
use wiz_result::Result;
use wiz_session::Session;
use wiz_syntax::syntax::annotation::AnnotationsSyntax;
use wiz_syntax::syntax::block::BlockSyntax;
use wiz_syntax::syntax::declaration::fun_syntax::{ArgDef, FunBody, FunSyntax};
use wiz_syntax::syntax::declaration::{
    DeclKind, StoredPropertySyntax, StructPropertySyntax, StructSyntax, UseSyntax,
};
use wiz_syntax::syntax::declaration::{ExtensionSyntax, VarSyntax};
use wiz_syntax::syntax::expression::{
    ArraySyntax, BinaryOperationSyntax, CallExprSyntax, Expr, IfExprSyntax, LambdaSyntax,
    MemberSyntax, NameExprSyntax, PostfixUnaryOperationSyntax, PrefixUnaryOperationSyntax,
    ReturnSyntax, SubscriptSyntax, TypeCastSyntax, UnaryOperationSyntax,
};
use wiz_syntax::syntax::literal::LiteralSyntax;
use wiz_syntax::syntax::statement::{
    AssignmentStmt, ForLoopSyntax, LoopStmt, Stmt, WhileLoopSyntax,
};
use wiz_syntax::syntax::type_name::{TypeName, TypeParam, UserTypeName};
use wiz_syntax::syntax::WizFile;
use wiz_syntax_parser::parser::wiz::parse_from_file_path;

pub mod node_id;
pub mod type_checker;
pub mod type_resolver;
pub mod wlib;

pub fn ast2hlir(
    session: &mut Session,
    arena: &mut Arena,
    s: WizFile,
    module_id: ModuleId,
) -> TypedSpellBook {
    let mut converter = AstLowering::new(session, arena);
    converter.lowing(s, module_id).unwrap()
}

pub struct AstLowering<'a> {
    session: &'a mut Session,
    arena: &'a mut Arena,
    namespace_id: DeclarationId,
}

impl<'a> AstLowering<'a> {
    pub fn new(session: &'a mut Session, arena: &'a mut Arena) -> Self {
        Self {
            session,
            arena,
            namespace_id: DeclarationId::ROOT,
        }
    }

    fn push_namespace<T, F: FnOnce(&mut Self) -> T>(&mut self, name: &str, f: F) -> T {
        let parent = self.namespace_id;

        self.namespace_id = self
            .arena
            .resolve_declaration_id(parent, &[name])
            .unwrap_or_else(|| {
                self.arena
                    .register_namespace(&parent, name, Default::default())
                    .unwrap_or_else(|| panic!("Can not create {}", name))
            });

        let result = f(self);

        self.namespace_id = parent;
        result
    }

    pub fn lowing(&mut self, s: WizFile, module_id: ModuleId) -> Result<TypedSpellBook> {
        let file = self.file(s);

        let mut resolver = TypeResolver::new(self.session, self.arena);

        // NOTE: detect decl names
        resolver.preload_file(&file)?;

        let file = resolver.file(file)?;
        Ok(file)
    }

    fn file(&mut self, f: WizFile) -> TypedSpellBook {
        let WizFile { name, syntax } = f;

        let (uses, body) = self.push_namespace(&name, |slf| {
            // NOTE: Inject default uses
            let mut uses = vec![
                TypedUse::from(vec!["core", "builtin", "*"]),
                TypedUse::from(vec!["std", "builtin", "*"]),
            ];
            let mut others = vec![];
            for l in syntax.body.into_iter() {
                if let DeclKind::Use(u) = l.kind {
                    uses.push(slf.use_syntax(u, l.annotations));
                } else if let DeclKind::Struct(s) = &l.kind {
                    let annotation = slf.annotations(&l.annotations);
                    match s.struct_keyword.token().as_str() {
                        "struct" => slf.arena.register_struct(
                            &slf.namespace_id,
                            &s.name.token(),
                            annotation,
                        ),
                        "protocol" => slf.arena.register_protocol(
                            &slf.namespace_id,
                            &s.name.token(),
                            annotation,
                        ),
                        _ => unreachable!(),
                    };
                    others.push(l);
                } else {
                    others.push(l);
                }
            }
            (
                uses,
                others
                    .into_iter()
                    .map(|d| slf.decl(d.kind, d.annotations))
                    .collect::<Vec<_>>(),
            )
        });
        TypedSpellBook { name, uses, body }
    }

    fn annotations(&mut self, a: &Option<AnnotationsSyntax>) -> TypedAnnotations {
        match a {
            None => TypedAnnotations::default(),
            Some(a) => TypedAnnotations::from(
                a.elements
                    .iter()
                    .map(|a| a.element.token())
                    .collect::<Vec<_>>(),
            ),
        }
    }

    fn stmt(&mut self, s: Stmt) -> TypedStmt {
        match s {
            Stmt::Decl(decl) => TypedStmt::Decl(self.decl(decl.kind, decl.annotations)),
            Stmt::Expr(expr) => TypedStmt::Expr(self.expr(expr)),
            Stmt::Assignment(a) => TypedStmt::Assignment(self.assignment(a)),
            Stmt::Loop(l) => TypedStmt::Loop(self.loop_stmt(l)),
        }
    }

    fn assignment(&mut self, a: AssignmentStmt) -> TypedAssignmentStmt {
        match a {
            AssignmentStmt::Assignment(a) => TypedAssignmentStmt::Assignment(TypedAssignment {
                target: self.expr(a.target),
                value: self.expr(a.value),
            }),
            AssignmentStmt::AssignmentAndOperator(a) => {
                TypedAssignmentStmt::AssignmentAndOperation(TypedAssignmentAndOperation {
                    target: self.expr(a.target),
                    operator: match &*a.operator.token() {
                        "+=" => TypedAssignmentAndOperator::Add,
                        "-=" => TypedAssignmentAndOperator::Sub,
                        "*=" => TypedAssignmentAndOperator::Mul,
                        "/=" => TypedAssignmentAndOperator::Div,
                        "%=" => TypedAssignmentAndOperator::Mod,
                        o => panic!("unknown operator {:?}", o),
                    },
                    value: self.expr(a.value),
                })
            }
        }
    }

    fn loop_stmt(&mut self, l: LoopStmt) -> TypedLoopStmt {
        match l {
            LoopStmt::While(WhileLoopSyntax {
                while_keyword: _,
                condition,
                block,
            }) => TypedLoopStmt::While(TypedWhileLoopStmt {
                condition: self.expr(condition),
                block: self.block(block),
            }),
            LoopStmt::For(ForLoopSyntax {
                for_keyword: _,
                values,
                in_keyword: _,
                iterator,
                block,
            }) => TypedLoopStmt::For(TypedForStmt {
                values: values.into_iter().map(|i| i.token()).collect(),
                iterator: self.expr(iterator),
                block: self.block(block),
            }),
        }
    }

    fn decl(&mut self, d: DeclKind, annotation: Option<AnnotationsSyntax>) -> TypedTopLevelDecl {
        TypedTopLevelDecl {
            annotations: self.annotations(&annotation),
            package: Package::from(&self.arena.resolve_fully_qualified_name(&self.namespace_id)),
            modifiers: vec![],
            kind: match d {
                DeclKind::Var(v) => TypedDeclKind::Var(self.var_syntax(v)),
                DeclKind::Fun(f) => TypedDeclKind::Fun(self.fun_syntax(f)),
                DeclKind::Struct(s) => match &*s.struct_keyword.token() {
                    "struct" => TypedDeclKind::Struct(self.struct_syntax(s)),
                    "protocol" => TypedDeclKind::Protocol(self.protocol_syntax(s)),
                    kw => panic!("Unknown keyword `{}`", kw),
                },
                DeclKind::ExternC { .. } => todo!(),
                DeclKind::Enum { .. } => TypedDeclKind::Enum,
                DeclKind::Extension(e) => TypedDeclKind::Extension(self.extension_syntax(e)),
                DeclKind::Use(_) => unreachable!(),
                DeclKind::Module(m) => {
                    let (name, file) = m;
                    let file = match file {
                        Some(file) => WizFile { name, syntax: file },
                        None => {
                            let mut s = self.session.local_spell_book_root().to_owned();
                            let fqn = self.arena.resolve_fully_qualified_name(&self.namespace_id);
                            for n in &fqn[1..] {
                                s = s.join(n);
                            }
                            s.set_extension("wiz");
                            println!("Module: {}", s.display());
                            parse_from_file_path(&self.session.parse_session, s, Some(&name)).unwrap()
                        }
                    };
                    TypedDeclKind::Module(self.file(file))
                }
            },
        }
    }

    pub fn var_syntax(&mut self, v: VarSyntax) -> TypedVar {
        let expr = self.expr(v.value);
        TypedVar {
            is_mut: v.mutability_keyword.token() == "var",
            name: v.name.token(),
            type_: v.type_annotation.map(|t| self.type_(t.type_)),
            value: expr,
        }
    }

    pub fn arg_def(&mut self, a: ArgDef) -> TypedArgDef {
        match a {
            ArgDef::Value(a) => TypedArgDef {
                label: match a.label {
                    None => a.name.token(),
                    Some(label) => label.token(),
                },
                name: a.name.token(),
                type_: self.type_(a.type_name),
            },
            ArgDef::Self_(s) => match s.reference {
                None => TypedArgDef {
                    label: "_".to_string(),
                    name: "self".to_string(),
                    type_: TypedType::Self_,
                },
                Some(_) => TypedArgDef {
                    label: "_".to_string(),
                    name: "self".to_string(),
                    type_: TypedType::Self_, // TODO: Reference
                },
            },
        }
    }

    pub fn fun_body(&mut self, body: FunBody) -> TypedFunBody {
        match body {
            FunBody::Block(block) => TypedFunBody::Block(self.block(block)),
            FunBody::Expr(expr) => TypedFunBody::Expr(self.expr(expr.expr)),
        }
    }

    pub fn fun_syntax(&mut self, f: FunSyntax) -> TypedFun {
        let FunSyntax {
            fun_keyword: _,
            name,
            type_params,
            arg_defs,
            return_type,
            type_constraints,
            body,
        } = f;
        let args: Vec<TypedArgDef> = arg_defs
            .elements
            .into_iter()
            .map(|a| self.arg_def(a.element))
            .collect();

        let simple_type_constraints = type_params.as_ref().map(|t| {
            t.elements
                .iter()
                .map(|t| t.element.clone())
                .collect::<Vec<_>>()
        });

        let type_constraints = type_constraints.map(|t| {
            t.type_constraints
                .into_iter()
                .map(|t| t.element)
                .collect::<Vec<_>>()
        });

        let type_constraints = match (simple_type_constraints, type_constraints) {
            (Some(mut a), Some(b)) => Some({
                a.extend(b);
                a
            }),
            (Some(a), _) | (_, Some(a)) => Some(a),
            (_, _) => None,
        }
        .map(|type_constraints| {
            let mut group = HashMap::<_, Vec<_>>::new();
            for type_constraint in type_constraints {
                let name = type_constraint.name.token();
                group
                    .entry(name)
                    .or_default()
                    .push(type_constraint.type_constraint);
            }
            group
                .into_iter()
                .map(|(k, v)| TypedTypeConstraint {
                    type_: TypedType::Type(Box::new(TypedType::Value(TypedValueType::Value(
                        TypedNamedValueType {
                            package: TypedPackage::Raw(Package::global()),
                            name: k,
                            type_args: None,
                        },
                    )))),
                    constraints: v
                        .into_iter()
                        .flatten()
                        .map(|s| self.type_(s.constraint))
                        .collect(),
                })
                .collect()
        });

        let body = body.map(|b| self.fun_body(b));

        TypedFun {
            name: name.token(),
            type_params: type_params.map(|v| {
                v.elements
                    .into_iter()
                    .map(|p| TypedTypeParam {
                        name: p.element.name.token(),
                    })
                    .collect()
            }),
            type_constraints,
            arg_defs: args,
            body,
            return_type: return_type
                .map(|t| self.type_(t.type_))
                .unwrap_or_else(TypedType::unit),
        }
    }

    pub fn type_(&mut self, tn: TypeName) -> TypedType {
        match tn {
            TypeName::Simple(stn) => {
                if stn.name.token() == "Self" {
                    TypedType::Self_
                } else {
                    TypedType::Value(TypedValueType::Value(TypedNamedValueType {
                        package: TypedPackage::Raw(Package::new()),
                        name: stn.name.token(),
                        type_args: stn.type_args.map(|v| {
                            v.elements
                                .into_iter()
                                .map(|t| self.type_(t.element))
                                .collect()
                        }),
                    }))
                }
            }
            TypeName::Decorated(d) => {
                let t = self.type_(d.type_);
                match &*d.decoration.token() {
                    "&" => TypedType::reference(t),
                    "*" => TypedType::unsafe_pointer(t),
                    a => panic!("Unexpected token {}", a),
                }
            }
            TypeName::NameSpaced(n) => {
                let UserTypeName {
                    name_space,
                    type_name,
                } = *n;
                TypedType::Value(TypedValueType::Value(TypedNamedValueType {
                    package: TypedPackage::Raw(Package::from(
                        &name_space
                            .iter()
                            .map(|i| i.simple_type.name.token())
                            .collect::<Vec<_>>(),
                    )),
                    name: type_name.name.token(),
                    type_args: type_name.type_args.map(|v| {
                        v.elements
                            .into_iter()
                            .map(|t| self.type_(t.element))
                            .collect()
                    }),
                }))
            }
            TypeName::Parenthesized(p) => self.type_(*p.type_name),
            TypeName::Array(a) => TypedType::Value(TypedValueType::Array(
                Box::new(self.type_(a.type_)),
                a.size.token().parse().unwrap(),
            )),
        }
    }

    fn type_param(&mut self, tp: TypeParam) -> TypedTypeParam {
        TypedTypeParam {
            name: tp.name.token(),
        }
    }

    pub fn struct_syntax(&mut self, s: StructSyntax) -> TypedStruct {
        let mut stored_properties: Vec<TypedStoredProperty> = vec![];
        let mut computed_properties: Vec<TypedComputedProperty> = vec![];
        let mut member_functions: Vec<TypedFun> = vec![];
        for p in s.body.properties {
            match p {
                StructPropertySyntax::StoredProperty(v) => {
                    stored_properties.push(self.stored_property_syntax(v));
                }
                StructPropertySyntax::ComputedProperty => {}
                StructPropertySyntax::Method(method) => {
                    member_functions.push(self.member_function(method))
                }
                StructPropertySyntax::Deinit(deinit) => {
                    todo!("deinit {:?}", deinit)
                }
            };
        }

        // add size_of struct
        member_functions.push(TypedFun::size(TypedType::Self_));

        TypedStruct {
            name: s.name.token(),
            type_params: s.type_params.map(|v| {
                v.elements
                    .into_iter()
                    .map(|tp| self.type_param(tp.element))
                    .collect()
            }),
            stored_properties,
            computed_properties,
            member_functions,
        }
    }

    pub fn stored_property_syntax(&mut self, p: StoredPropertySyntax) -> TypedStoredProperty {
        TypedStoredProperty {
            name: p.name.token(),
            type_: self.type_(p.type_.type_),
        }
    }

    pub fn member_function(&mut self, member_function: FunSyntax) -> TypedFun {
        let FunSyntax {
            fun_keyword: _,
            name,
            type_params,
            arg_defs,
            return_type,
            type_constraints,
            body,
        } = member_function;

        let rt = return_type
            .map(|r| self.type_(r.type_))
            .unwrap_or_else(TypedType::unit);
        let fb = body.map(|b| self.fun_body(b));
        TypedFun {
            name: name.token(),
            arg_defs: arg_defs
                .elements
                .into_iter()
                .map(|a| self.arg_def(a.element))
                .collect(),
            type_params: type_params.map(|tps| {
                tps.elements
                    .into_iter()
                    .map(|p| self.type_param(p.element))
                    .collect()
            }),
            body: fb,
            return_type: rt,
            type_constraints: None, // TODO:
        }
    }

    pub fn use_syntax(&mut self, u: UseSyntax, annotations: Option<AnnotationsSyntax>) -> TypedUse {
        let mut names: Vec<_> = u
            .package_name
            .map(|pn| pn.names.into_iter().map(|i| i.name.token()).collect())
            .unwrap_or_default();
        names.push(u.used_name.token());
        TypedUse {
            annotations: self.annotations(&annotations),
            package: Package { names },
            alias: u.alias.map(|a| a.name.token()),
        }
    }

    fn extension_syntax(&mut self, e: ExtensionSyntax) -> TypedExtension {
        let mut computed_properties = vec![];
        let mut member_functions = vec![];
        for prop in e.body.properties {
            match prop {
                StructPropertySyntax::StoredProperty(_) => {
                    panic!("Stored property not allowed here.")
                }
                StructPropertySyntax::ComputedProperty => todo!(),
                StructPropertySyntax::Deinit(_) => panic!("Deinit is not allowed here."),
                StructPropertySyntax::Method(m) => member_functions.push(self.member_function(m)),
            }
        }
        TypedExtension {
            name: self.type_(e.name),
            protocol: e.protocol_extension.map(|tps| self.type_(tps.protocol)),
            computed_properties,
            member_functions,
        }
    }

    fn protocol_syntax(&mut self, p: StructSyntax) -> TypedProtocol {
        let mut computed_properties: Vec<TypedComputedProperty> = vec![];
        let mut member_functions: Vec<TypedFun> = vec![];
        for p in p.body.properties {
            match p {
                StructPropertySyntax::StoredProperty(v) => {
                    panic!("protocol is not allowed stored property {:?}", v)
                }
                StructPropertySyntax::ComputedProperty => {}
                StructPropertySyntax::Method(method) => {
                    member_functions.push(self.member_function(method))
                }
                StructPropertySyntax::Deinit(deinit) => {
                    panic!("protocol is not allowed deinit {:?}", deinit)
                }
            };
        }
        TypedProtocol {
            name: p.name.token(),
            type_params: p.type_params.map(|v| {
                v.elements
                    .into_iter()
                    .map(|tp| self.type_param(tp.element))
                    .collect()
            }),
            computed_properties,
            member_functions,
        }
    }

    pub fn expr(&mut self, e: Expr) -> TypedExpr {
        match e {
            Expr::Name(n) => TypedExpr::new(TypedExprKind::Name(self.name_syntax(n)), None),
            Expr::Literal(literal) => {
                TypedExpr::new(TypedExprKind::Literal(self.literal_syntax(literal)), None)
            }
            Expr::BinOp(b) => {
                TypedExpr::new(TypedExprKind::BinOp(self.binary_operation_syntax(b)), None)
            }
            Expr::UnaryOp(u) => {
                TypedExpr::new(TypedExprKind::UnaryOp(self.unary_operation_syntax(u)), None)
            }
            Expr::Subscript(s) => {
                TypedExpr::new(TypedExprKind::Subscript(self.subscript_syntax(s)), None)
            }
            Expr::Member(m) => TypedExpr::new(TypedExprKind::Member(self.member_syntax(m)), None),
            Expr::Array(a) => TypedExpr::new(TypedExprKind::Array(self.array_syntax(a)), None),
            Expr::Tuple { .. } => TypedExpr::new(TypedExprKind::Tuple, None),
            Expr::Dict { .. } => TypedExpr::new(TypedExprKind::Dict, None),
            Expr::StringBuilder { .. } => TypedExpr::new(TypedExprKind::StringBuilder, None),
            Expr::Call(c) => TypedExpr::new(TypedExprKind::Call(self.call_syntax(c)), None),
            Expr::If(i) => TypedExpr::new(TypedExprKind::If(self.if_syntax(i)), None),
            Expr::When { .. } => TypedExpr::new(TypedExprKind::When, None),
            Expr::Lambda(l) => TypedExpr::new(TypedExprKind::Lambda(self.lambda_syntax(l)), None),
            Expr::Return(r) => TypedExpr::new(TypedExprKind::Return(self.return_syntax(r)), None),
            Expr::TypeCast(t) => TypedExpr::new(TypedExprKind::TypeCast(self.type_cast(t)), None),
            Expr::Parenthesized(p) => self.expr(*p.expr),
        }
    }

    pub fn literal_syntax(&mut self, literal: LiteralSyntax) -> TypedLiteralKind {
        match literal {
            LiteralSyntax::Integer(value) => TypedLiteralKind::Integer(value.token()),
            LiteralSyntax::FloatingPoint(value) => TypedLiteralKind::FloatingPoint(value.token()),
            LiteralSyntax::String {
                open_quote: _,
                value,
                close_quote: _,
            } => TypedLiteralKind::String(value),
            LiteralSyntax::Boolean(syntax) => TypedLiteralKind::Boolean(syntax.token()),
            LiteralSyntax::Null => TypedLiteralKind::NullLiteral,
        }
    }

    pub fn name_syntax(&mut self, n: NameExprSyntax) -> TypedName {
        let NameExprSyntax {
            name_space,
            name,
            type_arguments,
        } = n;
        TypedName {
            package: match name_space {
                None => TypedPackage::Raw(Package::new()),
                Some(name_space) => TypedPackage::Raw(Package::from(
                    &name_space
                        .elements
                        .iter()
                        .map(|e| e.name.token())
                        .collect::<Vec<_>>(),
                )),
            },
            name: name.token(),
            type_arguments: type_arguments.map(|t| {
                t.elements
                    .into_iter()
                    .map(|e| self.type_(e.element))
                    .collect()
            }),
        }
    }

    pub fn binary_operation_syntax(&mut self, b: BinaryOperationSyntax) -> TypedBinOp {
        let BinaryOperationSyntax {
            left,
            operator: kind,
            right,
        } = b;
        let left = Box::new(self.expr(*left));
        let right = Box::new(self.expr(*right));
        TypedBinOp {
            left,
            operator: match &*kind.token() {
                "+" => TypedBinaryOperator::Add,
                "-" => TypedBinaryOperator::Sub,
                "*" => TypedBinaryOperator::Mul,
                "/" => TypedBinaryOperator::Div,
                "%" => TypedBinaryOperator::Mod,
                "==" => TypedBinaryOperator::Equal,
                ">=" => TypedBinaryOperator::GrateThanEqual,
                ">" => TypedBinaryOperator::GrateThan,
                "<=" => TypedBinaryOperator::LessThanEqual,
                "<" => TypedBinaryOperator::LessThan,
                "!=" => TypedBinaryOperator::NotEqual,
                _ => TypedBinaryOperator::InfixFunctionCall(kind.token()),
            },
            right,
        }
    }

    pub fn unary_operation_syntax(&mut self, u: UnaryOperationSyntax) -> TypedUnaryOp {
        match u {
            UnaryOperationSyntax::Prefix(p) => {
                TypedUnaryOp::Prefix(self.prefix_unary_operation_syntax(p))
            }
            UnaryOperationSyntax::Postfix(p) => {
                TypedUnaryOp::Postfix(self.postfix_unary_operation_syntax(p))
            }
        }
    }

    pub fn prefix_unary_operation_syntax(
        &mut self,
        p: PrefixUnaryOperationSyntax,
    ) -> TypedPrefixUnaryOp {
        let PrefixUnaryOperationSyntax { operator, target } = p;
        let target = self.expr(*target);
        TypedPrefixUnaryOp {
            target: Box::new(target),
            operator: match &*operator.token() {
                "+" => TypedPrefixUnaryOperator::Positive,
                "-" => TypedPrefixUnaryOperator::Negative,
                "*" => TypedPrefixUnaryOperator::Dereference,
                "&" => TypedPrefixUnaryOperator::Reference,
                "!" => TypedPrefixUnaryOperator::Not,
                _ => panic!(),
            },
        }
    }

    pub fn postfix_unary_operation_syntax(
        &mut self,
        p: PostfixUnaryOperationSyntax,
    ) -> TypedPostfixUnaryOp {
        let PostfixUnaryOperationSyntax { target, operator } = p;
        let target = self.expr(*target);
        TypedPostfixUnaryOp {
            target: Box::new(target),
            operator: match &*operator.token() {
                "!!" => TypedPostfixUnaryOperator::Unwrap,
                _ => panic!(),
            },
        }
    }

    pub fn array_syntax(&mut self, a: ArraySyntax) -> TypedArray {
        TypedArray {
            elements: a
                .elements
                .into_iter()
                .map(|e| self.expr(e.element))
                .collect(),
        }
    }

    pub fn subscript_syntax(&mut self, s: SubscriptSyntax) -> TypedSubscript {
        let target = Box::new(self.expr(*s.target));
        let indexes: Vec<_> = s
            .idx_or_keys
            .elements
            .into_iter()
            .map(|i| self.expr(i.element))
            .collect();
        TypedSubscript { target, indexes }
    }

    pub fn member_syntax(&mut self, m: MemberSyntax) -> TypedInstanceMember {
        let MemberSyntax {
            target,
            name,
            navigation_operator,
        } = m;
        let target = self.expr(*target);
        TypedInstanceMember {
            target: Box::new(target),
            name: name.token(),
            is_safe: navigation_operator.token().ends_with('?'),
        }
    }

    pub fn call_syntax(&mut self, c: CallExprSyntax) -> TypedCall {
        let CallExprSyntax {
            target,
            args,
            tailing_lambda,
        } = c;
        let mut args: Vec<TypedCallArg> = args
            .unwrap_or_default()
            .elements
            .into_iter()
            .map(|a| TypedCallArg {
                label: a.element.label.map(|l| l.label.token()),
                arg: Box::new(self.expr(*a.element.arg)),
                is_vararg: a.element.asterisk.is_some(),
            })
            .collect();
        if let Some(lambda) = tailing_lambda {
            args.insert(
                args.len(),
                TypedCallArg {
                    label: None,
                    arg: Box::new(TypedExpr::new(
                        TypedExprKind::Lambda(self.lambda_syntax(lambda)),
                        None,
                    )),
                    is_vararg: false,
                },
            )
        }
        TypedCall {
            target: Box::new(self.expr(*target)),
            args,
        }
    }

    pub fn if_syntax(&mut self, i: IfExprSyntax) -> TypedIf {
        let IfExprSyntax {
            if_keyword: _,
            condition,
            body,
            else_body,
        } = i;
        let block = self.block(body);
        TypedIf {
            condition: Box::new(self.expr(*condition)),
            body: block,
            else_body: else_body.map(|b| self.block(b.body)),
        }
    }

    pub fn lambda_syntax(&mut self, l: LambdaSyntax) -> TypedLambda {
        todo!("{:?}", l);
        let LambdaSyntax {
            open: _,
            stmts: _,
            close: _,
        } = l;
        TypedLambda {
            args: vec![],
            body: TypedBlock { body: vec![] },
        }
    }

    pub fn return_syntax(&mut self, r: ReturnSyntax) -> TypedReturn {
        let value = r.value.map(|v| Box::new(self.expr(*v)));
        TypedReturn { value }
    }

    pub fn type_cast(&mut self, t: TypeCastSyntax) -> TypedTypeCast {
        TypedTypeCast {
            target: Box::new(self.expr(*t.target)),
            is_safe: t.operator.token().ends_with('?'),
            type_: self.type_(t.type_),
        }
    }

    pub fn block(&mut self, block: BlockSyntax) -> TypedBlock {
        TypedBlock {
            body: block.body.into_iter().map(|s| self.stmt(s)).collect(),
        }
    }
}
