#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip_all))]
pub fn parse(header: &str) -> Vec<String> {
    let mut entries: Vec<(f32, String)> = header
        .split(',')
        .filter_map(|raw| {
            let mut parts = raw.trim().split(';');
            let tag = parts.next()?.trim();
            if tag.is_empty() {
                return None;
            }

            let mut quality = 1.0_f32;
            for part in parts {
                if let Some(value) = part.trim().strip_prefix("q=") {
                    quality = value.parse().unwrap_or({
                        #[cfg(feature = "tracing")]
                        tracing::trace!(
                            "Invalid quality value {:?} in Accept-Language header \"{}\": {:?}",
                            value,
                            header,
                            _error
                        );
                        1.0
                    });
                }
            }

            Some((quality, tag.to_string()))
        })
        .collect();

    entries.sort_unstable_by(|a, b| b.0.total_cmp(&a.0));

    let result = entries.into_iter().map(|(_, tag)| tag).collect();

    #[cfg(feature = "tracing")]
    tracing::trace!(
        "Parsed HTTP header \"{}\" into languages: {:?}",
        header,
        &result
    );

    result
}
