pub(crate) struct EvaluatedExprpath {
    pub(crate) result: bool,
    pub(crate) supported: bool,
}

/// Evaluates an expression path against the current compile-time environment.
pub(crate) fn evaluate_exprpath(
    exprpath: &proc_macro2::TokenStream,
) -> EvaluatedExprpath {
    let exprpath_string = exprpath.to_string();
    let mut exprpath_str = exprpath_string.as_str();
    exprpath_str = exprpath_str.strip_prefix("#[cfg(").unwrap_or(exprpath_str);
    exprpath_str = exprpath_str.strip_suffix(")]").unwrap_or(exprpath_str);

    // the next unwrap seems safe because we are parsing a valid expression
    // already checked by the compiler
    let expr = cfg_expr::Expression::parse(exprpath_str).unwrap();

    let mut supported = true;
    let result = expr.eval(|pred| {
        match pred {
            cfg_expr::Predicate::Test => cfg!(test),
            cfg_expr::Predicate::DebugAssertions => cfg!(debug_assertions),
            cfg_expr::Predicate::Target(target_predicate) => {
                if let Some(builtin_target) =
                    cfg_expr::targets::get_builtin_target_by_triple(
                        current_platform::CURRENT_PLATFORM,
                    )
                {
                    target_predicate.matches(builtin_target)
                } else {
                    supported = false;
                    false
                }
            }
            // accept only leptos-fluent-macro features
            cfg_expr::Predicate::Feature(f) => {
                if *f == "nightly" {
                    cfg!(feature = "nightly")
                } else if *f == "ssr" {
                    cfg!(feature = "ssr")
                } else if *f == "hydrate" {
                    cfg!(feature = "hydrate")
                } else if *f == "system" {
                    cfg!(feature = "system")
                } else if *f == "tracing" {
                    cfg!(feature = "tracing")
                } else if *f == "actix" {
                    cfg!(feature = "actix")
                } else if *f == "axum" {
                    cfg!(feature = "axum")
                } else if *f == "json" {
                    cfg!(feature = "json")
                } else if *f == "json5" {
                    cfg!(feature = "json5")
                } else if *f == "yaml" {
                    cfg!(feature = "yaml")
                } else if *f == "debug" {
                    cfg!(feature = "debug")
                } else {
                    supported = false;
                    false
                }
            }
            // proc_macro predicates are impossible to check because works at crate level
            // and we are inside the leptos-fluent-macro expansion
            //cfg_expr::Predicate::ProcMacro => unimplemented!(),
            // key-value predicates are impossible to check at compile
            // time because they are not constant expressions
            //cfg_expr::Predicate::TargetFeature(_) => unimplemented!(),
            //cfg_expr::Predicate::Flag(_) => unimplemented!(),
            //cfg_expr::Predicate::KeyValue { .. } => unimplemented!(),
            _ => {
                supported = false;
                false
            }
        }
    });

    EvaluatedExprpath { result, supported }
}
