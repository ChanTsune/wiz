use wiz_arena::{Arena, DeclarationId};
use wiz_hir::typed_annotation::TypedAnnotations;
use wiz_session::Session;
use wiz_syntax::syntax::annotation::AnnotationsSyntax;
use wiz_syntax::syntax::declaration::DeclKind;
use wiz_syntax::syntax::file::{SourceSet, WizFile};
use wiz_utils::utils::path_string_to_page_name;

pub struct AstTypeCollector<'a> {
    session: &'a Session,
    arena: &'a mut Arena,
    namespace_id: DeclarationId,
}

impl<'a> AstTypeCollector<'a> {
    pub(crate) fn new(session: &'a Session, arena: &'a mut Arena) -> Self {
        Self {
            session,
            arena,
            namespace_id: DeclarationId::ROOT,
        }
    }
}

impl<'a> AstTypeCollector<'a> {
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
}

impl<'a> AstTypeCollector<'a> {
    pub(crate) fn start(&mut self, source_set: &SourceSet) {
        self.source_set(source_set);
    }

    fn source_set(&mut self, s: &SourceSet) {
        match s {
            SourceSet::File(f) => self.file(f),
            SourceSet::Dir { name, items } => self.push_namespace(&name.clone(), |slf| {
                for item in items {
                    slf.source_set(item);
                }
            }),
        };
    }

    fn file(&mut self, f: &WizFile) {
        let WizFile { name, syntax } = f;

        let name = path_string_to_page_name(name);

        self.push_namespace(name, |slf| {
            for l in syntax.body.iter() {
                if let DeclKind::Struct(s) = &l.kind {
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
                }
            }
        })
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
}
