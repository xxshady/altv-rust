use std::fs;

use crate::value_type::ValueType;

#[derive(Debug, PartialEq, Clone, Copy)]
enum TopLevelSection {
    Imports,
    Exports,
}

#[derive(Debug)]
pub struct Param {
    pub name: String,
    pub param_type: ValueType,
}

#[derive(Debug)]
pub struct Func {
    pub name: String,
    pub params: Vec<Param>,
    pub ret: Option<ValueType>,
}

#[derive(Debug)]
struct Section {
    kind: TopLevelSection,
    funcs: Vec<Func>,
}

#[derive(Debug, Default)]
struct AllSections {
    imports: Option<Vec<Func>>,
    exports: Option<Vec<Func>>,
}

#[derive(Debug, Default)]
struct SectionController {
    current: Option<Section>,
    all: AllSections,
}

impl SectionController {
    fn change(&mut self, to: TopLevelSection) {
        let section = self.current.take();
        if let Some(section) = section {
            assert_ne!(section.kind, to);
            self.collect_funcs(section);
        }
        self.current.replace(Section {
            kind: to,
            funcs: vec![],
        });
    }

    fn add_func(&mut self, func: Func) {
        self.current.as_mut().unwrap().funcs.push(func);
    }

    fn collect_funcs(&mut self, section: Section) {
        match section.kind {
            TopLevelSection::Exports => {
                self.all.exports.replace(section.funcs);
            }
            TopLevelSection::Imports => {
                self.all.imports.replace(section.funcs);
            }
        }
    }

    fn parse(mut self, input: String) -> Interface {
        let mut word = String::new();
        let mut current_func: Option<Func> = None;
        let mut current_param_name: Option<String> = None;
        let mut return_type_parsing = false;
        let mut return_type_parsing_arrow = false;

        for char in input.chars() {
            match char {
                ':' | '(' | ')' | ',' | '{' | '}' => {
                    match (std::mem::take(&mut word).trim(), char) {
                        // top level sections
                        ("imports", '{') => {
                            println!("starting with imports");
                            self.change(TopLevelSection::Imports);
                        }
                        ("exports", '{') => {
                            println!("starting with exports");
                            self.change(TopLevelSection::Exports);
                        }
                        (unknown, '{') => {
                            panic!("Unknown top-level section: {unknown:?}");
                        }

                        // func names
                        (func_name, '(') => {
                            dbg!(func_name);
                            assert!(current_func.is_none());

                            if self
                                .current
                                .as_ref()
                                .unwrap()
                                .funcs
                                .iter()
                                .any(|f| f.name == func_name)
                            {
                                panic!("Detected func name duplicate: {func_name:?}");
                            }

                            current_func.replace(Func {
                                name: func_name.to_string(),
                                params: vec![],
                                ret: None,
                            });
                        }

                        // params
                        ("", ')') => {
                            assert!(current_func.is_some());
                            println!("empty params");
                            return_type_parsing = true;
                        }

                        (param_name, ':') => {
                            assert!(current_param_name.is_none());

                            let Some(func) = current_func.as_ref() else {
                                panic!("Expected current func");
                            };
                            if func.params.iter().any(|p| p.name == param_name) {
                                panic!("Detected func param name duplicate: {param_name:?} in func: {:?}", func.name);
                            }

                            current_param_name.replace(param_name.to_string());
                        }

                        (param_type, ',' | ')') => {
                            let (Some(func), Some(param_name)) =
                                (current_func.as_mut(), current_param_name.take())
                            else {
                                panic!("Expected current func and param name");
                            };

                            func.params.push(Param {
                                name: param_name,
                                param_type: param_type.into(),
                            });

                            if char == ')' {
                                return_type_parsing = true;
                            }
                        }
                        _ => {}
                    }
                }
                _ => {
                    if return_type_parsing {
                        match char {
                            '-' => {
                                return_type_parsing_arrow = true;
                            }
                            '>' => {
                                assert!(return_type_parsing_arrow);
                                return_type_parsing_arrow = false;
                            }
                            '\n' => {
                                return_type_parsing = false;
                                let Some(mut func) = current_func.take() else {
                                    panic!("Expected current func");
                                };
                                let word = &std::mem::take(&mut word);
                                let return_type = word.trim();
                                if !return_type.is_empty() {
                                    let return_type = &return_type.split("->").last().unwrap();
                                    let return_type = return_type.trim();
                                    dbg!(return_type);
                                    func.ret.replace(return_type.into());
                                }

                                self.add_func(func);
                            }
                            _ => {
                                assert!(!return_type_parsing_arrow);
                            }
                        }
                    }

                    word += &char.to_string();
                }
            }
        }

        if let Some(section) = self.current.take() {
            self.collect_funcs(section);
        }

        Interface {
            imports: self.all.imports.unwrap(),
            exports: self.all.exports.unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct Interface {
    pub imports: Vec<Func>,
    pub exports: Vec<Func>,
}

pub fn parse_interface(path: &str) -> Interface {
    let input = fs::read_to_string(path).unwrap();
    let controller = SectionController::default();
    controller.parse(input)
}
