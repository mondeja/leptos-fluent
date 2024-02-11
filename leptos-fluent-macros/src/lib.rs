extern crate proc_macro;

mod languages;

use languages::read_languages_file;
use proc_macro2::TokenStream;
use quote::quote;
use std::path::PathBuf;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input, token, Ident, Result,
};

fn parse_litstr_or_expr_param(
    fields: ParseStream,
    strlit: &mut Option<syn::LitStr>,
    expr: &mut Option<syn::Expr>,
    param_name: &'static str,
) -> Option<syn::Error> {
    match fields.parse::<syn::LitStr>() {
        Ok(lit) => {
            *strlit = Some(lit);
            None
        }
        Err(_) => match fields.parse::<syn::Expr>() {
            Ok(e) => {
                *expr = Some(e);
                None
            }
            Err(_) => Some(syn::Error::new(
                fields.span(),
                format!(
                    concat!(
                        "Not a valid value for '{}' of leptos_fluent! macro.",
                        " Must be a literal string or a valid expression.",
                        " Found {:?}",
                    ),
                    param_name, fields,
                ),
            )),
        },
    }
}

fn parse_litbool_or_expr_param(
    fields: ParseStream,
    litbool: &mut Option<syn::LitBool>,
    expr: &mut Option<syn::Expr>,
    param_name: &'static str,
) -> Option<syn::Error> {
    match fields.parse::<syn::LitBool>() {
        Ok(lit) => {
            *litbool = Some(lit);
            None
        }
        Err(_) => match fields.parse::<syn::Expr>() {
            Ok(e) => {
                *expr = Some(e);
                None
            }
            Err(_) => Some(syn::Error::new(
                fields.span(),
                format!(
                    concat!(
                        "Not a valid value for '{}' of leptos_fluent! macro.",
                        " Must be a literal boolean or a valid expression.",
                        " Found {:?}",
                    ),
                    param_name, fields,
                ),
            )),
        },
    }
}

struct I18nLoader {
    locales_ident: syn::Ident,
    languages_file: PathBuf,
    sync_html_tag_lang_bool: Option<syn::LitBool>,
    sync_html_tag_lang_expr: Option<syn::Expr>,
    initial_language_from_url_bool: Option<syn::LitBool>,
    initial_language_from_url_expr: Option<syn::Expr>,
    initial_language_from_url_param_str: Option<syn::LitStr>,
    initial_language_from_url_param_expr: Option<syn::Expr>,
    initial_language_from_url_to_localstorage_bool: Option<syn::LitBool>,
    initial_language_from_url_to_localstorage_expr: Option<syn::Expr>,
    initial_language_from_localstorage_bool: Option<syn::LitBool>,
    initial_language_from_localstorage_expr: Option<syn::Expr>,
    initial_language_from_navigator_bool: Option<syn::LitBool>,
    initial_language_from_navigator_expr: Option<syn::Expr>,
    localstorage_key_str: Option<syn::LitStr>,
    localstorage_key_expr: Option<syn::Expr>,
}

impl Parse for I18nLoader {
    fn parse(input: ParseStream) -> Result<Self> {
        let workspace_path = std::path::PathBuf::from(
            std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| "./".into()),
        );

        let fields;
        braced!(fields in input);
        let mut locales_identifier: Option<syn::Ident> = None;
        let mut languages_path: Option<syn::LitStr> = None;
        let mut sync_html_tag_lang_bool: Option<syn::LitBool> = None;
        let mut sync_html_tag_lang_expr: Option<syn::Expr> = None;
        let mut initial_language_from_url_bool: Option<syn::LitBool> = None;
        let mut initial_language_from_url_expr: Option<syn::Expr> = None;
        let mut initial_language_from_url_param_str: Option<syn::LitStr> = None;
        let mut initial_language_from_url_param_expr: Option<syn::Expr> = None;
        let mut initial_language_from_url_to_localstorage_bool: Option<
            syn::LitBool,
        > = None;
        let mut initial_language_from_url_to_localstorage_expr: Option<
            syn::Expr,
        > = None;
        let mut initial_language_from_localstorage_bool: Option<syn::LitBool> =
            None;
        let mut initial_language_from_localstorage_expr: Option<syn::Expr> =
            None;
        let mut initial_language_from_navigator_bool: Option<syn::LitBool> =
            None;
        let mut initial_language_from_navigator_expr: Option<syn::Expr> = None;
        let mut localstorage_key_str: Option<syn::LitStr> = None;
        let mut localstorage_key_expr: Option<syn::Expr> = None;

