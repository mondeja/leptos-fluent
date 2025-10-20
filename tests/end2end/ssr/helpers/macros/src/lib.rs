use proc_macro::TokenStream;
use quote::{quote, ToTokens};
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
        let body_as_string = fn_body.clone().into_token_stream().to_string();

        // The world needs the driver if:
        //   a) the body contains "driver()"
        //   b) the function signature contains an argument named `world` of type `WorldWithDriver`
        let world_needs_driver = body_as_string.contains("driver()")
            || fn_signature_contains_world_with_driver(&item_fn.sig.inputs);
        let (setup_driver_quote, teardown_driver_quote) = if world_needs_driver
        {
            (
                quote! {
                    let world: ::end2end_ssr_helpers::WorldWithDriver = world.with_driver().await;
                },
                quote! {
                    world.driver().clone().quit().await?;
                },
            )
        } else {
            (quote!(), quote!())
        };
        (
            quote!(),
            quote! {
                let __server_pid = ::end2end_ssr_helpers::init_server(#fn_name_str).await;
                let world: ::end2end_ssr_helpers::World = ::end2end_ssr_helpers::World::new(__server_pid);
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

fn fn_signature_contains_world_with_driver(
    sig_inputs: &syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
) -> bool {
    for input in sig_inputs {
        // check if the argument is named `world` and is of type `WorldWithDriver`
        if let syn::FnArg::Typed(pat_type) = input {
            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                if pat_ident.ident == "world" {
                    if let syn::Type::Path(type_path) = &*pat_type.ty {
                        if let Some(last_segment) =
                            type_path.path.segments.last()
                        {
                            if last_segment.ident == "WorldWithDriver" {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    false
}
