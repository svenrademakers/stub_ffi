use crate::ir_definitions::*;
use lang_c::{ast::DeclarationSpecifier, span::Node};
use std::path::Path;

pub fn parse(file_name: &Path) -> TranslationUnitIr {
    let config = lang_c::driver::Config::default();
    let ast = lang_c::driver::parse(&config, file_name).unwrap();
    println!("{:#?}", &ast);

    let mut functions = Vec::new();
    for node in ast.unit.0 {
        match node.node {
            lang_c::ast::ExternalDeclaration::Declaration(d) => {
                let typ = d.node.specifiers.iter().find_map(get_type_specifier);

                let mut ident = None;
                for init_decl in d.node.declarators {
                    if let lang_c::ast::DeclaratorKind::Identifier(i) =
                        init_decl.node.declarator.node.kind.node
                    {
                        ident = Some(i.node);
                        break;
                    }
                }

                if ident.is_some() && typ.is_some() {
                    functions.push(FnIr {
                        ident: ident.unwrap(),
                        return_type: typ.unwrap().to_owned(),
                    });
                }
            }
            _ => (),
        }
    }

    TranslationUnitIr {
        name: file_name
            .file_stem()
            .expect("could not get basename")
            .to_string_lossy()
            .to_string(),
        functions: functions,
        type_definitions: Vec::new(),
    }
}

fn get_type_specifier(node: &Node<DeclarationSpecifier>) -> Option<&lang_c::ast::TypeSpecifier> {
    if let lang_c::ast::DeclarationSpecifier::TypeSpecifier(c) = &node.node {
        Some(&c.node)
    } else {
        None
    }
}

#[cfg(test)]
mod Test {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn my_first_parse() {
        let mut header = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        header.push("test");
        header.push("test.h");

        let ir = TranslationUnitIr {
            name: "test".to_string(),
            functions: vec![FnIr {
                ident: lang_c::ast::Identifier {
                    name: "testfn".to_string(),
                },
                return_type: lang_c::ast::TypeSpecifier::Void,
            }],
            type_definitions: vec![],
        };
        assert_eq!(ir, parse(&header));
    }
}