        while !fields.is_empty() {
            let k = fields.parse::<Ident>()?;
            fields.parse::<syn::Token![:]>()?;

            if k == "locales" {
                locales_identifier = Some(fields.parse()?);
            } else if k == "languages" {
                languages_path = Some(fields.parse()?);
            } else if k == "sync_html_tag_lang" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut sync_html_tag_lang_bool,
                    &mut sync_html_tag_lang_expr,
                    "sync_html_tag_lang",
                ) {
                    return Err(err);
                }
            } else if k == "initial_language_from_url" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut initial_language_from_url_bool,
                    &mut initial_language_from_url_expr,
                    "initial_language_from_url",
                ) {
                    return Err(err);
                }
            } else if k == "initial_language_from_url_param" {
                if let Some(err) = parse_litstr_or_expr_param(
                    &fields,
                    &mut initial_language_from_url_param_str,
                    &mut initial_language_from_url_param_expr,
                    "initial_language_from_url_param",
                ) {
                    return Err(err);
                }
            } else if k == "initial_language_from_url_to_localstorage" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut initial_language_from_url_to_localstorage_bool,
                    &mut initial_language_from_url_to_localstorage_expr,
                    "initial_language_from_url_to_localstorage",
                ) {
                    return Err(err);
                }
            } else if k == "initial_language_from_localstorage" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut initial_language_from_localstorage_bool,
                    &mut initial_language_from_localstorage_expr,
                    "initial_language_from_localstorage",
                ) {
                    return Err(err);
                }
            } else if k == "initial_language_from_navigator" {
                if let Some(err) = parse_litbool_or_expr_param(
                    &fields,
                    &mut initial_language_from_navigator_bool,
                    &mut initial_language_from_navigator_expr,
                    "initial_language_from_navigator",
                ) {
                    return Err(err);
                }
            } else if k == "localstorage_key" {
                if let Some(err) = parse_litstr_or_expr_param(
                    &fields,
                    &mut localstorage_key_str,
                    &mut localstorage_key_expr,
                    "localstorage_key",
                ) {
                    return Err(err);
                }
            } else {
                return Err(syn::Error::new(
                    k.span(),
                    "Not a valid parameter for leptos_fluent! macro.",
                ));
            }

            if fields.is_empty() {
                break;
            }
            fields.parse::<token::Comma>()?;
        }

        // languages
        let languages = languages_path.ok_or_else(|| {
            syn::Error::new(input.span(), "Missing `languages` field")
        })?;

        let languages_file = workspace_path.join(languages.value());

        if std::fs::metadata(&languages_file).is_err() {
            return Err(syn::Error::new(
                languages.span(),
                format!(
                    concat!(
                        "Couldn't read languages file, this path should",
                        " be relative to your crate's `Cargo.toml`.",
                        " Looking for: {:?}",
                    ),
                    languages_file,
                ),
            ));
        }

        let locales_ident = locales_identifier.ok_or_else(|| {
            syn::Error::new(input.span(), "Missing `locales` field")
        })?;

        Ok(Self {
            locales_ident,
            languages_file,
            sync_html_tag_lang_bool,
            sync_html_tag_lang_expr,
            initial_language_from_url_bool,
            initial_language_from_url_expr,
            initial_language_from_url_param_str,
            initial_language_from_url_param_expr,
            initial_language_from_url_to_localstorage_bool,
            initial_language_from_url_to_localstorage_expr,
            initial_language_from_localstorage_bool,
            initial_language_from_localstorage_expr,
            initial_language_from_navigator_bool,
            initial_language_from_navigator_expr,
            localstorage_key_str,
            localstorage_key_expr,
        })
    }
}

