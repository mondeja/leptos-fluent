#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip_all))]
pub fn parse(header: &str) -> Vec<String> {
    let mut parsed_lang: Vec<_> = header
        .split(';')
        .map(|lang| {
            let mut langs = lang.split(',').peekable();
            let q = if let Some(a) = langs
                .peek()
                .and_then(|maybe_q| maybe_q.trim().strip_prefix("q="))
            {
                let q = a.parse::<f32>().unwrap_or(1.0);
                langs.next();
                q
            } else {
                1.0
            };
            (q, langs)
        })
        .collect();

    parsed_lang.sort_unstable_by(|a, b| b.0.total_cmp(&a.0));

    let result = parsed_lang
        .into_iter()
        .flat_map(|(_q, langs)| langs.map(str::trim).map(String::from))
        .collect();

    #[cfg(feature = "tracing")]
    tracing::trace!(
        "Parsed HTTP header \"{}\" into languages: {:?}",
        header,
        &result
    );

    result
}
