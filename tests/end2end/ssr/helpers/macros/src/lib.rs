use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

fn discover_tests() -> Vec<String> {
    let tests_filter = std::env::var("TESTS").unwrap_or_default();
    let test_names = if tests_filter.is_empty() {
        vec![]
    } else {
        tests_filter
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    };
    test_names
}

#[proc_macro_attribute]
pub fn e2e_test(_args: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn = parse_macro_input!(item as ItemFn);
    let fn_name = item_fn.sig.ident;
    let fn_name_str = fn_name.to_string();
    let fn_body = item_fn.block;

    let test_names = discover_tests();
    let is_ignored =
        !test_names.is_empty() && !test_names.contains(&fn_name_str);
    let (ignore_attr, setup_quote, body_quote, teadown_quote) = if is_ignored {
        (quote!(#[ignore]), quote!(), quote!(Ok(())), quote!())
    } else {
        // Inject the world as a pytest fixture
        let maybe_world_param =
            extract_world_param_from_fn_sig_inputs(&item_fn.sig.inputs);
        let mut setup_world_quote = quote!();
        let mut setup_driver_quote = quote!();
        let mut teardown_driver_quote = quote!();
        if let Some((world_ident, with_driver)) = maybe_world_param {
            setup_world_quote = quote! {
                let #world_ident: ::end2end_ssr_helpers::World = ::end2end_ssr_helpers::World::new(__server_pid);
            };
            if with_driver {
                setup_driver_quote = quote! {
                    let #world_ident: ::end2end_ssr_helpers::WorldWithDriver = #world_ident.with_driver().await;
                };
                teardown_driver_quote = quote! {
                    #world_ident.driver().clone().quit().await?;
                };
            }
        }
        (
            quote!(),
            quote! {
                let __server_pid = ::end2end_ssr_helpers::init_server(#fn_name_str).await;
                #setup_world_quote
                #setup_driver_quote
            },
            quote! {
                use thirtyfour::prelude::*;
                use futures_util::future::FutureExt;
                let __body_result = std::panic::AssertUnwindSafe(async {
                    (async {
                        #fn_body;
                        Ok::<(), anyhow::Error>(())
                    }).await
                })
                .catch_unwind()
                .await;
            },
            quote! {
                #teardown_driver_quote
                ::end2end_ssr_helpers::terminate_server(world.server_pid()).await;
                match __body_result {
                    Ok(Ok(())) => Ok(()),
                    Ok(Err(e)) => Err(e),
                    Err(p) => std::panic::resume_unwind(p),
                }
            },
        )
    };

    // Generamos un wrapper condicional
    let result = quote! {
        #ignore_attr
        #[::tokio::test]
        #[::serial_test::serial]
        async fn #fn_name() -> anyhow::Result<()> {
            #setup_quote
            #body_quote
            #teadown_quote
        }
    };

    result.into()
}

/// Extract a `World` or `WorldWithDriver` parameter from function signature inputs.
fn extract_world_param_from_fn_sig_inputs(
    sig_inputs: &syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
) -> Option<(&syn::Ident, bool)> {
    for input in sig_inputs {
        if let syn::FnArg::Typed(pat_type) = input {
            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                if let syn::Type::Path(type_path) = &*pat_type.ty {
                    if let Some(last_segment) = type_path.path.segments.last() {
                        if last_segment.ident == "WorldWithDriver" {
                            return Some((&pat_ident.ident, true));
                        } else if last_segment.ident == "World" {
                            return Some((&pat_ident.ident, false));
                        }
                    }
                }
            }
        }
    }
    None
}
