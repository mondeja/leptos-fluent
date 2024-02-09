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

struct I18nLoader {
    locales_ident: syn::Ident,
    languages_file: PathBuf,
    sync_html_tag_lang: bool,
    initial_language_from_url: bool,
    initial_language_from_url_param: syn::LitStr,
    initial_language_from_url_to_localstorage: bool,
    initial_language_from_localstorage: bool,
    initial_language_from_navigator: bool,
    localstorage_key: syn::LitStr,
}

impl Parse for I18nLoader {
    fn parse(input: ParseStream) -> Result<Self> {
        let workspace_path = std::path::PathBuf::from(
            std::env::var("CARGO_MANIFEST_DIR")
                .unwrap_or_else(|_| String::from("./")),
        );

        let fields;
        braced!(fields in input);
        let mut locales_identifier: Option<syn::Ident> = None;
        let mut languages_path: Option<syn::LitStr> = None;
        let mut sync_html_tag_lang_litbool: Option<syn::LitBool> = None;
        let mut initial_language_from_url_litbool: Option<syn::LitBool> = None;
        let mut initial_language_from_url_param: Option<syn::LitStr> = None;
        let mut initial_language_from_url_to_localstorage_litbool: Option<
            syn::LitBool,
        > = None;
        let mut initial_language_from_localstorage: Option<syn::LitBool> = None;
        let mut initial_language_from_navigator: Option<syn::LitBool> = None;
        let mut localstorage_key: Option<syn::LitStr> = None;

        while !fields.is_empty() {
            let k = fields.parse::<Ident>()?;
            fields.parse::<syn::Token![:]>()?;

            if k == "locales" {
                locales_identifier = Some(fields.parse()?);
            } else if k == "languages" {
                languages_path = Some(fields.parse()?);
            } else if k == "sync_html_tag_lang" {
                sync_html_tag_lang_litbool = Some(fields.parse()?);
            } else if k == "initial_language_from_url" {
                initial_language_from_url_litbool = Some(fields.parse()?);
            } else if k == "initial_language_from_url_param" {
                initial_language_from_url_param = Some(fields.parse()?);
            } else if k == "initial_language_from_url_to_localstorage" {
                initial_language_from_url_to_localstorage_litbool =
                    Some(fields.parse()?);
            } else if k == "initial_language_from_localstorage" {
                initial_language_from_localstorage = Some(fields.parse()?);
            } else if k == "initial_language_from_navigator" {
                initial_language_from_navigator = Some(fields.parse()?);
            } else if k == "localstorage_key" {
                localstorage_key = Some(fields.parse()?);
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
            sync_html_tag_lang: match sync_html_tag_lang_litbool {
                Some(lit) => lit.value,
                None => false,
            },
            initial_language_from_url: match initial_language_from_url_litbool {
                Some(lit) => lit.value,
                None => false,
            },
            initial_language_from_url_param:
                match initial_language_from_url_param {
                    Some(lit) => lit,
                    None => {
                        syn::LitStr::new("lang", proc_macro2::Span::call_site())
                    }
                },
            initial_language_from_url_to_localstorage:
                match initial_language_from_url_to_localstorage_litbool {
                    Some(lit) => lit.value,
                    None => false,
                },
            initial_language_from_localstorage:
                match initial_language_from_localstorage {
                    Some(lit) => lit.value,
                    None => false,
                },
            initial_language_from_navigator:
                match initial_language_from_navigator {
                    Some(lit) => lit.value,
                    None => false,
                },
            localstorage_key: localstorage_key.unwrap_or_else(|| {
                syn::LitStr::new("lang", proc_macro2::Span::call_site())
            }),
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
///     let ctx = leptos_fluent! {{
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
///     ctx.provide_context(None);
///
///     view! {
///         ...
///     }
/// }
/// ```
///
/// The `LOCALES` returned by `static_loader!` is injected into the [`leptos_fluent!`]
/// macro to provide the transations to the i18n context.
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
/// - **`initial_language_from_url`** (_`false`_): Either to load the initial language of the user
///   from a URL parameter.
/// - **`initial_language_from_url_param`** (_`"lang"`_): The parameter name to look for the initial
///   language in the URL.
/// - **`initial_language_from_url_to_localstorage`** (_`false`_): Either to save the initial language
///   of the user from the URL to [local storage].
/// - **`initial_language_from_localstorage`** (_`false`_): Either to load the initial language of the
///   user from [local storage] if not found in the URL param.
/// - **`initial_language_from_navigator`** (_`false`_): Either to load the initial language of the user
///   from
///   [`navigator.languages`]
///   if not found in [local storage].
/// - **`localstorage_key`** (_`"lang"`_): The [local storage] field to get and save the current language
///   of the user.
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
        sync_html_tag_lang,
        initial_language_from_url,
        initial_language_from_url_param,
        initial_language_from_url_to_localstorage,
        initial_language_from_localstorage,
        initial_language_from_navigator,
        localstorage_key,
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

    let sync_html_tag_lang_quote = if sync_html_tag_lang {
        quote! {
            use leptos::wasm_bindgen::JsCast;
            ::leptos::create_effect(move |_| ::leptos::document()
                .document_element()
                .unwrap()
                .unchecked_into::<::web_sys::HtmlElement>()
                .set_attribute(
                    "lang",
                    &::leptos::expect_context::<::leptos_fluent::I18n>().language.get().id.to_string()
                )
            );
        }
    } else {
        quote! {}
    };

    let quote = quote! {
        {
            const LANGUAGES: [
                &::leptos_fluent::Language; #n_languages
            ] = #languages_quote;
            let i18n = ::leptos_fluent::I18n {
                language: ::std::rc::Rc::new(
                    ::leptos_fluent::LanguageSignal(
                        ::leptos::create_rw_signal(LANGUAGES[0])
                    )
                ),
                languages: &LANGUAGES,
                locales: &#locales_ident,
                initial_language_from_url: #initial_language_from_url,
                initial_language_from_url_param: #initial_language_from_url_param,
                initial_language_from_url_to_localstorage: #initial_language_from_url_to_localstorage,
                initial_language_from_localstorage: #initial_language_from_localstorage,
                initial_language_from_navigator: #initial_language_from_navigator,
                localstorage_key: #localstorage_key,
            };
            #sync_html_tag_lang_quote;
            i18n
        }
    };

    // println!("{}", quote);
    proc_macro::TokenStream::from(quote)
}
