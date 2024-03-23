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

    parsed_lang
        .into_iter()
        .flat_map(|(_q, langs)| langs.map(str::trim).map(String::from))
        .collect()
}
