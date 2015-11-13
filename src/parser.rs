#![allow(unused_imports)]
#![allow(unused_variables)]
use ast::*;
use intern::{intern, InternedString};
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Krate {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use ast::*;
    use intern::{intern, InternedString};
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Krate<
        'input,
    >(
        krate: &mut Krate,
        input: &'input str,
    ) -> Result<(), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match try!(__state0(krate, input, None, &mut __tokens, __lookahead)) {
            (_, Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (_, None, __Nonterminal::____Krate(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        _28_22as_22_20_3cId_3e_29(InternedString),
        _28_22as_22_20_3cId_3e_29_3f(::std::option::Option<InternedString>),
        _28_29(()),
        _28_3cPath_3e_20_22_3b_22_29(PathId),
        _28_3cPath_3e_20_22_3b_22_29_2a(::std::vec::Vec<PathId>),
        _28_3cPath_3e_20_22_3b_22_29_2b(::std::vec::Vec<PathId>),
        Code(CodeId),
        Glob(GlobId),
        Id(InternedString),
        Import(ImportId),
        Item(ItemId),
        Item_2a(::std::vec::Vec<ItemId>),
        Item_2b(::std::vec::Vec<ItemId>),
        Krate(()),
        MacroDef(MacroDefId),
        MacroRef(MacroRefId),
        Module(ModuleId),
        Path(PathId),
        Privacy(Privacy),
        Structure(StructureId),
        ____Krate(()),
    }

    // State 0
    //   Code = (*) "{" (<Path> ";")+ "}" [EOF]
    //   Code = (*) "{" (<Path> ";")+ "}" ["::"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["macro_rules"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["mod"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["pub"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["self"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["struct"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["use"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["{"]
    //   Code = (*) "{" (<Path> ";")+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Code = (*) "{" "}" [EOF]
    //   Code = (*) "{" "}" ["::"]
    //   Code = (*) "{" "}" ["macro_rules"]
    //   Code = (*) "{" "}" ["mod"]
    //   Code = (*) "{" "}" ["pub"]
    //   Code = (*) "{" "}" ["self"]
    //   Code = (*) "{" "}" ["struct"]
    //   Code = (*) "{" "}" ["use"]
    //   Code = (*) "{" "}" ["{"]
    //   Code = (*) "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" [EOF]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["::"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["macro_rules"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["mod"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["pub"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["self"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["struct"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["use"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["{"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["!"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["::"]
    //   Import = (*) Privacy "use" Path ";" [EOF]
    //   Import = (*) Privacy "use" Path ";" ["::"]
    //   Import = (*) Privacy "use" Path ";" ["macro_rules"]
    //   Import = (*) Privacy "use" Path ";" ["mod"]
    //   Import = (*) Privacy "use" Path ";" ["pub"]
    //   Import = (*) Privacy "use" Path ";" ["self"]
    //   Import = (*) Privacy "use" Path ";" ["struct"]
    //   Import = (*) Privacy "use" Path ";" ["use"]
    //   Import = (*) Privacy "use" Path ";" ["{"]
    //   Import = (*) Privacy "use" Path ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Import = (*) Privacy "use" Path "as" Id ";" [EOF]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["::"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["macro_rules"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["mod"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["pub"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["self"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["struct"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["use"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["{"]
    //   Import = (*) Privacy "use" Path "as" Id ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Code [EOF]
    //   Item = (*) Code ["::"]
    //   Item = (*) Code ["macro_rules"]
    //   Item = (*) Code ["mod"]
    //   Item = (*) Code ["pub"]
    //   Item = (*) Code ["self"]
    //   Item = (*) Code ["struct"]
    //   Item = (*) Code ["use"]
    //   Item = (*) Code ["{"]
    //   Item = (*) Code [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Glob [EOF]
    //   Item = (*) Glob ["::"]
    //   Item = (*) Glob ["macro_rules"]
    //   Item = (*) Glob ["mod"]
    //   Item = (*) Glob ["pub"]
    //   Item = (*) Glob ["self"]
    //   Item = (*) Glob ["struct"]
    //   Item = (*) Glob ["use"]
    //   Item = (*) Glob ["{"]
    //   Item = (*) Glob [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Import [EOF]
    //   Item = (*) Import ["::"]
    //   Item = (*) Import ["macro_rules"]
    //   Item = (*) Import ["mod"]
    //   Item = (*) Import ["pub"]
    //   Item = (*) Import ["self"]
    //   Item = (*) Import ["struct"]
    //   Item = (*) Import ["use"]
    //   Item = (*) Import ["{"]
    //   Item = (*) Import [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) MacroDef [EOF]
    //   Item = (*) MacroDef ["::"]
    //   Item = (*) MacroDef ["macro_rules"]
    //   Item = (*) MacroDef ["mod"]
    //   Item = (*) MacroDef ["pub"]
    //   Item = (*) MacroDef ["self"]
    //   Item = (*) MacroDef ["struct"]
    //   Item = (*) MacroDef ["use"]
    //   Item = (*) MacroDef ["{"]
    //   Item = (*) MacroDef [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) MacroRef [EOF]
    //   Item = (*) MacroRef ["::"]
    //   Item = (*) MacroRef ["macro_rules"]
    //   Item = (*) MacroRef ["mod"]
    //   Item = (*) MacroRef ["pub"]
    //   Item = (*) MacroRef ["self"]
    //   Item = (*) MacroRef ["struct"]
    //   Item = (*) MacroRef ["use"]
    //   Item = (*) MacroRef ["{"]
    //   Item = (*) MacroRef [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Module [EOF]
    //   Item = (*) Module ["::"]
    //   Item = (*) Module ["macro_rules"]
    //   Item = (*) Module ["mod"]
    //   Item = (*) Module ["pub"]
    //   Item = (*) Module ["self"]
    //   Item = (*) Module ["struct"]
    //   Item = (*) Module ["use"]
    //   Item = (*) Module ["{"]
    //   Item = (*) Module [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Structure [EOF]
    //   Item = (*) Structure ["::"]
    //   Item = (*) Structure ["macro_rules"]
    //   Item = (*) Structure ["mod"]
    //   Item = (*) Structure ["pub"]
    //   Item = (*) Structure ["self"]
    //   Item = (*) Structure ["struct"]
    //   Item = (*) Structure ["use"]
    //   Item = (*) Structure ["{"]
    //   Item = (*) Structure [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item+ = (*) Item [EOF]
    //   Item+ = (*) Item ["::"]
    //   Item+ = (*) Item ["macro_rules"]
    //   Item+ = (*) Item ["mod"]
    //   Item+ = (*) Item ["pub"]
    //   Item+ = (*) Item ["self"]
    //   Item+ = (*) Item ["struct"]
    //   Item+ = (*) Item ["use"]
    //   Item+ = (*) Item ["{"]
    //   Item+ = (*) Item [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item+ = (*) Item+ Item [EOF]
    //   Item+ = (*) Item+ Item ["::"]
    //   Item+ = (*) Item+ Item ["macro_rules"]
    //   Item+ = (*) Item+ Item ["mod"]
    //   Item+ = (*) Item+ Item ["pub"]
    //   Item+ = (*) Item+ Item ["self"]
    //   Item+ = (*) Item+ Item ["struct"]
    //   Item+ = (*) Item+ Item ["use"]
    //   Item+ = (*) Item+ Item ["{"]
    //   Item+ = (*) Item+ Item [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Krate = (*) Item+ [EOF]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" [EOF]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["::"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["macro_rules"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["mod"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["pub"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["self"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["struct"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["use"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["{"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" [EOF]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["::"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["macro_rules"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["mod"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["pub"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["self"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["struct"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["use"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["{"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroRef = (*) Path "!" ";" [EOF]
    //   MacroRef = (*) Path "!" ";" ["::"]
    //   MacroRef = (*) Path "!" ";" ["macro_rules"]
    //   MacroRef = (*) Path "!" ";" ["mod"]
    //   MacroRef = (*) Path "!" ";" ["pub"]
    //   MacroRef = (*) Path "!" ";" ["self"]
    //   MacroRef = (*) Path "!" ";" ["struct"]
    //   MacroRef = (*) Path "!" ";" ["use"]
    //   MacroRef = (*) Path "!" ";" ["{"]
    //   MacroRef = (*) Path "!" ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" [EOF]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["::"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["macro_rules"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["mod"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["pub"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["self"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["struct"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["use"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["{"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = (*) Privacy "mod" Id "{" "}" [EOF]
    //   Module = (*) Privacy "mod" Id "{" "}" ["::"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["macro_rules"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["mod"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["pub"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["self"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["struct"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["use"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["{"]
    //   Module = (*) Privacy "mod" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Path = (*) Id ["!"]
    //   Path = (*) Id ["::"]
    //   Path = (*) Path "::" Id ["!"]
    //   Path = (*) Path "::" Id ["::"]
    //   Path = (*) "::" Id ["!"]
    //   Path = (*) "::" Id ["::"]
    //   Path = (*) "self" "::" Id ["!"]
    //   Path = (*) "self" "::" Id ["::"]
    //   Privacy = (*) ["macro_rules"]
    //   Privacy = (*) ["mod"]
    //   Privacy = (*) ["struct"]
    //   Privacy = (*) ["use"]
    //   Privacy = (*) "pub" ["macro_rules"]
    //   Privacy = (*) "pub" ["mod"]
    //   Privacy = (*) "pub" ["struct"]
    //   Privacy = (*) "pub" ["use"]
    //   Structure = (*) Privacy "struct" Id "{" "}" [EOF]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["::"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["macro_rules"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["mod"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["pub"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["self"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["struct"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["use"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["{"]
    //   Structure = (*) Privacy "struct" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   __Krate = (*) Krate [EOF]
    //
    //   "::" -> Shift(S14)
    //   "macro_rules" -> Reduce(Privacy =  => ActionFn(39);)
    //   "mod" -> Reduce(Privacy =  => ActionFn(39);)
    //   "pub" -> Shift(S15)
    //   "self" -> Shift(S16)
    //   "struct" -> Reduce(Privacy =  => ActionFn(39);)
    //   "use" -> Reduce(Privacy =  => ActionFn(39);)
    //   "{" -> Shift(S17)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S18)
    //
    //   Code -> S1
    //   Glob -> S2
    //   Id -> S3
    //   Import -> S4
    //   Item -> S5
    //   Item+ -> S6
    //   Krate -> S7
    //   MacroDef -> S8
    //   MacroRef -> S9
    //   Module -> S10
    //   Path -> S11
    //   Privacy -> S12
    //   Structure -> S13
    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state14(krate, input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state15(krate, input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state16(krate, input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state17(krate, input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state18(krate, input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) => {
                let __nt = super::__action39(krate, input, &__lookbehind, &__lookahead);
                __result = (__lookbehind, __lookahead, __Nonterminal::Privacy(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Code(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(krate, input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Glob(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(krate, input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Id(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state3(krate, input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Import(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state4(krate, input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Item(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state5(krate, input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Item_2b(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state6(krate, input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Krate(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state7(krate, input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::MacroDef(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state8(krate, input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::MacroRef(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state9(krate, input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Module(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state10(krate, input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Path(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state11(krate, input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Privacy(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state12(krate, input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Structure(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state13(krate, input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   Item = Code (*) [EOF]
    //   Item = Code (*) ["::"]
    //   Item = Code (*) ["macro_rules"]
    //   Item = Code (*) ["mod"]
    //   Item = Code (*) ["pub"]
    //   Item = Code (*) ["self"]
    //   Item = Code (*) ["struct"]
    //   Item = Code (*) ["use"]
    //   Item = Code (*) ["{"]
    //   Item = Code (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   EOF -> Reduce(Item = Code => ActionFn(8);)
    //   "::" -> Reduce(Item = Code => ActionFn(8);)
    //   "macro_rules" -> Reduce(Item = Code => ActionFn(8);)
    //   "mod" -> Reduce(Item = Code => ActionFn(8);)
    //   "pub" -> Reduce(Item = Code => ActionFn(8);)
    //   "self" -> Reduce(Item = Code => ActionFn(8);)
    //   "struct" -> Reduce(Item = Code => ActionFn(8);)
    //   "use" -> Reduce(Item = Code => ActionFn(8);)
    //   "{" -> Reduce(Item = Code => ActionFn(8);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Item = Code => ActionFn(8);)
    //
    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<CodeId>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(krate, input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Item(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 2
    //   Item = Glob (*) [EOF]
    //   Item = Glob (*) ["::"]
    //   Item = Glob (*) ["macro_rules"]
    //   Item = Glob (*) ["mod"]
    //   Item = Glob (*) ["pub"]
    //   Item = Glob (*) ["self"]
    //   Item = Glob (*) ["struct"]
    //   Item = Glob (*) ["use"]
    //   Item = Glob (*) ["{"]
    //   Item = Glob (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   EOF -> Reduce(Item = Glob => ActionFn(5);)
    //   "::" -> Reduce(Item = Glob => ActionFn(5);)
    //   "macro_rules" -> Reduce(Item = Glob => ActionFn(5);)
    //   "mod" -> Reduce(Item = Glob => ActionFn(5);)
    //   "pub" -> Reduce(Item = Glob => ActionFn(5);)
    //   "self" -> Reduce(Item = Glob => ActionFn(5);)
    //   "struct" -> Reduce(Item = Glob => ActionFn(5);)
    //   "use" -> Reduce(Item = Glob => ActionFn(5);)
    //   "{" -> Reduce(Item = Glob => ActionFn(5);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Item = Glob => ActionFn(5);)
    //
    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<GlobId>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action5(krate, input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Item(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 3
    //   Path = Id (*) ["!"]
    //   Path = Id (*) ["::"]
    //
    //   "!" -> Reduce(Path = Id => ActionFn(18);)
    //   "::" -> Reduce(Path = Id => ActionFn(18);)
    //
    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<InternedString>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action18(krate, input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Path(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 4
    //   Item = Import (*) [EOF]
    //   Item = Import (*) ["::"]
    //   Item = Import (*) ["macro_rules"]
    //   Item = Import (*) ["mod"]
    //   Item = Import (*) ["pub"]
    //   Item = Import (*) ["self"]
    //   Item = Import (*) ["struct"]
    //   Item = Import (*) ["use"]
    //   Item = Import (*) ["{"]
    //   Item = Import (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   EOF -> Reduce(Item = Import => ActionFn(4);)
    //   "::" -> Reduce(Item = Import => ActionFn(4);)
    //   "macro_rules" -> Reduce(Item = Import => ActionFn(4);)
    //   "mod" -> Reduce(Item = Import => ActionFn(4);)
    //   "pub" -> Reduce(Item = Import => ActionFn(4);)
    //   "self" -> Reduce(Item = Import => ActionFn(4);)
    //   "struct" -> Reduce(Item = Import => ActionFn(4);)
    //   "use" -> Reduce(Item = Import => ActionFn(4);)
    //   "{" -> Reduce(Item = Import => ActionFn(4);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Item = Import => ActionFn(4);)
    //
    pub fn __state4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<ImportId>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action4(krate, input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Item(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 5
    //   Item+ = Item (*) [EOF]
    //   Item+ = Item (*) ["::"]
    //   Item+ = Item (*) ["macro_rules"]
    //   Item+ = Item (*) ["mod"]
    //   Item+ = Item (*) ["pub"]
    //   Item+ = Item (*) ["self"]
    //   Item+ = Item (*) ["struct"]
    //   Item+ = Item (*) ["use"]
    //   Item+ = Item (*) ["{"]
    //   Item+ = Item (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   EOF -> Reduce(Item+ = Item => ActionFn(32);)
    //   "::" -> Reduce(Item+ = Item => ActionFn(32);)
    //   "macro_rules" -> Reduce(Item+ = Item => ActionFn(32);)
    //   "mod" -> Reduce(Item+ = Item => ActionFn(32);)
    //   "pub" -> Reduce(Item+ = Item => ActionFn(32);)
    //   "self" -> Reduce(Item+ = Item => ActionFn(32);)
    //   "struct" -> Reduce(Item+ = Item => ActionFn(32);)
    //   "use" -> Reduce(Item+ = Item => ActionFn(32);)
    //   "{" -> Reduce(Item+ = Item => ActionFn(32);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Item+ = Item => ActionFn(32);)
    //
    pub fn __state5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<ItemId>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action32(krate, input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Item_2b(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 6
    //   Code = (*) "{" (<Path> ";")+ "}" [EOF]
    //   Code = (*) "{" (<Path> ";")+ "}" ["::"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["macro_rules"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["mod"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["pub"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["self"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["struct"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["use"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["{"]
    //   Code = (*) "{" (<Path> ";")+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Code = (*) "{" "}" [EOF]
    //   Code = (*) "{" "}" ["::"]
    //   Code = (*) "{" "}" ["macro_rules"]
    //   Code = (*) "{" "}" ["mod"]
    //   Code = (*) "{" "}" ["pub"]
    //   Code = (*) "{" "}" ["self"]
    //   Code = (*) "{" "}" ["struct"]
    //   Code = (*) "{" "}" ["use"]
    //   Code = (*) "{" "}" ["{"]
    //   Code = (*) "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" [EOF]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["::"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["macro_rules"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["mod"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["pub"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["self"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["struct"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["use"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["{"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["!"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["::"]
    //   Import = (*) Privacy "use" Path ";" [EOF]
    //   Import = (*) Privacy "use" Path ";" ["::"]
    //   Import = (*) Privacy "use" Path ";" ["macro_rules"]
    //   Import = (*) Privacy "use" Path ";" ["mod"]
    //   Import = (*) Privacy "use" Path ";" ["pub"]
    //   Import = (*) Privacy "use" Path ";" ["self"]
    //   Import = (*) Privacy "use" Path ";" ["struct"]
    //   Import = (*) Privacy "use" Path ";" ["use"]
    //   Import = (*) Privacy "use" Path ";" ["{"]
    //   Import = (*) Privacy "use" Path ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Import = (*) Privacy "use" Path "as" Id ";" [EOF]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["::"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["macro_rules"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["mod"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["pub"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["self"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["struct"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["use"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["{"]
    //   Import = (*) Privacy "use" Path "as" Id ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Code [EOF]
    //   Item = (*) Code ["::"]
    //   Item = (*) Code ["macro_rules"]
    //   Item = (*) Code ["mod"]
    //   Item = (*) Code ["pub"]
    //   Item = (*) Code ["self"]
    //   Item = (*) Code ["struct"]
    //   Item = (*) Code ["use"]
    //   Item = (*) Code ["{"]
    //   Item = (*) Code [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Glob [EOF]
    //   Item = (*) Glob ["::"]
    //   Item = (*) Glob ["macro_rules"]
    //   Item = (*) Glob ["mod"]
    //   Item = (*) Glob ["pub"]
    //   Item = (*) Glob ["self"]
    //   Item = (*) Glob ["struct"]
    //   Item = (*) Glob ["use"]
    //   Item = (*) Glob ["{"]
    //   Item = (*) Glob [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Import [EOF]
    //   Item = (*) Import ["::"]
    //   Item = (*) Import ["macro_rules"]
    //   Item = (*) Import ["mod"]
    //   Item = (*) Import ["pub"]
    //   Item = (*) Import ["self"]
    //   Item = (*) Import ["struct"]
    //   Item = (*) Import ["use"]
    //   Item = (*) Import ["{"]
    //   Item = (*) Import [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) MacroDef [EOF]
    //   Item = (*) MacroDef ["::"]
    //   Item = (*) MacroDef ["macro_rules"]
    //   Item = (*) MacroDef ["mod"]
    //   Item = (*) MacroDef ["pub"]
    //   Item = (*) MacroDef ["self"]
    //   Item = (*) MacroDef ["struct"]
    //   Item = (*) MacroDef ["use"]
    //   Item = (*) MacroDef ["{"]
    //   Item = (*) MacroDef [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) MacroRef [EOF]
    //   Item = (*) MacroRef ["::"]
    //   Item = (*) MacroRef ["macro_rules"]
    //   Item = (*) MacroRef ["mod"]
    //   Item = (*) MacroRef ["pub"]
    //   Item = (*) MacroRef ["self"]
    //   Item = (*) MacroRef ["struct"]
    //   Item = (*) MacroRef ["use"]
    //   Item = (*) MacroRef ["{"]
    //   Item = (*) MacroRef [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Module [EOF]
    //   Item = (*) Module ["::"]
    //   Item = (*) Module ["macro_rules"]
    //   Item = (*) Module ["mod"]
    //   Item = (*) Module ["pub"]
    //   Item = (*) Module ["self"]
    //   Item = (*) Module ["struct"]
    //   Item = (*) Module ["use"]
    //   Item = (*) Module ["{"]
    //   Item = (*) Module [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Structure [EOF]
    //   Item = (*) Structure ["::"]
    //   Item = (*) Structure ["macro_rules"]
    //   Item = (*) Structure ["mod"]
    //   Item = (*) Structure ["pub"]
    //   Item = (*) Structure ["self"]
    //   Item = (*) Structure ["struct"]
    //   Item = (*) Structure ["use"]
    //   Item = (*) Structure ["{"]
    //   Item = (*) Structure [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item+ = Item+ (*) Item [EOF]
    //   Item+ = Item+ (*) Item ["::"]
    //   Item+ = Item+ (*) Item ["macro_rules"]
    //   Item+ = Item+ (*) Item ["mod"]
    //   Item+ = Item+ (*) Item ["pub"]
    //   Item+ = Item+ (*) Item ["self"]
    //   Item+ = Item+ (*) Item ["struct"]
    //   Item+ = Item+ (*) Item ["use"]
    //   Item+ = Item+ (*) Item ["{"]
    //   Item+ = Item+ (*) Item [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Krate = Item+ (*) [EOF]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" [EOF]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["::"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["macro_rules"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["mod"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["pub"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["self"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["struct"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["use"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["{"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" [EOF]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["::"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["macro_rules"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["mod"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["pub"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["self"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["struct"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["use"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["{"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroRef = (*) Path "!" ";" [EOF]
    //   MacroRef = (*) Path "!" ";" ["::"]
    //   MacroRef = (*) Path "!" ";" ["macro_rules"]
    //   MacroRef = (*) Path "!" ";" ["mod"]
    //   MacroRef = (*) Path "!" ";" ["pub"]
    //   MacroRef = (*) Path "!" ";" ["self"]
    //   MacroRef = (*) Path "!" ";" ["struct"]
    //   MacroRef = (*) Path "!" ";" ["use"]
    //   MacroRef = (*) Path "!" ";" ["{"]
    //   MacroRef = (*) Path "!" ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" [EOF]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["::"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["macro_rules"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["mod"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["pub"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["self"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["struct"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["use"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["{"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = (*) Privacy "mod" Id "{" "}" [EOF]
    //   Module = (*) Privacy "mod" Id "{" "}" ["::"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["macro_rules"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["mod"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["pub"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["self"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["struct"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["use"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["{"]
    //   Module = (*) Privacy "mod" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Path = (*) Id ["!"]
    //   Path = (*) Id ["::"]
    //   Path = (*) Path "::" Id ["!"]
    //   Path = (*) Path "::" Id ["::"]
    //   Path = (*) "::" Id ["!"]
    //   Path = (*) "::" Id ["::"]
    //   Path = (*) "self" "::" Id ["!"]
    //   Path = (*) "self" "::" Id ["::"]
    //   Privacy = (*) ["macro_rules"]
    //   Privacy = (*) ["mod"]
    //   Privacy = (*) ["struct"]
    //   Privacy = (*) ["use"]
    //   Privacy = (*) "pub" ["macro_rules"]
    //   Privacy = (*) "pub" ["mod"]
    //   Privacy = (*) "pub" ["struct"]
    //   Privacy = (*) "pub" ["use"]
    //   Structure = (*) Privacy "struct" Id "{" "}" [EOF]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["::"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["macro_rules"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["mod"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["pub"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["self"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["struct"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["use"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["{"]
    //   Structure = (*) Privacy "struct" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   EOF -> Reduce(Krate = Item+ => ActionFn(1);)
    //   "::" -> Shift(S14)
    //   "macro_rules" -> Reduce(Privacy =  => ActionFn(39);)
    //   "mod" -> Reduce(Privacy =  => ActionFn(39);)
    //   "pub" -> Shift(S15)
    //   "self" -> Shift(S16)
    //   "struct" -> Reduce(Privacy =  => ActionFn(39);)
    //   "use" -> Reduce(Privacy =  => ActionFn(39);)
    //   "{" -> Shift(S17)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S18)
    //
    //   Code -> S1
    //   Glob -> S2
    //   Id -> S3
    //   Import -> S4
    //   Item -> S19
    //   MacroDef -> S8
    //   MacroRef -> S9
    //   Module -> S10
    //   Path -> S11
    //   Privacy -> S12
    //   Structure -> S13
    pub fn __state6<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<ItemId>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state14(krate, input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state15(krate, input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state16(krate, input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state17(krate, input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state18(krate, input, __lookbehind, __tokens, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action1(krate, input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Krate(__nt)));
            }
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) => {
                let __nt = super::__action39(krate, input, &__lookbehind, &__lookahead);
                __result = (__lookbehind, __lookahead, __Nonterminal::Privacy(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Code(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state1(krate, input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Glob(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state2(krate, input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Id(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state3(krate, input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Import(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state4(krate, input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Item(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state19(krate, input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::MacroDef(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state8(krate, input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::MacroRef(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state9(krate, input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Module(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state10(krate, input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Path(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state11(krate, input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Privacy(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(krate, input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Structure(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state13(krate, input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 7
    //   __Krate = Krate (*) [EOF]
    //
    //   EOF -> Reduce(__Krate = Krate => ActionFn(0);)
    //
    pub fn __state7<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<()>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action0(krate, input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____Krate(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 8
    //   Item = MacroDef (*) [EOF]
    //   Item = MacroDef (*) ["::"]
    //   Item = MacroDef (*) ["macro_rules"]
    //   Item = MacroDef (*) ["mod"]
    //   Item = MacroDef (*) ["pub"]
    //   Item = MacroDef (*) ["self"]
    //   Item = MacroDef (*) ["struct"]
    //   Item = MacroDef (*) ["use"]
    //   Item = MacroDef (*) ["{"]
    //   Item = MacroDef (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   EOF -> Reduce(Item = MacroDef => ActionFn(6);)
    //   "::" -> Reduce(Item = MacroDef => ActionFn(6);)
    //   "macro_rules" -> Reduce(Item = MacroDef => ActionFn(6);)
    //   "mod" -> Reduce(Item = MacroDef => ActionFn(6);)
    //   "pub" -> Reduce(Item = MacroDef => ActionFn(6);)
    //   "self" -> Reduce(Item = MacroDef => ActionFn(6);)
    //   "struct" -> Reduce(Item = MacroDef => ActionFn(6);)
    //   "use" -> Reduce(Item = MacroDef => ActionFn(6);)
    //   "{" -> Reduce(Item = MacroDef => ActionFn(6);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Item = MacroDef => ActionFn(6);)
    //
    pub fn __state8<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<MacroDefId>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(krate, input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Item(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 9
    //   Item = MacroRef (*) [EOF]
    //   Item = MacroRef (*) ["::"]
    //   Item = MacroRef (*) ["macro_rules"]
    //   Item = MacroRef (*) ["mod"]
    //   Item = MacroRef (*) ["pub"]
    //   Item = MacroRef (*) ["self"]
    //   Item = MacroRef (*) ["struct"]
    //   Item = MacroRef (*) ["use"]
    //   Item = MacroRef (*) ["{"]
    //   Item = MacroRef (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   EOF -> Reduce(Item = MacroRef => ActionFn(7);)
    //   "::" -> Reduce(Item = MacroRef => ActionFn(7);)
    //   "macro_rules" -> Reduce(Item = MacroRef => ActionFn(7);)
    //   "mod" -> Reduce(Item = MacroRef => ActionFn(7);)
    //   "pub" -> Reduce(Item = MacroRef => ActionFn(7);)
    //   "self" -> Reduce(Item = MacroRef => ActionFn(7);)
    //   "struct" -> Reduce(Item = MacroRef => ActionFn(7);)
    //   "use" -> Reduce(Item = MacroRef => ActionFn(7);)
    //   "{" -> Reduce(Item = MacroRef => ActionFn(7);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Item = MacroRef => ActionFn(7);)
    //
    pub fn __state9<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<MacroRefId>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(krate, input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Item(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 10
    //   Item = Module (*) [EOF]
    //   Item = Module (*) ["::"]
    //   Item = Module (*) ["macro_rules"]
    //   Item = Module (*) ["mod"]
    //   Item = Module (*) ["pub"]
    //   Item = Module (*) ["self"]
    //   Item = Module (*) ["struct"]
    //   Item = Module (*) ["use"]
    //   Item = Module (*) ["{"]
    //   Item = Module (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   EOF -> Reduce(Item = Module => ActionFn(2);)
    //   "::" -> Reduce(Item = Module => ActionFn(2);)
    //   "macro_rules" -> Reduce(Item = Module => ActionFn(2);)
    //   "mod" -> Reduce(Item = Module => ActionFn(2);)
    //   "pub" -> Reduce(Item = Module => ActionFn(2);)
    //   "self" -> Reduce(Item = Module => ActionFn(2);)
    //   "struct" -> Reduce(Item = Module => ActionFn(2);)
    //   "use" -> Reduce(Item = Module => ActionFn(2);)
    //   "{" -> Reduce(Item = Module => ActionFn(2);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Item = Module => ActionFn(2);)
    //
    pub fn __state10<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<ModuleId>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action2(krate, input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Item(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 11
    //   MacroRef = Path (*) "!" ";" [EOF]
    //   MacroRef = Path (*) "!" ";" ["::"]
    //   MacroRef = Path (*) "!" ";" ["macro_rules"]
    //   MacroRef = Path (*) "!" ";" ["mod"]
    //   MacroRef = Path (*) "!" ";" ["pub"]
    //   MacroRef = Path (*) "!" ";" ["self"]
    //   MacroRef = Path (*) "!" ";" ["struct"]
    //   MacroRef = Path (*) "!" ";" ["use"]
    //   MacroRef = Path (*) "!" ";" ["{"]
    //   MacroRef = Path (*) "!" ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Path = Path (*) "::" Id ["!"]
    //   Path = Path (*) "::" Id ["::"]
    //
    //   "!" -> Shift(S20)
    //   "::" -> Shift(S21)
    //
    pub fn __state11<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<PathId>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state20(krate, input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state21(krate, input, __lookbehind, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 12
    //   Glob = Privacy (*) "use" Path "::" "*" ";" [EOF]
    //   Glob = Privacy (*) "use" Path "::" "*" ";" ["::"]
    //   Glob = Privacy (*) "use" Path "::" "*" ";" ["macro_rules"]
    //   Glob = Privacy (*) "use" Path "::" "*" ";" ["mod"]
    //   Glob = Privacy (*) "use" Path "::" "*" ";" ["pub"]
    //   Glob = Privacy (*) "use" Path "::" "*" ";" ["self"]
    //   Glob = Privacy (*) "use" Path "::" "*" ";" ["struct"]
    //   Glob = Privacy (*) "use" Path "::" "*" ";" ["use"]
    //   Glob = Privacy (*) "use" Path "::" "*" ";" ["{"]
    //   Glob = Privacy (*) "use" Path "::" "*" ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Import = Privacy (*) "use" Path ";" [EOF]
    //   Import = Privacy (*) "use" Path ";" ["::"]
    //   Import = Privacy (*) "use" Path ";" ["macro_rules"]
    //   Import = Privacy (*) "use" Path ";" ["mod"]
    //   Import = Privacy (*) "use" Path ";" ["pub"]
    //   Import = Privacy (*) "use" Path ";" ["self"]
    //   Import = Privacy (*) "use" Path ";" ["struct"]
    //   Import = Privacy (*) "use" Path ";" ["use"]
    //   Import = Privacy (*) "use" Path ";" ["{"]
    //   Import = Privacy (*) "use" Path ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Import = Privacy (*) "use" Path "as" Id ";" [EOF]
    //   Import = Privacy (*) "use" Path "as" Id ";" ["::"]
    //   Import = Privacy (*) "use" Path "as" Id ";" ["macro_rules"]
    //   Import = Privacy (*) "use" Path "as" Id ";" ["mod"]
    //   Import = Privacy (*) "use" Path "as" Id ";" ["pub"]
    //   Import = Privacy (*) "use" Path "as" Id ";" ["self"]
    //   Import = Privacy (*) "use" Path "as" Id ";" ["struct"]
    //   Import = Privacy (*) "use" Path "as" Id ";" ["use"]
    //   Import = Privacy (*) "use" Path "as" Id ";" ["{"]
    //   Import = Privacy (*) "use" Path "as" Id ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" Item+ "}" [EOF]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" Item+ "}" ["::"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" Item+ "}" ["macro_rules"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" Item+ "}" ["mod"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" Item+ "}" ["pub"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" Item+ "}" ["self"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" Item+ "}" ["struct"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" Item+ "}" ["use"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" Item+ "}" ["{"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" "}" [EOF]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" "}" ["::"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" "}" ["macro_rules"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" "}" ["mod"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" "}" ["pub"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" "}" ["self"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" "}" ["struct"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" "}" ["use"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" "}" ["{"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = Privacy (*) "mod" Id "{" Item+ "}" [EOF]
    //   Module = Privacy (*) "mod" Id "{" Item+ "}" ["::"]
    //   Module = Privacy (*) "mod" Id "{" Item+ "}" ["macro_rules"]
    //   Module = Privacy (*) "mod" Id "{" Item+ "}" ["mod"]
    //   Module = Privacy (*) "mod" Id "{" Item+ "}" ["pub"]
    //   Module = Privacy (*) "mod" Id "{" Item+ "}" ["self"]
    //   Module = Privacy (*) "mod" Id "{" Item+ "}" ["struct"]
    //   Module = Privacy (*) "mod" Id "{" Item+ "}" ["use"]
    //   Module = Privacy (*) "mod" Id "{" Item+ "}" ["{"]
    //   Module = Privacy (*) "mod" Id "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = Privacy (*) "mod" Id "{" "}" [EOF]
    //   Module = Privacy (*) "mod" Id "{" "}" ["::"]
    //   Module = Privacy (*) "mod" Id "{" "}" ["macro_rules"]
    //   Module = Privacy (*) "mod" Id "{" "}" ["mod"]
    //   Module = Privacy (*) "mod" Id "{" "}" ["pub"]
    //   Module = Privacy (*) "mod" Id "{" "}" ["self"]
    //   Module = Privacy (*) "mod" Id "{" "}" ["struct"]
    //   Module = Privacy (*) "mod" Id "{" "}" ["use"]
    //   Module = Privacy (*) "mod" Id "{" "}" ["{"]
    //   Module = Privacy (*) "mod" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Structure = Privacy (*) "struct" Id "{" "}" [EOF]
    //   Structure = Privacy (*) "struct" Id "{" "}" ["::"]
    //   Structure = Privacy (*) "struct" Id "{" "}" ["macro_rules"]
    //   Structure = Privacy (*) "struct" Id "{" "}" ["mod"]
    //   Structure = Privacy (*) "struct" Id "{" "}" ["pub"]
    //   Structure = Privacy (*) "struct" Id "{" "}" ["self"]
    //   Structure = Privacy (*) "struct" Id "{" "}" ["struct"]
    //   Structure = Privacy (*) "struct" Id "{" "}" ["use"]
    //   Structure = Privacy (*) "struct" Id "{" "}" ["{"]
    //   Structure = Privacy (*) "struct" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "macro_rules" -> Shift(S22)
    //   "mod" -> Shift(S23)
    //   "struct" -> Shift(S24)
    //   "use" -> Shift(S25)
    //
    pub fn __state12<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Privacy>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state22(krate, input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state23(krate, input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state24(krate, input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state25(krate, input, __lookbehind, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 13
    //   Item = Structure (*) [EOF]
    //   Item = Structure (*) ["::"]
    //   Item = Structure (*) ["macro_rules"]
    //   Item = Structure (*) ["mod"]
    //   Item = Structure (*) ["pub"]
    //   Item = Structure (*) ["self"]
    //   Item = Structure (*) ["struct"]
    //   Item = Structure (*) ["use"]
    //   Item = Structure (*) ["{"]
    //   Item = Structure (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   EOF -> Reduce(Item = Structure => ActionFn(3);)
    //   "::" -> Reduce(Item = Structure => ActionFn(3);)
    //   "macro_rules" -> Reduce(Item = Structure => ActionFn(3);)
    //   "mod" -> Reduce(Item = Structure => ActionFn(3);)
    //   "pub" -> Reduce(Item = Structure => ActionFn(3);)
    //   "self" -> Reduce(Item = Structure => ActionFn(3);)
    //   "struct" -> Reduce(Item = Structure => ActionFn(3);)
    //   "use" -> Reduce(Item = Structure => ActionFn(3);)
    //   "{" -> Reduce(Item = Structure => ActionFn(3);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Item = Structure => ActionFn(3);)
    //
    pub fn __state13<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<StructureId>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(krate, input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Item(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 14
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["!"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["::"]
    //   Path = "::" (*) Id ["!"]
    //   Path = "::" (*) Id ["::"]
    //
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S18)
    //
    //   Id -> S26
    pub fn __state14<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state18(krate, input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Id(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state26(krate, input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 15
    //   Privacy = "pub" (*) ["macro_rules"]
    //   Privacy = "pub" (*) ["mod"]
    //   Privacy = "pub" (*) ["struct"]
    //   Privacy = "pub" (*) ["use"]
    //
    //   "macro_rules" -> Reduce(Privacy = "pub" => ActionFn(20);)
    //   "mod" -> Reduce(Privacy = "pub" => ActionFn(20);)
    //   "struct" -> Reduce(Privacy = "pub" => ActionFn(20);)
    //   "use" -> Reduce(Privacy = "pub" => ActionFn(20);)
    //
    pub fn __state15<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action20(krate, input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Privacy(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 16
    //   Path = "self" (*) "::" Id ["!"]
    //   Path = "self" (*) "::" Id ["::"]
    //
    //   "::" -> Shift(S27)
    //
    pub fn __state16<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state27(krate, input, __lookbehind, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 17
    //   (<Path> ";")+ = (*) (<Path> ";")+ Path ";" ["::"]
    //   (<Path> ";")+ = (*) (<Path> ";")+ Path ";" ["self"]
    //   (<Path> ";")+ = (*) (<Path> ";")+ Path ";" ["}"]
    //   (<Path> ";")+ = (*) (<Path> ";")+ Path ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   (<Path> ";")+ = (*) Path ";" ["::"]
    //   (<Path> ";")+ = (*) Path ";" ["self"]
    //   (<Path> ";")+ = (*) Path ";" ["}"]
    //   (<Path> ";")+ = (*) Path ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Code = "{" (*) (<Path> ";")+ "}" [EOF]
    //   Code = "{" (*) (<Path> ";")+ "}" ["::"]
    //   Code = "{" (*) (<Path> ";")+ "}" ["macro_rules"]
    //   Code = "{" (*) (<Path> ";")+ "}" ["mod"]
    //   Code = "{" (*) (<Path> ";")+ "}" ["pub"]
    //   Code = "{" (*) (<Path> ";")+ "}" ["self"]
    //   Code = "{" (*) (<Path> ";")+ "}" ["struct"]
    //   Code = "{" (*) (<Path> ";")+ "}" ["use"]
    //   Code = "{" (*) (<Path> ";")+ "}" ["{"]
    //   Code = "{" (*) (<Path> ";")+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Code = "{" (*) "}" [EOF]
    //   Code = "{" (*) "}" ["::"]
    //   Code = "{" (*) "}" ["macro_rules"]
    //   Code = "{" (*) "}" ["mod"]
    //   Code = "{" (*) "}" ["pub"]
    //   Code = "{" (*) "}" ["self"]
    //   Code = "{" (*) "}" ["struct"]
    //   Code = "{" (*) "}" ["use"]
    //   Code = "{" (*) "}" ["{"]
    //   Code = "{" (*) "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["::"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# [";"]
    //   Path = (*) Id ["::"]
    //   Path = (*) Id [";"]
    //   Path = (*) Path "::" Id ["::"]
    //   Path = (*) Path "::" Id [";"]
    //   Path = (*) "::" Id ["::"]
    //   Path = (*) "::" Id [";"]
    //   Path = (*) "self" "::" Id ["::"]
    //   Path = (*) "self" "::" Id [";"]
    //
    //   "::" -> Shift(S31)
    //   "self" -> Shift(S32)
    //   "}" -> Shift(S33)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S34)
    //
    //   (<Path> ";")+ -> S28
    //   Id -> S29
    //   Path -> S30
    pub fn __state17<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state31(krate, input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state32(krate, input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (12, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state33(krate, input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state34(krate, input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28_3cPath_3e_20_22_3b_22_29_2b(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state28(krate, input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Id(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state29(krate, input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Path(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state30(krate, input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 18
    //   Id = r#"[a-zA-Z_][a-zA-Z0-9_]*"# (*) ["!"]
    //   Id = r#"[a-zA-Z_][a-zA-Z0-9_]*"# (*) ["::"]
    //
    //   "!" -> Reduce(Id = r#"[a-zA-Z_][a-zA-Z0-9_]*"# => ActionFn(22);)
    //   "::" -> Reduce(Id = r#"[a-zA-Z_][a-zA-Z0-9_]*"# => ActionFn(22);)
    //
    pub fn __state18<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action22(krate, input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Id(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 19
    //   Item+ = Item+ Item (*) [EOF]
    //   Item+ = Item+ Item (*) ["::"]
    //   Item+ = Item+ Item (*) ["macro_rules"]
    //   Item+ = Item+ Item (*) ["mod"]
    //   Item+ = Item+ Item (*) ["pub"]
    //   Item+ = Item+ Item (*) ["self"]
    //   Item+ = Item+ Item (*) ["struct"]
    //   Item+ = Item+ Item (*) ["use"]
    //   Item+ = Item+ Item (*) ["{"]
    //   Item+ = Item+ Item (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   EOF -> Reduce(Item+ = Item+, Item => ActionFn(33);)
    //   "::" -> Reduce(Item+ = Item+, Item => ActionFn(33);)
    //   "macro_rules" -> Reduce(Item+ = Item+, Item => ActionFn(33);)
    //   "mod" -> Reduce(Item+ = Item+, Item => ActionFn(33);)
    //   "pub" -> Reduce(Item+ = Item+, Item => ActionFn(33);)
    //   "self" -> Reduce(Item+ = Item+, Item => ActionFn(33);)
    //   "struct" -> Reduce(Item+ = Item+, Item => ActionFn(33);)
    //   "use" -> Reduce(Item+ = Item+, Item => ActionFn(33);)
    //   "{" -> Reduce(Item+ = Item+, Item => ActionFn(33);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Item+ = Item+, Item => ActionFn(33);)
    //
    pub fn __state19<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<ItemId>>,
        __sym1: &mut Option<ItemId>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action33(krate, input, __sym0, __sym1, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Item_2b(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 20
    //   MacroRef = Path "!" (*) ";" [EOF]
    //   MacroRef = Path "!" (*) ";" ["::"]
    //   MacroRef = Path "!" (*) ";" ["macro_rules"]
    //   MacroRef = Path "!" (*) ";" ["mod"]
    //   MacroRef = Path "!" (*) ";" ["pub"]
    //   MacroRef = Path "!" (*) ";" ["self"]
    //   MacroRef = Path "!" (*) ";" ["struct"]
    //   MacroRef = Path "!" (*) ";" ["use"]
    //   MacroRef = Path "!" (*) ";" ["{"]
    //   MacroRef = Path "!" (*) ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   ";" -> Shift(S35)
    //
    pub fn __state20<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<PathId>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state35(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 21
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["!"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["::"]
    //   Path = Path "::" (*) Id ["!"]
    //   Path = Path "::" (*) Id ["::"]
    //
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S18)
    //
    //   Id -> S36
    pub fn __state21<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<PathId>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state18(krate, input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Id(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state36(krate, input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 22
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" Item+ "}" [EOF]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" Item+ "}" ["::"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" Item+ "}" ["macro_rules"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" Item+ "}" ["mod"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" Item+ "}" ["pub"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" Item+ "}" ["self"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" Item+ "}" ["struct"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" Item+ "}" ["use"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" Item+ "}" ["{"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" "}" [EOF]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" "}" ["::"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" "}" ["macro_rules"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" "}" ["mod"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" "}" ["pub"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" "}" ["self"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" "}" ["struct"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" "}" ["use"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" "}" ["{"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "!" -> Shift(S37)
    //
    pub fn __state22<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state37(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 23
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["{"]
    //   Module = Privacy "mod" (*) Id "{" Item+ "}" [EOF]
    //   Module = Privacy "mod" (*) Id "{" Item+ "}" ["::"]
    //   Module = Privacy "mod" (*) Id "{" Item+ "}" ["macro_rules"]
    //   Module = Privacy "mod" (*) Id "{" Item+ "}" ["mod"]
    //   Module = Privacy "mod" (*) Id "{" Item+ "}" ["pub"]
    //   Module = Privacy "mod" (*) Id "{" Item+ "}" ["self"]
    //   Module = Privacy "mod" (*) Id "{" Item+ "}" ["struct"]
    //   Module = Privacy "mod" (*) Id "{" Item+ "}" ["use"]
    //   Module = Privacy "mod" (*) Id "{" Item+ "}" ["{"]
    //   Module = Privacy "mod" (*) Id "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = Privacy "mod" (*) Id "{" "}" [EOF]
    //   Module = Privacy "mod" (*) Id "{" "}" ["::"]
    //   Module = Privacy "mod" (*) Id "{" "}" ["macro_rules"]
    //   Module = Privacy "mod" (*) Id "{" "}" ["mod"]
    //   Module = Privacy "mod" (*) Id "{" "}" ["pub"]
    //   Module = Privacy "mod" (*) Id "{" "}" ["self"]
    //   Module = Privacy "mod" (*) Id "{" "}" ["struct"]
    //   Module = Privacy "mod" (*) Id "{" "}" ["use"]
    //   Module = Privacy "mod" (*) Id "{" "}" ["{"]
    //   Module = Privacy "mod" (*) Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S39)
    //
    //   Id -> S38
    pub fn __state23<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state39(krate, input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Id(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state38(krate, input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 24
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["{"]
    //   Structure = Privacy "struct" (*) Id "{" "}" [EOF]
    //   Structure = Privacy "struct" (*) Id "{" "}" ["::"]
    //   Structure = Privacy "struct" (*) Id "{" "}" ["macro_rules"]
    //   Structure = Privacy "struct" (*) Id "{" "}" ["mod"]
    //   Structure = Privacy "struct" (*) Id "{" "}" ["pub"]
    //   Structure = Privacy "struct" (*) Id "{" "}" ["self"]
    //   Structure = Privacy "struct" (*) Id "{" "}" ["struct"]
    //   Structure = Privacy "struct" (*) Id "{" "}" ["use"]
    //   Structure = Privacy "struct" (*) Id "{" "}" ["{"]
    //   Structure = Privacy "struct" (*) Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S39)
    //
    //   Id -> S40
    pub fn __state24<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state39(krate, input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Id(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state40(krate, input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 25
    //   Glob = Privacy "use" (*) Path "::" "*" ";" [EOF]
    //   Glob = Privacy "use" (*) Path "::" "*" ";" ["::"]
    //   Glob = Privacy "use" (*) Path "::" "*" ";" ["macro_rules"]
    //   Glob = Privacy "use" (*) Path "::" "*" ";" ["mod"]
    //   Glob = Privacy "use" (*) Path "::" "*" ";" ["pub"]
    //   Glob = Privacy "use" (*) Path "::" "*" ";" ["self"]
    //   Glob = Privacy "use" (*) Path "::" "*" ";" ["struct"]
    //   Glob = Privacy "use" (*) Path "::" "*" ";" ["use"]
    //   Glob = Privacy "use" (*) Path "::" "*" ";" ["{"]
    //   Glob = Privacy "use" (*) Path "::" "*" ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["::"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# [";"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["as"]
    //   Import = Privacy "use" (*) Path ";" [EOF]
    //   Import = Privacy "use" (*) Path ";" ["::"]
    //   Import = Privacy "use" (*) Path ";" ["macro_rules"]
    //   Import = Privacy "use" (*) Path ";" ["mod"]
    //   Import = Privacy "use" (*) Path ";" ["pub"]
    //   Import = Privacy "use" (*) Path ";" ["self"]
    //   Import = Privacy "use" (*) Path ";" ["struct"]
    //   Import = Privacy "use" (*) Path ";" ["use"]
    //   Import = Privacy "use" (*) Path ";" ["{"]
    //   Import = Privacy "use" (*) Path ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Import = Privacy "use" (*) Path "as" Id ";" [EOF]
    //   Import = Privacy "use" (*) Path "as" Id ";" ["::"]
    //   Import = Privacy "use" (*) Path "as" Id ";" ["macro_rules"]
    //   Import = Privacy "use" (*) Path "as" Id ";" ["mod"]
    //   Import = Privacy "use" (*) Path "as" Id ";" ["pub"]
    //   Import = Privacy "use" (*) Path "as" Id ";" ["self"]
    //   Import = Privacy "use" (*) Path "as" Id ";" ["struct"]
    //   Import = Privacy "use" (*) Path "as" Id ";" ["use"]
    //   Import = Privacy "use" (*) Path "as" Id ";" ["{"]
    //   Import = Privacy "use" (*) Path "as" Id ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Path = (*) Id ["::"]
    //   Path = (*) Id [";"]
    //   Path = (*) Id ["as"]
    //   Path = (*) Path "::" Id ["::"]
    //   Path = (*) Path "::" Id [";"]
    //   Path = (*) Path "::" Id ["as"]
    //   Path = (*) "::" Id ["::"]
    //   Path = (*) "::" Id [";"]
    //   Path = (*) "::" Id ["as"]
    //   Path = (*) "self" "::" Id ["::"]
    //   Path = (*) "self" "::" Id [";"]
    //   Path = (*) "self" "::" Id ["as"]
    //
    //   "::" -> Shift(S43)
    //   "self" -> Shift(S44)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S45)
    //
    //   Id -> S41
    //   Path -> S42
    pub fn __state25<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state43(krate, input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state44(krate, input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state45(krate, input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Id(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state41(krate, input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Path(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state42(krate, input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 26
    //   Path = "::" Id (*) ["!"]
    //   Path = "::" Id (*) ["::"]
    //
    //   "!" -> Reduce(Path = "::", Id => ActionFn(17);)
    //   "::" -> Reduce(Path = "::", Id => ActionFn(17);)
    //
    pub fn __state26<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<InternedString>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action17(krate, input, __sym0, __sym1, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Path(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 27
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["!"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["::"]
    //   Path = "self" "::" (*) Id ["!"]
    //   Path = "self" "::" (*) Id ["::"]
    //
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S18)
    //
    //   Id -> S46
    pub fn __state27<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state18(krate, input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Id(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state46(krate, input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 28
    //   (<Path> ";")+ = (<Path> ";")+ (*) Path ";" ["::"]
    //   (<Path> ";")+ = (<Path> ";")+ (*) Path ";" ["self"]
    //   (<Path> ";")+ = (<Path> ";")+ (*) Path ";" ["}"]
    //   (<Path> ";")+ = (<Path> ";")+ (*) Path ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Code = "{" (<Path> ";")+ (*) "}" [EOF]
    //   Code = "{" (<Path> ";")+ (*) "}" ["::"]
    //   Code = "{" (<Path> ";")+ (*) "}" ["macro_rules"]
    //   Code = "{" (<Path> ";")+ (*) "}" ["mod"]
    //   Code = "{" (<Path> ";")+ (*) "}" ["pub"]
    //   Code = "{" (<Path> ";")+ (*) "}" ["self"]
    //   Code = "{" (<Path> ";")+ (*) "}" ["struct"]
    //   Code = "{" (<Path> ";")+ (*) "}" ["use"]
    //   Code = "{" (<Path> ";")+ (*) "}" ["{"]
    //   Code = "{" (<Path> ";")+ (*) "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["::"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# [";"]
    //   Path = (*) Id ["::"]
    //   Path = (*) Id [";"]
    //   Path = (*) Path "::" Id ["::"]
    //   Path = (*) Path "::" Id [";"]
    //   Path = (*) "::" Id ["::"]
    //   Path = (*) "::" Id [";"]
    //   Path = (*) "self" "::" Id ["::"]
    //   Path = (*) "self" "::" Id [";"]
    //
    //   "::" -> Shift(S31)
    //   "self" -> Shift(S32)
    //   "}" -> Shift(S48)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S34)
    //
    //   Id -> S29
    //   Path -> S47
    pub fn __state28<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<::std::vec::Vec<PathId>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state31(krate, input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state32(krate, input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (12, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state48(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state34(krate, input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Id(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state29(krate, input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Path(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state47(krate, input, __lookbehind, __tokens, __lookahead, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 29
    //   Path = Id (*) ["::"]
    //   Path = Id (*) [";"]
    //
    //   "::" -> Reduce(Path = Id => ActionFn(18);)
    //   ";" -> Reduce(Path = Id => ActionFn(18);)
    //
    pub fn __state29<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<InternedString>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action18(krate, input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Path(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 30
    //   (<Path> ";")+ = Path (*) ";" ["::"]
    //   (<Path> ";")+ = Path (*) ";" ["self"]
    //   (<Path> ";")+ = Path (*) ";" ["}"]
    //   (<Path> ";")+ = Path (*) ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Path = Path (*) "::" Id ["::"]
    //   Path = Path (*) "::" Id [";"]
    //
    //   "::" -> Shift(S49)
    //   ";" -> Shift(S50)
    //
    pub fn __state30<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<PathId>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state49(krate, input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state50(krate, input, __lookbehind, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 31
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["::"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# [";"]
    //   Path = "::" (*) Id ["::"]
    //   Path = "::" (*) Id [";"]
    //
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S34)
    //
    //   Id -> S51
    pub fn __state31<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state34(krate, input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Id(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state51(krate, input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 32
    //   Path = "self" (*) "::" Id ["::"]
    //   Path = "self" (*) "::" Id [";"]
    //
    //   "::" -> Shift(S52)
    //
    pub fn __state32<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state52(krate, input, __lookbehind, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 33
    //   Code = "{" "}" (*) [EOF]
    //   Code = "{" "}" (*) ["::"]
    //   Code = "{" "}" (*) ["macro_rules"]
    //   Code = "{" "}" (*) ["mod"]
    //   Code = "{" "}" (*) ["pub"]
    //   Code = "{" "}" (*) ["self"]
    //   Code = "{" "}" (*) ["struct"]
    //   Code = "{" "}" (*) ["use"]
    //   Code = "{" "}" (*) ["{"]
    //   Code = "{" "}" (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   EOF -> Reduce(Code = "{", "}" => ActionFn(42);)
    //   "::" -> Reduce(Code = "{", "}" => ActionFn(42);)
    //   "macro_rules" -> Reduce(Code = "{", "}" => ActionFn(42);)
    //   "mod" -> Reduce(Code = "{", "}" => ActionFn(42);)
    //   "pub" -> Reduce(Code = "{", "}" => ActionFn(42);)
    //   "self" -> Reduce(Code = "{", "}" => ActionFn(42);)
    //   "struct" -> Reduce(Code = "{", "}" => ActionFn(42);)
    //   "use" -> Reduce(Code = "{", "}" => ActionFn(42);)
    //   "{" -> Reduce(Code = "{", "}" => ActionFn(42);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Code = "{", "}" => ActionFn(42);)
    //
    pub fn __state33<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action42(krate, input, __sym0, __sym1, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Code(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 34
    //   Id = r#"[a-zA-Z_][a-zA-Z0-9_]*"# (*) ["::"]
    //   Id = r#"[a-zA-Z_][a-zA-Z0-9_]*"# (*) [";"]
    //
    //   "::" -> Reduce(Id = r#"[a-zA-Z_][a-zA-Z0-9_]*"# => ActionFn(22);)
    //   ";" -> Reduce(Id = r#"[a-zA-Z_][a-zA-Z0-9_]*"# => ActionFn(22);)
    //
    pub fn __state34<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action22(krate, input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Id(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 35
    //   MacroRef = Path "!" ";" (*) [EOF]
    //   MacroRef = Path "!" ";" (*) ["::"]
    //   MacroRef = Path "!" ";" (*) ["macro_rules"]
    //   MacroRef = Path "!" ";" (*) ["mod"]
    //   MacroRef = Path "!" ";" (*) ["pub"]
    //   MacroRef = Path "!" ";" (*) ["self"]
    //   MacroRef = Path "!" ";" (*) ["struct"]
    //   MacroRef = Path "!" ";" (*) ["use"]
    //   MacroRef = Path "!" ";" (*) ["{"]
    //   MacroRef = Path "!" ";" (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   EOF -> Reduce(MacroRef = Path, "!", ";" => ActionFn(14);)
    //   "::" -> Reduce(MacroRef = Path, "!", ";" => ActionFn(14);)
    //   "macro_rules" -> Reduce(MacroRef = Path, "!", ";" => ActionFn(14);)
    //   "mod" -> Reduce(MacroRef = Path, "!", ";" => ActionFn(14);)
    //   "pub" -> Reduce(MacroRef = Path, "!", ";" => ActionFn(14);)
    //   "self" -> Reduce(MacroRef = Path, "!", ";" => ActionFn(14);)
    //   "struct" -> Reduce(MacroRef = Path, "!", ";" => ActionFn(14);)
    //   "use" -> Reduce(MacroRef = Path, "!", ";" => ActionFn(14);)
    //   "{" -> Reduce(MacroRef = Path, "!", ";" => ActionFn(14);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(MacroRef = Path, "!", ";" => ActionFn(14);)
    //
    pub fn __state35<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<PathId>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action14(krate, input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::MacroRef(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 36
    //   Path = Path "::" Id (*) ["!"]
    //   Path = Path "::" Id (*) ["::"]
    //
    //   "!" -> Reduce(Path = Path, "::", Id => ActionFn(19);)
    //   "::" -> Reduce(Path = Path, "::", Id => ActionFn(19);)
    //
    pub fn __state36<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<PathId>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<InternedString>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action19(krate, input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Path(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 37
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["{"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" Item+ "}" [EOF]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" Item+ "}" ["::"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" Item+ "}" ["macro_rules"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" Item+ "}" ["mod"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" Item+ "}" ["pub"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" Item+ "}" ["self"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" Item+ "}" ["struct"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" Item+ "}" ["use"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" Item+ "}" ["{"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" "}" [EOF]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" "}" ["::"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" "}" ["macro_rules"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" "}" ["mod"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" "}" ["pub"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" "}" ["self"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" "}" ["struct"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" "}" ["use"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" "}" ["{"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S39)
    //
    //   Id -> S53
    pub fn __state37<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state39(krate, input, __lookbehind, __tokens, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Id(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state53(krate, input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 38
    //   Module = Privacy "mod" Id (*) "{" Item+ "}" [EOF]
    //   Module = Privacy "mod" Id (*) "{" Item+ "}" ["::"]
    //   Module = Privacy "mod" Id (*) "{" Item+ "}" ["macro_rules"]
    //   Module = Privacy "mod" Id (*) "{" Item+ "}" ["mod"]
    //   Module = Privacy "mod" Id (*) "{" Item+ "}" ["pub"]
    //   Module = Privacy "mod" Id (*) "{" Item+ "}" ["self"]
    //   Module = Privacy "mod" Id (*) "{" Item+ "}" ["struct"]
    //   Module = Privacy "mod" Id (*) "{" Item+ "}" ["use"]
    //   Module = Privacy "mod" Id (*) "{" Item+ "}" ["{"]
    //   Module = Privacy "mod" Id (*) "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = Privacy "mod" Id (*) "{" "}" [EOF]
    //   Module = Privacy "mod" Id (*) "{" "}" ["::"]
    //   Module = Privacy "mod" Id (*) "{" "}" ["macro_rules"]
    //   Module = Privacy "mod" Id (*) "{" "}" ["mod"]
    //   Module = Privacy "mod" Id (*) "{" "}" ["pub"]
    //   Module = Privacy "mod" Id (*) "{" "}" ["self"]
    //   Module = Privacy "mod" Id (*) "{" "}" ["struct"]
    //   Module = Privacy "mod" Id (*) "{" "}" ["use"]
    //   Module = Privacy "mod" Id (*) "{" "}" ["{"]
    //   Module = Privacy "mod" Id (*) "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "{" -> Shift(S54)
    //
    pub fn __state38<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<InternedString>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state54(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 39
    //   Id = r#"[a-zA-Z_][a-zA-Z0-9_]*"# (*) ["{"]
    //
    //   "{" -> Reduce(Id = r#"[a-zA-Z_][a-zA-Z0-9_]*"# => ActionFn(22);)
    //
    pub fn __state39<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (11, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action22(krate, input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Id(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 40
    //   Structure = Privacy "struct" Id (*) "{" "}" [EOF]
    //   Structure = Privacy "struct" Id (*) "{" "}" ["::"]
    //   Structure = Privacy "struct" Id (*) "{" "}" ["macro_rules"]
    //   Structure = Privacy "struct" Id (*) "{" "}" ["mod"]
    //   Structure = Privacy "struct" Id (*) "{" "}" ["pub"]
    //   Structure = Privacy "struct" Id (*) "{" "}" ["self"]
    //   Structure = Privacy "struct" Id (*) "{" "}" ["struct"]
    //   Structure = Privacy "struct" Id (*) "{" "}" ["use"]
    //   Structure = Privacy "struct" Id (*) "{" "}" ["{"]
    //   Structure = Privacy "struct" Id (*) "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "{" -> Shift(S55)
    //
    pub fn __state40<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<InternedString>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state55(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 41
    //   Path = Id (*) ["::"]
    //   Path = Id (*) [";"]
    //   Path = Id (*) ["as"]
    //
    //   "::" -> Reduce(Path = Id => ActionFn(18);)
    //   ";" -> Reduce(Path = Id => ActionFn(18);)
    //   "as" -> Reduce(Path = Id => ActionFn(18);)
    //
    pub fn __state41<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<InternedString>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action18(krate, input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Path(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 42
    //   Glob = Privacy "use" Path (*) "::" "*" ";" [EOF]
    //   Glob = Privacy "use" Path (*) "::" "*" ";" ["::"]
    //   Glob = Privacy "use" Path (*) "::" "*" ";" ["macro_rules"]
    //   Glob = Privacy "use" Path (*) "::" "*" ";" ["mod"]
    //   Glob = Privacy "use" Path (*) "::" "*" ";" ["pub"]
    //   Glob = Privacy "use" Path (*) "::" "*" ";" ["self"]
    //   Glob = Privacy "use" Path (*) "::" "*" ";" ["struct"]
    //   Glob = Privacy "use" Path (*) "::" "*" ";" ["use"]
    //   Glob = Privacy "use" Path (*) "::" "*" ";" ["{"]
    //   Glob = Privacy "use" Path (*) "::" "*" ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Import = Privacy "use" Path (*) ";" [EOF]
    //   Import = Privacy "use" Path (*) ";" ["::"]
    //   Import = Privacy "use" Path (*) ";" ["macro_rules"]
    //   Import = Privacy "use" Path (*) ";" ["mod"]
    //   Import = Privacy "use" Path (*) ";" ["pub"]
    //   Import = Privacy "use" Path (*) ";" ["self"]
    //   Import = Privacy "use" Path (*) ";" ["struct"]
    //   Import = Privacy "use" Path (*) ";" ["use"]
    //   Import = Privacy "use" Path (*) ";" ["{"]
    //   Import = Privacy "use" Path (*) ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Import = Privacy "use" Path (*) "as" Id ";" [EOF]
    //   Import = Privacy "use" Path (*) "as" Id ";" ["::"]
    //   Import = Privacy "use" Path (*) "as" Id ";" ["macro_rules"]
    //   Import = Privacy "use" Path (*) "as" Id ";" ["mod"]
    //   Import = Privacy "use" Path (*) "as" Id ";" ["pub"]
    //   Import = Privacy "use" Path (*) "as" Id ";" ["self"]
    //   Import = Privacy "use" Path (*) "as" Id ";" ["struct"]
    //   Import = Privacy "use" Path (*) "as" Id ";" ["use"]
    //   Import = Privacy "use" Path (*) "as" Id ";" ["{"]
    //   Import = Privacy "use" Path (*) "as" Id ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Path = Path (*) "::" Id ["::"]
    //   Path = Path (*) "::" Id [";"]
    //   Path = Path (*) "::" Id ["as"]
    //
    //   "::" -> Shift(S56)
    //   ";" -> Shift(S57)
    //   "as" -> Shift(S58)
    //
    pub fn __state42<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<PathId>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state56(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state57(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state58(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 43
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["::"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# [";"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["as"]
    //   Path = "::" (*) Id ["::"]
    //   Path = "::" (*) Id [";"]
    //   Path = "::" (*) Id ["as"]
    //
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S45)
    //
    //   Id -> S59
    pub fn __state43<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state45(krate, input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Id(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state59(krate, input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 44
    //   Path = "self" (*) "::" Id ["::"]
    //   Path = "self" (*) "::" Id [";"]
    //   Path = "self" (*) "::" Id ["as"]
    //
    //   "::" -> Shift(S60)
    //
    pub fn __state44<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state60(krate, input, __lookbehind, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 45
    //   Id = r#"[a-zA-Z_][a-zA-Z0-9_]*"# (*) ["::"]
    //   Id = r#"[a-zA-Z_][a-zA-Z0-9_]*"# (*) [";"]
    //   Id = r#"[a-zA-Z_][a-zA-Z0-9_]*"# (*) ["as"]
    //
    //   "::" -> Reduce(Id = r#"[a-zA-Z_][a-zA-Z0-9_]*"# => ActionFn(22);)
    //   ";" -> Reduce(Id = r#"[a-zA-Z_][a-zA-Z0-9_]*"# => ActionFn(22);)
    //   "as" -> Reduce(Id = r#"[a-zA-Z_][a-zA-Z0-9_]*"# => ActionFn(22);)
    //
    pub fn __state45<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action22(krate, input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Id(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 46
    //   Path = "self" "::" Id (*) ["!"]
    //   Path = "self" "::" Id (*) ["::"]
    //
    //   "!" -> Reduce(Path = "self", "::", Id => ActionFn(16);)
    //   "::" -> Reduce(Path = "self", "::", Id => ActionFn(16);)
    //
    pub fn __state46<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<InternedString>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, _), _)) |
            Some((_, (2, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action16(krate, input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Path(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 47
    //   (<Path> ";")+ = (<Path> ";")+ Path (*) ";" ["::"]
    //   (<Path> ";")+ = (<Path> ";")+ Path (*) ";" ["self"]
    //   (<Path> ";")+ = (<Path> ";")+ Path (*) ";" ["}"]
    //   (<Path> ";")+ = (<Path> ";")+ Path (*) ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Path = Path (*) "::" Id ["::"]
    //   Path = Path (*) "::" Id [";"]
    //
    //   "::" -> Shift(S49)
    //   ";" -> Shift(S61)
    //
    pub fn __state47<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<PathId>>,
        __sym1: &mut Option<PathId>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state49(krate, input, __lookbehind, __tokens, __sym1, __sym2));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state61(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 48
    //   Code = "{" (<Path> ";")+ "}" (*) [EOF]
    //   Code = "{" (<Path> ";")+ "}" (*) ["::"]
    //   Code = "{" (<Path> ";")+ "}" (*) ["macro_rules"]
    //   Code = "{" (<Path> ";")+ "}" (*) ["mod"]
    //   Code = "{" (<Path> ";")+ "}" (*) ["pub"]
    //   Code = "{" (<Path> ";")+ "}" (*) ["self"]
    //   Code = "{" (<Path> ";")+ "}" (*) ["struct"]
    //   Code = "{" (<Path> ";")+ "}" (*) ["use"]
    //   Code = "{" (<Path> ";")+ "}" (*) ["{"]
    //   Code = "{" (<Path> ";")+ "}" (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   EOF -> Reduce(Code = "{", (<Path> ";")+, "}" => ActionFn(43);)
    //   "::" -> Reduce(Code = "{", (<Path> ";")+, "}" => ActionFn(43);)
    //   "macro_rules" -> Reduce(Code = "{", (<Path> ";")+, "}" => ActionFn(43);)
    //   "mod" -> Reduce(Code = "{", (<Path> ";")+, "}" => ActionFn(43);)
    //   "pub" -> Reduce(Code = "{", (<Path> ";")+, "}" => ActionFn(43);)
    //   "self" -> Reduce(Code = "{", (<Path> ";")+, "}" => ActionFn(43);)
    //   "struct" -> Reduce(Code = "{", (<Path> ";")+, "}" => ActionFn(43);)
    //   "use" -> Reduce(Code = "{", (<Path> ";")+, "}" => ActionFn(43);)
    //   "{" -> Reduce(Code = "{", (<Path> ";")+, "}" => ActionFn(43);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Code = "{", (<Path> ";")+, "}" => ActionFn(43);)
    //
    pub fn __state48<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<::std::vec::Vec<PathId>>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action43(krate, input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Code(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 49
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["::"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# [";"]
    //   Path = Path "::" (*) Id ["::"]
    //   Path = Path "::" (*) Id [";"]
    //
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S34)
    //
    //   Id -> S62
    pub fn __state49<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<PathId>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state34(krate, input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Id(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state62(krate, input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 50
    //   (<Path> ";")+ = Path ";" (*) ["::"]
    //   (<Path> ";")+ = Path ";" (*) ["self"]
    //   (<Path> ";")+ = Path ";" (*) ["}"]
    //   (<Path> ";")+ = Path ";" (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "::" -> Reduce((<Path> ";")+ = Path, ";" => ActionFn(40);)
    //   "self" -> Reduce((<Path> ";")+ = Path, ";" => ActionFn(40);)
    //   "}" -> Reduce((<Path> ";")+ = Path, ";" => ActionFn(40);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce((<Path> ";")+ = Path, ";" => ActionFn(40);)
    //
    pub fn __state50<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<PathId>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action40(krate, input, __sym0, __sym1, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28_3cPath_3e_20_22_3b_22_29_2b(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 51
    //   Path = "::" Id (*) ["::"]
    //   Path = "::" Id (*) [";"]
    //
    //   "::" -> Reduce(Path = "::", Id => ActionFn(17);)
    //   ";" -> Reduce(Path = "::", Id => ActionFn(17);)
    //
    pub fn __state51<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<InternedString>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action17(krate, input, __sym0, __sym1, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Path(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 52
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["::"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# [";"]
    //   Path = "self" "::" (*) Id ["::"]
    //   Path = "self" "::" (*) Id [";"]
    //
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S34)
    //
    //   Id -> S63
    pub fn __state52<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state34(krate, input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Id(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state63(krate, input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 53
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" Item+ "}" [EOF]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" Item+ "}" ["::"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" Item+ "}" ["macro_rules"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" Item+ "}" ["mod"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" Item+ "}" ["pub"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" Item+ "}" ["self"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" Item+ "}" ["struct"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" Item+ "}" ["use"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" Item+ "}" ["{"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" "}" [EOF]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" "}" ["::"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" "}" ["macro_rules"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" "}" ["mod"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" "}" ["pub"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" "}" ["self"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" "}" ["struct"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" "}" ["use"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" "}" ["{"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "{" -> Shift(S64)
    //
    pub fn __state53<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<&'input str>,
        __sym3: &mut Option<InternedString>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state64(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3, __sym4));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 54
    //   Code = (*) "{" (<Path> ";")+ "}" ["::"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["macro_rules"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["mod"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["pub"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["self"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["struct"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["use"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["{"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["}"]
    //   Code = (*) "{" (<Path> ";")+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Code = (*) "{" "}" ["::"]
    //   Code = (*) "{" "}" ["macro_rules"]
    //   Code = (*) "{" "}" ["mod"]
    //   Code = (*) "{" "}" ["pub"]
    //   Code = (*) "{" "}" ["self"]
    //   Code = (*) "{" "}" ["struct"]
    //   Code = (*) "{" "}" ["use"]
    //   Code = (*) "{" "}" ["{"]
    //   Code = (*) "{" "}" ["}"]
    //   Code = (*) "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["::"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["macro_rules"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["mod"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["pub"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["self"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["struct"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["use"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["{"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["}"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["!"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["::"]
    //   Import = (*) Privacy "use" Path ";" ["::"]
    //   Import = (*) Privacy "use" Path ";" ["macro_rules"]
    //   Import = (*) Privacy "use" Path ";" ["mod"]
    //   Import = (*) Privacy "use" Path ";" ["pub"]
    //   Import = (*) Privacy "use" Path ";" ["self"]
    //   Import = (*) Privacy "use" Path ";" ["struct"]
    //   Import = (*) Privacy "use" Path ";" ["use"]
    //   Import = (*) Privacy "use" Path ";" ["{"]
    //   Import = (*) Privacy "use" Path ";" ["}"]
    //   Import = (*) Privacy "use" Path ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["::"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["macro_rules"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["mod"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["pub"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["self"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["struct"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["use"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["{"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["}"]
    //   Import = (*) Privacy "use" Path "as" Id ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Code ["::"]
    //   Item = (*) Code ["macro_rules"]
    //   Item = (*) Code ["mod"]
    //   Item = (*) Code ["pub"]
    //   Item = (*) Code ["self"]
    //   Item = (*) Code ["struct"]
    //   Item = (*) Code ["use"]
    //   Item = (*) Code ["{"]
    //   Item = (*) Code ["}"]
    //   Item = (*) Code [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Glob ["::"]
    //   Item = (*) Glob ["macro_rules"]
    //   Item = (*) Glob ["mod"]
    //   Item = (*) Glob ["pub"]
    //   Item = (*) Glob ["self"]
    //   Item = (*) Glob ["struct"]
    //   Item = (*) Glob ["use"]
    //   Item = (*) Glob ["{"]
    //   Item = (*) Glob ["}"]
    //   Item = (*) Glob [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Import ["::"]
    //   Item = (*) Import ["macro_rules"]
    //   Item = (*) Import ["mod"]
    //   Item = (*) Import ["pub"]
    //   Item = (*) Import ["self"]
    //   Item = (*) Import ["struct"]
    //   Item = (*) Import ["use"]
    //   Item = (*) Import ["{"]
    //   Item = (*) Import ["}"]
    //   Item = (*) Import [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) MacroDef ["::"]
    //   Item = (*) MacroDef ["macro_rules"]
    //   Item = (*) MacroDef ["mod"]
    //   Item = (*) MacroDef ["pub"]
    //   Item = (*) MacroDef ["self"]
    //   Item = (*) MacroDef ["struct"]
    //   Item = (*) MacroDef ["use"]
    //   Item = (*) MacroDef ["{"]
    //   Item = (*) MacroDef ["}"]
    //   Item = (*) MacroDef [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) MacroRef ["::"]
    //   Item = (*) MacroRef ["macro_rules"]
    //   Item = (*) MacroRef ["mod"]
    //   Item = (*) MacroRef ["pub"]
    //   Item = (*) MacroRef ["self"]
    //   Item = (*) MacroRef ["struct"]
    //   Item = (*) MacroRef ["use"]
    //   Item = (*) MacroRef ["{"]
    //   Item = (*) MacroRef ["}"]
    //   Item = (*) MacroRef [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Module ["::"]
    //   Item = (*) Module ["macro_rules"]
    //   Item = (*) Module ["mod"]
    //   Item = (*) Module ["pub"]
    //   Item = (*) Module ["self"]
    //   Item = (*) Module ["struct"]
    //   Item = (*) Module ["use"]
    //   Item = (*) Module ["{"]
    //   Item = (*) Module ["}"]
    //   Item = (*) Module [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Structure ["::"]
    //   Item = (*) Structure ["macro_rules"]
    //   Item = (*) Structure ["mod"]
    //   Item = (*) Structure ["pub"]
    //   Item = (*) Structure ["self"]
    //   Item = (*) Structure ["struct"]
    //   Item = (*) Structure ["use"]
    //   Item = (*) Structure ["{"]
    //   Item = (*) Structure ["}"]
    //   Item = (*) Structure [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item+ = (*) Item ["::"]
    //   Item+ = (*) Item ["macro_rules"]
    //   Item+ = (*) Item ["mod"]
    //   Item+ = (*) Item ["pub"]
    //   Item+ = (*) Item ["self"]
    //   Item+ = (*) Item ["struct"]
    //   Item+ = (*) Item ["use"]
    //   Item+ = (*) Item ["{"]
    //   Item+ = (*) Item ["}"]
    //   Item+ = (*) Item [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item+ = (*) Item+ Item ["::"]
    //   Item+ = (*) Item+ Item ["macro_rules"]
    //   Item+ = (*) Item+ Item ["mod"]
    //   Item+ = (*) Item+ Item ["pub"]
    //   Item+ = (*) Item+ Item ["self"]
    //   Item+ = (*) Item+ Item ["struct"]
    //   Item+ = (*) Item+ Item ["use"]
    //   Item+ = (*) Item+ Item ["{"]
    //   Item+ = (*) Item+ Item ["}"]
    //   Item+ = (*) Item+ Item [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["::"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["macro_rules"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["mod"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["pub"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["self"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["struct"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["use"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["{"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["}"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["::"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["macro_rules"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["mod"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["pub"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["self"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["struct"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["use"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["{"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["}"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroRef = (*) Path "!" ";" ["::"]
    //   MacroRef = (*) Path "!" ";" ["macro_rules"]
    //   MacroRef = (*) Path "!" ";" ["mod"]
    //   MacroRef = (*) Path "!" ";" ["pub"]
    //   MacroRef = (*) Path "!" ";" ["self"]
    //   MacroRef = (*) Path "!" ";" ["struct"]
    //   MacroRef = (*) Path "!" ";" ["use"]
    //   MacroRef = (*) Path "!" ";" ["{"]
    //   MacroRef = (*) Path "!" ";" ["}"]
    //   MacroRef = (*) Path "!" ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["::"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["macro_rules"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["mod"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["pub"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["self"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["struct"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["use"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["{"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["}"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = Privacy "mod" Id "{" (*) Item+ "}" [EOF]
    //   Module = Privacy "mod" Id "{" (*) Item+ "}" ["::"]
    //   Module = Privacy "mod" Id "{" (*) Item+ "}" ["macro_rules"]
    //   Module = Privacy "mod" Id "{" (*) Item+ "}" ["mod"]
    //   Module = Privacy "mod" Id "{" (*) Item+ "}" ["pub"]
    //   Module = Privacy "mod" Id "{" (*) Item+ "}" ["self"]
    //   Module = Privacy "mod" Id "{" (*) Item+ "}" ["struct"]
    //   Module = Privacy "mod" Id "{" (*) Item+ "}" ["use"]
    //   Module = Privacy "mod" Id "{" (*) Item+ "}" ["{"]
    //   Module = Privacy "mod" Id "{" (*) Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = (*) Privacy "mod" Id "{" "}" ["::"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["macro_rules"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["mod"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["pub"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["self"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["struct"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["use"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["{"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["}"]
    //   Module = (*) Privacy "mod" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = Privacy "mod" Id "{" (*) "}" [EOF]
    //   Module = Privacy "mod" Id "{" (*) "}" ["::"]
    //   Module = Privacy "mod" Id "{" (*) "}" ["macro_rules"]
    //   Module = Privacy "mod" Id "{" (*) "}" ["mod"]
    //   Module = Privacy "mod" Id "{" (*) "}" ["pub"]
    //   Module = Privacy "mod" Id "{" (*) "}" ["self"]
    //   Module = Privacy "mod" Id "{" (*) "}" ["struct"]
    //   Module = Privacy "mod" Id "{" (*) "}" ["use"]
    //   Module = Privacy "mod" Id "{" (*) "}" ["{"]
    //   Module = Privacy "mod" Id "{" (*) "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Path = (*) Id ["!"]
    //   Path = (*) Id ["::"]
    //   Path = (*) Path "::" Id ["!"]
    //   Path = (*) Path "::" Id ["::"]
    //   Path = (*) "::" Id ["!"]
    //   Path = (*) "::" Id ["::"]
    //   Path = (*) "self" "::" Id ["!"]
    //   Path = (*) "self" "::" Id ["::"]
    //   Privacy = (*) ["macro_rules"]
    //   Privacy = (*) ["mod"]
    //   Privacy = (*) ["struct"]
    //   Privacy = (*) ["use"]
    //   Privacy = (*) "pub" ["macro_rules"]
    //   Privacy = (*) "pub" ["mod"]
    //   Privacy = (*) "pub" ["struct"]
    //   Privacy = (*) "pub" ["use"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["::"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["macro_rules"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["mod"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["pub"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["self"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["struct"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["use"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["{"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["}"]
    //   Structure = (*) Privacy "struct" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "::" -> Shift(S14)
    //   "macro_rules" -> Reduce(Privacy =  => ActionFn(39);)
    //   "mod" -> Reduce(Privacy =  => ActionFn(39);)
    //   "pub" -> Shift(S15)
    //   "self" -> Shift(S16)
    //   "struct" -> Reduce(Privacy =  => ActionFn(39);)
    //   "use" -> Reduce(Privacy =  => ActionFn(39);)
    //   "{" -> Shift(S76)
    //   "}" -> Shift(S77)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S18)
    //
    //   Code -> S65
    //   Glob -> S66
    //   Id -> S3
    //   Import -> S67
    //   Item -> S68
    //   Item+ -> S69
    //   MacroDef -> S70
    //   MacroRef -> S71
    //   Module -> S72
    //   Path -> S73
    //   Privacy -> S74
    //   Structure -> S75
    pub fn __state54<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<InternedString>,
        __sym3: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state14(krate, input, __lookbehind, __tokens, __sym4));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state15(krate, input, __lookbehind, __tokens, __sym4));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state16(krate, input, __lookbehind, __tokens, __sym4));
            }
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state76(krate, input, __lookbehind, __tokens, __sym4));
            }
            Some((_, (12, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state77(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3, __sym4));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state18(krate, input, __lookbehind, __tokens, __sym4));
            }
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) => {
                let __nt = super::__action39(krate, input, &__lookbehind, &__lookahead);
                __result = (__lookbehind, __lookahead, __Nonterminal::Privacy(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym3.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Code(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state65(krate, input, __lookbehind, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::Glob(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state66(krate, input, __lookbehind, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::Id(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state3(krate, input, __lookbehind, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::Import(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state67(krate, input, __lookbehind, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::Item(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state68(krate, input, __lookbehind, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::Item_2b(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state69(krate, input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3, __sym4));
                }
                __Nonterminal::MacroDef(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state70(krate, input, __lookbehind, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::MacroRef(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state71(krate, input, __lookbehind, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::Module(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state72(krate, input, __lookbehind, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::Path(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state73(krate, input, __lookbehind, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::Privacy(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state74(krate, input, __lookbehind, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::Structure(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state75(krate, input, __lookbehind, __tokens, __lookahead, __sym4));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 55
    //   Structure = Privacy "struct" Id "{" (*) "}" [EOF]
    //   Structure = Privacy "struct" Id "{" (*) "}" ["::"]
    //   Structure = Privacy "struct" Id "{" (*) "}" ["macro_rules"]
    //   Structure = Privacy "struct" Id "{" (*) "}" ["mod"]
    //   Structure = Privacy "struct" Id "{" (*) "}" ["pub"]
    //   Structure = Privacy "struct" Id "{" (*) "}" ["self"]
    //   Structure = Privacy "struct" Id "{" (*) "}" ["struct"]
    //   Structure = Privacy "struct" Id "{" (*) "}" ["use"]
    //   Structure = Privacy "struct" Id "{" (*) "}" ["{"]
    //   Structure = Privacy "struct" Id "{" (*) "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "}" -> Shift(S78)
    //
    pub fn __state55<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<InternedString>,
        __sym3: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (12, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state78(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3, __sym4));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 56
    //   Glob = Privacy "use" Path "::" (*) "*" ";" [EOF]
    //   Glob = Privacy "use" Path "::" (*) "*" ";" ["::"]
    //   Glob = Privacy "use" Path "::" (*) "*" ";" ["macro_rules"]
    //   Glob = Privacy "use" Path "::" (*) "*" ";" ["mod"]
    //   Glob = Privacy "use" Path "::" (*) "*" ";" ["pub"]
    //   Glob = Privacy "use" Path "::" (*) "*" ";" ["self"]
    //   Glob = Privacy "use" Path "::" (*) "*" ";" ["struct"]
    //   Glob = Privacy "use" Path "::" (*) "*" ";" ["use"]
    //   Glob = Privacy "use" Path "::" (*) "*" ";" ["{"]
    //   Glob = Privacy "use" Path "::" (*) "*" ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["::"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# [";"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["as"]
    //   Path = Path "::" (*) Id ["::"]
    //   Path = Path "::" (*) Id [";"]
    //   Path = Path "::" (*) Id ["as"]
    //
    //   "*" -> Shift(S80)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S45)
    //
    //   Id -> S79
    pub fn __state56<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<PathId>,
        __sym3: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state80(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3, __sym4));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state45(krate, input, __lookbehind, __tokens, __sym4));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym3.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Id(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state79(krate, input, __lookbehind, __tokens, __lookahead, __sym2, __sym3, __sym4));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 57
    //   Import = Privacy "use" Path ";" (*) [EOF]
    //   Import = Privacy "use" Path ";" (*) ["::"]
    //   Import = Privacy "use" Path ";" (*) ["macro_rules"]
    //   Import = Privacy "use" Path ";" (*) ["mod"]
    //   Import = Privacy "use" Path ";" (*) ["pub"]
    //   Import = Privacy "use" Path ";" (*) ["self"]
    //   Import = Privacy "use" Path ";" (*) ["struct"]
    //   Import = Privacy "use" Path ";" (*) ["use"]
    //   Import = Privacy "use" Path ";" (*) ["{"]
    //   Import = Privacy "use" Path ";" (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   EOF -> Reduce(Import = Privacy, "use", Path, ";" => ActionFn(38);)
    //   "::" -> Reduce(Import = Privacy, "use", Path, ";" => ActionFn(38);)
    //   "macro_rules" -> Reduce(Import = Privacy, "use", Path, ";" => ActionFn(38);)
    //   "mod" -> Reduce(Import = Privacy, "use", Path, ";" => ActionFn(38);)
    //   "pub" -> Reduce(Import = Privacy, "use", Path, ";" => ActionFn(38);)
    //   "self" -> Reduce(Import = Privacy, "use", Path, ";" => ActionFn(38);)
    //   "struct" -> Reduce(Import = Privacy, "use", Path, ";" => ActionFn(38);)
    //   "use" -> Reduce(Import = Privacy, "use", Path, ";" => ActionFn(38);)
    //   "{" -> Reduce(Import = Privacy, "use", Path, ";" => ActionFn(38);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Import = Privacy, "use", Path, ";" => ActionFn(38);)
    //
    pub fn __state57<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<PathId>,
        __sym3: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action38(krate, input, __sym0, __sym1, __sym2, __sym3, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Import(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 58
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# [";"]
    //   Import = Privacy "use" Path "as" (*) Id ";" [EOF]
    //   Import = Privacy "use" Path "as" (*) Id ";" ["::"]
    //   Import = Privacy "use" Path "as" (*) Id ";" ["macro_rules"]
    //   Import = Privacy "use" Path "as" (*) Id ";" ["mod"]
    //   Import = Privacy "use" Path "as" (*) Id ";" ["pub"]
    //   Import = Privacy "use" Path "as" (*) Id ";" ["self"]
    //   Import = Privacy "use" Path "as" (*) Id ";" ["struct"]
    //   Import = Privacy "use" Path "as" (*) Id ";" ["use"]
    //   Import = Privacy "use" Path "as" (*) Id ";" ["{"]
    //   Import = Privacy "use" Path "as" (*) Id ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S82)
    //
    //   Id -> S81
    pub fn __state58<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<PathId>,
        __sym3: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state82(krate, input, __lookbehind, __tokens, __sym4));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym3.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Id(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state81(krate, input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3, __sym4));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 59
    //   Path = "::" Id (*) ["::"]
    //   Path = "::" Id (*) [";"]
    //   Path = "::" Id (*) ["as"]
    //
    //   "::" -> Reduce(Path = "::", Id => ActionFn(17);)
    //   ";" -> Reduce(Path = "::", Id => ActionFn(17);)
    //   "as" -> Reduce(Path = "::", Id => ActionFn(17);)
    //
    pub fn __state59<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<InternedString>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action17(krate, input, __sym0, __sym1, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Path(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 60
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["::"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# [";"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["as"]
    //   Path = "self" "::" (*) Id ["::"]
    //   Path = "self" "::" (*) Id [";"]
    //   Path = "self" "::" (*) Id ["as"]
    //
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S45)
    //
    //   Id -> S83
    pub fn __state60<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state45(krate, input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Id(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state83(krate, input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 61
    //   (<Path> ";")+ = (<Path> ";")+ Path ";" (*) ["::"]
    //   (<Path> ";")+ = (<Path> ";")+ Path ";" (*) ["self"]
    //   (<Path> ";")+ = (<Path> ";")+ Path ";" (*) ["}"]
    //   (<Path> ";")+ = (<Path> ";")+ Path ";" (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "::" -> Reduce((<Path> ";")+ = (<Path> ";")+, Path, ";" => ActionFn(41);)
    //   "self" -> Reduce((<Path> ";")+ = (<Path> ";")+, Path, ";" => ActionFn(41);)
    //   "}" -> Reduce((<Path> ";")+ = (<Path> ";")+, Path, ";" => ActionFn(41);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce((<Path> ";")+ = (<Path> ";")+, Path, ";" => ActionFn(41);)
    //
    pub fn __state61<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<::std::vec::Vec<PathId>>,
        __sym1: &mut Option<PathId>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action41(krate, input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::_28_3cPath_3e_20_22_3b_22_29_2b(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 62
    //   Path = Path "::" Id (*) ["::"]
    //   Path = Path "::" Id (*) [";"]
    //
    //   "::" -> Reduce(Path = Path, "::", Id => ActionFn(19);)
    //   ";" -> Reduce(Path = Path, "::", Id => ActionFn(19);)
    //
    pub fn __state62<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<PathId>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<InternedString>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action19(krate, input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Path(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 63
    //   Path = "self" "::" Id (*) ["::"]
    //   Path = "self" "::" Id (*) [";"]
    //
    //   "::" -> Reduce(Path = "self", "::", Id => ActionFn(16);)
    //   ";" -> Reduce(Path = "self", "::", Id => ActionFn(16);)
    //
    pub fn __state63<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<InternedString>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action16(krate, input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Path(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 64
    //   Code = (*) "{" (<Path> ";")+ "}" ["::"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["macro_rules"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["mod"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["pub"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["self"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["struct"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["use"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["{"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["}"]
    //   Code = (*) "{" (<Path> ";")+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Code = (*) "{" "}" ["::"]
    //   Code = (*) "{" "}" ["macro_rules"]
    //   Code = (*) "{" "}" ["mod"]
    //   Code = (*) "{" "}" ["pub"]
    //   Code = (*) "{" "}" ["self"]
    //   Code = (*) "{" "}" ["struct"]
    //   Code = (*) "{" "}" ["use"]
    //   Code = (*) "{" "}" ["{"]
    //   Code = (*) "{" "}" ["}"]
    //   Code = (*) "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["::"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["macro_rules"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["mod"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["pub"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["self"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["struct"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["use"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["{"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["}"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["!"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["::"]
    //   Import = (*) Privacy "use" Path ";" ["::"]
    //   Import = (*) Privacy "use" Path ";" ["macro_rules"]
    //   Import = (*) Privacy "use" Path ";" ["mod"]
    //   Import = (*) Privacy "use" Path ";" ["pub"]
    //   Import = (*) Privacy "use" Path ";" ["self"]
    //   Import = (*) Privacy "use" Path ";" ["struct"]
    //   Import = (*) Privacy "use" Path ";" ["use"]
    //   Import = (*) Privacy "use" Path ";" ["{"]
    //   Import = (*) Privacy "use" Path ";" ["}"]
    //   Import = (*) Privacy "use" Path ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["::"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["macro_rules"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["mod"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["pub"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["self"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["struct"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["use"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["{"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["}"]
    //   Import = (*) Privacy "use" Path "as" Id ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Code ["::"]
    //   Item = (*) Code ["macro_rules"]
    //   Item = (*) Code ["mod"]
    //   Item = (*) Code ["pub"]
    //   Item = (*) Code ["self"]
    //   Item = (*) Code ["struct"]
    //   Item = (*) Code ["use"]
    //   Item = (*) Code ["{"]
    //   Item = (*) Code ["}"]
    //   Item = (*) Code [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Glob ["::"]
    //   Item = (*) Glob ["macro_rules"]
    //   Item = (*) Glob ["mod"]
    //   Item = (*) Glob ["pub"]
    //   Item = (*) Glob ["self"]
    //   Item = (*) Glob ["struct"]
    //   Item = (*) Glob ["use"]
    //   Item = (*) Glob ["{"]
    //   Item = (*) Glob ["}"]
    //   Item = (*) Glob [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Import ["::"]
    //   Item = (*) Import ["macro_rules"]
    //   Item = (*) Import ["mod"]
    //   Item = (*) Import ["pub"]
    //   Item = (*) Import ["self"]
    //   Item = (*) Import ["struct"]
    //   Item = (*) Import ["use"]
    //   Item = (*) Import ["{"]
    //   Item = (*) Import ["}"]
    //   Item = (*) Import [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) MacroDef ["::"]
    //   Item = (*) MacroDef ["macro_rules"]
    //   Item = (*) MacroDef ["mod"]
    //   Item = (*) MacroDef ["pub"]
    //   Item = (*) MacroDef ["self"]
    //   Item = (*) MacroDef ["struct"]
    //   Item = (*) MacroDef ["use"]
    //   Item = (*) MacroDef ["{"]
    //   Item = (*) MacroDef ["}"]
    //   Item = (*) MacroDef [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) MacroRef ["::"]
    //   Item = (*) MacroRef ["macro_rules"]
    //   Item = (*) MacroRef ["mod"]
    //   Item = (*) MacroRef ["pub"]
    //   Item = (*) MacroRef ["self"]
    //   Item = (*) MacroRef ["struct"]
    //   Item = (*) MacroRef ["use"]
    //   Item = (*) MacroRef ["{"]
    //   Item = (*) MacroRef ["}"]
    //   Item = (*) MacroRef [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Module ["::"]
    //   Item = (*) Module ["macro_rules"]
    //   Item = (*) Module ["mod"]
    //   Item = (*) Module ["pub"]
    //   Item = (*) Module ["self"]
    //   Item = (*) Module ["struct"]
    //   Item = (*) Module ["use"]
    //   Item = (*) Module ["{"]
    //   Item = (*) Module ["}"]
    //   Item = (*) Module [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Structure ["::"]
    //   Item = (*) Structure ["macro_rules"]
    //   Item = (*) Structure ["mod"]
    //   Item = (*) Structure ["pub"]
    //   Item = (*) Structure ["self"]
    //   Item = (*) Structure ["struct"]
    //   Item = (*) Structure ["use"]
    //   Item = (*) Structure ["{"]
    //   Item = (*) Structure ["}"]
    //   Item = (*) Structure [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item+ = (*) Item ["::"]
    //   Item+ = (*) Item ["macro_rules"]
    //   Item+ = (*) Item ["mod"]
    //   Item+ = (*) Item ["pub"]
    //   Item+ = (*) Item ["self"]
    //   Item+ = (*) Item ["struct"]
    //   Item+ = (*) Item ["use"]
    //   Item+ = (*) Item ["{"]
    //   Item+ = (*) Item ["}"]
    //   Item+ = (*) Item [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item+ = (*) Item+ Item ["::"]
    //   Item+ = (*) Item+ Item ["macro_rules"]
    //   Item+ = (*) Item+ Item ["mod"]
    //   Item+ = (*) Item+ Item ["pub"]
    //   Item+ = (*) Item+ Item ["self"]
    //   Item+ = (*) Item+ Item ["struct"]
    //   Item+ = (*) Item+ Item ["use"]
    //   Item+ = (*) Item+ Item ["{"]
    //   Item+ = (*) Item+ Item ["}"]
    //   Item+ = (*) Item+ Item [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["::"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["macro_rules"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["mod"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["pub"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["self"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["struct"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["use"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["{"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["}"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) Item+ "}" [EOF]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) Item+ "}" ["::"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) Item+ "}" ["macro_rules"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) Item+ "}" ["mod"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) Item+ "}" ["pub"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) Item+ "}" ["self"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) Item+ "}" ["struct"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) Item+ "}" ["use"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) Item+ "}" ["{"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["::"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["macro_rules"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["mod"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["pub"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["self"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["struct"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["use"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["{"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["}"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) "}" [EOF]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) "}" ["::"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) "}" ["macro_rules"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) "}" ["mod"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) "}" ["pub"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) "}" ["self"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) "}" ["struct"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) "}" ["use"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) "}" ["{"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroRef = (*) Path "!" ";" ["::"]
    //   MacroRef = (*) Path "!" ";" ["macro_rules"]
    //   MacroRef = (*) Path "!" ";" ["mod"]
    //   MacroRef = (*) Path "!" ";" ["pub"]
    //   MacroRef = (*) Path "!" ";" ["self"]
    //   MacroRef = (*) Path "!" ";" ["struct"]
    //   MacroRef = (*) Path "!" ";" ["use"]
    //   MacroRef = (*) Path "!" ";" ["{"]
    //   MacroRef = (*) Path "!" ";" ["}"]
    //   MacroRef = (*) Path "!" ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["::"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["macro_rules"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["mod"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["pub"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["self"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["struct"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["use"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["{"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["}"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = (*) Privacy "mod" Id "{" "}" ["::"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["macro_rules"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["mod"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["pub"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["self"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["struct"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["use"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["{"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["}"]
    //   Module = (*) Privacy "mod" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Path = (*) Id ["!"]
    //   Path = (*) Id ["::"]
    //   Path = (*) Path "::" Id ["!"]
    //   Path = (*) Path "::" Id ["::"]
    //   Path = (*) "::" Id ["!"]
    //   Path = (*) "::" Id ["::"]
    //   Path = (*) "self" "::" Id ["!"]
    //   Path = (*) "self" "::" Id ["::"]
    //   Privacy = (*) ["macro_rules"]
    //   Privacy = (*) ["mod"]
    //   Privacy = (*) ["struct"]
    //   Privacy = (*) ["use"]
    //   Privacy = (*) "pub" ["macro_rules"]
    //   Privacy = (*) "pub" ["mod"]
    //   Privacy = (*) "pub" ["struct"]
    //   Privacy = (*) "pub" ["use"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["::"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["macro_rules"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["mod"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["pub"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["self"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["struct"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["use"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["{"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["}"]
    //   Structure = (*) Privacy "struct" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "::" -> Shift(S14)
    //   "macro_rules" -> Reduce(Privacy =  => ActionFn(39);)
    //   "mod" -> Reduce(Privacy =  => ActionFn(39);)
    //   "pub" -> Shift(S15)
    //   "self" -> Shift(S16)
    //   "struct" -> Reduce(Privacy =  => ActionFn(39);)
    //   "use" -> Reduce(Privacy =  => ActionFn(39);)
    //   "{" -> Shift(S76)
    //   "}" -> Shift(S85)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S18)
    //
    //   Code -> S65
    //   Glob -> S66
    //   Id -> S3
    //   Import -> S67
    //   Item -> S68
    //   Item+ -> S84
    //   MacroDef -> S70
    //   MacroRef -> S71
    //   Module -> S72
    //   Path -> S73
    //   Privacy -> S74
    //   Structure -> S75
    pub fn __state64<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<&'input str>,
        __sym3: &mut Option<InternedString>,
        __sym4: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym5 = &mut Some((__tok0));
                __result = try!(__state14(krate, input, __lookbehind, __tokens, __sym5));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym5 = &mut Some((__tok0));
                __result = try!(__state15(krate, input, __lookbehind, __tokens, __sym5));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym5 = &mut Some((__tok0));
                __result = try!(__state16(krate, input, __lookbehind, __tokens, __sym5));
            }
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym5 = &mut Some((__tok0));
                __result = try!(__state76(krate, input, __lookbehind, __tokens, __sym5));
            }
            Some((_, (12, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym5 = &mut Some((__tok0));
                __result = try!(__state85(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym5 = &mut Some((__tok0));
                __result = try!(__state18(krate, input, __lookbehind, __tokens, __sym5));
            }
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) => {
                let __nt = super::__action39(krate, input, &__lookbehind, &__lookahead);
                __result = (__lookbehind, __lookahead, __Nonterminal::Privacy(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym4.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Code(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state65(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Glob(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state66(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Id(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state3(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Import(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state67(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Item(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state68(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Item_2b(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state84(krate, input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5));
                }
                __Nonterminal::MacroDef(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state70(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::MacroRef(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state71(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Module(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state72(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Path(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state73(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Privacy(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state74(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Structure(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state75(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 65
    //   Item = Code (*) ["::"]
    //   Item = Code (*) ["macro_rules"]
    //   Item = Code (*) ["mod"]
    //   Item = Code (*) ["pub"]
    //   Item = Code (*) ["self"]
    //   Item = Code (*) ["struct"]
    //   Item = Code (*) ["use"]
    //   Item = Code (*) ["{"]
    //   Item = Code (*) ["}"]
    //   Item = Code (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "::" -> Reduce(Item = Code => ActionFn(8);)
    //   "macro_rules" -> Reduce(Item = Code => ActionFn(8);)
    //   "mod" -> Reduce(Item = Code => ActionFn(8);)
    //   "pub" -> Reduce(Item = Code => ActionFn(8);)
    //   "self" -> Reduce(Item = Code => ActionFn(8);)
    //   "struct" -> Reduce(Item = Code => ActionFn(8);)
    //   "use" -> Reduce(Item = Code => ActionFn(8);)
    //   "{" -> Reduce(Item = Code => ActionFn(8);)
    //   "}" -> Reduce(Item = Code => ActionFn(8);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Item = Code => ActionFn(8);)
    //
    pub fn __state65<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<CodeId>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action8(krate, input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Item(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 66
    //   Item = Glob (*) ["::"]
    //   Item = Glob (*) ["macro_rules"]
    //   Item = Glob (*) ["mod"]
    //   Item = Glob (*) ["pub"]
    //   Item = Glob (*) ["self"]
    //   Item = Glob (*) ["struct"]
    //   Item = Glob (*) ["use"]
    //   Item = Glob (*) ["{"]
    //   Item = Glob (*) ["}"]
    //   Item = Glob (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "::" -> Reduce(Item = Glob => ActionFn(5);)
    //   "macro_rules" -> Reduce(Item = Glob => ActionFn(5);)
    //   "mod" -> Reduce(Item = Glob => ActionFn(5);)
    //   "pub" -> Reduce(Item = Glob => ActionFn(5);)
    //   "self" -> Reduce(Item = Glob => ActionFn(5);)
    //   "struct" -> Reduce(Item = Glob => ActionFn(5);)
    //   "use" -> Reduce(Item = Glob => ActionFn(5);)
    //   "{" -> Reduce(Item = Glob => ActionFn(5);)
    //   "}" -> Reduce(Item = Glob => ActionFn(5);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Item = Glob => ActionFn(5);)
    //
    pub fn __state66<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<GlobId>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action5(krate, input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Item(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 67
    //   Item = Import (*) ["::"]
    //   Item = Import (*) ["macro_rules"]
    //   Item = Import (*) ["mod"]
    //   Item = Import (*) ["pub"]
    //   Item = Import (*) ["self"]
    //   Item = Import (*) ["struct"]
    //   Item = Import (*) ["use"]
    //   Item = Import (*) ["{"]
    //   Item = Import (*) ["}"]
    //   Item = Import (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "::" -> Reduce(Item = Import => ActionFn(4);)
    //   "macro_rules" -> Reduce(Item = Import => ActionFn(4);)
    //   "mod" -> Reduce(Item = Import => ActionFn(4);)
    //   "pub" -> Reduce(Item = Import => ActionFn(4);)
    //   "self" -> Reduce(Item = Import => ActionFn(4);)
    //   "struct" -> Reduce(Item = Import => ActionFn(4);)
    //   "use" -> Reduce(Item = Import => ActionFn(4);)
    //   "{" -> Reduce(Item = Import => ActionFn(4);)
    //   "}" -> Reduce(Item = Import => ActionFn(4);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Item = Import => ActionFn(4);)
    //
    pub fn __state67<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<ImportId>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action4(krate, input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Item(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 68
    //   Item+ = Item (*) ["::"]
    //   Item+ = Item (*) ["macro_rules"]
    //   Item+ = Item (*) ["mod"]
    //   Item+ = Item (*) ["pub"]
    //   Item+ = Item (*) ["self"]
    //   Item+ = Item (*) ["struct"]
    //   Item+ = Item (*) ["use"]
    //   Item+ = Item (*) ["{"]
    //   Item+ = Item (*) ["}"]
    //   Item+ = Item (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "::" -> Reduce(Item+ = Item => ActionFn(32);)
    //   "macro_rules" -> Reduce(Item+ = Item => ActionFn(32);)
    //   "mod" -> Reduce(Item+ = Item => ActionFn(32);)
    //   "pub" -> Reduce(Item+ = Item => ActionFn(32);)
    //   "self" -> Reduce(Item+ = Item => ActionFn(32);)
    //   "struct" -> Reduce(Item+ = Item => ActionFn(32);)
    //   "use" -> Reduce(Item+ = Item => ActionFn(32);)
    //   "{" -> Reduce(Item+ = Item => ActionFn(32);)
    //   "}" -> Reduce(Item+ = Item => ActionFn(32);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Item+ = Item => ActionFn(32);)
    //
    pub fn __state68<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<ItemId>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action32(krate, input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Item_2b(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 69
    //   Code = (*) "{" (<Path> ";")+ "}" ["::"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["macro_rules"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["mod"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["pub"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["self"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["struct"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["use"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["{"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["}"]
    //   Code = (*) "{" (<Path> ";")+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Code = (*) "{" "}" ["::"]
    //   Code = (*) "{" "}" ["macro_rules"]
    //   Code = (*) "{" "}" ["mod"]
    //   Code = (*) "{" "}" ["pub"]
    //   Code = (*) "{" "}" ["self"]
    //   Code = (*) "{" "}" ["struct"]
    //   Code = (*) "{" "}" ["use"]
    //   Code = (*) "{" "}" ["{"]
    //   Code = (*) "{" "}" ["}"]
    //   Code = (*) "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["::"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["macro_rules"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["mod"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["pub"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["self"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["struct"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["use"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["{"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["}"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["!"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["::"]
    //   Import = (*) Privacy "use" Path ";" ["::"]
    //   Import = (*) Privacy "use" Path ";" ["macro_rules"]
    //   Import = (*) Privacy "use" Path ";" ["mod"]
    //   Import = (*) Privacy "use" Path ";" ["pub"]
    //   Import = (*) Privacy "use" Path ";" ["self"]
    //   Import = (*) Privacy "use" Path ";" ["struct"]
    //   Import = (*) Privacy "use" Path ";" ["use"]
    //   Import = (*) Privacy "use" Path ";" ["{"]
    //   Import = (*) Privacy "use" Path ";" ["}"]
    //   Import = (*) Privacy "use" Path ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["::"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["macro_rules"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["mod"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["pub"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["self"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["struct"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["use"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["{"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["}"]
    //   Import = (*) Privacy "use" Path "as" Id ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Code ["::"]
    //   Item = (*) Code ["macro_rules"]
    //   Item = (*) Code ["mod"]
    //   Item = (*) Code ["pub"]
    //   Item = (*) Code ["self"]
    //   Item = (*) Code ["struct"]
    //   Item = (*) Code ["use"]
    //   Item = (*) Code ["{"]
    //   Item = (*) Code ["}"]
    //   Item = (*) Code [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Glob ["::"]
    //   Item = (*) Glob ["macro_rules"]
    //   Item = (*) Glob ["mod"]
    //   Item = (*) Glob ["pub"]
    //   Item = (*) Glob ["self"]
    //   Item = (*) Glob ["struct"]
    //   Item = (*) Glob ["use"]
    //   Item = (*) Glob ["{"]
    //   Item = (*) Glob ["}"]
    //   Item = (*) Glob [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Import ["::"]
    //   Item = (*) Import ["macro_rules"]
    //   Item = (*) Import ["mod"]
    //   Item = (*) Import ["pub"]
    //   Item = (*) Import ["self"]
    //   Item = (*) Import ["struct"]
    //   Item = (*) Import ["use"]
    //   Item = (*) Import ["{"]
    //   Item = (*) Import ["}"]
    //   Item = (*) Import [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) MacroDef ["::"]
    //   Item = (*) MacroDef ["macro_rules"]
    //   Item = (*) MacroDef ["mod"]
    //   Item = (*) MacroDef ["pub"]
    //   Item = (*) MacroDef ["self"]
    //   Item = (*) MacroDef ["struct"]
    //   Item = (*) MacroDef ["use"]
    //   Item = (*) MacroDef ["{"]
    //   Item = (*) MacroDef ["}"]
    //   Item = (*) MacroDef [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) MacroRef ["::"]
    //   Item = (*) MacroRef ["macro_rules"]
    //   Item = (*) MacroRef ["mod"]
    //   Item = (*) MacroRef ["pub"]
    //   Item = (*) MacroRef ["self"]
    //   Item = (*) MacroRef ["struct"]
    //   Item = (*) MacroRef ["use"]
    //   Item = (*) MacroRef ["{"]
    //   Item = (*) MacroRef ["}"]
    //   Item = (*) MacroRef [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Module ["::"]
    //   Item = (*) Module ["macro_rules"]
    //   Item = (*) Module ["mod"]
    //   Item = (*) Module ["pub"]
    //   Item = (*) Module ["self"]
    //   Item = (*) Module ["struct"]
    //   Item = (*) Module ["use"]
    //   Item = (*) Module ["{"]
    //   Item = (*) Module ["}"]
    //   Item = (*) Module [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Structure ["::"]
    //   Item = (*) Structure ["macro_rules"]
    //   Item = (*) Structure ["mod"]
    //   Item = (*) Structure ["pub"]
    //   Item = (*) Structure ["self"]
    //   Item = (*) Structure ["struct"]
    //   Item = (*) Structure ["use"]
    //   Item = (*) Structure ["{"]
    //   Item = (*) Structure ["}"]
    //   Item = (*) Structure [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item+ = Item+ (*) Item ["::"]
    //   Item+ = Item+ (*) Item ["macro_rules"]
    //   Item+ = Item+ (*) Item ["mod"]
    //   Item+ = Item+ (*) Item ["pub"]
    //   Item+ = Item+ (*) Item ["self"]
    //   Item+ = Item+ (*) Item ["struct"]
    //   Item+ = Item+ (*) Item ["use"]
    //   Item+ = Item+ (*) Item ["{"]
    //   Item+ = Item+ (*) Item ["}"]
    //   Item+ = Item+ (*) Item [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["::"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["macro_rules"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["mod"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["pub"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["self"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["struct"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["use"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["{"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["}"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["::"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["macro_rules"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["mod"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["pub"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["self"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["struct"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["use"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["{"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["}"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroRef = (*) Path "!" ";" ["::"]
    //   MacroRef = (*) Path "!" ";" ["macro_rules"]
    //   MacroRef = (*) Path "!" ";" ["mod"]
    //   MacroRef = (*) Path "!" ";" ["pub"]
    //   MacroRef = (*) Path "!" ";" ["self"]
    //   MacroRef = (*) Path "!" ";" ["struct"]
    //   MacroRef = (*) Path "!" ";" ["use"]
    //   MacroRef = (*) Path "!" ";" ["{"]
    //   MacroRef = (*) Path "!" ";" ["}"]
    //   MacroRef = (*) Path "!" ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["::"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["macro_rules"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["mod"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["pub"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["self"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["struct"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["use"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["{"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["}"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = Privacy "mod" Id "{" Item+ (*) "}" [EOF]
    //   Module = Privacy "mod" Id "{" Item+ (*) "}" ["::"]
    //   Module = Privacy "mod" Id "{" Item+ (*) "}" ["macro_rules"]
    //   Module = Privacy "mod" Id "{" Item+ (*) "}" ["mod"]
    //   Module = Privacy "mod" Id "{" Item+ (*) "}" ["pub"]
    //   Module = Privacy "mod" Id "{" Item+ (*) "}" ["self"]
    //   Module = Privacy "mod" Id "{" Item+ (*) "}" ["struct"]
    //   Module = Privacy "mod" Id "{" Item+ (*) "}" ["use"]
    //   Module = Privacy "mod" Id "{" Item+ (*) "}" ["{"]
    //   Module = Privacy "mod" Id "{" Item+ (*) "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = (*) Privacy "mod" Id "{" "}" ["::"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["macro_rules"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["mod"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["pub"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["self"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["struct"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["use"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["{"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["}"]
    //   Module = (*) Privacy "mod" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Path = (*) Id ["!"]
    //   Path = (*) Id ["::"]
    //   Path = (*) Path "::" Id ["!"]
    //   Path = (*) Path "::" Id ["::"]
    //   Path = (*) "::" Id ["!"]
    //   Path = (*) "::" Id ["::"]
    //   Path = (*) "self" "::" Id ["!"]
    //   Path = (*) "self" "::" Id ["::"]
    //   Privacy = (*) ["macro_rules"]
    //   Privacy = (*) ["mod"]
    //   Privacy = (*) ["struct"]
    //   Privacy = (*) ["use"]
    //   Privacy = (*) "pub" ["macro_rules"]
    //   Privacy = (*) "pub" ["mod"]
    //   Privacy = (*) "pub" ["struct"]
    //   Privacy = (*) "pub" ["use"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["::"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["macro_rules"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["mod"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["pub"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["self"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["struct"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["use"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["{"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["}"]
    //   Structure = (*) Privacy "struct" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "::" -> Shift(S14)
    //   "macro_rules" -> Reduce(Privacy =  => ActionFn(39);)
    //   "mod" -> Reduce(Privacy =  => ActionFn(39);)
    //   "pub" -> Shift(S15)
    //   "self" -> Shift(S16)
    //   "struct" -> Reduce(Privacy =  => ActionFn(39);)
    //   "use" -> Reduce(Privacy =  => ActionFn(39);)
    //   "{" -> Shift(S76)
    //   "}" -> Shift(S87)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S18)
    //
    //   Code -> S65
    //   Glob -> S66
    //   Id -> S3
    //   Import -> S67
    //   Item -> S86
    //   MacroDef -> S70
    //   MacroRef -> S71
    //   Module -> S72
    //   Path -> S73
    //   Privacy -> S74
    //   Structure -> S75
    pub fn __state69<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<InternedString>,
        __sym3: &mut Option<&'input str>,
        __sym4: &mut Option<::std::vec::Vec<ItemId>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym5 = &mut Some((__tok0));
                __result = try!(__state14(krate, input, __lookbehind, __tokens, __sym5));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym5 = &mut Some((__tok0));
                __result = try!(__state15(krate, input, __lookbehind, __tokens, __sym5));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym5 = &mut Some((__tok0));
                __result = try!(__state16(krate, input, __lookbehind, __tokens, __sym5));
            }
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym5 = &mut Some((__tok0));
                __result = try!(__state76(krate, input, __lookbehind, __tokens, __sym5));
            }
            Some((_, (12, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym5 = &mut Some((__tok0));
                __result = try!(__state87(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym5 = &mut Some((__tok0));
                __result = try!(__state18(krate, input, __lookbehind, __tokens, __sym5));
            }
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) => {
                let __nt = super::__action39(krate, input, &__lookbehind, &__lookahead);
                __result = (__lookbehind, __lookahead, __Nonterminal::Privacy(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym4.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Code(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state65(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Glob(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state66(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Id(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state3(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Import(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state67(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Item(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state86(krate, input, __lookbehind, __tokens, __lookahead, __sym4, __sym5));
                }
                __Nonterminal::MacroDef(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state70(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::MacroRef(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state71(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Module(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state72(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Path(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state73(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Privacy(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state74(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Structure(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state75(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 70
    //   Item = MacroDef (*) ["::"]
    //   Item = MacroDef (*) ["macro_rules"]
    //   Item = MacroDef (*) ["mod"]
    //   Item = MacroDef (*) ["pub"]
    //   Item = MacroDef (*) ["self"]
    //   Item = MacroDef (*) ["struct"]
    //   Item = MacroDef (*) ["use"]
    //   Item = MacroDef (*) ["{"]
    //   Item = MacroDef (*) ["}"]
    //   Item = MacroDef (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "::" -> Reduce(Item = MacroDef => ActionFn(6);)
    //   "macro_rules" -> Reduce(Item = MacroDef => ActionFn(6);)
    //   "mod" -> Reduce(Item = MacroDef => ActionFn(6);)
    //   "pub" -> Reduce(Item = MacroDef => ActionFn(6);)
    //   "self" -> Reduce(Item = MacroDef => ActionFn(6);)
    //   "struct" -> Reduce(Item = MacroDef => ActionFn(6);)
    //   "use" -> Reduce(Item = MacroDef => ActionFn(6);)
    //   "{" -> Reduce(Item = MacroDef => ActionFn(6);)
    //   "}" -> Reduce(Item = MacroDef => ActionFn(6);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Item = MacroDef => ActionFn(6);)
    //
    pub fn __state70<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<MacroDefId>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(krate, input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Item(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 71
    //   Item = MacroRef (*) ["::"]
    //   Item = MacroRef (*) ["macro_rules"]
    //   Item = MacroRef (*) ["mod"]
    //   Item = MacroRef (*) ["pub"]
    //   Item = MacroRef (*) ["self"]
    //   Item = MacroRef (*) ["struct"]
    //   Item = MacroRef (*) ["use"]
    //   Item = MacroRef (*) ["{"]
    //   Item = MacroRef (*) ["}"]
    //   Item = MacroRef (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "::" -> Reduce(Item = MacroRef => ActionFn(7);)
    //   "macro_rules" -> Reduce(Item = MacroRef => ActionFn(7);)
    //   "mod" -> Reduce(Item = MacroRef => ActionFn(7);)
    //   "pub" -> Reduce(Item = MacroRef => ActionFn(7);)
    //   "self" -> Reduce(Item = MacroRef => ActionFn(7);)
    //   "struct" -> Reduce(Item = MacroRef => ActionFn(7);)
    //   "use" -> Reduce(Item = MacroRef => ActionFn(7);)
    //   "{" -> Reduce(Item = MacroRef => ActionFn(7);)
    //   "}" -> Reduce(Item = MacroRef => ActionFn(7);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Item = MacroRef => ActionFn(7);)
    //
    pub fn __state71<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<MacroRefId>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(krate, input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Item(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 72
    //   Item = Module (*) ["::"]
    //   Item = Module (*) ["macro_rules"]
    //   Item = Module (*) ["mod"]
    //   Item = Module (*) ["pub"]
    //   Item = Module (*) ["self"]
    //   Item = Module (*) ["struct"]
    //   Item = Module (*) ["use"]
    //   Item = Module (*) ["{"]
    //   Item = Module (*) ["}"]
    //   Item = Module (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "::" -> Reduce(Item = Module => ActionFn(2);)
    //   "macro_rules" -> Reduce(Item = Module => ActionFn(2);)
    //   "mod" -> Reduce(Item = Module => ActionFn(2);)
    //   "pub" -> Reduce(Item = Module => ActionFn(2);)
    //   "self" -> Reduce(Item = Module => ActionFn(2);)
    //   "struct" -> Reduce(Item = Module => ActionFn(2);)
    //   "use" -> Reduce(Item = Module => ActionFn(2);)
    //   "{" -> Reduce(Item = Module => ActionFn(2);)
    //   "}" -> Reduce(Item = Module => ActionFn(2);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Item = Module => ActionFn(2);)
    //
    pub fn __state72<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<ModuleId>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action2(krate, input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Item(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 73
    //   MacroRef = Path (*) "!" ";" ["::"]
    //   MacroRef = Path (*) "!" ";" ["macro_rules"]
    //   MacroRef = Path (*) "!" ";" ["mod"]
    //   MacroRef = Path (*) "!" ";" ["pub"]
    //   MacroRef = Path (*) "!" ";" ["self"]
    //   MacroRef = Path (*) "!" ";" ["struct"]
    //   MacroRef = Path (*) "!" ";" ["use"]
    //   MacroRef = Path (*) "!" ";" ["{"]
    //   MacroRef = Path (*) "!" ";" ["}"]
    //   MacroRef = Path (*) "!" ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Path = Path (*) "::" Id ["!"]
    //   Path = Path (*) "::" Id ["::"]
    //
    //   "!" -> Shift(S88)
    //   "::" -> Shift(S21)
    //
    pub fn __state73<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<PathId>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state88(krate, input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state21(krate, input, __lookbehind, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 74
    //   Glob = Privacy (*) "use" Path "::" "*" ";" ["::"]
    //   Glob = Privacy (*) "use" Path "::" "*" ";" ["macro_rules"]
    //   Glob = Privacy (*) "use" Path "::" "*" ";" ["mod"]
    //   Glob = Privacy (*) "use" Path "::" "*" ";" ["pub"]
    //   Glob = Privacy (*) "use" Path "::" "*" ";" ["self"]
    //   Glob = Privacy (*) "use" Path "::" "*" ";" ["struct"]
    //   Glob = Privacy (*) "use" Path "::" "*" ";" ["use"]
    //   Glob = Privacy (*) "use" Path "::" "*" ";" ["{"]
    //   Glob = Privacy (*) "use" Path "::" "*" ";" ["}"]
    //   Glob = Privacy (*) "use" Path "::" "*" ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Import = Privacy (*) "use" Path ";" ["::"]
    //   Import = Privacy (*) "use" Path ";" ["macro_rules"]
    //   Import = Privacy (*) "use" Path ";" ["mod"]
    //   Import = Privacy (*) "use" Path ";" ["pub"]
    //   Import = Privacy (*) "use" Path ";" ["self"]
    //   Import = Privacy (*) "use" Path ";" ["struct"]
    //   Import = Privacy (*) "use" Path ";" ["use"]
    //   Import = Privacy (*) "use" Path ";" ["{"]
    //   Import = Privacy (*) "use" Path ";" ["}"]
    //   Import = Privacy (*) "use" Path ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Import = Privacy (*) "use" Path "as" Id ";" ["::"]
    //   Import = Privacy (*) "use" Path "as" Id ";" ["macro_rules"]
    //   Import = Privacy (*) "use" Path "as" Id ";" ["mod"]
    //   Import = Privacy (*) "use" Path "as" Id ";" ["pub"]
    //   Import = Privacy (*) "use" Path "as" Id ";" ["self"]
    //   Import = Privacy (*) "use" Path "as" Id ";" ["struct"]
    //   Import = Privacy (*) "use" Path "as" Id ";" ["use"]
    //   Import = Privacy (*) "use" Path "as" Id ";" ["{"]
    //   Import = Privacy (*) "use" Path "as" Id ";" ["}"]
    //   Import = Privacy (*) "use" Path "as" Id ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" Item+ "}" ["::"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" Item+ "}" ["macro_rules"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" Item+ "}" ["mod"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" Item+ "}" ["pub"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" Item+ "}" ["self"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" Item+ "}" ["struct"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" Item+ "}" ["use"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" Item+ "}" ["{"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" Item+ "}" ["}"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" "}" ["::"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" "}" ["macro_rules"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" "}" ["mod"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" "}" ["pub"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" "}" ["self"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" "}" ["struct"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" "}" ["use"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" "}" ["{"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" "}" ["}"]
    //   MacroDef = Privacy (*) "macro_rules" "!" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = Privacy (*) "mod" Id "{" Item+ "}" ["::"]
    //   Module = Privacy (*) "mod" Id "{" Item+ "}" ["macro_rules"]
    //   Module = Privacy (*) "mod" Id "{" Item+ "}" ["mod"]
    //   Module = Privacy (*) "mod" Id "{" Item+ "}" ["pub"]
    //   Module = Privacy (*) "mod" Id "{" Item+ "}" ["self"]
    //   Module = Privacy (*) "mod" Id "{" Item+ "}" ["struct"]
    //   Module = Privacy (*) "mod" Id "{" Item+ "}" ["use"]
    //   Module = Privacy (*) "mod" Id "{" Item+ "}" ["{"]
    //   Module = Privacy (*) "mod" Id "{" Item+ "}" ["}"]
    //   Module = Privacy (*) "mod" Id "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = Privacy (*) "mod" Id "{" "}" ["::"]
    //   Module = Privacy (*) "mod" Id "{" "}" ["macro_rules"]
    //   Module = Privacy (*) "mod" Id "{" "}" ["mod"]
    //   Module = Privacy (*) "mod" Id "{" "}" ["pub"]
    //   Module = Privacy (*) "mod" Id "{" "}" ["self"]
    //   Module = Privacy (*) "mod" Id "{" "}" ["struct"]
    //   Module = Privacy (*) "mod" Id "{" "}" ["use"]
    //   Module = Privacy (*) "mod" Id "{" "}" ["{"]
    //   Module = Privacy (*) "mod" Id "{" "}" ["}"]
    //   Module = Privacy (*) "mod" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Structure = Privacy (*) "struct" Id "{" "}" ["::"]
    //   Structure = Privacy (*) "struct" Id "{" "}" ["macro_rules"]
    //   Structure = Privacy (*) "struct" Id "{" "}" ["mod"]
    //   Structure = Privacy (*) "struct" Id "{" "}" ["pub"]
    //   Structure = Privacy (*) "struct" Id "{" "}" ["self"]
    //   Structure = Privacy (*) "struct" Id "{" "}" ["struct"]
    //   Structure = Privacy (*) "struct" Id "{" "}" ["use"]
    //   Structure = Privacy (*) "struct" Id "{" "}" ["{"]
    //   Structure = Privacy (*) "struct" Id "{" "}" ["}"]
    //   Structure = Privacy (*) "struct" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "macro_rules" -> Shift(S89)
    //   "mod" -> Shift(S90)
    //   "struct" -> Shift(S91)
    //   "use" -> Shift(S92)
    //
    pub fn __state74<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Privacy>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state89(krate, input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state90(krate, input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (9, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state91(krate, input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (10, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state92(krate, input, __lookbehind, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 75
    //   Item = Structure (*) ["::"]
    //   Item = Structure (*) ["macro_rules"]
    //   Item = Structure (*) ["mod"]
    //   Item = Structure (*) ["pub"]
    //   Item = Structure (*) ["self"]
    //   Item = Structure (*) ["struct"]
    //   Item = Structure (*) ["use"]
    //   Item = Structure (*) ["{"]
    //   Item = Structure (*) ["}"]
    //   Item = Structure (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "::" -> Reduce(Item = Structure => ActionFn(3);)
    //   "macro_rules" -> Reduce(Item = Structure => ActionFn(3);)
    //   "mod" -> Reduce(Item = Structure => ActionFn(3);)
    //   "pub" -> Reduce(Item = Structure => ActionFn(3);)
    //   "self" -> Reduce(Item = Structure => ActionFn(3);)
    //   "struct" -> Reduce(Item = Structure => ActionFn(3);)
    //   "use" -> Reduce(Item = Structure => ActionFn(3);)
    //   "{" -> Reduce(Item = Structure => ActionFn(3);)
    //   "}" -> Reduce(Item = Structure => ActionFn(3);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Item = Structure => ActionFn(3);)
    //
    pub fn __state75<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<StructureId>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(krate, input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Item(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 76
    //   (<Path> ";")+ = (*) (<Path> ";")+ Path ";" ["::"]
    //   (<Path> ";")+ = (*) (<Path> ";")+ Path ";" ["self"]
    //   (<Path> ";")+ = (*) (<Path> ";")+ Path ";" ["}"]
    //   (<Path> ";")+ = (*) (<Path> ";")+ Path ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   (<Path> ";")+ = (*) Path ";" ["::"]
    //   (<Path> ";")+ = (*) Path ";" ["self"]
    //   (<Path> ";")+ = (*) Path ";" ["}"]
    //   (<Path> ";")+ = (*) Path ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Code = "{" (*) (<Path> ";")+ "}" ["::"]
    //   Code = "{" (*) (<Path> ";")+ "}" ["macro_rules"]
    //   Code = "{" (*) (<Path> ";")+ "}" ["mod"]
    //   Code = "{" (*) (<Path> ";")+ "}" ["pub"]
    //   Code = "{" (*) (<Path> ";")+ "}" ["self"]
    //   Code = "{" (*) (<Path> ";")+ "}" ["struct"]
    //   Code = "{" (*) (<Path> ";")+ "}" ["use"]
    //   Code = "{" (*) (<Path> ";")+ "}" ["{"]
    //   Code = "{" (*) (<Path> ";")+ "}" ["}"]
    //   Code = "{" (*) (<Path> ";")+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Code = "{" (*) "}" ["::"]
    //   Code = "{" (*) "}" ["macro_rules"]
    //   Code = "{" (*) "}" ["mod"]
    //   Code = "{" (*) "}" ["pub"]
    //   Code = "{" (*) "}" ["self"]
    //   Code = "{" (*) "}" ["struct"]
    //   Code = "{" (*) "}" ["use"]
    //   Code = "{" (*) "}" ["{"]
    //   Code = "{" (*) "}" ["}"]
    //   Code = "{" (*) "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["::"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# [";"]
    //   Path = (*) Id ["::"]
    //   Path = (*) Id [";"]
    //   Path = (*) Path "::" Id ["::"]
    //   Path = (*) Path "::" Id [";"]
    //   Path = (*) "::" Id ["::"]
    //   Path = (*) "::" Id [";"]
    //   Path = (*) "self" "::" Id ["::"]
    //   Path = (*) "self" "::" Id [";"]
    //
    //   "::" -> Shift(S31)
    //   "self" -> Shift(S32)
    //   "}" -> Shift(S94)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S34)
    //
    //   (<Path> ";")+ -> S93
    //   Id -> S29
    //   Path -> S30
    pub fn __state76<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state31(krate, input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state32(krate, input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (12, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state94(krate, input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state34(krate, input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28_3cPath_3e_20_22_3b_22_29_2b(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state93(krate, input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Id(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state29(krate, input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Path(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state30(krate, input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 77
    //   Module = Privacy "mod" Id "{" "}" (*) [EOF]
    //   Module = Privacy "mod" Id "{" "}" (*) ["::"]
    //   Module = Privacy "mod" Id "{" "}" (*) ["macro_rules"]
    //   Module = Privacy "mod" Id "{" "}" (*) ["mod"]
    //   Module = Privacy "mod" Id "{" "}" (*) ["pub"]
    //   Module = Privacy "mod" Id "{" "}" (*) ["self"]
    //   Module = Privacy "mod" Id "{" "}" (*) ["struct"]
    //   Module = Privacy "mod" Id "{" "}" (*) ["use"]
    //   Module = Privacy "mod" Id "{" "}" (*) ["{"]
    //   Module = Privacy "mod" Id "{" "}" (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   EOF -> Reduce(Module = Privacy, "mod", Id, "{", "}" => ActionFn(46);)
    //   "::" -> Reduce(Module = Privacy, "mod", Id, "{", "}" => ActionFn(46);)
    //   "macro_rules" -> Reduce(Module = Privacy, "mod", Id, "{", "}" => ActionFn(46);)
    //   "mod" -> Reduce(Module = Privacy, "mod", Id, "{", "}" => ActionFn(46);)
    //   "pub" -> Reduce(Module = Privacy, "mod", Id, "{", "}" => ActionFn(46);)
    //   "self" -> Reduce(Module = Privacy, "mod", Id, "{", "}" => ActionFn(46);)
    //   "struct" -> Reduce(Module = Privacy, "mod", Id, "{", "}" => ActionFn(46);)
    //   "use" -> Reduce(Module = Privacy, "mod", Id, "{", "}" => ActionFn(46);)
    //   "{" -> Reduce(Module = Privacy, "mod", Id, "{", "}" => ActionFn(46);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Module = Privacy, "mod", Id, "{", "}" => ActionFn(46);)
    //
    pub fn __state77<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<InternedString>,
        __sym3: &mut Option<&'input str>,
        __sym4: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __sym4 = __sym4.take().unwrap();
                let __nt = super::__action46(krate, input, __sym0, __sym1, __sym2, __sym3, __sym4, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Module(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 78
    //   Structure = Privacy "struct" Id "{" "}" (*) [EOF]
    //   Structure = Privacy "struct" Id "{" "}" (*) ["::"]
    //   Structure = Privacy "struct" Id "{" "}" (*) ["macro_rules"]
    //   Structure = Privacy "struct" Id "{" "}" (*) ["mod"]
    //   Structure = Privacy "struct" Id "{" "}" (*) ["pub"]
    //   Structure = Privacy "struct" Id "{" "}" (*) ["self"]
    //   Structure = Privacy "struct" Id "{" "}" (*) ["struct"]
    //   Structure = Privacy "struct" Id "{" "}" (*) ["use"]
    //   Structure = Privacy "struct" Id "{" "}" (*) ["{"]
    //   Structure = Privacy "struct" Id "{" "}" (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   EOF -> Reduce(Structure = Privacy, "struct", Id, "{", "}" => ActionFn(10);)
    //   "::" -> Reduce(Structure = Privacy, "struct", Id, "{", "}" => ActionFn(10);)
    //   "macro_rules" -> Reduce(Structure = Privacy, "struct", Id, "{", "}" => ActionFn(10);)
    //   "mod" -> Reduce(Structure = Privacy, "struct", Id, "{", "}" => ActionFn(10);)
    //   "pub" -> Reduce(Structure = Privacy, "struct", Id, "{", "}" => ActionFn(10);)
    //   "self" -> Reduce(Structure = Privacy, "struct", Id, "{", "}" => ActionFn(10);)
    //   "struct" -> Reduce(Structure = Privacy, "struct", Id, "{", "}" => ActionFn(10);)
    //   "use" -> Reduce(Structure = Privacy, "struct", Id, "{", "}" => ActionFn(10);)
    //   "{" -> Reduce(Structure = Privacy, "struct", Id, "{", "}" => ActionFn(10);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Structure = Privacy, "struct", Id, "{", "}" => ActionFn(10);)
    //
    pub fn __state78<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<InternedString>,
        __sym3: &mut Option<&'input str>,
        __sym4: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __sym4 = __sym4.take().unwrap();
                let __nt = super::__action10(krate, input, __sym0, __sym1, __sym2, __sym3, __sym4, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Structure(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 79
    //   Path = Path "::" Id (*) ["::"]
    //   Path = Path "::" Id (*) [";"]
    //   Path = Path "::" Id (*) ["as"]
    //
    //   "::" -> Reduce(Path = Path, "::", Id => ActionFn(19);)
    //   ";" -> Reduce(Path = Path, "::", Id => ActionFn(19);)
    //   "as" -> Reduce(Path = Path, "::", Id => ActionFn(19);)
    //
    pub fn __state79<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<PathId>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<InternedString>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action19(krate, input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Path(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 80
    //   Glob = Privacy "use" Path "::" "*" (*) ";" [EOF]
    //   Glob = Privacy "use" Path "::" "*" (*) ";" ["::"]
    //   Glob = Privacy "use" Path "::" "*" (*) ";" ["macro_rules"]
    //   Glob = Privacy "use" Path "::" "*" (*) ";" ["mod"]
    //   Glob = Privacy "use" Path "::" "*" (*) ";" ["pub"]
    //   Glob = Privacy "use" Path "::" "*" (*) ";" ["self"]
    //   Glob = Privacy "use" Path "::" "*" (*) ";" ["struct"]
    //   Glob = Privacy "use" Path "::" "*" (*) ";" ["use"]
    //   Glob = Privacy "use" Path "::" "*" (*) ";" ["{"]
    //   Glob = Privacy "use" Path "::" "*" (*) ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   ";" -> Shift(S95)
    //
    pub fn __state80<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<PathId>,
        __sym3: &mut Option<&'input str>,
        __sym4: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym5 = &mut Some((__tok0));
                __result = try!(__state95(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 81
    //   Import = Privacy "use" Path "as" Id (*) ";" [EOF]
    //   Import = Privacy "use" Path "as" Id (*) ";" ["::"]
    //   Import = Privacy "use" Path "as" Id (*) ";" ["macro_rules"]
    //   Import = Privacy "use" Path "as" Id (*) ";" ["mod"]
    //   Import = Privacy "use" Path "as" Id (*) ";" ["pub"]
    //   Import = Privacy "use" Path "as" Id (*) ";" ["self"]
    //   Import = Privacy "use" Path "as" Id (*) ";" ["struct"]
    //   Import = Privacy "use" Path "as" Id (*) ";" ["use"]
    //   Import = Privacy "use" Path "as" Id (*) ";" ["{"]
    //   Import = Privacy "use" Path "as" Id (*) ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   ";" -> Shift(S96)
    //
    pub fn __state81<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<PathId>,
        __sym3: &mut Option<&'input str>,
        __sym4: &mut Option<InternedString>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym5 = &mut Some((__tok0));
                __result = try!(__state96(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 82
    //   Id = r#"[a-zA-Z_][a-zA-Z0-9_]*"# (*) [";"]
    //
    //   ";" -> Reduce(Id = r#"[a-zA-Z_][a-zA-Z0-9_]*"# => ActionFn(22);)
    //
    pub fn __state82<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (3, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action22(krate, input, __sym0, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Id(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 83
    //   Path = "self" "::" Id (*) ["::"]
    //   Path = "self" "::" Id (*) [";"]
    //   Path = "self" "::" Id (*) ["as"]
    //
    //   "::" -> Reduce(Path = "self", "::", Id => ActionFn(16);)
    //   ";" -> Reduce(Path = "self", "::", Id => ActionFn(16);)
    //   "as" -> Reduce(Path = "self", "::", Id => ActionFn(16);)
    //
    pub fn __state83<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<InternedString>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action16(krate, input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Path(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 84
    //   Code = (*) "{" (<Path> ";")+ "}" ["::"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["macro_rules"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["mod"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["pub"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["self"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["struct"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["use"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["{"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["}"]
    //   Code = (*) "{" (<Path> ";")+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Code = (*) "{" "}" ["::"]
    //   Code = (*) "{" "}" ["macro_rules"]
    //   Code = (*) "{" "}" ["mod"]
    //   Code = (*) "{" "}" ["pub"]
    //   Code = (*) "{" "}" ["self"]
    //   Code = (*) "{" "}" ["struct"]
    //   Code = (*) "{" "}" ["use"]
    //   Code = (*) "{" "}" ["{"]
    //   Code = (*) "{" "}" ["}"]
    //   Code = (*) "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["::"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["macro_rules"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["mod"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["pub"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["self"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["struct"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["use"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["{"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["}"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["!"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["::"]
    //   Import = (*) Privacy "use" Path ";" ["::"]
    //   Import = (*) Privacy "use" Path ";" ["macro_rules"]
    //   Import = (*) Privacy "use" Path ";" ["mod"]
    //   Import = (*) Privacy "use" Path ";" ["pub"]
    //   Import = (*) Privacy "use" Path ";" ["self"]
    //   Import = (*) Privacy "use" Path ";" ["struct"]
    //   Import = (*) Privacy "use" Path ";" ["use"]
    //   Import = (*) Privacy "use" Path ";" ["{"]
    //   Import = (*) Privacy "use" Path ";" ["}"]
    //   Import = (*) Privacy "use" Path ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["::"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["macro_rules"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["mod"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["pub"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["self"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["struct"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["use"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["{"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["}"]
    //   Import = (*) Privacy "use" Path "as" Id ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Code ["::"]
    //   Item = (*) Code ["macro_rules"]
    //   Item = (*) Code ["mod"]
    //   Item = (*) Code ["pub"]
    //   Item = (*) Code ["self"]
    //   Item = (*) Code ["struct"]
    //   Item = (*) Code ["use"]
    //   Item = (*) Code ["{"]
    //   Item = (*) Code ["}"]
    //   Item = (*) Code [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Glob ["::"]
    //   Item = (*) Glob ["macro_rules"]
    //   Item = (*) Glob ["mod"]
    //   Item = (*) Glob ["pub"]
    //   Item = (*) Glob ["self"]
    //   Item = (*) Glob ["struct"]
    //   Item = (*) Glob ["use"]
    //   Item = (*) Glob ["{"]
    //   Item = (*) Glob ["}"]
    //   Item = (*) Glob [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Import ["::"]
    //   Item = (*) Import ["macro_rules"]
    //   Item = (*) Import ["mod"]
    //   Item = (*) Import ["pub"]
    //   Item = (*) Import ["self"]
    //   Item = (*) Import ["struct"]
    //   Item = (*) Import ["use"]
    //   Item = (*) Import ["{"]
    //   Item = (*) Import ["}"]
    //   Item = (*) Import [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) MacroDef ["::"]
    //   Item = (*) MacroDef ["macro_rules"]
    //   Item = (*) MacroDef ["mod"]
    //   Item = (*) MacroDef ["pub"]
    //   Item = (*) MacroDef ["self"]
    //   Item = (*) MacroDef ["struct"]
    //   Item = (*) MacroDef ["use"]
    //   Item = (*) MacroDef ["{"]
    //   Item = (*) MacroDef ["}"]
    //   Item = (*) MacroDef [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) MacroRef ["::"]
    //   Item = (*) MacroRef ["macro_rules"]
    //   Item = (*) MacroRef ["mod"]
    //   Item = (*) MacroRef ["pub"]
    //   Item = (*) MacroRef ["self"]
    //   Item = (*) MacroRef ["struct"]
    //   Item = (*) MacroRef ["use"]
    //   Item = (*) MacroRef ["{"]
    //   Item = (*) MacroRef ["}"]
    //   Item = (*) MacroRef [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Module ["::"]
    //   Item = (*) Module ["macro_rules"]
    //   Item = (*) Module ["mod"]
    //   Item = (*) Module ["pub"]
    //   Item = (*) Module ["self"]
    //   Item = (*) Module ["struct"]
    //   Item = (*) Module ["use"]
    //   Item = (*) Module ["{"]
    //   Item = (*) Module ["}"]
    //   Item = (*) Module [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Structure ["::"]
    //   Item = (*) Structure ["macro_rules"]
    //   Item = (*) Structure ["mod"]
    //   Item = (*) Structure ["pub"]
    //   Item = (*) Structure ["self"]
    //   Item = (*) Structure ["struct"]
    //   Item = (*) Structure ["use"]
    //   Item = (*) Structure ["{"]
    //   Item = (*) Structure ["}"]
    //   Item = (*) Structure [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item+ = Item+ (*) Item ["::"]
    //   Item+ = Item+ (*) Item ["macro_rules"]
    //   Item+ = Item+ (*) Item ["mod"]
    //   Item+ = Item+ (*) Item ["pub"]
    //   Item+ = Item+ (*) Item ["self"]
    //   Item+ = Item+ (*) Item ["struct"]
    //   Item+ = Item+ (*) Item ["use"]
    //   Item+ = Item+ (*) Item ["{"]
    //   Item+ = Item+ (*) Item ["}"]
    //   Item+ = Item+ (*) Item [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["::"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["macro_rules"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["mod"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["pub"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["self"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["struct"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["use"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["{"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["}"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ (*) "}" [EOF]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ (*) "}" ["::"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ (*) "}" ["macro_rules"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ (*) "}" ["mod"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ (*) "}" ["pub"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ (*) "}" ["self"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ (*) "}" ["struct"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ (*) "}" ["use"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ (*) "}" ["{"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ (*) "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["::"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["macro_rules"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["mod"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["pub"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["self"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["struct"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["use"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["{"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["}"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroRef = (*) Path "!" ";" ["::"]
    //   MacroRef = (*) Path "!" ";" ["macro_rules"]
    //   MacroRef = (*) Path "!" ";" ["mod"]
    //   MacroRef = (*) Path "!" ";" ["pub"]
    //   MacroRef = (*) Path "!" ";" ["self"]
    //   MacroRef = (*) Path "!" ";" ["struct"]
    //   MacroRef = (*) Path "!" ";" ["use"]
    //   MacroRef = (*) Path "!" ";" ["{"]
    //   MacroRef = (*) Path "!" ";" ["}"]
    //   MacroRef = (*) Path "!" ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["::"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["macro_rules"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["mod"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["pub"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["self"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["struct"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["use"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["{"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["}"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = (*) Privacy "mod" Id "{" "}" ["::"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["macro_rules"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["mod"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["pub"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["self"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["struct"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["use"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["{"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["}"]
    //   Module = (*) Privacy "mod" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Path = (*) Id ["!"]
    //   Path = (*) Id ["::"]
    //   Path = (*) Path "::" Id ["!"]
    //   Path = (*) Path "::" Id ["::"]
    //   Path = (*) "::" Id ["!"]
    //   Path = (*) "::" Id ["::"]
    //   Path = (*) "self" "::" Id ["!"]
    //   Path = (*) "self" "::" Id ["::"]
    //   Privacy = (*) ["macro_rules"]
    //   Privacy = (*) ["mod"]
    //   Privacy = (*) ["struct"]
    //   Privacy = (*) ["use"]
    //   Privacy = (*) "pub" ["macro_rules"]
    //   Privacy = (*) "pub" ["mod"]
    //   Privacy = (*) "pub" ["struct"]
    //   Privacy = (*) "pub" ["use"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["::"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["macro_rules"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["mod"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["pub"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["self"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["struct"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["use"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["{"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["}"]
    //   Structure = (*) Privacy "struct" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "::" -> Shift(S14)
    //   "macro_rules" -> Reduce(Privacy =  => ActionFn(39);)
    //   "mod" -> Reduce(Privacy =  => ActionFn(39);)
    //   "pub" -> Shift(S15)
    //   "self" -> Shift(S16)
    //   "struct" -> Reduce(Privacy =  => ActionFn(39);)
    //   "use" -> Reduce(Privacy =  => ActionFn(39);)
    //   "{" -> Shift(S76)
    //   "}" -> Shift(S97)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S18)
    //
    //   Code -> S65
    //   Glob -> S66
    //   Id -> S3
    //   Import -> S67
    //   Item -> S86
    //   MacroDef -> S70
    //   MacroRef -> S71
    //   Module -> S72
    //   Path -> S73
    //   Privacy -> S74
    //   Structure -> S75
    pub fn __state84<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<&'input str>,
        __sym3: &mut Option<InternedString>,
        __sym4: &mut Option<&'input str>,
        __sym5: &mut Option<::std::vec::Vec<ItemId>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym6 = &mut Some((__tok0));
                __result = try!(__state14(krate, input, __lookbehind, __tokens, __sym6));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym6 = &mut Some((__tok0));
                __result = try!(__state15(krate, input, __lookbehind, __tokens, __sym6));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym6 = &mut Some((__tok0));
                __result = try!(__state16(krate, input, __lookbehind, __tokens, __sym6));
            }
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym6 = &mut Some((__tok0));
                __result = try!(__state76(krate, input, __lookbehind, __tokens, __sym6));
            }
            Some((_, (12, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym6 = &mut Some((__tok0));
                __result = try!(__state97(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym6 = &mut Some((__tok0));
                __result = try!(__state18(krate, input, __lookbehind, __tokens, __sym6));
            }
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) => {
                let __nt = super::__action39(krate, input, &__lookbehind, &__lookahead);
                __result = (__lookbehind, __lookahead, __Nonterminal::Privacy(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym5.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Code(__nt) => {
                    let __sym6 = &mut Some(__nt);
                    __result = try!(__state65(krate, input, __lookbehind, __tokens, __lookahead, __sym6));
                }
                __Nonterminal::Glob(__nt) => {
                    let __sym6 = &mut Some(__nt);
                    __result = try!(__state66(krate, input, __lookbehind, __tokens, __lookahead, __sym6));
                }
                __Nonterminal::Id(__nt) => {
                    let __sym6 = &mut Some(__nt);
                    __result = try!(__state3(krate, input, __lookbehind, __tokens, __lookahead, __sym6));
                }
                __Nonterminal::Import(__nt) => {
                    let __sym6 = &mut Some(__nt);
                    __result = try!(__state67(krate, input, __lookbehind, __tokens, __lookahead, __sym6));
                }
                __Nonterminal::Item(__nt) => {
                    let __sym6 = &mut Some(__nt);
                    __result = try!(__state86(krate, input, __lookbehind, __tokens, __lookahead, __sym5, __sym6));
                }
                __Nonterminal::MacroDef(__nt) => {
                    let __sym6 = &mut Some(__nt);
                    __result = try!(__state70(krate, input, __lookbehind, __tokens, __lookahead, __sym6));
                }
                __Nonterminal::MacroRef(__nt) => {
                    let __sym6 = &mut Some(__nt);
                    __result = try!(__state71(krate, input, __lookbehind, __tokens, __lookahead, __sym6));
                }
                __Nonterminal::Module(__nt) => {
                    let __sym6 = &mut Some(__nt);
                    __result = try!(__state72(krate, input, __lookbehind, __tokens, __lookahead, __sym6));
                }
                __Nonterminal::Path(__nt) => {
                    let __sym6 = &mut Some(__nt);
                    __result = try!(__state73(krate, input, __lookbehind, __tokens, __lookahead, __sym6));
                }
                __Nonterminal::Privacy(__nt) => {
                    let __sym6 = &mut Some(__nt);
                    __result = try!(__state74(krate, input, __lookbehind, __tokens, __lookahead, __sym6));
                }
                __Nonterminal::Structure(__nt) => {
                    let __sym6 = &mut Some(__nt);
                    __result = try!(__state75(krate, input, __lookbehind, __tokens, __lookahead, __sym6));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 85
    //   MacroDef = Privacy "macro_rules" "!" Id "{" "}" (*) [EOF]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" "}" (*) ["::"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" "}" (*) ["macro_rules"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" "}" (*) ["mod"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" "}" (*) ["pub"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" "}" (*) ["self"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" "}" (*) ["struct"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" "}" (*) ["use"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" "}" (*) ["{"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" "}" (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   EOF -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", "}" => ActionFn(44);)
    //   "::" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", "}" => ActionFn(44);)
    //   "macro_rules" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", "}" => ActionFn(44);)
    //   "mod" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", "}" => ActionFn(44);)
    //   "pub" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", "}" => ActionFn(44);)
    //   "self" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", "}" => ActionFn(44);)
    //   "struct" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", "}" => ActionFn(44);)
    //   "use" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", "}" => ActionFn(44);)
    //   "{" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", "}" => ActionFn(44);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", "}" => ActionFn(44);)
    //
    pub fn __state85<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<&'input str>,
        __sym3: &mut Option<InternedString>,
        __sym4: &mut Option<&'input str>,
        __sym5: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __sym4 = __sym4.take().unwrap();
                let __sym5 = __sym5.take().unwrap();
                let __nt = super::__action44(krate, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::MacroDef(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 86
    //   Item+ = Item+ Item (*) ["::"]
    //   Item+ = Item+ Item (*) ["macro_rules"]
    //   Item+ = Item+ Item (*) ["mod"]
    //   Item+ = Item+ Item (*) ["pub"]
    //   Item+ = Item+ Item (*) ["self"]
    //   Item+ = Item+ Item (*) ["struct"]
    //   Item+ = Item+ Item (*) ["use"]
    //   Item+ = Item+ Item (*) ["{"]
    //   Item+ = Item+ Item (*) ["}"]
    //   Item+ = Item+ Item (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "::" -> Reduce(Item+ = Item+, Item => ActionFn(33);)
    //   "macro_rules" -> Reduce(Item+ = Item+, Item => ActionFn(33);)
    //   "mod" -> Reduce(Item+ = Item+, Item => ActionFn(33);)
    //   "pub" -> Reduce(Item+ = Item+, Item => ActionFn(33);)
    //   "self" -> Reduce(Item+ = Item+, Item => ActionFn(33);)
    //   "struct" -> Reduce(Item+ = Item+, Item => ActionFn(33);)
    //   "use" -> Reduce(Item+ = Item+, Item => ActionFn(33);)
    //   "{" -> Reduce(Item+ = Item+, Item => ActionFn(33);)
    //   "}" -> Reduce(Item+ = Item+, Item => ActionFn(33);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Item+ = Item+, Item => ActionFn(33);)
    //
    pub fn __state86<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<::std::vec::Vec<ItemId>>,
        __sym1: &mut Option<ItemId>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action33(krate, input, __sym0, __sym1, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Item_2b(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 87
    //   Module = Privacy "mod" Id "{" Item+ "}" (*) [EOF]
    //   Module = Privacy "mod" Id "{" Item+ "}" (*) ["::"]
    //   Module = Privacy "mod" Id "{" Item+ "}" (*) ["macro_rules"]
    //   Module = Privacy "mod" Id "{" Item+ "}" (*) ["mod"]
    //   Module = Privacy "mod" Id "{" Item+ "}" (*) ["pub"]
    //   Module = Privacy "mod" Id "{" Item+ "}" (*) ["self"]
    //   Module = Privacy "mod" Id "{" Item+ "}" (*) ["struct"]
    //   Module = Privacy "mod" Id "{" Item+ "}" (*) ["use"]
    //   Module = Privacy "mod" Id "{" Item+ "}" (*) ["{"]
    //   Module = Privacy "mod" Id "{" Item+ "}" (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   EOF -> Reduce(Module = Privacy, "mod", Id, "{", Item+, "}" => ActionFn(47);)
    //   "::" -> Reduce(Module = Privacy, "mod", Id, "{", Item+, "}" => ActionFn(47);)
    //   "macro_rules" -> Reduce(Module = Privacy, "mod", Id, "{", Item+, "}" => ActionFn(47);)
    //   "mod" -> Reduce(Module = Privacy, "mod", Id, "{", Item+, "}" => ActionFn(47);)
    //   "pub" -> Reduce(Module = Privacy, "mod", Id, "{", Item+, "}" => ActionFn(47);)
    //   "self" -> Reduce(Module = Privacy, "mod", Id, "{", Item+, "}" => ActionFn(47);)
    //   "struct" -> Reduce(Module = Privacy, "mod", Id, "{", Item+, "}" => ActionFn(47);)
    //   "use" -> Reduce(Module = Privacy, "mod", Id, "{", Item+, "}" => ActionFn(47);)
    //   "{" -> Reduce(Module = Privacy, "mod", Id, "{", Item+, "}" => ActionFn(47);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Module = Privacy, "mod", Id, "{", Item+, "}" => ActionFn(47);)
    //
    pub fn __state87<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<InternedString>,
        __sym3: &mut Option<&'input str>,
        __sym4: &mut Option<::std::vec::Vec<ItemId>>,
        __sym5: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __sym4 = __sym4.take().unwrap();
                let __sym5 = __sym5.take().unwrap();
                let __nt = super::__action47(krate, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Module(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 88
    //   MacroRef = Path "!" (*) ";" ["::"]
    //   MacroRef = Path "!" (*) ";" ["macro_rules"]
    //   MacroRef = Path "!" (*) ";" ["mod"]
    //   MacroRef = Path "!" (*) ";" ["pub"]
    //   MacroRef = Path "!" (*) ";" ["self"]
    //   MacroRef = Path "!" (*) ";" ["struct"]
    //   MacroRef = Path "!" (*) ";" ["use"]
    //   MacroRef = Path "!" (*) ";" ["{"]
    //   MacroRef = Path "!" (*) ";" ["}"]
    //   MacroRef = Path "!" (*) ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   ";" -> Shift(S98)
    //
    pub fn __state88<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<PathId>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state98(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 89
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" Item+ "}" ["::"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" Item+ "}" ["macro_rules"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" Item+ "}" ["mod"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" Item+ "}" ["pub"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" Item+ "}" ["self"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" Item+ "}" ["struct"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" Item+ "}" ["use"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" Item+ "}" ["{"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" Item+ "}" ["}"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" "}" ["::"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" "}" ["macro_rules"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" "}" ["mod"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" "}" ["pub"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" "}" ["self"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" "}" ["struct"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" "}" ["use"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" "}" ["{"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" "}" ["}"]
    //   MacroDef = Privacy "macro_rules" (*) "!" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "!" -> Shift(S99)
    //
    pub fn __state89<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state99(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 90
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["{"]
    //   Module = Privacy "mod" (*) Id "{" Item+ "}" ["::"]
    //   Module = Privacy "mod" (*) Id "{" Item+ "}" ["macro_rules"]
    //   Module = Privacy "mod" (*) Id "{" Item+ "}" ["mod"]
    //   Module = Privacy "mod" (*) Id "{" Item+ "}" ["pub"]
    //   Module = Privacy "mod" (*) Id "{" Item+ "}" ["self"]
    //   Module = Privacy "mod" (*) Id "{" Item+ "}" ["struct"]
    //   Module = Privacy "mod" (*) Id "{" Item+ "}" ["use"]
    //   Module = Privacy "mod" (*) Id "{" Item+ "}" ["{"]
    //   Module = Privacy "mod" (*) Id "{" Item+ "}" ["}"]
    //   Module = Privacy "mod" (*) Id "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = Privacy "mod" (*) Id "{" "}" ["::"]
    //   Module = Privacy "mod" (*) Id "{" "}" ["macro_rules"]
    //   Module = Privacy "mod" (*) Id "{" "}" ["mod"]
    //   Module = Privacy "mod" (*) Id "{" "}" ["pub"]
    //   Module = Privacy "mod" (*) Id "{" "}" ["self"]
    //   Module = Privacy "mod" (*) Id "{" "}" ["struct"]
    //   Module = Privacy "mod" (*) Id "{" "}" ["use"]
    //   Module = Privacy "mod" (*) Id "{" "}" ["{"]
    //   Module = Privacy "mod" (*) Id "{" "}" ["}"]
    //   Module = Privacy "mod" (*) Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S39)
    //
    //   Id -> S100
    pub fn __state90<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state39(krate, input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Id(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state100(krate, input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 91
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["{"]
    //   Structure = Privacy "struct" (*) Id "{" "}" ["::"]
    //   Structure = Privacy "struct" (*) Id "{" "}" ["macro_rules"]
    //   Structure = Privacy "struct" (*) Id "{" "}" ["mod"]
    //   Structure = Privacy "struct" (*) Id "{" "}" ["pub"]
    //   Structure = Privacy "struct" (*) Id "{" "}" ["self"]
    //   Structure = Privacy "struct" (*) Id "{" "}" ["struct"]
    //   Structure = Privacy "struct" (*) Id "{" "}" ["use"]
    //   Structure = Privacy "struct" (*) Id "{" "}" ["{"]
    //   Structure = Privacy "struct" (*) Id "{" "}" ["}"]
    //   Structure = Privacy "struct" (*) Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S39)
    //
    //   Id -> S101
    pub fn __state91<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state39(krate, input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Id(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state101(krate, input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 92
    //   Glob = Privacy "use" (*) Path "::" "*" ";" ["::"]
    //   Glob = Privacy "use" (*) Path "::" "*" ";" ["macro_rules"]
    //   Glob = Privacy "use" (*) Path "::" "*" ";" ["mod"]
    //   Glob = Privacy "use" (*) Path "::" "*" ";" ["pub"]
    //   Glob = Privacy "use" (*) Path "::" "*" ";" ["self"]
    //   Glob = Privacy "use" (*) Path "::" "*" ";" ["struct"]
    //   Glob = Privacy "use" (*) Path "::" "*" ";" ["use"]
    //   Glob = Privacy "use" (*) Path "::" "*" ";" ["{"]
    //   Glob = Privacy "use" (*) Path "::" "*" ";" ["}"]
    //   Glob = Privacy "use" (*) Path "::" "*" ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["::"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# [";"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["as"]
    //   Import = Privacy "use" (*) Path ";" ["::"]
    //   Import = Privacy "use" (*) Path ";" ["macro_rules"]
    //   Import = Privacy "use" (*) Path ";" ["mod"]
    //   Import = Privacy "use" (*) Path ";" ["pub"]
    //   Import = Privacy "use" (*) Path ";" ["self"]
    //   Import = Privacy "use" (*) Path ";" ["struct"]
    //   Import = Privacy "use" (*) Path ";" ["use"]
    //   Import = Privacy "use" (*) Path ";" ["{"]
    //   Import = Privacy "use" (*) Path ";" ["}"]
    //   Import = Privacy "use" (*) Path ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Import = Privacy "use" (*) Path "as" Id ";" ["::"]
    //   Import = Privacy "use" (*) Path "as" Id ";" ["macro_rules"]
    //   Import = Privacy "use" (*) Path "as" Id ";" ["mod"]
    //   Import = Privacy "use" (*) Path "as" Id ";" ["pub"]
    //   Import = Privacy "use" (*) Path "as" Id ";" ["self"]
    //   Import = Privacy "use" (*) Path "as" Id ";" ["struct"]
    //   Import = Privacy "use" (*) Path "as" Id ";" ["use"]
    //   Import = Privacy "use" (*) Path "as" Id ";" ["{"]
    //   Import = Privacy "use" (*) Path "as" Id ";" ["}"]
    //   Import = Privacy "use" (*) Path "as" Id ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Path = (*) Id ["::"]
    //   Path = (*) Id [";"]
    //   Path = (*) Id ["as"]
    //   Path = (*) Path "::" Id ["::"]
    //   Path = (*) Path "::" Id [";"]
    //   Path = (*) Path "::" Id ["as"]
    //   Path = (*) "::" Id ["::"]
    //   Path = (*) "::" Id [";"]
    //   Path = (*) "::" Id ["as"]
    //   Path = (*) "self" "::" Id ["::"]
    //   Path = (*) "self" "::" Id [";"]
    //   Path = (*) "self" "::" Id ["as"]
    //
    //   "::" -> Shift(S43)
    //   "self" -> Shift(S44)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S45)
    //
    //   Id -> S41
    //   Path -> S102
    pub fn __state92<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state43(krate, input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state44(krate, input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state45(krate, input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Id(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state41(krate, input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Path(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state102(krate, input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 93
    //   (<Path> ";")+ = (<Path> ";")+ (*) Path ";" ["::"]
    //   (<Path> ";")+ = (<Path> ";")+ (*) Path ";" ["self"]
    //   (<Path> ";")+ = (<Path> ";")+ (*) Path ";" ["}"]
    //   (<Path> ";")+ = (<Path> ";")+ (*) Path ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Code = "{" (<Path> ";")+ (*) "}" ["::"]
    //   Code = "{" (<Path> ";")+ (*) "}" ["macro_rules"]
    //   Code = "{" (<Path> ";")+ (*) "}" ["mod"]
    //   Code = "{" (<Path> ";")+ (*) "}" ["pub"]
    //   Code = "{" (<Path> ";")+ (*) "}" ["self"]
    //   Code = "{" (<Path> ";")+ (*) "}" ["struct"]
    //   Code = "{" (<Path> ";")+ (*) "}" ["use"]
    //   Code = "{" (<Path> ";")+ (*) "}" ["{"]
    //   Code = "{" (<Path> ";")+ (*) "}" ["}"]
    //   Code = "{" (<Path> ";")+ (*) "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["::"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# [";"]
    //   Path = (*) Id ["::"]
    //   Path = (*) Id [";"]
    //   Path = (*) Path "::" Id ["::"]
    //   Path = (*) Path "::" Id [";"]
    //   Path = (*) "::" Id ["::"]
    //   Path = (*) "::" Id [";"]
    //   Path = (*) "self" "::" Id ["::"]
    //   Path = (*) "self" "::" Id [";"]
    //
    //   "::" -> Shift(S31)
    //   "self" -> Shift(S32)
    //   "}" -> Shift(S103)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S34)
    //
    //   Id -> S29
    //   Path -> S47
    pub fn __state93<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<::std::vec::Vec<PathId>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state31(krate, input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state32(krate, input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (12, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state103(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state34(krate, input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Id(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state29(krate, input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::Path(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state47(krate, input, __lookbehind, __tokens, __lookahead, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 94
    //   Code = "{" "}" (*) ["::"]
    //   Code = "{" "}" (*) ["macro_rules"]
    //   Code = "{" "}" (*) ["mod"]
    //   Code = "{" "}" (*) ["pub"]
    //   Code = "{" "}" (*) ["self"]
    //   Code = "{" "}" (*) ["struct"]
    //   Code = "{" "}" (*) ["use"]
    //   Code = "{" "}" (*) ["{"]
    //   Code = "{" "}" (*) ["}"]
    //   Code = "{" "}" (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "::" -> Reduce(Code = "{", "}" => ActionFn(42);)
    //   "macro_rules" -> Reduce(Code = "{", "}" => ActionFn(42);)
    //   "mod" -> Reduce(Code = "{", "}" => ActionFn(42);)
    //   "pub" -> Reduce(Code = "{", "}" => ActionFn(42);)
    //   "self" -> Reduce(Code = "{", "}" => ActionFn(42);)
    //   "struct" -> Reduce(Code = "{", "}" => ActionFn(42);)
    //   "use" -> Reduce(Code = "{", "}" => ActionFn(42);)
    //   "{" -> Reduce(Code = "{", "}" => ActionFn(42);)
    //   "}" -> Reduce(Code = "{", "}" => ActionFn(42);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Code = "{", "}" => ActionFn(42);)
    //
    pub fn __state94<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __nt = super::__action42(krate, input, __sym0, __sym1, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Code(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 95
    //   Glob = Privacy "use" Path "::" "*" ";" (*) [EOF]
    //   Glob = Privacy "use" Path "::" "*" ";" (*) ["::"]
    //   Glob = Privacy "use" Path "::" "*" ";" (*) ["macro_rules"]
    //   Glob = Privacy "use" Path "::" "*" ";" (*) ["mod"]
    //   Glob = Privacy "use" Path "::" "*" ";" (*) ["pub"]
    //   Glob = Privacy "use" Path "::" "*" ";" (*) ["self"]
    //   Glob = Privacy "use" Path "::" "*" ";" (*) ["struct"]
    //   Glob = Privacy "use" Path "::" "*" ";" (*) ["use"]
    //   Glob = Privacy "use" Path "::" "*" ";" (*) ["{"]
    //   Glob = Privacy "use" Path "::" "*" ";" (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   EOF -> Reduce(Glob = Privacy, "use", Path, "::", "*", ";" => ActionFn(12);)
    //   "::" -> Reduce(Glob = Privacy, "use", Path, "::", "*", ";" => ActionFn(12);)
    //   "macro_rules" -> Reduce(Glob = Privacy, "use", Path, "::", "*", ";" => ActionFn(12);)
    //   "mod" -> Reduce(Glob = Privacy, "use", Path, "::", "*", ";" => ActionFn(12);)
    //   "pub" -> Reduce(Glob = Privacy, "use", Path, "::", "*", ";" => ActionFn(12);)
    //   "self" -> Reduce(Glob = Privacy, "use", Path, "::", "*", ";" => ActionFn(12);)
    //   "struct" -> Reduce(Glob = Privacy, "use", Path, "::", "*", ";" => ActionFn(12);)
    //   "use" -> Reduce(Glob = Privacy, "use", Path, "::", "*", ";" => ActionFn(12);)
    //   "{" -> Reduce(Glob = Privacy, "use", Path, "::", "*", ";" => ActionFn(12);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Glob = Privacy, "use", Path, "::", "*", ";" => ActionFn(12);)
    //
    pub fn __state95<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<PathId>,
        __sym3: &mut Option<&'input str>,
        __sym4: &mut Option<&'input str>,
        __sym5: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __sym4 = __sym4.take().unwrap();
                let __sym5 = __sym5.take().unwrap();
                let __nt = super::__action12(krate, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Glob(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 96
    //   Import = Privacy "use" Path "as" Id ";" (*) [EOF]
    //   Import = Privacy "use" Path "as" Id ";" (*) ["::"]
    //   Import = Privacy "use" Path "as" Id ";" (*) ["macro_rules"]
    //   Import = Privacy "use" Path "as" Id ";" (*) ["mod"]
    //   Import = Privacy "use" Path "as" Id ";" (*) ["pub"]
    //   Import = Privacy "use" Path "as" Id ";" (*) ["self"]
    //   Import = Privacy "use" Path "as" Id ";" (*) ["struct"]
    //   Import = Privacy "use" Path "as" Id ";" (*) ["use"]
    //   Import = Privacy "use" Path "as" Id ";" (*) ["{"]
    //   Import = Privacy "use" Path "as" Id ";" (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   EOF -> Reduce(Import = Privacy, "use", Path, "as", Id, ";" => ActionFn(37);)
    //   "::" -> Reduce(Import = Privacy, "use", Path, "as", Id, ";" => ActionFn(37);)
    //   "macro_rules" -> Reduce(Import = Privacy, "use", Path, "as", Id, ";" => ActionFn(37);)
    //   "mod" -> Reduce(Import = Privacy, "use", Path, "as", Id, ";" => ActionFn(37);)
    //   "pub" -> Reduce(Import = Privacy, "use", Path, "as", Id, ";" => ActionFn(37);)
    //   "self" -> Reduce(Import = Privacy, "use", Path, "as", Id, ";" => ActionFn(37);)
    //   "struct" -> Reduce(Import = Privacy, "use", Path, "as", Id, ";" => ActionFn(37);)
    //   "use" -> Reduce(Import = Privacy, "use", Path, "as", Id, ";" => ActionFn(37);)
    //   "{" -> Reduce(Import = Privacy, "use", Path, "as", Id, ";" => ActionFn(37);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Import = Privacy, "use", Path, "as", Id, ";" => ActionFn(37);)
    //
    pub fn __state96<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<PathId>,
        __sym3: &mut Option<&'input str>,
        __sym4: &mut Option<InternedString>,
        __sym5: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __sym4 = __sym4.take().unwrap();
                let __sym5 = __sym5.take().unwrap();
                let __nt = super::__action37(krate, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Import(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 97
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ "}" (*) [EOF]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ "}" (*) ["::"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ "}" (*) ["macro_rules"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ "}" (*) ["mod"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ "}" (*) ["pub"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ "}" (*) ["self"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ "}" (*) ["struct"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ "}" (*) ["use"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ "}" (*) ["{"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ "}" (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   EOF -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", Item+, "}" => ActionFn(45);)
    //   "::" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", Item+, "}" => ActionFn(45);)
    //   "macro_rules" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", Item+, "}" => ActionFn(45);)
    //   "mod" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", Item+, "}" => ActionFn(45);)
    //   "pub" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", Item+, "}" => ActionFn(45);)
    //   "self" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", Item+, "}" => ActionFn(45);)
    //   "struct" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", Item+, "}" => ActionFn(45);)
    //   "use" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", Item+, "}" => ActionFn(45);)
    //   "{" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", Item+, "}" => ActionFn(45);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", Item+, "}" => ActionFn(45);)
    //
    pub fn __state97<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<&'input str>,
        __sym3: &mut Option<InternedString>,
        __sym4: &mut Option<&'input str>,
        __sym5: &mut Option<::std::vec::Vec<ItemId>>,
        __sym6: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __sym4 = __sym4.take().unwrap();
                let __sym5 = __sym5.take().unwrap();
                let __sym6 = __sym6.take().unwrap();
                let __nt = super::__action45(krate, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::MacroDef(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 98
    //   MacroRef = Path "!" ";" (*) ["::"]
    //   MacroRef = Path "!" ";" (*) ["macro_rules"]
    //   MacroRef = Path "!" ";" (*) ["mod"]
    //   MacroRef = Path "!" ";" (*) ["pub"]
    //   MacroRef = Path "!" ";" (*) ["self"]
    //   MacroRef = Path "!" ";" (*) ["struct"]
    //   MacroRef = Path "!" ";" (*) ["use"]
    //   MacroRef = Path "!" ";" (*) ["{"]
    //   MacroRef = Path "!" ";" (*) ["}"]
    //   MacroRef = Path "!" ";" (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "::" -> Reduce(MacroRef = Path, "!", ";" => ActionFn(14);)
    //   "macro_rules" -> Reduce(MacroRef = Path, "!", ";" => ActionFn(14);)
    //   "mod" -> Reduce(MacroRef = Path, "!", ";" => ActionFn(14);)
    //   "pub" -> Reduce(MacroRef = Path, "!", ";" => ActionFn(14);)
    //   "self" -> Reduce(MacroRef = Path, "!", ";" => ActionFn(14);)
    //   "struct" -> Reduce(MacroRef = Path, "!", ";" => ActionFn(14);)
    //   "use" -> Reduce(MacroRef = Path, "!", ";" => ActionFn(14);)
    //   "{" -> Reduce(MacroRef = Path, "!", ";" => ActionFn(14);)
    //   "}" -> Reduce(MacroRef = Path, "!", ";" => ActionFn(14);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(MacroRef = Path, "!", ";" => ActionFn(14);)
    //
    pub fn __state98<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<PathId>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action14(krate, input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::MacroRef(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 99
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["{"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" Item+ "}" ["::"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" Item+ "}" ["macro_rules"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" Item+ "}" ["mod"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" Item+ "}" ["pub"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" Item+ "}" ["self"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" Item+ "}" ["struct"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" Item+ "}" ["use"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" Item+ "}" ["{"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" Item+ "}" ["}"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" "}" ["::"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" "}" ["macro_rules"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" "}" ["mod"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" "}" ["pub"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" "}" ["self"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" "}" ["struct"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" "}" ["use"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" "}" ["{"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" "}" ["}"]
    //   MacroDef = Privacy "macro_rules" "!" (*) Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S39)
    //
    //   Id -> S104
    pub fn __state99<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state39(krate, input, __lookbehind, __tokens, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Id(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state104(krate, input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 100
    //   Module = Privacy "mod" Id (*) "{" Item+ "}" ["::"]
    //   Module = Privacy "mod" Id (*) "{" Item+ "}" ["macro_rules"]
    //   Module = Privacy "mod" Id (*) "{" Item+ "}" ["mod"]
    //   Module = Privacy "mod" Id (*) "{" Item+ "}" ["pub"]
    //   Module = Privacy "mod" Id (*) "{" Item+ "}" ["self"]
    //   Module = Privacy "mod" Id (*) "{" Item+ "}" ["struct"]
    //   Module = Privacy "mod" Id (*) "{" Item+ "}" ["use"]
    //   Module = Privacy "mod" Id (*) "{" Item+ "}" ["{"]
    //   Module = Privacy "mod" Id (*) "{" Item+ "}" ["}"]
    //   Module = Privacy "mod" Id (*) "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = Privacy "mod" Id (*) "{" "}" ["::"]
    //   Module = Privacy "mod" Id (*) "{" "}" ["macro_rules"]
    //   Module = Privacy "mod" Id (*) "{" "}" ["mod"]
    //   Module = Privacy "mod" Id (*) "{" "}" ["pub"]
    //   Module = Privacy "mod" Id (*) "{" "}" ["self"]
    //   Module = Privacy "mod" Id (*) "{" "}" ["struct"]
    //   Module = Privacy "mod" Id (*) "{" "}" ["use"]
    //   Module = Privacy "mod" Id (*) "{" "}" ["{"]
    //   Module = Privacy "mod" Id (*) "{" "}" ["}"]
    //   Module = Privacy "mod" Id (*) "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "{" -> Shift(S105)
    //
    pub fn __state100<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<InternedString>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state105(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 101
    //   Structure = Privacy "struct" Id (*) "{" "}" ["::"]
    //   Structure = Privacy "struct" Id (*) "{" "}" ["macro_rules"]
    //   Structure = Privacy "struct" Id (*) "{" "}" ["mod"]
    //   Structure = Privacy "struct" Id (*) "{" "}" ["pub"]
    //   Structure = Privacy "struct" Id (*) "{" "}" ["self"]
    //   Structure = Privacy "struct" Id (*) "{" "}" ["struct"]
    //   Structure = Privacy "struct" Id (*) "{" "}" ["use"]
    //   Structure = Privacy "struct" Id (*) "{" "}" ["{"]
    //   Structure = Privacy "struct" Id (*) "{" "}" ["}"]
    //   Structure = Privacy "struct" Id (*) "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "{" -> Shift(S106)
    //
    pub fn __state101<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<InternedString>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state106(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 102
    //   Glob = Privacy "use" Path (*) "::" "*" ";" ["::"]
    //   Glob = Privacy "use" Path (*) "::" "*" ";" ["macro_rules"]
    //   Glob = Privacy "use" Path (*) "::" "*" ";" ["mod"]
    //   Glob = Privacy "use" Path (*) "::" "*" ";" ["pub"]
    //   Glob = Privacy "use" Path (*) "::" "*" ";" ["self"]
    //   Glob = Privacy "use" Path (*) "::" "*" ";" ["struct"]
    //   Glob = Privacy "use" Path (*) "::" "*" ";" ["use"]
    //   Glob = Privacy "use" Path (*) "::" "*" ";" ["{"]
    //   Glob = Privacy "use" Path (*) "::" "*" ";" ["}"]
    //   Glob = Privacy "use" Path (*) "::" "*" ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Import = Privacy "use" Path (*) ";" ["::"]
    //   Import = Privacy "use" Path (*) ";" ["macro_rules"]
    //   Import = Privacy "use" Path (*) ";" ["mod"]
    //   Import = Privacy "use" Path (*) ";" ["pub"]
    //   Import = Privacy "use" Path (*) ";" ["self"]
    //   Import = Privacy "use" Path (*) ";" ["struct"]
    //   Import = Privacy "use" Path (*) ";" ["use"]
    //   Import = Privacy "use" Path (*) ";" ["{"]
    //   Import = Privacy "use" Path (*) ";" ["}"]
    //   Import = Privacy "use" Path (*) ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Import = Privacy "use" Path (*) "as" Id ";" ["::"]
    //   Import = Privacy "use" Path (*) "as" Id ";" ["macro_rules"]
    //   Import = Privacy "use" Path (*) "as" Id ";" ["mod"]
    //   Import = Privacy "use" Path (*) "as" Id ";" ["pub"]
    //   Import = Privacy "use" Path (*) "as" Id ";" ["self"]
    //   Import = Privacy "use" Path (*) "as" Id ";" ["struct"]
    //   Import = Privacy "use" Path (*) "as" Id ";" ["use"]
    //   Import = Privacy "use" Path (*) "as" Id ";" ["{"]
    //   Import = Privacy "use" Path (*) "as" Id ";" ["}"]
    //   Import = Privacy "use" Path (*) "as" Id ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Path = Path (*) "::" Id ["::"]
    //   Path = Path (*) "::" Id [";"]
    //   Path = Path (*) "::" Id ["as"]
    //
    //   "::" -> Shift(S107)
    //   ";" -> Shift(S108)
    //   "as" -> Shift(S109)
    //
    pub fn __state102<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<PathId>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state107(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state108(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state109(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 103
    //   Code = "{" (<Path> ";")+ "}" (*) ["::"]
    //   Code = "{" (<Path> ";")+ "}" (*) ["macro_rules"]
    //   Code = "{" (<Path> ";")+ "}" (*) ["mod"]
    //   Code = "{" (<Path> ";")+ "}" (*) ["pub"]
    //   Code = "{" (<Path> ";")+ "}" (*) ["self"]
    //   Code = "{" (<Path> ";")+ "}" (*) ["struct"]
    //   Code = "{" (<Path> ";")+ "}" (*) ["use"]
    //   Code = "{" (<Path> ";")+ "}" (*) ["{"]
    //   Code = "{" (<Path> ";")+ "}" (*) ["}"]
    //   Code = "{" (<Path> ";")+ "}" (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "::" -> Reduce(Code = "{", (<Path> ";")+, "}" => ActionFn(43);)
    //   "macro_rules" -> Reduce(Code = "{", (<Path> ";")+, "}" => ActionFn(43);)
    //   "mod" -> Reduce(Code = "{", (<Path> ";")+, "}" => ActionFn(43);)
    //   "pub" -> Reduce(Code = "{", (<Path> ";")+, "}" => ActionFn(43);)
    //   "self" -> Reduce(Code = "{", (<Path> ";")+, "}" => ActionFn(43);)
    //   "struct" -> Reduce(Code = "{", (<Path> ";")+, "}" => ActionFn(43);)
    //   "use" -> Reduce(Code = "{", (<Path> ";")+, "}" => ActionFn(43);)
    //   "{" -> Reduce(Code = "{", (<Path> ";")+, "}" => ActionFn(43);)
    //   "}" -> Reduce(Code = "{", (<Path> ";")+, "}" => ActionFn(43);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Code = "{", (<Path> ";")+, "}" => ActionFn(43);)
    //
    pub fn __state103<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<::std::vec::Vec<PathId>>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action43(krate, input, __sym0, __sym1, __sym2, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Code(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 104
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" Item+ "}" ["::"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" Item+ "}" ["macro_rules"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" Item+ "}" ["mod"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" Item+ "}" ["pub"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" Item+ "}" ["self"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" Item+ "}" ["struct"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" Item+ "}" ["use"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" Item+ "}" ["{"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" Item+ "}" ["}"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" "}" ["::"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" "}" ["macro_rules"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" "}" ["mod"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" "}" ["pub"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" "}" ["self"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" "}" ["struct"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" "}" ["use"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" "}" ["{"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" "}" ["}"]
    //   MacroDef = Privacy "macro_rules" "!" Id (*) "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "{" -> Shift(S110)
    //
    pub fn __state104<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<&'input str>,
        __sym3: &mut Option<InternedString>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state110(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3, __sym4));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 105
    //   Code = (*) "{" (<Path> ";")+ "}" ["::"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["macro_rules"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["mod"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["pub"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["self"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["struct"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["use"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["{"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["}"]
    //   Code = (*) "{" (<Path> ";")+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Code = (*) "{" "}" ["::"]
    //   Code = (*) "{" "}" ["macro_rules"]
    //   Code = (*) "{" "}" ["mod"]
    //   Code = (*) "{" "}" ["pub"]
    //   Code = (*) "{" "}" ["self"]
    //   Code = (*) "{" "}" ["struct"]
    //   Code = (*) "{" "}" ["use"]
    //   Code = (*) "{" "}" ["{"]
    //   Code = (*) "{" "}" ["}"]
    //   Code = (*) "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["::"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["macro_rules"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["mod"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["pub"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["self"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["struct"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["use"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["{"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["}"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["!"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["::"]
    //   Import = (*) Privacy "use" Path ";" ["::"]
    //   Import = (*) Privacy "use" Path ";" ["macro_rules"]
    //   Import = (*) Privacy "use" Path ";" ["mod"]
    //   Import = (*) Privacy "use" Path ";" ["pub"]
    //   Import = (*) Privacy "use" Path ";" ["self"]
    //   Import = (*) Privacy "use" Path ";" ["struct"]
    //   Import = (*) Privacy "use" Path ";" ["use"]
    //   Import = (*) Privacy "use" Path ";" ["{"]
    //   Import = (*) Privacy "use" Path ";" ["}"]
    //   Import = (*) Privacy "use" Path ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["::"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["macro_rules"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["mod"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["pub"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["self"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["struct"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["use"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["{"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["}"]
    //   Import = (*) Privacy "use" Path "as" Id ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Code ["::"]
    //   Item = (*) Code ["macro_rules"]
    //   Item = (*) Code ["mod"]
    //   Item = (*) Code ["pub"]
    //   Item = (*) Code ["self"]
    //   Item = (*) Code ["struct"]
    //   Item = (*) Code ["use"]
    //   Item = (*) Code ["{"]
    //   Item = (*) Code ["}"]
    //   Item = (*) Code [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Glob ["::"]
    //   Item = (*) Glob ["macro_rules"]
    //   Item = (*) Glob ["mod"]
    //   Item = (*) Glob ["pub"]
    //   Item = (*) Glob ["self"]
    //   Item = (*) Glob ["struct"]
    //   Item = (*) Glob ["use"]
    //   Item = (*) Glob ["{"]
    //   Item = (*) Glob ["}"]
    //   Item = (*) Glob [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Import ["::"]
    //   Item = (*) Import ["macro_rules"]
    //   Item = (*) Import ["mod"]
    //   Item = (*) Import ["pub"]
    //   Item = (*) Import ["self"]
    //   Item = (*) Import ["struct"]
    //   Item = (*) Import ["use"]
    //   Item = (*) Import ["{"]
    //   Item = (*) Import ["}"]
    //   Item = (*) Import [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) MacroDef ["::"]
    //   Item = (*) MacroDef ["macro_rules"]
    //   Item = (*) MacroDef ["mod"]
    //   Item = (*) MacroDef ["pub"]
    //   Item = (*) MacroDef ["self"]
    //   Item = (*) MacroDef ["struct"]
    //   Item = (*) MacroDef ["use"]
    //   Item = (*) MacroDef ["{"]
    //   Item = (*) MacroDef ["}"]
    //   Item = (*) MacroDef [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) MacroRef ["::"]
    //   Item = (*) MacroRef ["macro_rules"]
    //   Item = (*) MacroRef ["mod"]
    //   Item = (*) MacroRef ["pub"]
    //   Item = (*) MacroRef ["self"]
    //   Item = (*) MacroRef ["struct"]
    //   Item = (*) MacroRef ["use"]
    //   Item = (*) MacroRef ["{"]
    //   Item = (*) MacroRef ["}"]
    //   Item = (*) MacroRef [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Module ["::"]
    //   Item = (*) Module ["macro_rules"]
    //   Item = (*) Module ["mod"]
    //   Item = (*) Module ["pub"]
    //   Item = (*) Module ["self"]
    //   Item = (*) Module ["struct"]
    //   Item = (*) Module ["use"]
    //   Item = (*) Module ["{"]
    //   Item = (*) Module ["}"]
    //   Item = (*) Module [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Structure ["::"]
    //   Item = (*) Structure ["macro_rules"]
    //   Item = (*) Structure ["mod"]
    //   Item = (*) Structure ["pub"]
    //   Item = (*) Structure ["self"]
    //   Item = (*) Structure ["struct"]
    //   Item = (*) Structure ["use"]
    //   Item = (*) Structure ["{"]
    //   Item = (*) Structure ["}"]
    //   Item = (*) Structure [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item+ = (*) Item ["::"]
    //   Item+ = (*) Item ["macro_rules"]
    //   Item+ = (*) Item ["mod"]
    //   Item+ = (*) Item ["pub"]
    //   Item+ = (*) Item ["self"]
    //   Item+ = (*) Item ["struct"]
    //   Item+ = (*) Item ["use"]
    //   Item+ = (*) Item ["{"]
    //   Item+ = (*) Item ["}"]
    //   Item+ = (*) Item [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item+ = (*) Item+ Item ["::"]
    //   Item+ = (*) Item+ Item ["macro_rules"]
    //   Item+ = (*) Item+ Item ["mod"]
    //   Item+ = (*) Item+ Item ["pub"]
    //   Item+ = (*) Item+ Item ["self"]
    //   Item+ = (*) Item+ Item ["struct"]
    //   Item+ = (*) Item+ Item ["use"]
    //   Item+ = (*) Item+ Item ["{"]
    //   Item+ = (*) Item+ Item ["}"]
    //   Item+ = (*) Item+ Item [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["::"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["macro_rules"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["mod"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["pub"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["self"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["struct"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["use"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["{"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["}"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["::"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["macro_rules"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["mod"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["pub"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["self"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["struct"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["use"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["{"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["}"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroRef = (*) Path "!" ";" ["::"]
    //   MacroRef = (*) Path "!" ";" ["macro_rules"]
    //   MacroRef = (*) Path "!" ";" ["mod"]
    //   MacroRef = (*) Path "!" ";" ["pub"]
    //   MacroRef = (*) Path "!" ";" ["self"]
    //   MacroRef = (*) Path "!" ";" ["struct"]
    //   MacroRef = (*) Path "!" ";" ["use"]
    //   MacroRef = (*) Path "!" ";" ["{"]
    //   MacroRef = (*) Path "!" ";" ["}"]
    //   MacroRef = (*) Path "!" ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["::"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["macro_rules"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["mod"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["pub"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["self"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["struct"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["use"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["{"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["}"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = Privacy "mod" Id "{" (*) Item+ "}" ["::"]
    //   Module = Privacy "mod" Id "{" (*) Item+ "}" ["macro_rules"]
    //   Module = Privacy "mod" Id "{" (*) Item+ "}" ["mod"]
    //   Module = Privacy "mod" Id "{" (*) Item+ "}" ["pub"]
    //   Module = Privacy "mod" Id "{" (*) Item+ "}" ["self"]
    //   Module = Privacy "mod" Id "{" (*) Item+ "}" ["struct"]
    //   Module = Privacy "mod" Id "{" (*) Item+ "}" ["use"]
    //   Module = Privacy "mod" Id "{" (*) Item+ "}" ["{"]
    //   Module = Privacy "mod" Id "{" (*) Item+ "}" ["}"]
    //   Module = Privacy "mod" Id "{" (*) Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = (*) Privacy "mod" Id "{" "}" ["::"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["macro_rules"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["mod"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["pub"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["self"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["struct"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["use"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["{"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["}"]
    //   Module = (*) Privacy "mod" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = Privacy "mod" Id "{" (*) "}" ["::"]
    //   Module = Privacy "mod" Id "{" (*) "}" ["macro_rules"]
    //   Module = Privacy "mod" Id "{" (*) "}" ["mod"]
    //   Module = Privacy "mod" Id "{" (*) "}" ["pub"]
    //   Module = Privacy "mod" Id "{" (*) "}" ["self"]
    //   Module = Privacy "mod" Id "{" (*) "}" ["struct"]
    //   Module = Privacy "mod" Id "{" (*) "}" ["use"]
    //   Module = Privacy "mod" Id "{" (*) "}" ["{"]
    //   Module = Privacy "mod" Id "{" (*) "}" ["}"]
    //   Module = Privacy "mod" Id "{" (*) "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Path = (*) Id ["!"]
    //   Path = (*) Id ["::"]
    //   Path = (*) Path "::" Id ["!"]
    //   Path = (*) Path "::" Id ["::"]
    //   Path = (*) "::" Id ["!"]
    //   Path = (*) "::" Id ["::"]
    //   Path = (*) "self" "::" Id ["!"]
    //   Path = (*) "self" "::" Id ["::"]
    //   Privacy = (*) ["macro_rules"]
    //   Privacy = (*) ["mod"]
    //   Privacy = (*) ["struct"]
    //   Privacy = (*) ["use"]
    //   Privacy = (*) "pub" ["macro_rules"]
    //   Privacy = (*) "pub" ["mod"]
    //   Privacy = (*) "pub" ["struct"]
    //   Privacy = (*) "pub" ["use"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["::"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["macro_rules"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["mod"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["pub"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["self"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["struct"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["use"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["{"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["}"]
    //   Structure = (*) Privacy "struct" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "::" -> Shift(S14)
    //   "macro_rules" -> Reduce(Privacy =  => ActionFn(39);)
    //   "mod" -> Reduce(Privacy =  => ActionFn(39);)
    //   "pub" -> Shift(S15)
    //   "self" -> Shift(S16)
    //   "struct" -> Reduce(Privacy =  => ActionFn(39);)
    //   "use" -> Reduce(Privacy =  => ActionFn(39);)
    //   "{" -> Shift(S76)
    //   "}" -> Shift(S112)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S18)
    //
    //   Code -> S65
    //   Glob -> S66
    //   Id -> S3
    //   Import -> S67
    //   Item -> S68
    //   Item+ -> S111
    //   MacroDef -> S70
    //   MacroRef -> S71
    //   Module -> S72
    //   Path -> S73
    //   Privacy -> S74
    //   Structure -> S75
    pub fn __state105<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<InternedString>,
        __sym3: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state14(krate, input, __lookbehind, __tokens, __sym4));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state15(krate, input, __lookbehind, __tokens, __sym4));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state16(krate, input, __lookbehind, __tokens, __sym4));
            }
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state76(krate, input, __lookbehind, __tokens, __sym4));
            }
            Some((_, (12, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state112(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3, __sym4));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state18(krate, input, __lookbehind, __tokens, __sym4));
            }
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) => {
                let __nt = super::__action39(krate, input, &__lookbehind, &__lookahead);
                __result = (__lookbehind, __lookahead, __Nonterminal::Privacy(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym3.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Code(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state65(krate, input, __lookbehind, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::Glob(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state66(krate, input, __lookbehind, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::Id(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state3(krate, input, __lookbehind, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::Import(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state67(krate, input, __lookbehind, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::Item(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state68(krate, input, __lookbehind, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::Item_2b(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state111(krate, input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3, __sym4));
                }
                __Nonterminal::MacroDef(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state70(krate, input, __lookbehind, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::MacroRef(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state71(krate, input, __lookbehind, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::Module(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state72(krate, input, __lookbehind, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::Path(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state73(krate, input, __lookbehind, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::Privacy(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state74(krate, input, __lookbehind, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::Structure(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state75(krate, input, __lookbehind, __tokens, __lookahead, __sym4));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 106
    //   Structure = Privacy "struct" Id "{" (*) "}" ["::"]
    //   Structure = Privacy "struct" Id "{" (*) "}" ["macro_rules"]
    //   Structure = Privacy "struct" Id "{" (*) "}" ["mod"]
    //   Structure = Privacy "struct" Id "{" (*) "}" ["pub"]
    //   Structure = Privacy "struct" Id "{" (*) "}" ["self"]
    //   Structure = Privacy "struct" Id "{" (*) "}" ["struct"]
    //   Structure = Privacy "struct" Id "{" (*) "}" ["use"]
    //   Structure = Privacy "struct" Id "{" (*) "}" ["{"]
    //   Structure = Privacy "struct" Id "{" (*) "}" ["}"]
    //   Structure = Privacy "struct" Id "{" (*) "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "}" -> Shift(S113)
    //
    pub fn __state106<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<InternedString>,
        __sym3: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (12, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state113(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3, __sym4));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 107
    //   Glob = Privacy "use" Path "::" (*) "*" ";" ["::"]
    //   Glob = Privacy "use" Path "::" (*) "*" ";" ["macro_rules"]
    //   Glob = Privacy "use" Path "::" (*) "*" ";" ["mod"]
    //   Glob = Privacy "use" Path "::" (*) "*" ";" ["pub"]
    //   Glob = Privacy "use" Path "::" (*) "*" ";" ["self"]
    //   Glob = Privacy "use" Path "::" (*) "*" ";" ["struct"]
    //   Glob = Privacy "use" Path "::" (*) "*" ";" ["use"]
    //   Glob = Privacy "use" Path "::" (*) "*" ";" ["{"]
    //   Glob = Privacy "use" Path "::" (*) "*" ";" ["}"]
    //   Glob = Privacy "use" Path "::" (*) "*" ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["::"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# [";"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["as"]
    //   Path = Path "::" (*) Id ["::"]
    //   Path = Path "::" (*) Id [";"]
    //   Path = Path "::" (*) Id ["as"]
    //
    //   "*" -> Shift(S114)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S45)
    //
    //   Id -> S79
    pub fn __state107<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<PathId>,
        __sym3: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state114(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3, __sym4));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state45(krate, input, __lookbehind, __tokens, __sym4));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym3.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Id(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state79(krate, input, __lookbehind, __tokens, __lookahead, __sym2, __sym3, __sym4));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 108
    //   Import = Privacy "use" Path ";" (*) ["::"]
    //   Import = Privacy "use" Path ";" (*) ["macro_rules"]
    //   Import = Privacy "use" Path ";" (*) ["mod"]
    //   Import = Privacy "use" Path ";" (*) ["pub"]
    //   Import = Privacy "use" Path ";" (*) ["self"]
    //   Import = Privacy "use" Path ";" (*) ["struct"]
    //   Import = Privacy "use" Path ";" (*) ["use"]
    //   Import = Privacy "use" Path ";" (*) ["{"]
    //   Import = Privacy "use" Path ";" (*) ["}"]
    //   Import = Privacy "use" Path ";" (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "::" -> Reduce(Import = Privacy, "use", Path, ";" => ActionFn(38);)
    //   "macro_rules" -> Reduce(Import = Privacy, "use", Path, ";" => ActionFn(38);)
    //   "mod" -> Reduce(Import = Privacy, "use", Path, ";" => ActionFn(38);)
    //   "pub" -> Reduce(Import = Privacy, "use", Path, ";" => ActionFn(38);)
    //   "self" -> Reduce(Import = Privacy, "use", Path, ";" => ActionFn(38);)
    //   "struct" -> Reduce(Import = Privacy, "use", Path, ";" => ActionFn(38);)
    //   "use" -> Reduce(Import = Privacy, "use", Path, ";" => ActionFn(38);)
    //   "{" -> Reduce(Import = Privacy, "use", Path, ";" => ActionFn(38);)
    //   "}" -> Reduce(Import = Privacy, "use", Path, ";" => ActionFn(38);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Import = Privacy, "use", Path, ";" => ActionFn(38);)
    //
    pub fn __state108<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<PathId>,
        __sym3: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __nt = super::__action38(krate, input, __sym0, __sym1, __sym2, __sym3, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Import(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 109
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# [";"]
    //   Import = Privacy "use" Path "as" (*) Id ";" ["::"]
    //   Import = Privacy "use" Path "as" (*) Id ";" ["macro_rules"]
    //   Import = Privacy "use" Path "as" (*) Id ";" ["mod"]
    //   Import = Privacy "use" Path "as" (*) Id ";" ["pub"]
    //   Import = Privacy "use" Path "as" (*) Id ";" ["self"]
    //   Import = Privacy "use" Path "as" (*) Id ";" ["struct"]
    //   Import = Privacy "use" Path "as" (*) Id ";" ["use"]
    //   Import = Privacy "use" Path "as" (*) Id ";" ["{"]
    //   Import = Privacy "use" Path "as" (*) Id ";" ["}"]
    //   Import = Privacy "use" Path "as" (*) Id ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S82)
    //
    //   Id -> S115
    pub fn __state109<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<PathId>,
        __sym3: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym4 = &mut Some((__tok0));
                __result = try!(__state82(krate, input, __lookbehind, __tokens, __sym4));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym3.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Id(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state115(krate, input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3, __sym4));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 110
    //   Code = (*) "{" (<Path> ";")+ "}" ["::"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["macro_rules"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["mod"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["pub"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["self"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["struct"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["use"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["{"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["}"]
    //   Code = (*) "{" (<Path> ";")+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Code = (*) "{" "}" ["::"]
    //   Code = (*) "{" "}" ["macro_rules"]
    //   Code = (*) "{" "}" ["mod"]
    //   Code = (*) "{" "}" ["pub"]
    //   Code = (*) "{" "}" ["self"]
    //   Code = (*) "{" "}" ["struct"]
    //   Code = (*) "{" "}" ["use"]
    //   Code = (*) "{" "}" ["{"]
    //   Code = (*) "{" "}" ["}"]
    //   Code = (*) "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["::"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["macro_rules"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["mod"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["pub"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["self"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["struct"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["use"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["{"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["}"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["!"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["::"]
    //   Import = (*) Privacy "use" Path ";" ["::"]
    //   Import = (*) Privacy "use" Path ";" ["macro_rules"]
    //   Import = (*) Privacy "use" Path ";" ["mod"]
    //   Import = (*) Privacy "use" Path ";" ["pub"]
    //   Import = (*) Privacy "use" Path ";" ["self"]
    //   Import = (*) Privacy "use" Path ";" ["struct"]
    //   Import = (*) Privacy "use" Path ";" ["use"]
    //   Import = (*) Privacy "use" Path ";" ["{"]
    //   Import = (*) Privacy "use" Path ";" ["}"]
    //   Import = (*) Privacy "use" Path ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["::"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["macro_rules"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["mod"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["pub"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["self"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["struct"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["use"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["{"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["}"]
    //   Import = (*) Privacy "use" Path "as" Id ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Code ["::"]
    //   Item = (*) Code ["macro_rules"]
    //   Item = (*) Code ["mod"]
    //   Item = (*) Code ["pub"]
    //   Item = (*) Code ["self"]
    //   Item = (*) Code ["struct"]
    //   Item = (*) Code ["use"]
    //   Item = (*) Code ["{"]
    //   Item = (*) Code ["}"]
    //   Item = (*) Code [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Glob ["::"]
    //   Item = (*) Glob ["macro_rules"]
    //   Item = (*) Glob ["mod"]
    //   Item = (*) Glob ["pub"]
    //   Item = (*) Glob ["self"]
    //   Item = (*) Glob ["struct"]
    //   Item = (*) Glob ["use"]
    //   Item = (*) Glob ["{"]
    //   Item = (*) Glob ["}"]
    //   Item = (*) Glob [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Import ["::"]
    //   Item = (*) Import ["macro_rules"]
    //   Item = (*) Import ["mod"]
    //   Item = (*) Import ["pub"]
    //   Item = (*) Import ["self"]
    //   Item = (*) Import ["struct"]
    //   Item = (*) Import ["use"]
    //   Item = (*) Import ["{"]
    //   Item = (*) Import ["}"]
    //   Item = (*) Import [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) MacroDef ["::"]
    //   Item = (*) MacroDef ["macro_rules"]
    //   Item = (*) MacroDef ["mod"]
    //   Item = (*) MacroDef ["pub"]
    //   Item = (*) MacroDef ["self"]
    //   Item = (*) MacroDef ["struct"]
    //   Item = (*) MacroDef ["use"]
    //   Item = (*) MacroDef ["{"]
    //   Item = (*) MacroDef ["}"]
    //   Item = (*) MacroDef [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) MacroRef ["::"]
    //   Item = (*) MacroRef ["macro_rules"]
    //   Item = (*) MacroRef ["mod"]
    //   Item = (*) MacroRef ["pub"]
    //   Item = (*) MacroRef ["self"]
    //   Item = (*) MacroRef ["struct"]
    //   Item = (*) MacroRef ["use"]
    //   Item = (*) MacroRef ["{"]
    //   Item = (*) MacroRef ["}"]
    //   Item = (*) MacroRef [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Module ["::"]
    //   Item = (*) Module ["macro_rules"]
    //   Item = (*) Module ["mod"]
    //   Item = (*) Module ["pub"]
    //   Item = (*) Module ["self"]
    //   Item = (*) Module ["struct"]
    //   Item = (*) Module ["use"]
    //   Item = (*) Module ["{"]
    //   Item = (*) Module ["}"]
    //   Item = (*) Module [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Structure ["::"]
    //   Item = (*) Structure ["macro_rules"]
    //   Item = (*) Structure ["mod"]
    //   Item = (*) Structure ["pub"]
    //   Item = (*) Structure ["self"]
    //   Item = (*) Structure ["struct"]
    //   Item = (*) Structure ["use"]
    //   Item = (*) Structure ["{"]
    //   Item = (*) Structure ["}"]
    //   Item = (*) Structure [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item+ = (*) Item ["::"]
    //   Item+ = (*) Item ["macro_rules"]
    //   Item+ = (*) Item ["mod"]
    //   Item+ = (*) Item ["pub"]
    //   Item+ = (*) Item ["self"]
    //   Item+ = (*) Item ["struct"]
    //   Item+ = (*) Item ["use"]
    //   Item+ = (*) Item ["{"]
    //   Item+ = (*) Item ["}"]
    //   Item+ = (*) Item [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item+ = (*) Item+ Item ["::"]
    //   Item+ = (*) Item+ Item ["macro_rules"]
    //   Item+ = (*) Item+ Item ["mod"]
    //   Item+ = (*) Item+ Item ["pub"]
    //   Item+ = (*) Item+ Item ["self"]
    //   Item+ = (*) Item+ Item ["struct"]
    //   Item+ = (*) Item+ Item ["use"]
    //   Item+ = (*) Item+ Item ["{"]
    //   Item+ = (*) Item+ Item ["}"]
    //   Item+ = (*) Item+ Item [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["::"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["macro_rules"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["mod"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["pub"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["self"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["struct"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["use"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["{"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["}"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) Item+ "}" ["::"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) Item+ "}" ["macro_rules"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) Item+ "}" ["mod"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) Item+ "}" ["pub"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) Item+ "}" ["self"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) Item+ "}" ["struct"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) Item+ "}" ["use"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) Item+ "}" ["{"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) Item+ "}" ["}"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["::"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["macro_rules"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["mod"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["pub"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["self"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["struct"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["use"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["{"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["}"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) "}" ["::"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) "}" ["macro_rules"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) "}" ["mod"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) "}" ["pub"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) "}" ["self"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) "}" ["struct"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) "}" ["use"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) "}" ["{"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) "}" ["}"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" (*) "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroRef = (*) Path "!" ";" ["::"]
    //   MacroRef = (*) Path "!" ";" ["macro_rules"]
    //   MacroRef = (*) Path "!" ";" ["mod"]
    //   MacroRef = (*) Path "!" ";" ["pub"]
    //   MacroRef = (*) Path "!" ";" ["self"]
    //   MacroRef = (*) Path "!" ";" ["struct"]
    //   MacroRef = (*) Path "!" ";" ["use"]
    //   MacroRef = (*) Path "!" ";" ["{"]
    //   MacroRef = (*) Path "!" ";" ["}"]
    //   MacroRef = (*) Path "!" ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["::"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["macro_rules"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["mod"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["pub"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["self"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["struct"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["use"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["{"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["}"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = (*) Privacy "mod" Id "{" "}" ["::"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["macro_rules"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["mod"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["pub"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["self"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["struct"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["use"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["{"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["}"]
    //   Module = (*) Privacy "mod" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Path = (*) Id ["!"]
    //   Path = (*) Id ["::"]
    //   Path = (*) Path "::" Id ["!"]
    //   Path = (*) Path "::" Id ["::"]
    //   Path = (*) "::" Id ["!"]
    //   Path = (*) "::" Id ["::"]
    //   Path = (*) "self" "::" Id ["!"]
    //   Path = (*) "self" "::" Id ["::"]
    //   Privacy = (*) ["macro_rules"]
    //   Privacy = (*) ["mod"]
    //   Privacy = (*) ["struct"]
    //   Privacy = (*) ["use"]
    //   Privacy = (*) "pub" ["macro_rules"]
    //   Privacy = (*) "pub" ["mod"]
    //   Privacy = (*) "pub" ["struct"]
    //   Privacy = (*) "pub" ["use"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["::"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["macro_rules"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["mod"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["pub"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["self"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["struct"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["use"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["{"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["}"]
    //   Structure = (*) Privacy "struct" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "::" -> Shift(S14)
    //   "macro_rules" -> Reduce(Privacy =  => ActionFn(39);)
    //   "mod" -> Reduce(Privacy =  => ActionFn(39);)
    //   "pub" -> Shift(S15)
    //   "self" -> Shift(S16)
    //   "struct" -> Reduce(Privacy =  => ActionFn(39);)
    //   "use" -> Reduce(Privacy =  => ActionFn(39);)
    //   "{" -> Shift(S76)
    //   "}" -> Shift(S117)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S18)
    //
    //   Code -> S65
    //   Glob -> S66
    //   Id -> S3
    //   Import -> S67
    //   Item -> S68
    //   Item+ -> S116
    //   MacroDef -> S70
    //   MacroRef -> S71
    //   Module -> S72
    //   Path -> S73
    //   Privacy -> S74
    //   Structure -> S75
    pub fn __state110<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<&'input str>,
        __sym3: &mut Option<InternedString>,
        __sym4: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym5 = &mut Some((__tok0));
                __result = try!(__state14(krate, input, __lookbehind, __tokens, __sym5));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym5 = &mut Some((__tok0));
                __result = try!(__state15(krate, input, __lookbehind, __tokens, __sym5));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym5 = &mut Some((__tok0));
                __result = try!(__state16(krate, input, __lookbehind, __tokens, __sym5));
            }
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym5 = &mut Some((__tok0));
                __result = try!(__state76(krate, input, __lookbehind, __tokens, __sym5));
            }
            Some((_, (12, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym5 = &mut Some((__tok0));
                __result = try!(__state117(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym5 = &mut Some((__tok0));
                __result = try!(__state18(krate, input, __lookbehind, __tokens, __sym5));
            }
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) => {
                let __nt = super::__action39(krate, input, &__lookbehind, &__lookahead);
                __result = (__lookbehind, __lookahead, __Nonterminal::Privacy(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym4.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Code(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state65(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Glob(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state66(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Id(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state3(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Import(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state67(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Item(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state68(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Item_2b(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state116(krate, input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5));
                }
                __Nonterminal::MacroDef(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state70(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::MacroRef(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state71(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Module(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state72(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Path(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state73(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Privacy(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state74(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Structure(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state75(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 111
    //   Code = (*) "{" (<Path> ";")+ "}" ["::"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["macro_rules"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["mod"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["pub"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["self"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["struct"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["use"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["{"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["}"]
    //   Code = (*) "{" (<Path> ";")+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Code = (*) "{" "}" ["::"]
    //   Code = (*) "{" "}" ["macro_rules"]
    //   Code = (*) "{" "}" ["mod"]
    //   Code = (*) "{" "}" ["pub"]
    //   Code = (*) "{" "}" ["self"]
    //   Code = (*) "{" "}" ["struct"]
    //   Code = (*) "{" "}" ["use"]
    //   Code = (*) "{" "}" ["{"]
    //   Code = (*) "{" "}" ["}"]
    //   Code = (*) "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["::"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["macro_rules"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["mod"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["pub"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["self"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["struct"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["use"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["{"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["}"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["!"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["::"]
    //   Import = (*) Privacy "use" Path ";" ["::"]
    //   Import = (*) Privacy "use" Path ";" ["macro_rules"]
    //   Import = (*) Privacy "use" Path ";" ["mod"]
    //   Import = (*) Privacy "use" Path ";" ["pub"]
    //   Import = (*) Privacy "use" Path ";" ["self"]
    //   Import = (*) Privacy "use" Path ";" ["struct"]
    //   Import = (*) Privacy "use" Path ";" ["use"]
    //   Import = (*) Privacy "use" Path ";" ["{"]
    //   Import = (*) Privacy "use" Path ";" ["}"]
    //   Import = (*) Privacy "use" Path ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["::"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["macro_rules"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["mod"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["pub"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["self"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["struct"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["use"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["{"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["}"]
    //   Import = (*) Privacy "use" Path "as" Id ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Code ["::"]
    //   Item = (*) Code ["macro_rules"]
    //   Item = (*) Code ["mod"]
    //   Item = (*) Code ["pub"]
    //   Item = (*) Code ["self"]
    //   Item = (*) Code ["struct"]
    //   Item = (*) Code ["use"]
    //   Item = (*) Code ["{"]
    //   Item = (*) Code ["}"]
    //   Item = (*) Code [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Glob ["::"]
    //   Item = (*) Glob ["macro_rules"]
    //   Item = (*) Glob ["mod"]
    //   Item = (*) Glob ["pub"]
    //   Item = (*) Glob ["self"]
    //   Item = (*) Glob ["struct"]
    //   Item = (*) Glob ["use"]
    //   Item = (*) Glob ["{"]
    //   Item = (*) Glob ["}"]
    //   Item = (*) Glob [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Import ["::"]
    //   Item = (*) Import ["macro_rules"]
    //   Item = (*) Import ["mod"]
    //   Item = (*) Import ["pub"]
    //   Item = (*) Import ["self"]
    //   Item = (*) Import ["struct"]
    //   Item = (*) Import ["use"]
    //   Item = (*) Import ["{"]
    //   Item = (*) Import ["}"]
    //   Item = (*) Import [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) MacroDef ["::"]
    //   Item = (*) MacroDef ["macro_rules"]
    //   Item = (*) MacroDef ["mod"]
    //   Item = (*) MacroDef ["pub"]
    //   Item = (*) MacroDef ["self"]
    //   Item = (*) MacroDef ["struct"]
    //   Item = (*) MacroDef ["use"]
    //   Item = (*) MacroDef ["{"]
    //   Item = (*) MacroDef ["}"]
    //   Item = (*) MacroDef [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) MacroRef ["::"]
    //   Item = (*) MacroRef ["macro_rules"]
    //   Item = (*) MacroRef ["mod"]
    //   Item = (*) MacroRef ["pub"]
    //   Item = (*) MacroRef ["self"]
    //   Item = (*) MacroRef ["struct"]
    //   Item = (*) MacroRef ["use"]
    //   Item = (*) MacroRef ["{"]
    //   Item = (*) MacroRef ["}"]
    //   Item = (*) MacroRef [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Module ["::"]
    //   Item = (*) Module ["macro_rules"]
    //   Item = (*) Module ["mod"]
    //   Item = (*) Module ["pub"]
    //   Item = (*) Module ["self"]
    //   Item = (*) Module ["struct"]
    //   Item = (*) Module ["use"]
    //   Item = (*) Module ["{"]
    //   Item = (*) Module ["}"]
    //   Item = (*) Module [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Structure ["::"]
    //   Item = (*) Structure ["macro_rules"]
    //   Item = (*) Structure ["mod"]
    //   Item = (*) Structure ["pub"]
    //   Item = (*) Structure ["self"]
    //   Item = (*) Structure ["struct"]
    //   Item = (*) Structure ["use"]
    //   Item = (*) Structure ["{"]
    //   Item = (*) Structure ["}"]
    //   Item = (*) Structure [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item+ = Item+ (*) Item ["::"]
    //   Item+ = Item+ (*) Item ["macro_rules"]
    //   Item+ = Item+ (*) Item ["mod"]
    //   Item+ = Item+ (*) Item ["pub"]
    //   Item+ = Item+ (*) Item ["self"]
    //   Item+ = Item+ (*) Item ["struct"]
    //   Item+ = Item+ (*) Item ["use"]
    //   Item+ = Item+ (*) Item ["{"]
    //   Item+ = Item+ (*) Item ["}"]
    //   Item+ = Item+ (*) Item [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["::"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["macro_rules"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["mod"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["pub"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["self"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["struct"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["use"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["{"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["}"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["::"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["macro_rules"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["mod"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["pub"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["self"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["struct"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["use"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["{"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["}"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroRef = (*) Path "!" ";" ["::"]
    //   MacroRef = (*) Path "!" ";" ["macro_rules"]
    //   MacroRef = (*) Path "!" ";" ["mod"]
    //   MacroRef = (*) Path "!" ";" ["pub"]
    //   MacroRef = (*) Path "!" ";" ["self"]
    //   MacroRef = (*) Path "!" ";" ["struct"]
    //   MacroRef = (*) Path "!" ";" ["use"]
    //   MacroRef = (*) Path "!" ";" ["{"]
    //   MacroRef = (*) Path "!" ";" ["}"]
    //   MacroRef = (*) Path "!" ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["::"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["macro_rules"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["mod"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["pub"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["self"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["struct"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["use"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["{"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["}"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = Privacy "mod" Id "{" Item+ (*) "}" ["::"]
    //   Module = Privacy "mod" Id "{" Item+ (*) "}" ["macro_rules"]
    //   Module = Privacy "mod" Id "{" Item+ (*) "}" ["mod"]
    //   Module = Privacy "mod" Id "{" Item+ (*) "}" ["pub"]
    //   Module = Privacy "mod" Id "{" Item+ (*) "}" ["self"]
    //   Module = Privacy "mod" Id "{" Item+ (*) "}" ["struct"]
    //   Module = Privacy "mod" Id "{" Item+ (*) "}" ["use"]
    //   Module = Privacy "mod" Id "{" Item+ (*) "}" ["{"]
    //   Module = Privacy "mod" Id "{" Item+ (*) "}" ["}"]
    //   Module = Privacy "mod" Id "{" Item+ (*) "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = (*) Privacy "mod" Id "{" "}" ["::"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["macro_rules"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["mod"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["pub"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["self"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["struct"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["use"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["{"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["}"]
    //   Module = (*) Privacy "mod" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Path = (*) Id ["!"]
    //   Path = (*) Id ["::"]
    //   Path = (*) Path "::" Id ["!"]
    //   Path = (*) Path "::" Id ["::"]
    //   Path = (*) "::" Id ["!"]
    //   Path = (*) "::" Id ["::"]
    //   Path = (*) "self" "::" Id ["!"]
    //   Path = (*) "self" "::" Id ["::"]
    //   Privacy = (*) ["macro_rules"]
    //   Privacy = (*) ["mod"]
    //   Privacy = (*) ["struct"]
    //   Privacy = (*) ["use"]
    //   Privacy = (*) "pub" ["macro_rules"]
    //   Privacy = (*) "pub" ["mod"]
    //   Privacy = (*) "pub" ["struct"]
    //   Privacy = (*) "pub" ["use"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["::"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["macro_rules"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["mod"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["pub"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["self"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["struct"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["use"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["{"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["}"]
    //   Structure = (*) Privacy "struct" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "::" -> Shift(S14)
    //   "macro_rules" -> Reduce(Privacy =  => ActionFn(39);)
    //   "mod" -> Reduce(Privacy =  => ActionFn(39);)
    //   "pub" -> Shift(S15)
    //   "self" -> Shift(S16)
    //   "struct" -> Reduce(Privacy =  => ActionFn(39);)
    //   "use" -> Reduce(Privacy =  => ActionFn(39);)
    //   "{" -> Shift(S76)
    //   "}" -> Shift(S118)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S18)
    //
    //   Code -> S65
    //   Glob -> S66
    //   Id -> S3
    //   Import -> S67
    //   Item -> S86
    //   MacroDef -> S70
    //   MacroRef -> S71
    //   Module -> S72
    //   Path -> S73
    //   Privacy -> S74
    //   Structure -> S75
    pub fn __state111<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<InternedString>,
        __sym3: &mut Option<&'input str>,
        __sym4: &mut Option<::std::vec::Vec<ItemId>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym5 = &mut Some((__tok0));
                __result = try!(__state14(krate, input, __lookbehind, __tokens, __sym5));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym5 = &mut Some((__tok0));
                __result = try!(__state15(krate, input, __lookbehind, __tokens, __sym5));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym5 = &mut Some((__tok0));
                __result = try!(__state16(krate, input, __lookbehind, __tokens, __sym5));
            }
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym5 = &mut Some((__tok0));
                __result = try!(__state76(krate, input, __lookbehind, __tokens, __sym5));
            }
            Some((_, (12, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym5 = &mut Some((__tok0));
                __result = try!(__state118(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym5 = &mut Some((__tok0));
                __result = try!(__state18(krate, input, __lookbehind, __tokens, __sym5));
            }
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) => {
                let __nt = super::__action39(krate, input, &__lookbehind, &__lookahead);
                __result = (__lookbehind, __lookahead, __Nonterminal::Privacy(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym4.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Code(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state65(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Glob(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state66(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Id(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state3(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Import(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state67(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Item(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state86(krate, input, __lookbehind, __tokens, __lookahead, __sym4, __sym5));
                }
                __Nonterminal::MacroDef(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state70(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::MacroRef(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state71(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Module(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state72(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Path(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state73(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Privacy(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state74(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Structure(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state75(krate, input, __lookbehind, __tokens, __lookahead, __sym5));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 112
    //   Module = Privacy "mod" Id "{" "}" (*) ["::"]
    //   Module = Privacy "mod" Id "{" "}" (*) ["macro_rules"]
    //   Module = Privacy "mod" Id "{" "}" (*) ["mod"]
    //   Module = Privacy "mod" Id "{" "}" (*) ["pub"]
    //   Module = Privacy "mod" Id "{" "}" (*) ["self"]
    //   Module = Privacy "mod" Id "{" "}" (*) ["struct"]
    //   Module = Privacy "mod" Id "{" "}" (*) ["use"]
    //   Module = Privacy "mod" Id "{" "}" (*) ["{"]
    //   Module = Privacy "mod" Id "{" "}" (*) ["}"]
    //   Module = Privacy "mod" Id "{" "}" (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "::" -> Reduce(Module = Privacy, "mod", Id, "{", "}" => ActionFn(46);)
    //   "macro_rules" -> Reduce(Module = Privacy, "mod", Id, "{", "}" => ActionFn(46);)
    //   "mod" -> Reduce(Module = Privacy, "mod", Id, "{", "}" => ActionFn(46);)
    //   "pub" -> Reduce(Module = Privacy, "mod", Id, "{", "}" => ActionFn(46);)
    //   "self" -> Reduce(Module = Privacy, "mod", Id, "{", "}" => ActionFn(46);)
    //   "struct" -> Reduce(Module = Privacy, "mod", Id, "{", "}" => ActionFn(46);)
    //   "use" -> Reduce(Module = Privacy, "mod", Id, "{", "}" => ActionFn(46);)
    //   "{" -> Reduce(Module = Privacy, "mod", Id, "{", "}" => ActionFn(46);)
    //   "}" -> Reduce(Module = Privacy, "mod", Id, "{", "}" => ActionFn(46);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Module = Privacy, "mod", Id, "{", "}" => ActionFn(46);)
    //
    pub fn __state112<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<InternedString>,
        __sym3: &mut Option<&'input str>,
        __sym4: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __sym4 = __sym4.take().unwrap();
                let __nt = super::__action46(krate, input, __sym0, __sym1, __sym2, __sym3, __sym4, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Module(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 113
    //   Structure = Privacy "struct" Id "{" "}" (*) ["::"]
    //   Structure = Privacy "struct" Id "{" "}" (*) ["macro_rules"]
    //   Structure = Privacy "struct" Id "{" "}" (*) ["mod"]
    //   Structure = Privacy "struct" Id "{" "}" (*) ["pub"]
    //   Structure = Privacy "struct" Id "{" "}" (*) ["self"]
    //   Structure = Privacy "struct" Id "{" "}" (*) ["struct"]
    //   Structure = Privacy "struct" Id "{" "}" (*) ["use"]
    //   Structure = Privacy "struct" Id "{" "}" (*) ["{"]
    //   Structure = Privacy "struct" Id "{" "}" (*) ["}"]
    //   Structure = Privacy "struct" Id "{" "}" (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "::" -> Reduce(Structure = Privacy, "struct", Id, "{", "}" => ActionFn(10);)
    //   "macro_rules" -> Reduce(Structure = Privacy, "struct", Id, "{", "}" => ActionFn(10);)
    //   "mod" -> Reduce(Structure = Privacy, "struct", Id, "{", "}" => ActionFn(10);)
    //   "pub" -> Reduce(Structure = Privacy, "struct", Id, "{", "}" => ActionFn(10);)
    //   "self" -> Reduce(Structure = Privacy, "struct", Id, "{", "}" => ActionFn(10);)
    //   "struct" -> Reduce(Structure = Privacy, "struct", Id, "{", "}" => ActionFn(10);)
    //   "use" -> Reduce(Structure = Privacy, "struct", Id, "{", "}" => ActionFn(10);)
    //   "{" -> Reduce(Structure = Privacy, "struct", Id, "{", "}" => ActionFn(10);)
    //   "}" -> Reduce(Structure = Privacy, "struct", Id, "{", "}" => ActionFn(10);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Structure = Privacy, "struct", Id, "{", "}" => ActionFn(10);)
    //
    pub fn __state113<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<InternedString>,
        __sym3: &mut Option<&'input str>,
        __sym4: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __sym4 = __sym4.take().unwrap();
                let __nt = super::__action10(krate, input, __sym0, __sym1, __sym2, __sym3, __sym4, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Structure(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 114
    //   Glob = Privacy "use" Path "::" "*" (*) ";" ["::"]
    //   Glob = Privacy "use" Path "::" "*" (*) ";" ["macro_rules"]
    //   Glob = Privacy "use" Path "::" "*" (*) ";" ["mod"]
    //   Glob = Privacy "use" Path "::" "*" (*) ";" ["pub"]
    //   Glob = Privacy "use" Path "::" "*" (*) ";" ["self"]
    //   Glob = Privacy "use" Path "::" "*" (*) ";" ["struct"]
    //   Glob = Privacy "use" Path "::" "*" (*) ";" ["use"]
    //   Glob = Privacy "use" Path "::" "*" (*) ";" ["{"]
    //   Glob = Privacy "use" Path "::" "*" (*) ";" ["}"]
    //   Glob = Privacy "use" Path "::" "*" (*) ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   ";" -> Shift(S119)
    //
    pub fn __state114<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<PathId>,
        __sym3: &mut Option<&'input str>,
        __sym4: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym5 = &mut Some((__tok0));
                __result = try!(__state119(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 115
    //   Import = Privacy "use" Path "as" Id (*) ";" ["::"]
    //   Import = Privacy "use" Path "as" Id (*) ";" ["macro_rules"]
    //   Import = Privacy "use" Path "as" Id (*) ";" ["mod"]
    //   Import = Privacy "use" Path "as" Id (*) ";" ["pub"]
    //   Import = Privacy "use" Path "as" Id (*) ";" ["self"]
    //   Import = Privacy "use" Path "as" Id (*) ";" ["struct"]
    //   Import = Privacy "use" Path "as" Id (*) ";" ["use"]
    //   Import = Privacy "use" Path "as" Id (*) ";" ["{"]
    //   Import = Privacy "use" Path "as" Id (*) ";" ["}"]
    //   Import = Privacy "use" Path "as" Id (*) ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   ";" -> Shift(S120)
    //
    pub fn __state115<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<PathId>,
        __sym3: &mut Option<&'input str>,
        __sym4: &mut Option<InternedString>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym5 = &mut Some((__tok0));
                __result = try!(__state120(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 116
    //   Code = (*) "{" (<Path> ";")+ "}" ["::"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["macro_rules"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["mod"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["pub"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["self"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["struct"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["use"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["{"]
    //   Code = (*) "{" (<Path> ";")+ "}" ["}"]
    //   Code = (*) "{" (<Path> ";")+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Code = (*) "{" "}" ["::"]
    //   Code = (*) "{" "}" ["macro_rules"]
    //   Code = (*) "{" "}" ["mod"]
    //   Code = (*) "{" "}" ["pub"]
    //   Code = (*) "{" "}" ["self"]
    //   Code = (*) "{" "}" ["struct"]
    //   Code = (*) "{" "}" ["use"]
    //   Code = (*) "{" "}" ["{"]
    //   Code = (*) "{" "}" ["}"]
    //   Code = (*) "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["::"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["macro_rules"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["mod"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["pub"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["self"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["struct"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["use"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["{"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" ["}"]
    //   Glob = (*) Privacy "use" Path "::" "*" ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["!"]
    //   Id = (*) r#"[a-zA-Z_][a-zA-Z0-9_]*"# ["::"]
    //   Import = (*) Privacy "use" Path ";" ["::"]
    //   Import = (*) Privacy "use" Path ";" ["macro_rules"]
    //   Import = (*) Privacy "use" Path ";" ["mod"]
    //   Import = (*) Privacy "use" Path ";" ["pub"]
    //   Import = (*) Privacy "use" Path ";" ["self"]
    //   Import = (*) Privacy "use" Path ";" ["struct"]
    //   Import = (*) Privacy "use" Path ";" ["use"]
    //   Import = (*) Privacy "use" Path ";" ["{"]
    //   Import = (*) Privacy "use" Path ";" ["}"]
    //   Import = (*) Privacy "use" Path ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["::"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["macro_rules"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["mod"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["pub"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["self"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["struct"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["use"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["{"]
    //   Import = (*) Privacy "use" Path "as" Id ";" ["}"]
    //   Import = (*) Privacy "use" Path "as" Id ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Code ["::"]
    //   Item = (*) Code ["macro_rules"]
    //   Item = (*) Code ["mod"]
    //   Item = (*) Code ["pub"]
    //   Item = (*) Code ["self"]
    //   Item = (*) Code ["struct"]
    //   Item = (*) Code ["use"]
    //   Item = (*) Code ["{"]
    //   Item = (*) Code ["}"]
    //   Item = (*) Code [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Glob ["::"]
    //   Item = (*) Glob ["macro_rules"]
    //   Item = (*) Glob ["mod"]
    //   Item = (*) Glob ["pub"]
    //   Item = (*) Glob ["self"]
    //   Item = (*) Glob ["struct"]
    //   Item = (*) Glob ["use"]
    //   Item = (*) Glob ["{"]
    //   Item = (*) Glob ["}"]
    //   Item = (*) Glob [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Import ["::"]
    //   Item = (*) Import ["macro_rules"]
    //   Item = (*) Import ["mod"]
    //   Item = (*) Import ["pub"]
    //   Item = (*) Import ["self"]
    //   Item = (*) Import ["struct"]
    //   Item = (*) Import ["use"]
    //   Item = (*) Import ["{"]
    //   Item = (*) Import ["}"]
    //   Item = (*) Import [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) MacroDef ["::"]
    //   Item = (*) MacroDef ["macro_rules"]
    //   Item = (*) MacroDef ["mod"]
    //   Item = (*) MacroDef ["pub"]
    //   Item = (*) MacroDef ["self"]
    //   Item = (*) MacroDef ["struct"]
    //   Item = (*) MacroDef ["use"]
    //   Item = (*) MacroDef ["{"]
    //   Item = (*) MacroDef ["}"]
    //   Item = (*) MacroDef [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) MacroRef ["::"]
    //   Item = (*) MacroRef ["macro_rules"]
    //   Item = (*) MacroRef ["mod"]
    //   Item = (*) MacroRef ["pub"]
    //   Item = (*) MacroRef ["self"]
    //   Item = (*) MacroRef ["struct"]
    //   Item = (*) MacroRef ["use"]
    //   Item = (*) MacroRef ["{"]
    //   Item = (*) MacroRef ["}"]
    //   Item = (*) MacroRef [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Module ["::"]
    //   Item = (*) Module ["macro_rules"]
    //   Item = (*) Module ["mod"]
    //   Item = (*) Module ["pub"]
    //   Item = (*) Module ["self"]
    //   Item = (*) Module ["struct"]
    //   Item = (*) Module ["use"]
    //   Item = (*) Module ["{"]
    //   Item = (*) Module ["}"]
    //   Item = (*) Module [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item = (*) Structure ["::"]
    //   Item = (*) Structure ["macro_rules"]
    //   Item = (*) Structure ["mod"]
    //   Item = (*) Structure ["pub"]
    //   Item = (*) Structure ["self"]
    //   Item = (*) Structure ["struct"]
    //   Item = (*) Structure ["use"]
    //   Item = (*) Structure ["{"]
    //   Item = (*) Structure ["}"]
    //   Item = (*) Structure [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Item+ = Item+ (*) Item ["::"]
    //   Item+ = Item+ (*) Item ["macro_rules"]
    //   Item+ = Item+ (*) Item ["mod"]
    //   Item+ = Item+ (*) Item ["pub"]
    //   Item+ = Item+ (*) Item ["self"]
    //   Item+ = Item+ (*) Item ["struct"]
    //   Item+ = Item+ (*) Item ["use"]
    //   Item+ = Item+ (*) Item ["{"]
    //   Item+ = Item+ (*) Item ["}"]
    //   Item+ = Item+ (*) Item [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["::"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["macro_rules"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["mod"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["pub"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["self"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["struct"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["use"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["{"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" ["}"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ (*) "}" ["::"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ (*) "}" ["macro_rules"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ (*) "}" ["mod"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ (*) "}" ["pub"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ (*) "}" ["self"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ (*) "}" ["struct"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ (*) "}" ["use"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ (*) "}" ["{"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ (*) "}" ["}"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ (*) "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["::"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["macro_rules"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["mod"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["pub"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["self"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["struct"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["use"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["{"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" ["}"]
    //   MacroDef = (*) Privacy "macro_rules" "!" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   MacroRef = (*) Path "!" ";" ["::"]
    //   MacroRef = (*) Path "!" ";" ["macro_rules"]
    //   MacroRef = (*) Path "!" ";" ["mod"]
    //   MacroRef = (*) Path "!" ";" ["pub"]
    //   MacroRef = (*) Path "!" ";" ["self"]
    //   MacroRef = (*) Path "!" ";" ["struct"]
    //   MacroRef = (*) Path "!" ";" ["use"]
    //   MacroRef = (*) Path "!" ";" ["{"]
    //   MacroRef = (*) Path "!" ";" ["}"]
    //   MacroRef = (*) Path "!" ";" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["::"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["macro_rules"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["mod"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["pub"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["self"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["struct"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["use"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["{"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" ["}"]
    //   Module = (*) Privacy "mod" Id "{" Item+ "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Module = (*) Privacy "mod" Id "{" "}" ["::"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["macro_rules"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["mod"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["pub"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["self"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["struct"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["use"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["{"]
    //   Module = (*) Privacy "mod" Id "{" "}" ["}"]
    //   Module = (*) Privacy "mod" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //   Path = (*) Id ["!"]
    //   Path = (*) Id ["::"]
    //   Path = (*) Path "::" Id ["!"]
    //   Path = (*) Path "::" Id ["::"]
    //   Path = (*) "::" Id ["!"]
    //   Path = (*) "::" Id ["::"]
    //   Path = (*) "self" "::" Id ["!"]
    //   Path = (*) "self" "::" Id ["::"]
    //   Privacy = (*) ["macro_rules"]
    //   Privacy = (*) ["mod"]
    //   Privacy = (*) ["struct"]
    //   Privacy = (*) ["use"]
    //   Privacy = (*) "pub" ["macro_rules"]
    //   Privacy = (*) "pub" ["mod"]
    //   Privacy = (*) "pub" ["struct"]
    //   Privacy = (*) "pub" ["use"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["::"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["macro_rules"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["mod"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["pub"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["self"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["struct"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["use"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["{"]
    //   Structure = (*) Privacy "struct" Id "{" "}" ["}"]
    //   Structure = (*) Privacy "struct" Id "{" "}" [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "::" -> Shift(S14)
    //   "macro_rules" -> Reduce(Privacy =  => ActionFn(39);)
    //   "mod" -> Reduce(Privacy =  => ActionFn(39);)
    //   "pub" -> Shift(S15)
    //   "self" -> Shift(S16)
    //   "struct" -> Reduce(Privacy =  => ActionFn(39);)
    //   "use" -> Reduce(Privacy =  => ActionFn(39);)
    //   "{" -> Shift(S76)
    //   "}" -> Shift(S121)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Shift(S18)
    //
    //   Code -> S65
    //   Glob -> S66
    //   Id -> S3
    //   Import -> S67
    //   Item -> S86
    //   MacroDef -> S70
    //   MacroRef -> S71
    //   Module -> S72
    //   Path -> S73
    //   Privacy -> S74
    //   Structure -> S75
    pub fn __state116<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<&'input str>,
        __sym3: &mut Option<InternedString>,
        __sym4: &mut Option<&'input str>,
        __sym5: &mut Option<::std::vec::Vec<ItemId>>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym6 = &mut Some((__tok0));
                __result = try!(__state14(krate, input, __lookbehind, __tokens, __sym6));
            }
            Some((_, (7, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym6 = &mut Some((__tok0));
                __result = try!(__state15(krate, input, __lookbehind, __tokens, __sym6));
            }
            Some((_, (8, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym6 = &mut Some((__tok0));
                __result = try!(__state16(krate, input, __lookbehind, __tokens, __sym6));
            }
            Some((_, (11, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym6 = &mut Some((__tok0));
                __result = try!(__state76(krate, input, __lookbehind, __tokens, __sym6));
            }
            Some((_, (12, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym6 = &mut Some((__tok0));
                __result = try!(__state121(krate, input, __lookbehind, __tokens, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6));
            }
            Some((_, (13, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym6 = &mut Some((__tok0));
                __result = try!(__state18(krate, input, __lookbehind, __tokens, __sym6));
            }
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) => {
                let __nt = super::__action39(krate, input, &__lookbehind, &__lookahead);
                __result = (__lookbehind, __lookahead, __Nonterminal::Privacy(__nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym5.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Code(__nt) => {
                    let __sym6 = &mut Some(__nt);
                    __result = try!(__state65(krate, input, __lookbehind, __tokens, __lookahead, __sym6));
                }
                __Nonterminal::Glob(__nt) => {
                    let __sym6 = &mut Some(__nt);
                    __result = try!(__state66(krate, input, __lookbehind, __tokens, __lookahead, __sym6));
                }
                __Nonterminal::Id(__nt) => {
                    let __sym6 = &mut Some(__nt);
                    __result = try!(__state3(krate, input, __lookbehind, __tokens, __lookahead, __sym6));
                }
                __Nonterminal::Import(__nt) => {
                    let __sym6 = &mut Some(__nt);
                    __result = try!(__state67(krate, input, __lookbehind, __tokens, __lookahead, __sym6));
                }
                __Nonterminal::Item(__nt) => {
                    let __sym6 = &mut Some(__nt);
                    __result = try!(__state86(krate, input, __lookbehind, __tokens, __lookahead, __sym5, __sym6));
                }
                __Nonterminal::MacroDef(__nt) => {
                    let __sym6 = &mut Some(__nt);
                    __result = try!(__state70(krate, input, __lookbehind, __tokens, __lookahead, __sym6));
                }
                __Nonterminal::MacroRef(__nt) => {
                    let __sym6 = &mut Some(__nt);
                    __result = try!(__state71(krate, input, __lookbehind, __tokens, __lookahead, __sym6));
                }
                __Nonterminal::Module(__nt) => {
                    let __sym6 = &mut Some(__nt);
                    __result = try!(__state72(krate, input, __lookbehind, __tokens, __lookahead, __sym6));
                }
                __Nonterminal::Path(__nt) => {
                    let __sym6 = &mut Some(__nt);
                    __result = try!(__state73(krate, input, __lookbehind, __tokens, __lookahead, __sym6));
                }
                __Nonterminal::Privacy(__nt) => {
                    let __sym6 = &mut Some(__nt);
                    __result = try!(__state74(krate, input, __lookbehind, __tokens, __lookahead, __sym6));
                }
                __Nonterminal::Structure(__nt) => {
                    let __sym6 = &mut Some(__nt);
                    __result = try!(__state75(krate, input, __lookbehind, __tokens, __lookahead, __sym6));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 117
    //   MacroDef = Privacy "macro_rules" "!" Id "{" "}" (*) ["::"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" "}" (*) ["macro_rules"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" "}" (*) ["mod"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" "}" (*) ["pub"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" "}" (*) ["self"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" "}" (*) ["struct"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" "}" (*) ["use"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" "}" (*) ["{"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" "}" (*) ["}"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" "}" (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "::" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", "}" => ActionFn(44);)
    //   "macro_rules" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", "}" => ActionFn(44);)
    //   "mod" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", "}" => ActionFn(44);)
    //   "pub" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", "}" => ActionFn(44);)
    //   "self" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", "}" => ActionFn(44);)
    //   "struct" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", "}" => ActionFn(44);)
    //   "use" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", "}" => ActionFn(44);)
    //   "{" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", "}" => ActionFn(44);)
    //   "}" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", "}" => ActionFn(44);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", "}" => ActionFn(44);)
    //
    pub fn __state117<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<&'input str>,
        __sym3: &mut Option<InternedString>,
        __sym4: &mut Option<&'input str>,
        __sym5: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __sym4 = __sym4.take().unwrap();
                let __sym5 = __sym5.take().unwrap();
                let __nt = super::__action44(krate, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::MacroDef(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 118
    //   Module = Privacy "mod" Id "{" Item+ "}" (*) ["::"]
    //   Module = Privacy "mod" Id "{" Item+ "}" (*) ["macro_rules"]
    //   Module = Privacy "mod" Id "{" Item+ "}" (*) ["mod"]
    //   Module = Privacy "mod" Id "{" Item+ "}" (*) ["pub"]
    //   Module = Privacy "mod" Id "{" Item+ "}" (*) ["self"]
    //   Module = Privacy "mod" Id "{" Item+ "}" (*) ["struct"]
    //   Module = Privacy "mod" Id "{" Item+ "}" (*) ["use"]
    //   Module = Privacy "mod" Id "{" Item+ "}" (*) ["{"]
    //   Module = Privacy "mod" Id "{" Item+ "}" (*) ["}"]
    //   Module = Privacy "mod" Id "{" Item+ "}" (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "::" -> Reduce(Module = Privacy, "mod", Id, "{", Item+, "}" => ActionFn(47);)
    //   "macro_rules" -> Reduce(Module = Privacy, "mod", Id, "{", Item+, "}" => ActionFn(47);)
    //   "mod" -> Reduce(Module = Privacy, "mod", Id, "{", Item+, "}" => ActionFn(47);)
    //   "pub" -> Reduce(Module = Privacy, "mod", Id, "{", Item+, "}" => ActionFn(47);)
    //   "self" -> Reduce(Module = Privacy, "mod", Id, "{", Item+, "}" => ActionFn(47);)
    //   "struct" -> Reduce(Module = Privacy, "mod", Id, "{", Item+, "}" => ActionFn(47);)
    //   "use" -> Reduce(Module = Privacy, "mod", Id, "{", Item+, "}" => ActionFn(47);)
    //   "{" -> Reduce(Module = Privacy, "mod", Id, "{", Item+, "}" => ActionFn(47);)
    //   "}" -> Reduce(Module = Privacy, "mod", Id, "{", Item+, "}" => ActionFn(47);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Module = Privacy, "mod", Id, "{", Item+, "}" => ActionFn(47);)
    //
    pub fn __state118<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<InternedString>,
        __sym3: &mut Option<&'input str>,
        __sym4: &mut Option<::std::vec::Vec<ItemId>>,
        __sym5: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __sym4 = __sym4.take().unwrap();
                let __sym5 = __sym5.take().unwrap();
                let __nt = super::__action47(krate, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Module(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 119
    //   Glob = Privacy "use" Path "::" "*" ";" (*) ["::"]
    //   Glob = Privacy "use" Path "::" "*" ";" (*) ["macro_rules"]
    //   Glob = Privacy "use" Path "::" "*" ";" (*) ["mod"]
    //   Glob = Privacy "use" Path "::" "*" ";" (*) ["pub"]
    //   Glob = Privacy "use" Path "::" "*" ";" (*) ["self"]
    //   Glob = Privacy "use" Path "::" "*" ";" (*) ["struct"]
    //   Glob = Privacy "use" Path "::" "*" ";" (*) ["use"]
    //   Glob = Privacy "use" Path "::" "*" ";" (*) ["{"]
    //   Glob = Privacy "use" Path "::" "*" ";" (*) ["}"]
    //   Glob = Privacy "use" Path "::" "*" ";" (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "::" -> Reduce(Glob = Privacy, "use", Path, "::", "*", ";" => ActionFn(12);)
    //   "macro_rules" -> Reduce(Glob = Privacy, "use", Path, "::", "*", ";" => ActionFn(12);)
    //   "mod" -> Reduce(Glob = Privacy, "use", Path, "::", "*", ";" => ActionFn(12);)
    //   "pub" -> Reduce(Glob = Privacy, "use", Path, "::", "*", ";" => ActionFn(12);)
    //   "self" -> Reduce(Glob = Privacy, "use", Path, "::", "*", ";" => ActionFn(12);)
    //   "struct" -> Reduce(Glob = Privacy, "use", Path, "::", "*", ";" => ActionFn(12);)
    //   "use" -> Reduce(Glob = Privacy, "use", Path, "::", "*", ";" => ActionFn(12);)
    //   "{" -> Reduce(Glob = Privacy, "use", Path, "::", "*", ";" => ActionFn(12);)
    //   "}" -> Reduce(Glob = Privacy, "use", Path, "::", "*", ";" => ActionFn(12);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Glob = Privacy, "use", Path, "::", "*", ";" => ActionFn(12);)
    //
    pub fn __state119<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<PathId>,
        __sym3: &mut Option<&'input str>,
        __sym4: &mut Option<&'input str>,
        __sym5: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __sym4 = __sym4.take().unwrap();
                let __sym5 = __sym5.take().unwrap();
                let __nt = super::__action12(krate, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Glob(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 120
    //   Import = Privacy "use" Path "as" Id ";" (*) ["::"]
    //   Import = Privacy "use" Path "as" Id ";" (*) ["macro_rules"]
    //   Import = Privacy "use" Path "as" Id ";" (*) ["mod"]
    //   Import = Privacy "use" Path "as" Id ";" (*) ["pub"]
    //   Import = Privacy "use" Path "as" Id ";" (*) ["self"]
    //   Import = Privacy "use" Path "as" Id ";" (*) ["struct"]
    //   Import = Privacy "use" Path "as" Id ";" (*) ["use"]
    //   Import = Privacy "use" Path "as" Id ";" (*) ["{"]
    //   Import = Privacy "use" Path "as" Id ";" (*) ["}"]
    //   Import = Privacy "use" Path "as" Id ";" (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "::" -> Reduce(Import = Privacy, "use", Path, "as", Id, ";" => ActionFn(37);)
    //   "macro_rules" -> Reduce(Import = Privacy, "use", Path, "as", Id, ";" => ActionFn(37);)
    //   "mod" -> Reduce(Import = Privacy, "use", Path, "as", Id, ";" => ActionFn(37);)
    //   "pub" -> Reduce(Import = Privacy, "use", Path, "as", Id, ";" => ActionFn(37);)
    //   "self" -> Reduce(Import = Privacy, "use", Path, "as", Id, ";" => ActionFn(37);)
    //   "struct" -> Reduce(Import = Privacy, "use", Path, "as", Id, ";" => ActionFn(37);)
    //   "use" -> Reduce(Import = Privacy, "use", Path, "as", Id, ";" => ActionFn(37);)
    //   "{" -> Reduce(Import = Privacy, "use", Path, "as", Id, ";" => ActionFn(37);)
    //   "}" -> Reduce(Import = Privacy, "use", Path, "as", Id, ";" => ActionFn(37);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(Import = Privacy, "use", Path, "as", Id, ";" => ActionFn(37);)
    //
    pub fn __state120<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<PathId>,
        __sym3: &mut Option<&'input str>,
        __sym4: &mut Option<InternedString>,
        __sym5: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __sym4 = __sym4.take().unwrap();
                let __sym5 = __sym5.take().unwrap();
                let __nt = super::__action37(krate, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Import(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 121
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ "}" (*) ["::"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ "}" (*) ["macro_rules"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ "}" (*) ["mod"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ "}" (*) ["pub"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ "}" (*) ["self"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ "}" (*) ["struct"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ "}" (*) ["use"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ "}" (*) ["{"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ "}" (*) ["}"]
    //   MacroDef = Privacy "macro_rules" "!" Id "{" Item+ "}" (*) [r#"[a-zA-Z_][a-zA-Z0-9_]*"#]
    //
    //   "::" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", Item+, "}" => ActionFn(45);)
    //   "macro_rules" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", Item+, "}" => ActionFn(45);)
    //   "mod" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", Item+, "}" => ActionFn(45);)
    //   "pub" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", Item+, "}" => ActionFn(45);)
    //   "self" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", Item+, "}" => ActionFn(45);)
    //   "struct" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", Item+, "}" => ActionFn(45);)
    //   "use" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", Item+, "}" => ActionFn(45);)
    //   "{" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", Item+, "}" => ActionFn(45);)
    //   "}" -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", Item+, "}" => ActionFn(45);)
    //   r#"[a-zA-Z_][a-zA-Z0-9_]*"# -> Reduce(MacroDef = Privacy, "macro_rules", "!", Id, "{", Item+, "}" => ActionFn(45);)
    //
    pub fn __state121<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        krate: &mut Krate,
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<Privacy>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<&'input str>,
        __sym3: &mut Option<InternedString>,
        __sym4: &mut Option<&'input str>,
        __sym5: &mut Option<::std::vec::Vec<ItemId>>,
        __sym6: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>), __ParseError<usize,(usize, &'input str),()>>
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (2, _), _)) |
            Some((_, (5, _), _)) |
            Some((_, (6, _), _)) |
            Some((_, (7, _), _)) |
            Some((_, (8, _), _)) |
            Some((_, (9, _), _)) |
            Some((_, (10, _), _)) |
            Some((_, (11, _), _)) |
            Some((_, (12, _), _)) |
            Some((_, (13, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __sym4 = __sym4.take().unwrap();
                let __sym5 = __sym5.take().unwrap();
                let __sym6 = __sym6.take().unwrap();
                let __nt = super::__action45(krate, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, &__lookbehind, &__lookahead);
                return Ok((__lookbehind, __lookahead, __Nonterminal::MacroDef(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }
}
pub use self::__parse__Krate::parse_Krate;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '!' => {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        '*' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        ':' => {
                            __current_state = 3;
                            continue;
                        }
                        ';' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '{' => {
                            __current_match = Some((11, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        '}' => {
                            __current_match = Some((12, __index + 1));
                            __current_state = 12;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        ':' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 14;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        's' => {
                            __current_match = Some((4, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 18;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 19;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 20;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 21;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                11 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                12 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                13 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                14 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                15 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                16 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                17 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 23;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                18 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((7, __index + 1));
                            __current_state = 24;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                19 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                20 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                21 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((10, __index + 1));
                            __current_state = 27;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                22 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 28;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                23 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                24 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                25 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((8, __index + 1));
                            __current_state = 29;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                26 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                27 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                28 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 31;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                29 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                30 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 32;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                31 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 33;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                32 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        't' => {
                            __current_match = Some((9, __index + 1));
                            __current_state = 34;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                33 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 35;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                34 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                35 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 36;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                36 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 37;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                37 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 38;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                38 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        's' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 39;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                39 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'A' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'B' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'C' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'D' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'E' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'F' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'G' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'H' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'I' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'J' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'K' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'L' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'M' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'N' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'O' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'P' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'R' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'S' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'T' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'U' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'V' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'W' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'X' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'Z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '_' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'a' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'b' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'c' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'd' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'e' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'f' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'g' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'h' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'i' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'j' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'k' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'l' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'm' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'n' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'o' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'p' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'q' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'r' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        's' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        't' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'u' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'v' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'w' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'x' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'y' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        'z' => {
                            __current_match = Some((13, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

pub fn __action0<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __0: (),
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ()
{
    (__0)
}

pub fn __action1<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    items: ::std::vec::Vec<ItemId>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ()
{
    krate.modules[ROOT_ID.0].items = items
}

pub fn __action2<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __0: ModuleId,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ItemId
{
    ItemId::Module(__0)
}

pub fn __action3<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __0: StructureId,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ItemId
{
    ItemId::Structure(__0)
}

pub fn __action4<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __0: ImportId,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ItemId
{
    ItemId::Import(__0)
}

pub fn __action5<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __0: GlobId,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ItemId
{
    ItemId::Glob(__0)
}

pub fn __action6<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __0: MacroDefId,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ItemId
{
    ItemId::MacroDef(__0)
}

pub fn __action7<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __0: MacroRefId,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ItemId
{
    ItemId::MacroRef(__0)
}

pub fn __action8<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __0: CodeId,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ItemId
{
    ItemId::Code(__0)
}

pub fn __action9<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    privacy: Privacy,
    _: &'input str,
    name: InternedString,
    _: &'input str,
    items: ::std::vec::Vec<ItemId>,
    _: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ModuleId
{
    krate.add_module(Module { privacy: privacy, name: name, items: items })
}

pub fn __action10<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    privacy: Privacy,
    _: &'input str,
    name: InternedString,
    _: &'input str,
    _: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> StructureId
{
    krate.add_structure(Structure { privacy: privacy, name: name })
}

pub fn __action11<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    privacy: Privacy,
    _: &'input str,
    path: PathId,
    a: ::std::option::Option<InternedString>,
    _: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ImportId
{
    krate.add_import(Import { privacy: privacy, path: path, alt_name: a })
}

pub fn __action12<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    privacy: Privacy,
    _: &'input str,
    path: PathId,
    _: &'input str,
    _: &'input str,
    _: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> GlobId
{
    krate.add_glob(Glob { privacy: privacy, path: path, })
}

pub fn __action13<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    privacy: Privacy,
    _: &'input str,
    _: &'input str,
    name: InternedString,
    _: &'input str,
    items: ::std::vec::Vec<ItemId>,
    _: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> MacroDefId
{
    krate.add_macro_def(MacroDef { privacy: privacy, name: name, items: items })
}

pub fn __action14<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    path: PathId,
    _: &'input str,
    _: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> MacroRefId
{
    krate.add_macro_ref(MacroRef { path: path })
}

pub fn __action15<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    _: &'input str,
    paths: ::std::vec::Vec<PathId>,
    _: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> CodeId
{
    krate.add_code(Code { paths: paths })
}

pub fn __action16<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    _: &'input str,
    _: &'input str,
    __0: InternedString,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> PathId
{
    krate.add_path(Path::Cons(THIS_PATH, __0))
}

pub fn __action17<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    _: &'input str,
    __0: InternedString,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> PathId
{
    krate.add_path(Path::Cons(ROOT_PATH, __0))
}

pub fn __action18<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __0: InternedString,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> PathId
{
    krate.add_path(Path::Cons(ROOT_PATH, __0))
}

pub fn __action19<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __0: PathId,
    _: &'input str,
    __1: InternedString,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> PathId
{
    krate.add_path(Path::Cons(__0, __1))
}

pub fn __action20<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __0: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Privacy
{
    Privacy::Pub
}

pub fn __action21<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __0: (),
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Privacy
{
    Privacy::Priv
}

pub fn __action22<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __0: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> InternedString
{
    intern(__0)
}

pub fn __action23<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ()
{
    ()
}

pub fn __action24<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ::std::vec::Vec<PathId>
{
    vec![]
}

pub fn __action25<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    v: ::std::vec::Vec<PathId>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ::std::vec::Vec<PathId>
{
    v
}

pub fn __action26<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __0: PathId,
    _: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> PathId
{
    (__0)
}

pub fn __action27<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __0: InternedString,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ::std::option::Option<InternedString>
{
    Some(__0)
}

pub fn __action28<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ::std::option::Option<InternedString>
{
    None
}

pub fn __action29<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    _: &'input str,
    __0: InternedString,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> InternedString
{
    (__0)
}

pub fn __action30<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ::std::vec::Vec<ItemId>
{
    vec![]
}

pub fn __action31<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    v: ::std::vec::Vec<ItemId>,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ::std::vec::Vec<ItemId>
{
    v
}

pub fn __action32<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __0: ItemId,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ::std::vec::Vec<ItemId>
{
    vec![__0]
}

pub fn __action33<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    v: ::std::vec::Vec<ItemId>,
    e: ItemId,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ::std::vec::Vec<ItemId>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action34<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __0: PathId,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ::std::vec::Vec<PathId>
{
    vec![__0]
}

pub fn __action35<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    v: ::std::vec::Vec<PathId>,
    e: PathId,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ::std::vec::Vec<PathId>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action36<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __0: &'input str,
    __1: InternedString,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ::std::option::Option<InternedString>
{
    let __temp0 = __action29(
        krate,
        input,
        __0,
        __1,
        __lookbehind,
        __lookahead,
    );
    __action27(
        krate,
        input,
        __temp0,
        __lookbehind,
        __lookahead,
    )
}

pub fn __action37<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __0: Privacy,
    __1: &'input str,
    __2: PathId,
    __3: &'input str,
    __4: InternedString,
    __5: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ImportId
{
    let __temp0 = __action36(
        krate,
        input,
        __3,
        __4,
        __lookbehind,
        __lookahead,
    );
    __action11(
        krate,
        input,
        __0,
        __1,
        __2,
        __temp0,
        __5,
        __lookbehind,
        __lookahead,
    )
}

pub fn __action38<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __0: Privacy,
    __1: &'input str,
    __2: PathId,
    __3: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ImportId
{
    let __temp0 = __action28(
        krate,
        input,
        __lookbehind,
        __lookahead,
    );
    __action11(
        krate,
        input,
        __0,
        __1,
        __2,
        __temp0,
        __3,
        __lookbehind,
        __lookahead,
    )
}

pub fn __action39<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> Privacy
{
    let __temp0 = __action23(
        krate,
        input,
        __lookbehind,
        __lookahead,
    );
    __action21(
        krate,
        input,
        __temp0,
        __lookbehind,
        __lookahead,
    )
}

pub fn __action40<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __0: PathId,
    __1: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ::std::vec::Vec<PathId>
{
    let __temp0 = __action26(
        krate,
        input,
        __0,
        __1,
        __lookbehind,
        __lookahead,
    );
    __action34(
        krate,
        input,
        __temp0,
        __lookbehind,
        __lookahead,
    )
}

pub fn __action41<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __0: ::std::vec::Vec<PathId>,
    __1: PathId,
    __2: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ::std::vec::Vec<PathId>
{
    let __temp0 = __action26(
        krate,
        input,
        __1,
        __2,
        __lookbehind,
        __lookahead,
    );
    __action35(
        krate,
        input,
        __0,
        __temp0,
        __lookbehind,
        __lookahead,
    )
}

pub fn __action42<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __0: &'input str,
    __1: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> CodeId
{
    let __temp0 = __action24(
        krate,
        input,
        __lookbehind,
        __lookahead,
    );
    __action15(
        krate,
        input,
        __0,
        __temp0,
        __1,
        __lookbehind,
        __lookahead,
    )
}

pub fn __action43<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __0: &'input str,
    __1: ::std::vec::Vec<PathId>,
    __2: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> CodeId
{
    let __temp0 = __action25(
        krate,
        input,
        __1,
        __lookbehind,
        __lookahead,
    );
    __action15(
        krate,
        input,
        __0,
        __temp0,
        __2,
        __lookbehind,
        __lookahead,
    )
}

pub fn __action44<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __0: Privacy,
    __1: &'input str,
    __2: &'input str,
    __3: InternedString,
    __4: &'input str,
    __5: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> MacroDefId
{
    let __temp0 = __action30(
        krate,
        input,
        __lookbehind,
        __lookahead,
    );
    __action13(
        krate,
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __5,
        __lookbehind,
        __lookahead,
    )
}

pub fn __action45<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __0: Privacy,
    __1: &'input str,
    __2: &'input str,
    __3: InternedString,
    __4: &'input str,
    __5: ::std::vec::Vec<ItemId>,
    __6: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> MacroDefId
{
    let __temp0 = __action31(
        krate,
        input,
        __5,
        __lookbehind,
        __lookahead,
    );
    __action13(
        krate,
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __6,
        __lookbehind,
        __lookahead,
    )
}

pub fn __action46<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __0: Privacy,
    __1: &'input str,
    __2: InternedString,
    __3: &'input str,
    __4: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ModuleId
{
    let __temp0 = __action30(
        krate,
        input,
        __lookbehind,
        __lookahead,
    );
    __action9(
        krate,
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
        __lookbehind,
        __lookahead,
    )
}

pub fn __action47<
    'input,
>(
    krate: &mut Krate,
    input: &'input str,
    __0: Privacy,
    __1: &'input str,
    __2: InternedString,
    __3: &'input str,
    __4: ::std::vec::Vec<ItemId>,
    __5: &'input str,
    __lookbehind: &Option<usize>,
    __lookahead: &Option<(usize, (usize, &'input str), usize)>,
) -> ModuleId
{
    let __temp0 = __action31(
        krate,
        input,
        __4,
        __lookbehind,
        __lookahead,
    );
    __action9(
        krate,
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __5,
        __lookbehind,
        __lookahead,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