/// A macro to create the i18n context for internationalization.
///
/// # Example
///
/// ```rust,ignore
/// use fluent_templates::static_loader;
/// use leptos::*;
/// use leptos_fluent::leptos_fluent;
///
/// static_loader! {
///     static LOCALES = {
///         locales: "./locales",
///         fallback_language: "en-US",
///     };
/// }
///
/// #[component]
/// pub fn App() -> impl IntoView {
///     leptos_fluent! {{
///         locales: LOCALES,
///         languages: "./locales/languages.json",
///         sync_html_tag_lang: true,
///         initial_language_from_url: true,
///         initial_language_from_url_param: "lang",
///         initial_language_from_url_to_localstorage: true,
///         initial_language_from_localstorage: true,
///         initial_language_from_navigator: true,
///         localstorage_key: "language",
///     }};
///
///     view! {
///         ...
///     }
/// }
/// ```
///
/// ## Arguments
///
/// - **`locales` \***: The locales to be used by the application. This should be the
///   same identifier used in the [`fluent_templates::static_loader!`] macro, which
///   returns [`once_cell:sync::Lazy`]`<`[`StaticLoader`]`>`.
/// - **`languages` \***: The path to the languages file, which should be a JSON
///   array of arrays, where each inner array contains the language identifier and
///   the language name, respectively. The language identifier should be a valid
///   language tag, such as `en-US`, `es-ES`, `en`, `es`, etc.
///   ```json
///   [
///     ["en-US", "English (United States)"],
///     ["es-ES", "Español (España)"]
///   ]
///   ```
/// - **`sync_html_tag_lang`** (_`false`_): Either to synchronize the
///   [`<html lang="...">` attribute] with the current language using [`leptos::create_effect`].
///   Can be a literal boolean or an expression that will be evaluated at runtime.
/// - **`initial_language_from_url`** (_`false`_): Either to load the initial language of the user
///   from a URL parameter. Can be a literal boolean or an expression that will be evaluated at
///   runtime.
/// - **`initial_language_from_url_param`** (_`"lang"`_): The parameter name to look for the initial
///   language in the URL. Can be a literal string or an expression that will be evaluated at
///   runtime.
/// - **`initial_language_from_url_to_localstorage`** (_`false`_): Either to save the initial language
///   of the user from the URL to [local storage]. Can be a literal boolean or an expression that will
///   be evaluated at runtime.
/// - **`initial_language_from_localstorage`** (_`false`_): Either to load the initial language of the
///   user from [local storage] if not found in the URL param. Can be a literal boolean or an expression
///   that will be evaluated at runtime.
/// - **`initial_language_from_navigator`** (_`false`_): Either to load the initial language of the user
///   from Can be a literal boolean or an expression that will be evaluated at
///   runtime.
///   [`navigator.languages`]
///   if not found in [local storage].
/// - **`localstorage_key`** (_`"lang"`_): The [local storage] field to get and save the current language
///   of the user. Can be a literal string or an expression that will be evaluated at runtime.
///
/// [`fluent_templates::static_loader!`]: https://docs.rs/fluent-templates/0.8.0/fluent_templates/macro.static_loader.html
/// [`once_cell:sync::Lazy`]: https://docs.rs/once_cell/latest/once_cell/sync/struct.Lazy.html
/// [`StaticLoader`]: https://docs.rs/fluent-templates/0.8.0/fluent_templates/struct.StaticLoader.html
/// [`<html lang="...">` attribute]: https://developer.mozilla.org/es/docs/Web/HTML/Global_attributes/lang
/// [local storage]: https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage
/// [`navigator.languages`]: https://developer.mozilla.org/en-US/docs/Web/API/Navigator/languages
/// [`leptos::create_effect`]: https://docs.rs/leptos/latest/leptos/fn.create_effect.html
#[proc_macro]
pub fn leptos_fluent(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let I18nLoader {
        locales_ident,
        languages_file,
        sync_html_tag_lang_bool,
        sync_html_tag_lang_expr,
        initial_language_from_url_bool,
        initial_language_from_url_expr,
        initial_language_from_url_param_str,
        initial_language_from_url_param_expr,
        initial_language_from_url_to_localstorage_bool,
        initial_language_from_url_to_localstorage_expr,
        initial_language_from_localstorage_bool,
        initial_language_from_localstorage_expr,
        initial_language_from_navigator_bool,
        initial_language_from_navigator_expr,
        localstorage_key_str,
        localstorage_key_expr,
    } = parse_macro_input!(input as I18nLoader);

    let languages = read_languages_file(&languages_file);
    let n_languages = languages.len();

    let languages_quote = format!(
        "[{}]",
        languages
            .iter()
            .map(|(id, name)| {
                format!(
                    concat!(
                        "&::leptos_fluent::Language{{",
                        " id: ::unic_langid::langid!(\"{}\"),",
                        " name: \"{}\"",
                        " }}",
                    ),
                    id, name
                )
            })
            .collect::<Vec<String>>()
            .join(",")
    )
    .parse::<TokenStream>()
    .unwrap();

    let sync_html_tag_lang_quote = match sync_html_tag_lang_bool {
        Some(lit) => match lit.value {
            true => quote! {
                use leptos::wasm_bindgen::JsCast;
                ::leptos::create_effect(move |_| ::leptos::document()
                    .document_element()
                    .unwrap()
                    .unchecked_into::<::leptos::web_sys::HtmlElement>()
                    .set_attribute(
                        "lang",
                        &::leptos::expect_context::<::leptos_fluent::I18n>().language.get().id.to_string()
                    )
                );
            },
            false => quote! {},
        },
        None => match sync_html_tag_lang_expr {
            Some(expr) => quote! {
                use leptos::wasm_bindgen::JsCast;
                if #expr {
                    ::leptos::create_effect(move |_| ::leptos::document()
                        .document_element()
                        .unwrap()
                        .unchecked_into::<::leptos::web_sys::HtmlElement>()
                        .set_attribute(
                            "lang",
                            &::leptos::expect_context::<::leptos_fluent::I18n>().language.get().id.to_string()
                        )
                    );
                }
            },
            None => quote! {},
        },
    };

    let initial_language_from_url_bool_value = initial_language_from_url_bool
        .as_ref()
        .map(|lit| lit.clone().value);

    let initial_language_from_url = match initial_language_from_url_bool {
        Some(lit) => quote! { #lit },
        None => match initial_language_from_url_expr {
            Some(expr) => quote! { #expr },
            None => quote! { false },
        },
    };

    let initial_language_from_localstorage_bool_value =
        initial_language_from_localstorage_bool
            .as_ref()
            .map(|lit| lit.clone().value);
    let initial_language_from_url_to_localstorage =
        match initial_language_from_url_to_localstorage_bool {
            Some(lit) => quote! { #lit },
            None => match initial_language_from_url_to_localstorage_expr {
                Some(expr) => quote! { #expr },
                None => quote! { false },
            },
        };

    let initial_language_from_localstorage =
        match initial_language_from_localstorage_bool {
            Some(lit) => quote! { #lit },
            None => match initial_language_from_localstorage_expr {
                Some(expr) => quote! { #expr },
                None => quote! { false },
            },
        };

    let initial_language_from_navigator_bool_value =
        initial_language_from_navigator_bool
            .as_ref()
            .map(|lit| lit.clone().value);
    let initial_language_from_navigator =
        match initial_language_from_navigator_bool {
            Some(lit) => quote! { #lit },
            None => match initial_language_from_navigator_expr {
                Some(expr) => quote! { #expr },
                None => quote! { false },
            },
        };

    let initial_language_from_url_param =
        match initial_language_from_url_param_str {
            Some(lit) => quote! { #lit },
            None => match initial_language_from_url_param_expr {
                Some(expr) => quote! { #expr },
                None => quote! { "lang" },
            },
        };

    let localstorage_key = match localstorage_key_str {
        Some(lit) => quote! { #lit },
        None => match localstorage_key_expr {
            Some(expr) => quote! { #expr },
            None => quote! { "lang" },
        },
    };

    let initial_language_from_url_quote =
        match initial_language_from_url_bool_value {
            Some(value) => match value {
                true => quote! {
                    if let Some(l) = ::leptos_fluent::url::get(#initial_language_from_url_param)
                    {
                        lang = i18n.language_from_str(&l);
                        if let Some(l) = lang {
                            if #initial_language_from_url_to_localstorage {
                                ::leptos_fluent::localstorage::set(
                                    #localstorage_key,
                                    &l.id.to_string(),
                                );
                            }
                        }
                    }
                },
                false => quote! {},
            },
            None => quote! {
                if #initial_language_from_url {
                    if let Some(l) = ::leptos_fluent::url::get(#initial_language_from_url_param)
                    {
                        lang = i18n.language_from_str(&l);
                        if let Some(l) = lang {
                            if #initial_language_from_url_to_localstorage {
                                ::leptos_fluent::localstorage::set(
                                    #localstorage_key,
                                    &l.id.to_string(),
                                );
                            }
                        }
                    }
                }
            },
        };

    let initial_language_from_localstorage_quote =
        match initial_language_from_localstorage_bool_value {
            Some(value) => match value {
                true => quote! {
                    if lang.is_none() {
                        if let Some(l) = ::leptos_fluent::localstorage::get(#localstorage_key)
                        {
                            lang = i18n.language_from_str(&l);
                        }
                    }
                },
                false => quote! {},
            },
            None => quote! {
                if #initial_language_from_localstorage && lang.is_none() {
                    if let Some(l) = ::leptos_fluent::localstorage::get(#localstorage_key)
                    {
                        lang = i18n.language_from_str(&l);
                    }
                }
            },
        };

    let initial_language_from_navigator_quote =
        match initial_language_from_navigator_bool_value {
            Some(value) => match value {
                true => quote! {
                    if lang.is_none() {
                        let languages = window().navigator().languages().to_vec();
                        for raw_language in languages {
                            let language = raw_language.as_string();
                            if language.is_none() {
                                continue;
                            }
                            if let Some(l) = i18n.language_from_str(&language.unwrap())
                            {
                                lang = Some(l);
                                break;
                            }
                        }
                    }
                },
                false => quote! {},
            },
            None => quote! {
                if #initial_language_from_navigator && lang.is_none() {
                    let languages = window().navigator().languages().to_vec();
                    for raw_language in languages {
                        let language = raw_language.as_string();
                        if language.is_none() {
                            continue;
                        }
                        if let Some(l) = i18n.language_from_str(&language.unwrap())
                        {
                            lang = Some(l);
                            break;
                        }
                    }
                }
            },
        };

    let initial_language_quote = quote! {
        let mut lang: Option<&'static ::leptos_fluent::Language> = None;
        let i18n = expect_context::<::leptos_fluent::I18n>();
        #initial_language_from_url_quote;
        #initial_language_from_localstorage_quote;
        #initial_language_from_navigator_quote;
        if let Some(l) = lang {
            i18n.language.set(l);
        }
    };

    let quote = quote! {
        {
            const LANGUAGES: [
                &::leptos_fluent::Language; #n_languages
            ] = #languages_quote;
            let i18n = ::leptos_fluent::I18n {
                language: ::std::rc::Rc::new(::leptos::create_rw_signal(LANGUAGES[0])),
                languages: &LANGUAGES,
                locales: &#locales_ident,
                localstorage_key: #localstorage_key,
            };
            provide_context::<::leptos_fluent::I18n>(i18n);
            #initial_language_quote;
            #sync_html_tag_lang_quote;
            i18n
        }
    };

    // println!("{}", quote);
    proc_macro::TokenStream::from(quote)
}
