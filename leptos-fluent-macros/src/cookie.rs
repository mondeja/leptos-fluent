/// Validates a string with cookie attributes.
#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace"))]
pub(crate) fn validate_cookie_attrs(cookie_attrs: &str) -> Vec<String> {
    let mut errors = Vec::new();
    let attrs = cookie_attrs.split(';');
    for attr in attrs {
        let attr = attr.trim();
        if attr.is_empty() {
            continue;
        }
        let key;
        let value;
        if attr.contains('=') {
            let attr = attr.split('=').collect::<Vec<_>>();
            key = attr[0];
            value = attr[1];
        } else {
            key = attr;
            value = "";
        }

        match key.to_ascii_lowercase().as_str() {
            "samesite" => {
                if !["strict", "lax", "none"]
                    .contains(&value.to_ascii_lowercase().as_str())
                {
                    errors.push(format!(
                        concat!(
                            "Invalid SameSite value: {}.",
                            " Must be Strict, Lax, or None.",
                        ),
                        value,
                    ));
                }
            }
            "secure" => {
                if !value.is_empty() {
                    errors.push(
                        "Secure attribute does not take a value".to_string(),
                    );
                }
            }
            "httponly" => {
                if !value.is_empty() {
                    errors.push(
                        "HttpOnly attribute does not take a value".to_string(),
                    );
                }
            }
            "domain" => {
                if value.is_empty() {
                    errors
                        .push("Domain attribute must have a value".to_string());
                }
            }
            "path" => {
                if value.is_empty() {
                    errors.push("Path attribute must have a value".to_string());
                }
            }
            "max-age" => {
                if value.parse::<i64>().is_err() {
                    errors
                        .push("Max-Age attribute must be a number".to_string());
                }
            }
            "expires" => {
                if value.is_empty() {
                    errors.push(
                        "Expires attribute must have a value".to_string(),
                    );
                }
            }
            "partitioned" => {
                if !value.is_empty() {
                    errors.push(
                        "Partitioned attribute does not take a value"
                            .to_string(),
                    );
                }
            }
            value => {
                let valid_attributes = [
                    "SameSite",
                    "Secure",
                    "HttpOnly",
                    "Domain",
                    "Path",
                    "Max-Age",
                    "Expires",
                    "Partitioned",
                ];
                errors.push(format!(
                    concat!(
                        "Invalid cookie attribute: '{}'.\n",
                        "  Valid attributes are: {}.",
                    ),
                    value,
                    valid_attributes.join(", "),
                ));
            }
        }
    }

    #[cfg(feature = "tracing")]
    if !errors.is_empty() {
        tracing::warn!("Cookie attributes validation errors: {:?}", errors);
    }

    errors
}
