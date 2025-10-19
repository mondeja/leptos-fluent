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
pub fn e2e_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
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
        (
            quote!(),
            quote! {
                let world = ::end2end_ssr_helpers::World::from_server_pid(
                    ::end2end_ssr_helpers::init_server(#fn_name_str).await,
                ).await;
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
                world.driver().clone().quit().await?;
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
