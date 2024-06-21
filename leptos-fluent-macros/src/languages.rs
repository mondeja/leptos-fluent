use std::fs;
use std::path::PathBuf;

#[cfg(any(feature = "json", feature = "yaml", feature = "json5"))]
#[derive(serde::Deserialize)]
#[serde(untagged)]
enum LanguagesFileLanguage {
    CodeName(String, String),
    CodeNameDir(String, String, String),
}

#[cfg(any(feature = "json", feature = "yaml", feature = "json5"))]
fn set_dir_to_languages_from_languages_file(
    languages: &[LanguagesFileLanguage],
) -> Vec<(String, String, String)> {
    let mut locales = vec![];
    for tuple in languages {
        match tuple {
            LanguagesFileLanguage::CodeName(lang_code, lang_name) => {
                locales.push((
                    lang_code.to_owned(),
                    lang_name.to_owned(),
                    iso639_to_dir(&code_to_iso639(lang_code)).to_string(),
                ));
            }
            LanguagesFileLanguage::CodeNameDir(lang_code, lang_name, dir) => {
                locales.push((
                    lang_code.to_owned(),
                    lang_name.to_owned(),
                    dir.to_owned(),
                ));
            }
        }
    }
    locales
}

pub(crate) fn read_languages_file(
    path: &PathBuf,
) -> Result<Vec<(String, String, String)>, String> {
    #[cfg(feature = "json")]
    {
        let file_extension = path.extension().unwrap_or_default();
        if file_extension == "json" {
            match fs::read_to_string(path) {
                Ok(content) => {
                    match serde_json::from_str::<Vec<LanguagesFileLanguage>>(
                        content.as_str(),
                    ) {
                        Ok(languages) => {
                            Ok(set_dir_to_languages_from_languages_file(
                                &languages,
                            ))
                        }
                        Err(e) => Err(format!(
                            "Invalid JSON in languages file {}: {}",
                            path.to_string_lossy(),
                            e
                        )),
                    }
                }
                Err(e) => Err(format!(
                    "Couldn't read languages file {}: {}",
                    path.to_string_lossy(),
                    e,
                )),
            }
        } else {
            Err(format!(
                concat!(
                    "The languages file should be a JSON file because",
                    " you've enabled the 'json' feature.",
                    " Found file extension {:?}"
                ),
                file_extension
            ))
        }
    }

    #[cfg(all(not(feature = "json"), feature = "yaml"))]
    {
        let file_extension = path.extension().unwrap_or_default();
        if file_extension == "yaml" || file_extension == "yml" {
            match fs::read_to_string(path) {
                Ok(content) => {
                    match serde_yaml::from_str::<Vec<LanguagesFileLanguage>>(
                        content.as_str(),
                    ) {
                        Ok(languages) => {
                            Ok(set_dir_to_languages_from_languages_file(
                                &languages,
                            ))
                        }
                        Err(e) => Err(format!(
                            "Invalid YAML in languages file {}: {}",
                            path.to_string_lossy(),
                            e.to_string()
                        )),
                    }
                }
                Err(e) => Err(format!(
                    "Couldn't read languages file {}: {}",
                    path.to_string_lossy(),
                    e.to_string(),
                )),
            }
        } else {
            Err(format!(
                concat!(
                    "The languages file should be a YAML file because",
                    " you've enabled the 'yaml' feature.",
                    " Found file extension {:?}"
                ),
                file_extension
            ))
        }
    }

    #[cfg(all(
        not(any(feature = "json", feature = "yaml")),
        feature = "json5"
    ))]
    {
        let file_extension = path.extension().unwrap_or_default();
        if file_extension == "json5" {
            match fs::read_to_string(path) {
                Ok(content) => {
                    match json5::from_str::<Vec<LanguagesFileLanguage>>(
                        content.as_str(),
                    ) {
                        Ok(languages) => {
                            Ok(set_dir_to_languages_from_languages_file(
                                &languages,
                            ))
                        }
                        Err(e) => Err(format!(
                            "Invalid JSON5 in languages file {}: {}",
                            path.to_string_lossy(),
                            e.to_string()
                        )),
                    }
                }
                Err(e) => Err(format!(
                    "Couldn't read languages file {}: {}",
                    path.to_string_lossy(),
                    e.to_string(),
                )),
            }
        } else {
            Err(format!(
                concat!(
                    "The languages file should be a JSON5 file because",
                    " you've enabled the 'json5' feature.",
                    " Found file extension {:?}"
                ),
                file_extension
            ))
        }
    }

    #[cfg(not(any(feature = "json", feature = "yaml", feature = "json5")))]
    {
        _ = path;
        Err(concat!(
            "No feature enabled to read languages file.",
            " Enable either the 'json', 'yaml' or 'json5' feature.",
        )
        .to_string())
    }
}

pub(crate) fn read_locales_folder(
    path: &PathBuf,
) -> Vec<(String, String, String)> {
    let mut iso639_language_codes = vec![];
    let mut entries = vec![];
    for entry in fs::read_dir(path).expect("Couldn't read locales folder") {
        let entry = entry.expect("Couldn't read entry");
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let lang_code = path.file_name().unwrap().to_str().unwrap().to_string();
        iso639_language_codes.push(code_to_iso639(&lang_code));
        entries.push(entry);
    }

    let mut locales = vec![];
    for entry in entries {
        let lang_code = entry
            .path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let iso639_code = code_to_iso639(&lang_code);
        let use_country_code = iso639_language_codes
            .iter()
            .filter(|&c| c == &iso639_code)
            .count()
            > 1;
        let lang_name =
            language_name_from_language_code(&lang_code, use_country_code);
        let lang_dir = iso639_to_dir(&iso639_code);
        locales.push((lang_code, lang_name.to_string(), lang_dir.to_string()));
    }
    locales.sort_by(|a, b| a.1.cmp(&b.1));
    locales
}

pub(crate) fn build_languages_quote(
    languages: &[(String, String, String)],
) -> proc_macro2::TokenStream {
    format!(
        "[{}]",
        languages
            .iter()
            .map(|(id, name, dir)| generate_code_for_static_language(
                id, name, dir
            ))
            .collect::<Vec<String>>()
            .join(",")
    )
    .parse::<proc_macro2::TokenStream>()
    .unwrap()
}

fn generate_code_for_static_language(
    id: &str,
    name: &str,
    dir: &str,
) -> String {
    format!(
        concat!(
            "&::leptos_fluent::Language{{",
            "id: ::fluent_templates::loader::langid!(\"{}\"),",
            "name: \"{}\",",
            "dir: {}",
            "}}",
        ),
        id,
        name,
        match dir {
            "ltr" => "&::leptos_fluent::WritingDirection::Ltr",
            "rtl" => "&::leptos_fluent::WritingDirection::Rtl",
            _ => "&::leptos_fluent::WritingDirection::Auto",
        }
    )
}

fn code_to_iso639(code: &str) -> String {
    let splitter = if code.contains('_') {
        '_'
    } else if code.contains('-') {
        '-'
    } else {
        return code.to_lowercase();
    };
    code.split(splitter).collect::<Vec<&str>>()[0].to_string()
}

/// Convert an ISO-639 language code to a directionality string.
///
/// Taken from https://github.com/chladog/iso-639-1-dir
fn iso639_to_dir(code: &str) -> &'static str {
    match code {
        "aa" => "ltr",
        "ab" => "ltr",
        "ae" => "ltr",
        "af" => "ltr",
        "ak" => "ltr",
        "am" => "ltr",
        "an" => "ltr",
        "ar" => "rtl",
        "as" => "ltr",
        "av" => "ltr",
        "ay" => "ltr",
        "az" => "ltr",
        "ba" => "ltr",
        "be" => "ltr",
        "bg" => "ltr",
        "bi" => "ltr",
        "bm" => "auto",
        "bn" => "ltr",
        "bo" => "ltr",
        "br" => "ltr",
        "bs" => "ltr",
        "ca" => "ltr",
        "ce" => "ltr",
        "ch" => "ltr",
        "co" => "ltr",
        "cr" => "ltr",
        "cs" => "ltr",
        "cu" => "ltr",
        "cv" => "ltr",
        "cy" => "ltr",
        "da" => "ltr",
        "de" => "ltr",
        "dv" => "rtl",
        "dz" => "ltr",
        "ee" => "ltr",
        "el" => "ltr",
        "en" => "ltr",
        "eo" => "ltr",
        "es" => "ltr",
        "et" => "ltr",
        "eu" => "ltr",
        "fa" => "rtl",
        "ff" => "ltr",
        "fi" => "ltr",
        "fj" => "ltr",
        "fo" => "ltr",
        "fr" => "ltr",
        "fy" => "ltr",
        "ga" => "ltr",
        "gd" => "ltr",
        "gl" => "ltr",
        "gn" => "ltr",
        "gu" => "ltr",
        "gv" => "ltr",
        "ha" => "ltr",
        "he" => "rtl",
        "hi" => "ltr",
        "ho" => "ltr",
        "hr" => "ltr",
        "ht" => "ltr",
        "hu" => "ltr",
        "hy" => "ltr",
        "hz" => "ltr",
        "ia" => "ltr",
        "id" => "ltr",
        "ie" => "ltr",
        "ig" => "ltr",
        "ii" => "ltr",
        "ik" => "ltr",
        "io" => "ltr",
        "is" => "ltr",
        "it" => "ltr",
        "iu" => "ltr",
        "ja" => "auto", // (top to bottom)
        "jv" => "ltr",
        "ka" => "ltr",
        "kg" => "ltr",
        "ki" => "ltr",
        "kj" => "ltr",
        "kk" => "ltr",
        "kl" => "ltr",
        "km" => "ltr",
        "kn" => "ltr",
        "ko" => "auto", // (top to bottom)
        "kr" => "ltr",
        "ks" => "rtl",
        "ku" => "rtl",
        "kv" => "ltr",
        "kw" => "ltr",
        "ky" => "ltr",
        "la" => "ltr",
        "lb" => "ltr",
        "lg" => "ltr",
        "li" => "ltr",
        "ln" => "ltr",
        "lo" => "ltr",
        "lt" => "ltr",
        "lu" => "ltr",
        "lv" => "ltr",
        "mg" => "ltr",
        "mh" => "ltr",
        "mi" => "ltr",
        "mk" => "ltr",
        "ml" => "ltr",
        "mn" => "auto", // (top to bottom)
        "mr" => "ltr",
        "ms" => "ltr",
        "mt" => "ltr",
        "my" => "ltr",
        "na" => "ltr",
        "nb" => "ltr",
        "nd" => "ltr",
        "ne" => "ltr",
        "ng" => "ltr",
        "nl" => "ltr",
        "nn" => "ltr",
        "no" => "ltr",
        "nr" => "ltr",
        "nv" => "ltr",
        "ny" => "ltr",
        "oc" => "ltr",
        "oj" => "ltr",
        "om" => "ltr",
        "or" => "ltr",
        "os" => "ltr",
        "pa" => "rtl",
        "pi" => "ltr",
        "pl" => "ltr",
        "ps" => "rtl",
        "pt" => "ltr",
        "qu" => "ltr",
        "rm" => "ltr",
        "rn" => "ltr",
        "ro" => "ltr",
        "ru" => "ltr",
        "rw" => "ltr",
        "sa" => "ltr",
        "sc" => "ltr",
        "sd" => "rtl",
        "se" => "ltr",
        "sg" => "ltr",
        "si" => "ltr",
        "sk" => "ltr",
        "sl" => "ltr",
        "sm" => "ltr",
        "sn" => "ltr",
        "so" => "ltr",
        "sq" => "ltr",
        "sr" => "ltr",
        "ss" => "ltr",
        "st" => "ltr",
        "su" => "ltr",
        "sv" => "ltr",
        "sw" => "ltr",
        "ta" => "ltr",
        "te" => "ltr",
        "tg" => "ltr",
        "th" => "ltr",
        "ti" => "ltr",
        "tk" => "rtl",
        "tl" => "ltr",
        "tn" => "ltr",
        "to" => "ltr",
        "tr" => "ltr",
        "ts" => "ltr",
        "tt" => "ltr",
        "tw" => "ltr",
        "ty" => "ltr",
        "ug" => "rtl",
        "uk" => "ltr",
        "ur" => "rtl",
        "uz" => "ltr",
        "ve" => "ltr",
        "vi" => "auto", // (top to bottom)
        "vo" => "ltr",
        "wa" => "ltr",
        "wo" => "ltr",
        "xh" => "ltr",
        "yi" => "rtl",
        "yo" => "ltr",
        "za" => "auto", // (top to bottom)
        "zh" => "auto", // (top to bottom)
        "zu" => "ltr",
        _ => "auto",
    }
}

fn language_name_from_language_code(
    code: &str,
    use_country_code: bool,
) -> &'static str {
    if use_country_code {
        let c = code.to_string().to_lowercase().replace('_', "-");
        match c.as_str() {
            // lang (2 letter) -> country (2 letter)
            "af-na" => return "Afrikaans (Namibia)",
            "af-za" => return "Afrikaans (South Africa)",
            "ak-gh" => return "Akan (Ghana)",
            "am-et" => return "አማርኛ (ኢትዮጵያ)",
            "ar-ae" => return "العربية (الإمارات العربية المتحدة)",
            "ar-bh" => return "العربية (البحرين)",
            "ar-dj" => return "العربية (جيبوتي)",
            "ar-dz" => return "العربية (الجزائر)",
            "ar-eg" => return "العربية (مصر)",
            "ar-eh" => return "العربية (الصحراء الغربية)",
            "ar-er" => return "العربية (إريتريا)",
            "ar-il" => return "العربية (إسرائيل)",
            "ar-iq" => return "العربية (العراق)",
            "ar-jo" => return "العربية (الأردن)",
            "ar-km" => return "العربية (جزر القمر)",
            "ar-kw" => return "العربية (الكويت)",
            "ar-lb" => return "العربية (لبنان)",
            "ar-ly" => return "العربية (ليبيا)",
            "ar-ma" => return "العربية (المغرب)",
            "ar-mr" => return "العربية (موريتانيا)",
            "ar-om" => return "العربية (عمان)",
            "ar-ps" => return "العربية (فلسطين)",
            "ar-qa" => return "العربية (قطر)",
            "ar-sa" => return "العربية (المملكة العربية السعودية)",
            "ar-sd" => return "العربية (السودان)",
            "ar-so" => return "العربية (الصومال)",
            "ar-ss" => return "العربية (جنوب السودان)",
            "ar-sy" => return "العربية (سوريا)",
            "ar-td" => return "العربية (تشاد)",
            "ar-tn" => return "العربية (تونس)",
            "ar-ye" => return "العربية (اليمن)",
            "as-in" => return "অসমীয়া (ভাৰত)",
            "az-az" => return "Azərbaycan dili (Azərbaycan)",
            "ba-ru" => return "башҡорт теле (Россия)",
            "be-by" => return "Беларуская мова (Беларусь)",
            "bg-bg" => return "български език (България)",
            "bm-ml" => return "ߓߊߡߊߣߊߣߞߊߣ (ߞߊ߲ߞߊ߲)",
            "bn-bd" => return "বাংলা (বাংলাদেশ)",
            "bn-in" => return "বাংলা (ভারত)",
            "bo-cn" => return "བོད་སྐད་ (ཀྲུང་ཧྭ)",
            "bo-in" => return "བོད་སྐད་ (ཀྲུང་ཧྭ)", // TODO: The same as bo-cn, looks bad
            "br-fr" => return "Brezhoneg (Frañs)",
            "bs-ba" => return "Bosanski (Bosna i Hercegovina)",
            "ca-ad" => return "Català (Andorra)",
            "ca-es" => return "Català (Espanya)",
            "ca-fr" => return "Català (França)",
            "ca-it" => return "Català (Itàlia)",
            "ce-ru" => return "нохчийн мотт (Росси)",
            "co-fr" => return "Corsu (France)",
            "cs-cz" => return "Čeština (Česká republika)",
            "cv-ru" => return "чӑваш чӗлхи (Росси)",
            "cy-gb" => return "Cymraeg (Y Deyrnas Unedig)",
            "da-dk" => return "Dansk (Danmark)",
            "da-gl" => return "Dansk (Grønland)",
            "de-at" => return "Deutsch (Österreich)",
            "de-be" => return "Deutsch (Belgien)",
            "de-ch" => return "Deutsch (Schweiz)",
            "de-de" => return "Deutsch (Deutschland)",
            "de-it" => return "Deutsch (Italien)",
            "de-li" => return "Deutsch (Liechtenstein)",
            "de-lu" => return "Deutsch (Luxemburg)",
            "dv-mv" => return "ދިވެހި (ދިވެހި)",
            "dz-bt" => return "རྫོང་ཁ་ (འབྲུག་ཡུལ)",
            "ee-gh" => return "Eʋegbe (Ghana)",
            "ee-tg" => return "Eʋegbe (Togo)",
            "el-cy" => return "Νέα Ελληνικά (Κύπρος)",
            "el-gr" => return "Νέα Ελληνικά (Ελλάδα)",
            "en-ad" => return "English (Andorra)",
            "en-ae" => return "English (United Arab Emirates)",
            "en-ag" => return "English (Antigua and Barbuda)",
            "en-ai" => return "English (Anguilla)",
            "en-al" => return "English (Albania)",
            "en-ar" => return "English (Argentina)",
            "en-as" => return "English (American Samoa)",
            "en-at" => return "English (Austria)",
            "en-au" => return "English (Australia)",
            "en-ba" => return "English (Bosnia and Herzegovina)",
            "en-bb" => return "English (Barbados)",
            "en-bd" => return "English (Bangladesh)",
            "en-be" => return "English (Belgium)",
            "en-bg" => return "English (Bulgaria)",
            "en-bi" => return "English (Burundi)",
            "en-bm" => return "English (Bermuda)",
            "en-br" => return "English (Brazil)",
            "en-bs" => return "English (Bahamas)",
            "en-bw" => return "English (Botswana)",
            "en-bz" => return "English (Belize)",
            "en-ca" => return "English (Canada)",
            "en-cc" => return "English (Cocos Islands)",
            "en-ch" => return "English (Switzerland)",
            "en-ck" => return "English (Cook Islands)",
            "en-cl" => return "English (Chile)",
            "en-cm" => return "English (Cameroon)",
            "en-cn" => return "English (China)",
            "en-co" => return "English (Colombia)",
            "en-cx" => return "English (Christmas Island)",
            "en-cy" => return "English (Cyprus)",
            "en-cz" => return "English (Czech Republic)",
            "en-de" => return "English (Germany)",
            "en-dg" => return "English (Diego Garcia)",
            "en-dk" => return "English (Denmark)",
            "en-dm" => return "English (Dominica)",
            "en-ee" => return "English (Estonia)",
            "en-er" => return "English (Eritrea)",
            "en-es" => return "English (Spain)",
            "en-fi" => return "English (Finland)",
            "en-fj" => return "English (Fiji)",
            "en-fk" => return "English (Falkland Islands)",
            "en-fm" => return "English (Micronesia)",
            "en-fr" => return "English (France)",
            "en-gb" => return "English (United Kingdom)",
            "en-gd" => return "English (Grenada)",
            "en-gg" => return "English (Guernsey)",
            "en-gh" => return "English (Ghana)",
            "en-gi" => return "English (Gibraltar)",
            "en-gm" => return "English (Gambia)",
            "en-gr" => return "English (Greece)",
            "en-gu" => return "English (Guam)",
            "en-gy" => return "English (Guyana)",
            "en-hk" => return "English (Hong Kong)",
            "en-hu" => return "English (Hungary)",
            "en-hr" => return "English (Croatia)",
            "en-id" => return "English (Indonesia)",
            "en-ie" => return "English (Ireland)",
            "en-il" => return "English (Israel)",
            "en-im" => return "English (Isle of Man)",
            "en-in" => return "English (India)",
            "en-io" => return "English (British Indian Ocean Territory)",
            "en-is" => return "English (Iceland)",
            "en-it" => return "English (Italy)",
            "en-je" => return "English (Jersey)",
            "en-jm" => return "English (Jamaica)",
            "en-jp" => return "English (Japan)",
            "en-ke" => return "English (Kenya)",
            "en-ki" => return "English (Kiribati)",
            "en-kn" => return "English (Saint Kitts and Nevis)",
            "en-kr" => return "English (South Korea)",
            "en-ky" => return "English (Cayman Islands)",
            "en-lc" => return "English (Saint Lucia)",
            "en-lr" => return "English (Liberia)",
            "en-ls" => return "English (Lesotho)",
            "en-lt" => return "English (Lithuania)",
            "en-lu" => return "English (Luxembourg)",
            "en-lv" => return "English (Latvia)",
            "en-me" => return "English (Montenegro)",
            "en-mh" => return "English (Marshall Islands)",
            "en-mg" => return "English (Madagascar)",
            "en-mm" => return "English (Myanmar)",
            "en-mp" => return "English (Northern Mariana Islands)",
            "en-mo" => return "English (Macao)",
            "en-ms" => return "English (Montserrat)",
            "en-mt" => return "English (Malta)",
            "en-mu" => return "English (Mauritius)",
            "en-mv" => return "English (Maldives)",
            "en-mw" => return "English (Malawi)",
            "en-mx" => return "English (Mexico)",
            "en-my" => return "English (Malaysia)",
            "en-na" => return "English (Namibia)",
            "en-nf" => return "English (Norfolk Island)",
            "en-ng" => return "English (Nigeria)",
            "en-nl" => return "English (Netherlands)",
            "en-no" => return "English (Norway)",
            "en-nr" => return "English (Nauru)",
            "en-nu" => return "English (Niue)",
            "en-nz" => return "English (New Zealand)",
            "en-pg" => return "English (Papua New Guinea)",
            "en-ph" => return "English (Philippines)",
            "en-pk" => return "English (Pakistan)",
            "en-pl" => return "English (Poland)",
            "en-pn" => return "English (Pitcairn Islands)",
            "en-pr" => return "English (Puerto Rico)",
            "en-pt" => return "English (Portugal)",
            "en-pw" => return "English (Palau)",
            "en-ro" => return "English (Romania)",
            "en-rs" => return "English (Serbia)",
            "en-ru" => return "English (Russia)",
            "en-rw" => return "English (Rwanda)",
            "en-sa" => return "English (Saudi Arabia)",
            "en-sb" => return "English (Solomon Islands)",
            "en-sc" => return "English (Seychelles)",
            "en-sd" => return "English (Sudan)",
            "en-se" => return "English (Sweden)",
            "en-sg" => return "English (Singapore)",
            "en-sh" => return "English (Saint Helena)",
            "en-si" => return "English (Slovenia)",
            "en-sk" => return "English (Slovakia)",
            "en-sl" => return "English (Sierra Leone)",
            "en-ss" => return "English (South Sudan)",
            "en-sx" => return "English (Sint Maarten)",
            "en-sz" => return "English (Swaziland)",
            "en-tc" => return "English (Turks and Caicos Islands)",
            "en-th" => return "English (Thailand)",
            "en-tk" => return "English (Tokelau)",
            "en-to" => return "English (Tonga)",
            "en-tr" => return "English (Turkey)",
            "en-tt" => return "English (Trinidad and Tobago)",
            "en-tv" => return "English (Tuvalu)",
            "en-tw" => return "English (Taiwan)",
            "en-tz" => return "English (Tanzania)",
            "en-ua" => return "English (Ukraine)",
            "en-ug" => return "English (Uganda)",
            "en-um" => return "English (United States Minor Outlying Islands)",
            "en-us" => return "English (United States)",
            "en-vc" => return "English (Saint Vincent and the Grenadines)",
            "en-vg" => return "English (British Virgin Islands)",
            "en-vi" => return "English (U.S. Virgin Islands)",
            "en-vu" => return "English (Vanuatu)",
            "en-ws" => return "English (Samoa)",
            "en-za" => return "English (South Africa)",
            "en-zm" => return "English (Zambia)",
            "en-zw" => return "English (Zimbabwe)",
            "es-ag" => return "Español (Antigua y Barbuda)",
            "es-ai" => return "Español (Anguilla)",
            "es-ar" => return "Español (Argentina)",
            "es-aw" => return "Español (Aruba)",
            "es-bb" => return "Español (Barbados)",
            "es-bl" => return "Español (Saint Barthélemy)",
            "es-bm" => return "Español (Bermuda)",
            "es-bo" => return "Español (Bolivia)",
            "es-bq" => return "Español (Caribe holandés)",
            "es-br" => return "Español (Brasil)",
            "es-bs" => return "Español (Bahamas)",
            "es-bz" => return "Español (Belice)",
            "es-ca" => return "Español (Canadá)",
            "es-cl" => return "Español (Chile)",
            "es-co" => return "Español (Colombia)",
            "es-cu" => return "Español (Cuba)",
            "es-cr" => return "Español (Costa Rica)",
            "es-cw" => return "Español (Curaçao)",
            "es-dm" => return "Español (Dominica)",
            "es-do" => return "Español (República Dominicana)",
            "es-ea" => return "Español (Ceuta y Melilla)",
            "es-fk" => return "Español (Islas Malvinas)",
            "es-gd" => return "Español (Granada)",
            "es-ec" => return "Español (Ecuador)",
            "es-es" => return "Español (España)",
            "es-gf" => return "Español (Guayana francesa)",
            "es-gl" => return "Español (Groenlandia)",
            "es-gp" => return "Español (Guadalupe)",
            "es-gq" => return "Español (Guinea Ecuatorial)",
            "es-gt" => return "Español (Guatemala)",
            "es-gy" => return "Español (Guayana)",
            "es-hn" => return "Español (Honduras)",
            "es-ht" => return "Español (Haití)",
            "es-ic" => return "Español (Islas Canarias)",
            "es-kn" => return "Español (San Cristóbal y Nieves)",
            "es-ky" => return "Español (Islas Caimán)",
            "es-lc" => return "Español (Santa Lucía)",
            "es-mf" => return "Español (San Martín)",
            "es-mq" => return "Español (Martinica)",
            "es-ms" => return "Español (Montserrat)",
            "es-mx" => return "Español (México)",
            "es-ni" => return "Español (Nicaragua)",
            "es-pa" => return "Español (Panamá)",
            "es-pe" => return "Español (Perú)",
            "es-ph" => return "Español (Filipinas)",
            "es-pm" => return "Español (San Pedro y Miquelón)",
            "es-pr" => return "Español (Puerto Rico)",
            "es-py" => return "Español (Paraguay)",
            "es-sr" => return "Español (Surinam)",
            "es-sv" => return "Español (El Salvador)",
            "es-sx" => return "Español (San Martín)",
            "es-tc" => return "Español (Islas Turcas y Caicos)",
            "es-tt" => return "Español (Trinidad y Tobago)",
            "es-us" => return "Español (Estados Unidos)",
            "es-uy" => return "Español (Uruguay)",
            "es-vc" => return "Español (San Vicente y las Granadinas)",
            "es-ve" => return "Español (Venezuela)",
            "es-vg" => return "Español (Islas Vírgenes Británicas)",
            "es-vi" => return "Español (Islas Vírgenes de los Estados Unidos)",
            "et-ee" => return "Eesti (Eesti)",
            "eu-es" => return "Euskara (Espainia)",
            "fa-af" => return "فارسی (افغانستان)",
            "fa-ir" => return "فارسی (ایران)",
            "ff-bf" => return "Fulfulde (Burkina Faso)",
            "ff-cm" => return "Fulfulde (Cameroun)",
            "ff-gh" => return "Fulfulde (Ghana)",
            "ff-gm" => return "Fulfulde (Gambia)",
            "ff-gn" => return "Fulfulde (Guinée)",
            "ff-gw" => return "Fulfulde (Guinée-Bissau)",
            "ff-mr" => return "Fulfulde (Mauritanie)",
            "ff-ne" => return "Fulfulde (Niger)",
            "ff-ng" => return "Fulfulde (Nigeria)",
            "ff-lr" => return "Fulfulde (Libéria)",
            "ff-sl" => return "Fulfulde (Sierra Leone)",
            "ff-sn" => return "Fulfulde (Sénégal)",
            "fi-fi" => return "suomi (Suomi)",
            "fo-dk" => return "Føroyskt (Danmark)",
            "fo-fo" => return "Føroyskt (Føroyar)",
            "fr-be" => return "Français (Belgique)",
            "fr-bf" => return "Français (Burkina Faso)",
            "fr-bi" => return "Français (Burundi)",
            "fr-bj" => return "Français (Bénin)",
            "fr-bl" => return "Français (Saint-Barthélemy)",
            "fr-ca" => return "Français (Canada)",
            "fr-cd" => return "Français (Congo)",
            "fr-cf" => return "Français (République centrafricaine)",
            "fr-cg" => return "Français (Congo)",
            "fr-ch" => return "Français (Suisse)",
            "fr-ci" => return "Français (Côte d'Ivoire)",
            "fr-cm" => return "Français (Cameroun)",
            "fr-dj" => return "Français (Djibouti)",
            "fr-dz" => return "Français (Algérie)",
            "fr-fr" => return "Français (France)",
            "fr-ga" => return "Français (Gabon)",
            "fr-gf" => return "Français (Guyane française)",
            "fr-gn" => return "Français (Guinée)",
            "fr-gp" => return "Français (Guadeloupe)",
            "fr-gq" => return "Français (Guinée équatoriale)",
            "fr-ht" => return "Français (Haïti)",
            "fr-km" => return "Français (Comores)",
            "fr-ma" => return "Français (Maroc)",
            "fr-mc" => return "Français (Monaco)",
            "fr-mf" => return "Français (Saint-Martin)",
            "fr-nc" => return "Français (Nouvelle-Calédonie)",
            "fr-ne" => return "Français (Niger)",
            "fr-lu" => return "Français (Luxembourg)",
            "fr-mg" => return "Français (Madagascar)",
            "fr-ml" => return "Français (Mali)",
            "fr-mq" => return "Français (Martinique)",
            "fr-mr" => return "Français (Mauritanie)",
            "fr-mu" => return "Français (Maurice)",
            "fr-pf" => return "Français (Polynésie française)",
            "fr-pm" => return "Français (Saint-Pierre-et-Miquelon)",
            "fr-re" => return "Français (Réunion)",
            "fr-rw" => return "Français (Rwanda)",
            "fr-sc" => return "Français (Seychelles)",
            "fr-sn" => return "Français (Sénégal)",
            "fr-sy" => return "Français (Syrie)",
            "fr-td" => return "Français (Tchad)",
            "fr-tg" => return "Français (Togo)",
            "fr-tn" => return "Français (Tunisie)",
            "fr-vu" => return "Français (Vanuatu)",
            "fr-wf" => return "Français (Wallis-et-Futuna)",
            "fr-yt" => return "Français (Mayotte)",
            "fy-nl" => return "Frysk (Nederlân)",
            "ga-ie" => return "Gaeilge (Éire)",
            "gd-gb" => return "Gàidhlig (An Rìoghachd Aonaichte)",
            "gl-es" => return "Galego (España)",
            "gn-py" => return "Avañe'ẽ (Paraguái)",
            "gu-in" => return "ગુજરાતી (ભારત)",
            "gv-im" => return "Gaelg (Ellan Vannin)",
            "ha-gh" => return "Hausa (Ghana)",
            "ha-ne" => return "Hausa (Nijar)",
            "ha-ng" => return "Hausa (Najeriya)",
            "he-il" => return "עברית (ישראל)",
            "hi-in" => return "हिन्दी (भारत)",
            "hr-ba" => return "Hrvatski (Bosna i Hercegovina)",
            "hr-hr" => return "Hrvatski (Hrvatska)",
            "hu-hu" => return "Magyar (Magyarország)",
            "hy-am" => return "Հայերէն (Հայաստան)",
            "id-id" => return "Bahasa Indonesia (Indonesia)",
            "ig-ng" => return "Igbo (Nigeria)",
            "ii-cn" => return "ꆈꌠꉙ (中国)",
            "is-is" => return "Íslenska (Ísland)",
            "it-ch" => return "Italiano (Svizzera)",
            "it-it" => return "Italiano (Italia)",
            "it-sm" => return "Italiano (San Marino)",
            "it-va" => return "Italiano (Città del Vaticano)",
            "iu-ca" => return "ᐃᓄᒃᑎᑐᑦ (Canada)",
            "ja-jp" => return "日本語 (日本)",
            "jv-id" => return "Basa Jawa (Indonesia)", // TODO: check this
            "ka-ge" => return "ქართული (საქართველო)",
            "ki-ke" => return "Gĩkũyũ (Kenya)",
            "kk-kz" => return "Қазақ тілі (Қазақстан)",
            "kl-gl" => return "Kalaallisut (Kalaallit Nunaat)",
            "km-kh" => return "ភាសាខ្មែរ (កម្ពុជា)",
            "kn-in" => return "ಕನ್ನಡ (ಭಾರತ)",
            "ko-kp" => return "한국어(북한)",
            "ks-in" => return "कश्मीरी (भारत)",
            "ku-tr" => return "Kurdî (Tirkiye)",
            "kw-gb" => return "Cornish (United Kingdom)", // TODO: check this
            "ky-kg" => return "Кыргызстандык (Кыргызстан)",
            "lb-lu" => return "Lëtzebuergesch (Lëtzebuerg)",
            "lg-ug" => return "Luganda (Yuganda)", // TODO: check this
            "ln-ao" => return "Lingála (Angola)",  // TODO: check this
            "ln-cf" => return "Lingála (République centrafricaine)", // TODO: check this
            "ln-cg" => return "Lingála (Congo)", // TODO: check this
            "lo-la" => return "ພາສາລາວ (ລາວ)",
            "lt-lt" => return "Lietuvių kalba (Lietuva)",
            "lu-cd" => return "Kiluba (Congo)",
            "lv-lv" => return "Latviešu valoda (Latvija)",
            "mg-mg" => return "Malagasy (Madagascar)",
            "mi-nz" => return "Māori (Aotearoa)",
            "mk-mk" => return "Македонски (Македонија)",
            "ml-in" => return "മലയാളം (ഭാരതം)",
            "mn-mn" => return "Монгол хэл (Монгол)",
            "mr-in" => return "मराठी (भारत)",
            "ms-bn" => return "Bahasa Melayu (Brunei)",
            "ms-my" => return "Bahasa Melayu (Malaysia)",
            "ms-sg" => return "Bahasa Melayu (Singapura)",
            "mt-mt" => return "Malti (Malta)",
            "my-mm" => return "ဗမာစာ (မြန်မာ)",
            "nb-no" => return "Norsk bokmål (Norge)",
            "nb-sj" => return "Norsk bokmål (Svalbard og Jan Mayen)",
            "nd-zw" => return "isiNdebele (Zimbabwe)",
            "ne-in" => return "नेपाली (भारत)",
            "ne-np" => return "नेपाली (नेपाल)",
            "nl-aw" => return "Nederlands (Aruba)",
            "nl-be" => return "Nederlands (België)",
            "nl-bq" => return "Nederlands (Caribisch Nederland)",
            "nl-cw" => return "Nederlands (Curaçao)",
            "nl-nl" => return "Nederlands (Nederland)",
            "nl-sr" => return "Nederlands (Suriname)",
            "nl-sx" => return "Nederlands (Sint Maarten)",
            "nn-no" => return "Norsk nynorsk (Noreg)",
            "nr-za" => return "isiNdebele (South Africa)",
            "ny-mw" => return "Chichewa (Malawi)",
            "oc-fr" => return "Occitan (France)", // TODO: Check this
            "os-ge" => return "Ирон æвзаг (Росси)",
            "om-et" => return "Afaan Oromoo (Itoophiyaa)",
            "om-ke" => return "Afaan Oromoo (Keeniyaa)",
            "or-in" => return "ଓଡ଼ିଆ (ଭାରତ)",
            "os-ru" => return "Ирон æвзаг (Росси)",
            "pa-in" => return "ਪੰਜਾਬੀ (ਭਾਰਤ)",
            "pa-pk" => return "پنجابی (پاکستان)",
            "pl-pl" => return "Polski (Polska)",
            "ps-af" => return "پښتو (افغانستان)",
            "ps-pk" => return "پښتو (پاکستان)",
            "pt-ao" => return "Português (Angola)",
            "pt-br" => return "Português (Brasil)",
            "pt-ch" => return "Português (Suíça)",
            "pt-cv" => return "Português (Cabo Verde)",
            "pt-fr" => return "Português (França)",
            "pt-gq" => return "Português (Guiné Equatorial)",
            "pt-gw" => return "Português (Guiné-Bissau)",
            "pt-mz" => return "Português (Moçambique)",
            "pt-lu" => return "Português (Luxemburgo)",
            "pt-mo" => return "Português (Macau)",
            "pt-pt" => return "Português (Portugal)",
            "pt-st" => return "Português (São Tomé e Príncipe)",
            "pt-tl" => return "Português (Timor-Leste)",
            "qu-bo" => return "Runa simi (Bolivia)",
            "qu-ec" => return "Runa simi (Ecuador)",
            "qu-pe" => return "Runa simi (Perú)",
            "rn-bi" => return "Ikirundi (Burundi)",
            "ro-md" => return "Română (Republica Moldova)",
            "ro-ro" => return "Română (România)",
            "ru-by" => return "Русский язык (Беларусь)",
            "ru-kg" => return "Русский язык (Киргизия)",
            "ru-kz" => return "Русский язык (Казахстан)",
            "ru-md" => return "Русский язык (Молдова)",
            "ru-ru" => return "Русский язык (Россия)",
            "ru-ua" => return "Русский язык (Украина)",
            "rw-rw" => return "Kinyarwanda (Rwanda)",
            "sa-in" => return "संस्कृतम् (भारतः)",
            "sa-it" => return "संस्कृतम् (इटली)",
            "sd-pk" => return "سنڌي (پاکستان)",
            "se-fi" => return "Davvisámegiella (Suopma)",
            "se-no" => return "Davvisámegiella (Norga)",
            "se-se" => return "Davvisámegiella (Ruoŧŧa)",
            "sg-cf" => return "Sängö (République centrafricaine)",
            "si-lk" => return "සිංහල (ශ්‍රී ලංකා)",
            "sk-sk" => return "Slovenčina (Slovenská republika)",
            "sl-si" => return "Slovenščina (Slovenija)",
            "sn-zw" => return "chiShona (Zimbabwe)",
            "so-et" => return "Soomaaliga (Itoobiya)",
            "so-ke" => return "Soomaaliga (Kenya)",
            "so-so" => return "Soomaaliga (Soomaaliya)",
            "sq-al" => return "Shqip (Shqipëri)",
            "sq-mk" => return "Shqip (Maqedoni)",
            "sq-xk" => return "Shqip (Kosovë)",
            "sr-ba" => return "Српски (Босна и Херцеговина)",
            "sr-me" => return "Српски (Црна Гора)",
            "sr-rs" => return "Српски (Србија)",
            "sr-xk" => return "Српски (Косово)",
            "ss-sz" => return "SiSwati (Swaziland)",
            "ss-za" => return "SiSwati (South Africa)",
            "st-ls" => return "Sesotho (Lesotho)",
            "st-za" => return "Sesotho (South Africa)",
            "sv-ax" => return "Svenska (Åland)",
            "sv-fi" => return "Svenska (Finland)",
            "sw-cd" => return "Kiswahili (Jamhuri ya Kidemokrasia ya Kongo)",
            "sw-ke" => return "Kiswahili (Kenya)",
            "sw-tz" => return "Kiswahili (Tanzania)",
            "sw-ug" => return "Kiswahili (Uganda)",
            "ta-in" => return "தமிழ் (இந்தியா)",
            "ta-lk" => return "தமிழ் (இலங்கை)",
            "ta-my" => return "தமிழ் (மலேசியா)",
            "ta-sg" => return "தமிழ் (சிங்கப்பூர்)",
            "te-in" => return "తెలుగు (భారత)",
            "tg-tj" => return "тоҷикӣ (Тоҷикистон)",
            "th-th" => return "ไทย (ไทย)",
            "ti-er" => return "ትግርኛ (ኤርትራ)",
            "ti-et" => return "ትግርኛ (ኢትዮጵያ)",
            "tk-tm" => return "Türkmençe (Türkmenistan)",
            "tn-bw" => return "Setswana (Botswana)",
            "tn-za" => return "Setswana (South Africa)",
            "to-to" => return "faka Tonga (Tonga)",
            "tr-cy" => return "Türkçe (Kıbrıs)",
            "tr-tr" => return "Türkçe (Türkiye)",
            "ts-za" => return "Xitsonga (South Africa)",
            "tt-ru" => return "Татар теле (Россия)",
            "ug-cn" => return "ئۇيغۇرچە (جۇڭگو)",
            "uk-ua" => return "Українська (Україна)",
            "ur-in" => return "اردو (بھارت)",
            "ur-pk" => return "اردو (پاکستان)",
            "uz-af" => return "O'zbekiston (Afg'oniston)",
            "uz-uz" => return "O'zbekiston (O'zbekiston)",
            "ve-za" => return "Tshivenḓa (South Africa)",
            "vi-vn" => return "Tiếng Việt (Việt Nam)",
            "wa-be" => return "Walon (Belgique)",
            "wo-sn" => return "Wolof (Sénégal)", // TODO: Check this, seems French
            "xh-za" => return "isiXhosa (South Africa)",
            "yo-bj" => return "Yorùbá (Bénin)",
            "yo-ng" => return "Yorùbá (Nàìjíríà)",
            "zh-cn" => return "中文 (简体)",
            "zh-hk" => return "中文 (香港)",
            "zh-mo" => return "中文 (澳門)",
            "zh-sg" => return "中文 (新加坡)",
            "zh-tw" => return "中文 (繁體)",
            "zu-za" => return "isiZulu (South Africa)",

            // lang (2 letter) -> country (3 letter)
            "af-nam" => return "Afrikaans (Namibia)",
            "af-zaf" => return "Afrikaans (South Africa)",
            "ak-gha" => return "Akan (Ghana)",
            "am-eth" => return "አማርኛ (ኢትዮጵያ)",
            "ar-001" => return "العربية",
            "ar-are" => return "العربية (الإمارات العربية المتحدة)",
            "ar-bhr" => return "العربية (البحرين)",
            "ar-com" => return "العربية (جزر القمر)",
            "ar-dji" => return "العربية (جيبوتي)",
            "ar-dza" => return "العربية (الجزائر)",
            "ar-egy" => return "العربية (مصر)",
            "ar-eri" => return "العربية (إريتريا)",
            "ar-esh" => return "العربية (الصحراء الغربية)",
            "ar-irq" => return "العربية (العراق)",
            "ar-isr" => return "العربية (إسرائيل)",
            "ar-jor" => return "العربية (الأردن)",
            "ar-kwt" => return "العربية (الكويت)",
            "ar-lbn" => return "العربية (لبنان)",
            "ar-lby" => return "العربية (ليبيا)",
            "ar-mar" => return "العربية (المغرب)",
            "ar-mrt" => return "العربية (موريتانيا)",
            "ar-omn" => return "العربية (عمان)",
            "ar-pse" => return "العربية (فلسطين)",
            "ar-qat" => return "العربية (قطر)",
            "ar-sau" => return "العربية (المملكة العربية السعودية)",
            "ar-sdn" => return "العربية (السودان)",
            "ar-som" => return "العربية (الصومال)",
            "ar-ssd" => return "العربية (جنوب السودان)",
            "ar-syr" => return "العربية (سوريا)",
            "ar-tcd" => return "العربية (تشاد)",
            "ar-tun" => return "العربية (تونس)",
            "as-ind" => return "অসমীয়া (ভাৰত)",
            "az-aze" => return "Azərbaycan dili (Azərbaycan)",
            "ba-rus" => return "башҡорт теле (Россия)",
            "be-blr" => return "Беларуская мова (Беларусь)",
            "bg-bgr" => return "български език (България)",
            "bm-mli" => return "ߓߊߡߊߣߊߣߞߊߣ (ߞߊ߲ߞߊ߲)",
            "bn-bgd" => return "বাংলা (বাংলাদেশ)",
            "bn-ind" => return "বাংলা (ভারত)",
            "bo-chn" => return "བོད་སྐད་ (ཀྲུང་ཧྭ)",
            "bo-ind" => return "བོད་སྐད་ (ཀྲུང་ཧྭ)", // TODO: The same as bo-cn, looks bad
            "br-fra" => return "Brezhoneg (Frañs)",
            "bs-bih" => return "Bosanski (Bosna i Hercegovina)",
            "ca-and" => return "Català (Andorra)",
            "ca-esp" => return "Català (Espanya)",
            "ca-fra" => return "Català (França)",
            "ca-ita" => return "Català (Itàlia)",
            "ce-rus" => return "нохчийн мотт (Росси)",
            "co-fra" => return "Corsu (France)",
            "cs-cze" => return "Čeština (Česká republika)",
            "cv-rus" => return "чӑваш чӗлхи (Росси)",
            "cy-gbr" => return "Cymraeg (Y Deyrnas Unedig)",
            "da-dnk" => return "Dansk (Danmark)",
            "da-grl" => return "Dansk (Grønland)",
            "de-aut" => return "Deutsch (Österreich)",
            "de-bel" => return "Deutsch (Belgien)",
            "de-che" => return "Deutsch (Schweiz)",
            "de-deu" => return "Deutsch (Deutschland)",
            "de-ita" => return "Deutsch (Italien)",
            "de-lie" => return "Deutsch (Liechtenstein)",
            "de-lux" => return "Deutsch (Luxemburg)",
            "dv-mdv" => return "ދިވެހި (ދިވެހި)",
            "dz-btn" => return "རྫོང་ཁ་ (འབྲུག་ཡུལ)",
            "ee-gha" => return "Eʋegbe (Ghana)",
            "ee-tgo" => return "Eʋegbe (Togo)",
            "el-cyp" => return "Νέα Ελληνικά (Κύπρος)",
            "el-grc" => return "Νέα Ελληνικά (Ελλάδα)",
            "en-001" => return "English",
            "en-150" => return "English (Europe)",
            "en-aia" => return "English (Anguilla)",
            "en-alb" => return "English (Albania)",
            "en-and" => return "English (Andorra)",
            "en-are" => return "English (United Arab Emirates)",
            "en-arg" => return "English (Argentina)",
            "en-asm" => return "English (American Samoa)",
            "en-atg" => return "English (Antigua and Barbuda)",
            "en-aus" => return "English (Australia)",
            "en-aut" => return "English (Austria)",
            "en-bdi" => return "English (Burundi)",
            "en-bel" => return "English (Belgium)",
            "en-bgd" => return "English (Bangladesh)",
            "en-bgr" => return "English (Bulgaria)",
            "en-bhs" => return "English (Bahamas)",
            "en-bih" => return "English (Bosnia and Herzegovina)",
            "en-bmu" => return "English (Bermuda)",
            "en-blz" => return "English (Belize)",
            "en-bra" => return "English (Brazil)",
            "en-brb" => return "English (Barbados)",
            "en-bwa" => return "English (Botswana)",
            "en-can" => return "English (Canada)",
            "en-cck" => return "English (Cocos Islands)",
            "en-che" => return "English (Switzerland)",
            "en-chl" => return "English (Chile)",
            "en-chn" => return "English (China)",
            "en-cmr" => return "English (Cameroon)",
            "en-cok" => return "English (Cook Islands)",
            "en-col" => return "English (Colombia)",
            "en-cxr" => return "English (Christmas Island)",
            "en-cym" => return "English (Cayman Islands)",
            "en-cyp" => return "English (Cyprus)",
            "en-cze" => return "English (Czech Republic)",
            "en-deu" => return "English (Germany)",
            "en-dga" => return "English (Diego Garcia)",
            "en-dma" => return "English (Dominica)",
            "en-dnk" => return "English (Denmark)",
            "en-esp" => return "English (Spain)",
            "en-fin" => return "English (Finland)",
            "en-eri" => return "English (Eritrea)",
            "en-est" => return "English (Estonia)",
            "en-fji" => return "English (Fiji)",
            "en-flk" => return "English (Falkland Islands)",
            "en-fra" => return "English (France)",
            "en-fsm" => return "English (Micronesia)",
            "en-gbr" => return "English (United Kingdom)",
            "en-ggy" => return "English (Guernsey)",
            "en-gha" => return "English (Ghana)",
            "en-gib" => return "English (Gibraltar)",
            "en-gmb" => return "English (Gambia)",
            "en-grc" => return "English (Greece)",
            "en-grd" => return "English (Grenada)",
            "en-gum" => return "English (Guam)",
            "en-guy" => return "English (Guyana)",
            "en-hkg" => return "English (Hong Kong)",
            "en-hrv" => return "English (Croatia)",
            "en-hun" => return "English (Hungary)",
            "en-imn" => return "English (Isle of Man)",
            "en-idn" => return "English (Indonesia)",
            "en-ind" => return "English (India)",
            "en-iot" => return "English (British Indian Ocean Territory)",
            "en-irl" => return "English (Ireland)",
            "en-isl" => return "English (Iceland)",
            "en-isr" => return "English (Israel)",
            "en-ita" => return "English (Italy)",
            "en-jam" => return "English (Jamaica)",
            "en-jey" => return "English (Jersey)",
            "en-jpn" => return "English (Japan)",
            "en-ken" => return "English (Kenya)",
            "en-kir" => return "English (Kiribati)",
            "en-kna" => return "English (Saint Kitts and Nevis)",
            "en-kor" => return "English (South Korea)",
            "en-lbr" => return "English (Liberia)",
            "en-lca" => return "English (Saint Lucia)",
            "en-lso" => return "English (Lesotho)",
            "en-ltu" => return "English (Lithuania)",
            "en-lux" => return "English (Luxembourg)",
            "en-lva" => return "English (Latvia)",
            "en-mac" => return "English (Macao)",
            "en-mdg" => return "English (Madagascar)",
            "en-mdv" => return "English (Maldives)",
            "en-mex" => return "English (Mexico)",
            "en-mhl" => return "English (Marshall Islands)",
            "en-mlt" => return "English (Malta)",
            "en-mmr" => return "English (Myanmar)",
            "en-mne" => return "English (Montenegro)",
            "en-mnp" => return "English (Northern Mariana Islands)",
            "en-msr" => return "English (Montserrat)",
            "en-mus" => return "English (Mauritius)",
            "en-mwi" => return "English (Malawi)",
            "en-mys" => return "English (Malaysia)",
            "en-nam" => return "English (Namibia)",
            "en-nfk" => return "English (Norfolk Island)",
            "en-nga" => return "English (Nigeria)",
            "en-niu" => return "English (Niue)",
            "en-nld" => return "English (Netherlands)",
            "en-nor" => return "English (Norway)",
            "en-nru" => return "English (Nauru)",
            "en-nzl" => return "English (New Zealand)",
            "en-pak" => return "English (Pakistan)",
            "en-pcn" => return "English (Pitcairn Islands)",
            "en-phl" => return "English (Philippines)",
            "en-plw" => return "English (Palau)",
            "en-png" => return "English (Papua New Guinea)",
            "en-pol" => return "English (Poland)",
            "en-pri" => return "English (Puerto Rico)",
            "en-prt" => return "English (Portugal)",
            "en-rou" => return "English (Romania)",
            "en-rus" => return "English (Russia)",
            "en-rwa" => return "English (Rwanda)",
            "en-sau" => return "English (Saudi Arabia)",
            "en-sdn" => return "English (Sudan)",
            "en-shn" => return "English (Saint Helena)",
            "en-sgp" => return "English (Singapore)",
            "en-slb" => return "English (Solomon Islands)",
            "en-sle" => return "English (Sierra Leone)",
            "en-srb" => return "English (Serbia)",
            "en-ssd" => return "English (South Sudan)",
            "en-svk" => return "English (Slovakia)",
            "en-svn" => return "English (Slovenia)",
            "en-swe" => return "English (Sweden)",
            "en-swz" => return "English (Swaziland)",
            "en-sxm" => return "English (Sint Maarten)",
            "en-syc" => return "English (Seychelles)",
            "en-tca" => return "English (Turks and Caicos Islands)",
            "en-tha" => return "English (Thailand)",
            "en-tkl" => return "English (Tokelau)",
            "en-ton" => return "English (Tonga)",
            "en-tto" => return "English (Trinidad and Tobago)",
            "en-tur" => return "English (Turkey)",
            "en-tuv" => return "English (Tuvalu)",
            "en-twn" => return "English (Taiwan)",
            "en-tza" => return "English (Tanzania)",
            "en-uga" => return "English (Uganda)",
            "en-ukr" => return "English (Ukraine)",
            "en-umi" => {
                return "English (United States Minor Outlying Islands)"
            }
            "en-usa" => return "English (United States)",
            "en-vct" => return "English (Saint Vincent and the Grenadines)",
            "en-vgb" => return "English (British Virgin Islands)",
            "en-vir" => return "English (U.S. Virgin Islands)",
            "en-vut" => return "English (Vanuatu)",
            "en-wsm" => return "English (Samoa)",
            "en-zaf" => return "English (South Africa)",
            "en-zmb" => return "English (Zambia)",
            "en-zwe" => return "English (Zimbabwe)",
            "eo-001" => return "Esperanto",
            "es-419" => return "Español (Latinoamérica)",
            "es-abw" => return "Español (Aruba)",
            "es-aia" => return "Español (Anguilla)",
            "es-arg" => return "Español (Argentina)",
            "es-atg" => return "Español (Antigua y Barbuda)",
            "es-bes" => return "Español (Caribe holandés)",
            "es-bhs" => return "Español (Bahamas)",
            "es-blm" => return "Español (San Bartolomé)",
            "es-blz" => return "Español (Belice)",
            "es-bmu" => return "Español (Bermuda)",
            "es-bol" => return "Español (Bolivia)",
            "es-bra" => return "Español (Brasil)",
            "es-brb" => return "Español (Barbados)",
            "es-can" => return "Español (Canadá)",
            "es-chl" => return "Español (Chile)",
            "es-col" => return "Español (Colombia)",
            "es-cri" => return "Español (Costa Rica)",
            "es-cub" => return "Español (Cuba)",
            "es-cuw" => return "Español (Curaçao)",
            "es-cym" => return "Español (Islas Caimán)",
            "es-dma" => return "Español (Dominica)",
            "es-dom" => return "Español (República Dominicana)",
            "es-ecu" => return "Español (Ecuador)",
            "es-esp" => return "Español (España)",
            "es-flk" => return "Español (Islas Malvinas)",
            "es-glp" => return "Español (Guadalupe)",
            "es-gnq" => return "Español (Guinea Ecuatorial)",
            "es-grd" => return "Español (Granada)",
            "es-grl" => return "Español (Groenlandia)",
            "es-gtm" => return "Español (Guatemala)",
            "es-guf" => return "Español (Guayana francesa)",
            "es-guy" => return "Español (Guayana)",
            "es-hnd" => return "Español (Honduras)",
            "es-hti" => return "Español (Haití)",
            "es-kna" => return "Español (San Cristóbal y Nieves)",
            "es-lca" => return "Español (Santa Lucía)",
            "es-maf" => return "Español (San Martín)",
            "es-mex" => return "Español (México)",
            "es-msr" => return "Español (Montserrat)",
            "es-mtq" => return "Español (Martinica)",
            "es-nic" => return "Español (Nicaragua)",
            "es-pan" => return "Español (Panamá)",
            "es-per" => return "Español (Perú)",
            "es-phl" => return "Español (Filipinas)",
            "es-pri" => return "Español (Puerto Rico)",
            "es-pry" => return "Español (Paraguay)",
            "es-slv" => return "Español (El Salvador)",
            "es-spm" => return "Español (San Pedro y Miquelón)",
            "es-sur" => return "Español (Surinam)",
            "es-sxm" => return "Español (San Martín)",
            "es-tca" => return "Español (Islas Turcas y Caicos)",
            "es-tto" => return "Español (Trinidad y Tobago)",
            "es-ury" => return "Español (Uruguay)",
            "es-usa" => return "Español (Estados Unidos)",
            "es-vct" => return "Español (San Vicente y las Granadinas)",
            "es-ven" => return "Español (Venezuela)",
            "es-vgb" => return "Español (Islas Vírgenes Británicas)",
            "es-vir" => {
                return "Español (Islas Vírgenes de los Estados Unidos)"
            }
            "et-est" => return "Eesti (Eesti)",
            "eu-esp" => return "Euskara (Espainia)",
            "fa-afg" => return "فارسی (افغانستان)",
            "fa-irn" => return "فارسی (ایران)",
            "ff-bfa" => return "Fulfulde (Burkina Faso)",
            "ff-cmr" => return "Fulfulde (Cameroun)",
            "ff-gha" => return "Fulfulde (Ghana)",
            "ff-gin" => return "Fulfulde (Guinée)",
            "ff-gmb" => return "Fulfulde (Gambia)",
            "ff-gnb" => return "Fulfulde (Guinée-Bissau)",
            "ff-lbr" => return "Fulfulde (Libéria)",
            "ff-mrt" => return "Fulfulde (Mauritanie)",
            "ff-ner" => return "Fulfulde (Niger)",
            "ff-nga" => return "Fulfulde (Nigeria)",
            "ff-sen" => return "Fulfulde (Sénégal)",
            "ff-sle" => return "Fulfulde (Sierra Leone)",
            "fi-fin" => return "suomi (Suomi)",
            "fo-dnk" => return "Føroyskt (Danmark)",
            "fo-fro" => return "Føroyskt (Føroyar)",
            "fr-bdi" => return "Français (Burundi)",
            "fr-bel" => return "Français (Belgique)",
            "fr-ben" => return "Français (Bénin)",
            "fr-bfa" => return "Français (Burkina Faso)",
            "fr-blm" => return "Français (Saint-Barthélemy)",
            "fr-caf" => return "Français (République centrafricaine)",
            "fr-can" => return "Français (Canada)",
            "fr-che" => return "Français (Suisse)",
            "fr-civ" => return "Français (Côte d'Ivoire)",
            "fr-cmr" => return "Français (Cameroun)",
            "fr-cod" => return "Français (Congo - Kinshasa)",
            "fr-cog" => return "Français (Congo - Brazzaville)",
            "fr-com" => return "Français (Comores)",
            "fr-dji" => return "Français (Djibouti)",
            "fr-dza" => return "Français (Algérie)",
            "fr-fra" => return "Français (France)",
            "fr-gin" => return "Français (Guinée)",
            "fr-gab" => return "Français (Gabon)",
            "fr-glp" => return "Français (Guadeloupe)",
            "fr-gnq" => return "Français (Guinée équatoriale)",
            "fr-guf" => return "Français (Guyane française)",
            "fr-hti" => return "Français (Haïti)",
            "fr-lux" => return "Français (Luxembourg)",
            "fr-maf" => return "Français (Saint-Martin)",
            "fr-mar" => return "Français (Maroc)",
            "fr-mco" => return "Français (Monaco)",
            "fr-mdg" => return "Français (Madagascar)",
            "fr-mli" => return "Français (Mali)",
            "fr-mrt" => return "Français (Mauritanie)",
            "fr-mtq" => return "Français (Martinique)",
            "fr-mus" => return "Français (Maurice)",
            "fr-myt" => return "Français (Mayotte)",
            "fr-ncl" => return "Français (Nouvelle-Calédonie)",
            "fr-ner" => return "Français (Niger)",
            "fr-pyf" => return "Français (Polynésie française)",
            "fr-reu" => return "Français (Réunion)",
            "fr-rwa" => return "Français (Rwanda)",
            "fr-sen" => return "Français (Sénégal)",
            "fr-spm" => return "Français (Saint-Pierre-et-Miquelon)",
            "fr-syc" => return "Français (Seychelles)",
            "fr-syr" => return "Français (Syrie)",
            "fr-tcd" => return "Français (Tchad)",
            "fr-tgo" => return "Français (Togo)",
            "fr-tun" => return "Français (Tunisie)",
            "fr-vut" => return "Français (Vanuatu)",
            "fr-wlf" => return "Français (Wallis-et-Futuna)",
            "fy-nld" => return "Frysk (Nederlân)",
            "ga-irl" => return "Gaeilge (Éire)",
            "gd-gbr" => return "Gàidhlig (An Rìoghachd Aonaichte)",
            "gl-esp" => return "Galego (España)",
            "gn-pry" => return "Avañe'ẽ (Paraguái)",
            "gu-ind" => return "ગુજરાતી (ભારત)",
            "gv-imn" => return "Gaelg (Ellan Vannin)",
            "ha-gha" => return "Hausa (Ghana)",
            "ha-ner" => return "Hausa (Nijar)",
            "ha-nga" => return "Hausa (Najeriya)",
            "he-isr" => return "עברית (ישראל)",
            "hi-ind" => return "हिन्दी (भारत)",
            "hr-bih" => return "Hrvatski (Bosna i Hercegovina)",
            "hr-hrv" => return "Hrvatski (Hrvatska)",
            "hu-hun" => return "Magyar (Magyarország)",
            "hy-arm" => return "Հայերէն (Հայաստան)",
            "id-idn" => return "Bahasa Indonesia (Indonesia)",
            "ig-nga" => return "Igbo (Nigeria)",
            "ii-chn" => return "ꆈꌠꉙ (中国)",
            "is-isl" => return "Íslenska (Ísland)",
            "it-che" => return "Italiano (Svizzera)",
            "it-ita" => return "Italiano (Italia)",
            "it-smr" => return "Italiano (San Marino)",
            "it-vat" => return "Italiano (Città del Vaticano)",
            "ia-001" => return "Interlingua",
            "io-001" => return "Ido",
            "iu-can" => return "ᐃᓄᒃᑎᑐᑦ (Canada)",
            "ja-jpn" => return "日本語 (日本)",
            "jv-idn" => return "Basa Jawa (Indonesia)", // TODO: check this
            "ka-geo" => return "ქართული (საქართველო)",
            "ki-ken" => return "Gĩkũyũ (Kenya)",
            "kk-kaz" => return "Қазақ тілі (Қазақстан)",
            "kl-grl" => return "Kalaallisut (Kalaallit Nunaat)",
            "km-khm" => return "ភាសាខ្មែរ (កម្ពុជា)",
            "kn-ind" => return "ಕನ್ನಡ (ಭಾರತ)",
            "ko-prk" => return "한국어(북한)",
            "ks-ind" => return "कश्मीरी (भारत)",
            "ku-tur" => return "Kurdî (Tirkiye)",
            "kw-gbr" => return "Cornish (United Kingdom)", // TODO: check this
            "ky-kgz" => return "Кыргызстандык (Кыргызстан)",
            "lb-lux" => return "Lëtzebuergesch (Lëtzebuerg)",
            "lg-uga" => return "Luganda (Yuganda)", // TODO: check this
            "ln-ago" => return "Lingála (Angola)",  // TODO: check this
            "ln-caf" => return "Lingála (République centrafricaine)", // TODO: check this
            "ln-cog" => return "Lingála (Congo)", // TODO: check this
            "lo-lao" => return "ພາສາລາວ (ລາວ)",
            "lt-ltu" => return "Lietuvių kalba (Lietuva)",
            "lu-cod" => return "Kiluba (Congo)",
            "lv-lva" => return "Latviešu valoda (Latvija)",
            "mg-mdg" => return "Malagasy (Madagascar)",
            "mi-nzl" => return "Māori (Aotearoa)",
            "mk-mkd" => return "Македонски (Македонија)",
            "ml-ind" => return "മലയാളം (ഭാരതം)",
            "mn-mng" => return "Монгол хэл (Монгол)",
            "mr-ind" => return "मराठी (भारत)",
            "ms-brn" => return "Bahasa Melayu (Brunei)",
            "ms-sgp" => return "Bahasa Melayu (Singapura)",
            "ms-mys" => return "Bahasa Melayu (Malaysia)",
            "mt-mlt" => return "Malti (Malta)",
            "my-mmr" => return "ဗမာစာ (မြန်မာ)",
            "nb-nor" => return "Norsk bokmål (Norge)",
            "nb-sjm" => return "Norsk bokmål (Svalbard og Jan Mayen)",
            "nd-zwe" => return "isiNdebele (Zimbabwe)",
            "ne-ind" => return "नेपाली (भारत)",
            "ne-npl" => return "नेपाली (नेपाल)",
            "nl-abw" => return "Nederlands (Aruba)",
            "nl-bel" => return "Nederlands (België)",
            "nl-bes" => return "Nederlands (Caribisch Nederland)",
            "nl-cuw" => return "Nederlands (Curaçao)",
            "nl-nld" => return "Nederlands (Nederland)",
            "nl-sur" => return "Nederlands (Suriname)",
            "nl-sxm" => return "Nederlands (Sint Maarten)",
            "nn-nor" => return "Norsk nynorsk (Noreg)",
            "nr-zaf" => return "isiNdebele (South Africa)",
            "ny-mwi" => return "Chichewa (Malawi)",
            "oc-fra" => return "Occitan (France)", // TODO: Check this
            "om-eth" => return "Afaan Oromoo (Itoophiyaa)",
            "om-ken" => return "Afaan Oromoo (Keeniyaa)",
            "or-ind" => return "ଓଡ଼ିଆ (ଭାରତ)",
            "os-geo" => return "Ирон æвзаг (Росси)",
            "os-rus" => return "Ирон æвзаг (Росси)",
            "pa-ind" => return "ਪੰਜਾਬੀ (ਭਾਰਤ)",
            "pa-pak" => return "پنجابی (پاکستان)",
            "pl-pol" => return "Polski (Polska)",
            "ps-afg" => return "پښتو (افغانستان)",
            "ps-pak" => return "پښتو (پاکستان)",
            "pt-ago" => return "Português (Angola)",
            "pt-bra" => return "Português (Brasil)",
            "pt-che" => return "Português (Suíça)",
            "pt-cpv" => return "Português (Cabo Verde)",
            "pt-fra" => return "Português (França)",
            "pt-gnb" => return "Português (Guiné-Bissau)",
            "pt-gnq" => return "Português (Guiné Equatorial)",
            "pt-lux" => return "Português (Luxemburgo)",
            "pt-mac" => return "Português (Macau)",
            "pt-moz" => return "Português (Moçambique)",
            "pt-prt" => return "Português (Portugal)",
            "pt-stp" => return "Português (São Tomé e Príncipe)",
            "pt-tls" => return "Português (Timor-Leste)",
            "qu-bol" => return "Runa simi (Bolivia)",
            "qu-ecu" => return "Runa simi (Ecuador)",
            "qu-per" => return "Runa simi (Perú)",
            "ro-mda" => return "Română (Republica Moldova)",
            "ro-rou" => return "Română (România)",
            "rn-bdi" => return "Ikirundi (Burundi)",
            "ru-blr" => return "Русский язык (Беларусь)",
            "ru-kaz" => return "Русский язык (Казахстан)",
            "ru-kgz" => return "Русский язык (Кыргызстан)",
            "ru-mda" => return "Русский язык (Молдова)",
            "ru-rus" => return "Русский язык (Россия)",
            "ru-ukr" => return "Русский язык (Украина)",
            "rw-rwa" => return "Kinyarwanda (Rwanda)",
            "sa-ind" => return "संस्कृतम् (भारतम्)",
            "sa-ita" => return "संस्कृतम् (इटली)",
            "sd-pak" => return "سنڌي (پاڪستان)",
            "se-fin" => return "Davvisámegiella (Suopma)",
            "se-nor" => return "Davvisámegiella (Norga)",
            "se-swe" => return "Davvisámegiella (Ruoŧŧa)",
            "sg-caf" => return "Sängö (République centrafricaine)",
            "sk-svk" => return "Slovenčina (Slovenská republika)",
            "si-lka" => return "සිංහල (ශ්‍රී ලංකා)",
            "sl-svn" => return "Slovenščina (Slovenija)",
            "sn-zwe" => return "chiShona (Zimbabwe)",
            "so-eth" => return "Soomaaliga (Itoobiya)",
            "so-ken" => return "Soomaaliga (Kenya)",
            "so-som" => return "Soomaaliga (Soomaaliya)",
            "sq-alb" => return "Shqip (Shqipëri)",
            "sq-mkd" => return "Shqip (Maqedoni)",
            "sq-xkk" => return "Shqip (Kosovë)",
            "sr-srb" => return "Српски (Србија)",
            "st-lso" => return "Sesotho (Lesotho)",
            "st-zaf" => return "Sesotho (South Africa)",
            "sr-bih" => return "Српски (Босна и Херцеговина)",
            "sr-mne" => return "Српски (Црна Гора)",
            "sr-xkk" => return "Српски (Косово)",
            "ss-swz" => return "SiSwati (Swaziland)",
            "ss-zaf" => return "SiSwati (South Africa)",
            "sv-ala" => return "Svenska (Åland)",
            "sv-fin" => return "Svenska (Finland)",
            "sw-cod" => return "Kiswahili (Jamhuri ya Kidemokrasia ya Kongo)",
            "sw-ken" => return "Kiswahili (Kenya)",
            "sw-tza" => return "Kiswahili (Tanzania)",
            "ta-ind" => return "தமிழ் (இந்தியா)",
            "ta-sgp" => return "தமிழ் (சிங்கப்பூர்)",
            "ta-lka" => return "தமிழ் (இலங்கை)",
            "ta-mys" => return "தமிழ் (மலேசியா)",
            "te-ind" => return "తెలుగు (భారత)",
            "tg-tjk" => return "тоҷикӣ (Тоҷикистон)",
            "th-tha" => return "ไทย (ไทย)",
            "ti-eri" => return "ትግርኛ (ኤርትራ)",
            "ti-eth" => return "ትግርኛ (ኢትዮጵያ)",
            "tk-tkm" => return "Türkmen (Türkmenistan)",
            "tn-bwa" => return "Setswana (Botswana)",
            "tn-zaf" => return "Setswana (South Africa)",
            "to-ton" => return "faka Tonga (Tonga)",
            "tr-cyp" => return "Türkçe (Kıbrıs)",
            "tr-tur" => return "Türkçe (Türkiye)",
            "ts-zaf" => return "Xitsonga (South Africa)",
            "tt-rus" => return "Татар теле (Россия)",
            "ug-chn" => return "ئۇيغۇرچە (جۇڭگو)",
            "uk-ukr" => return "Українська (Україна)",
            "ur-ind" => return "اردو (بھارت)",
            "ur-pak" => return "اردو (پاکستان)",
            "uz-afg" => return "O'zbekiston (Afg'oniston)",
            "uz-uzb" => return "O'zbekiston (O'zbekiston)",
            "ve-zaf" => return "Tshivenḓa (South Africa)",
            "vi-vnm" => return "Tiếng Việt (Việt Nam)",
            "wa-bel" => return "Walon (Belgique)",
            "wo-sen" => return "Wolof (Sénégal)", // TODO: Check this, seems French
            "xh-zaf" => return "isiXhosa (South Africa)",
            "yi-001" => return "ייִדיש",
            "yo-ben" => return "Yorùbá (Bénin)",
            "yo-nga" => return "Yorùbá (Nàìjíríà)",
            "zh-chn" => return "中文 (简体)",
            "zh-hkg" => return "中文 (香港)",
            "zh-mac" => return "中文 (澳門)",
            "zh-twn" => return "中文 (繁體)",
            "zh-sgp" => return "中文 (新加坡)",
            "zu-zaf" => return "isiZulu (South Africa)",

            // lang (3 letter) -> country (2 letter)
            //   Cabilian language, a Berber language spoken in Algeria
            "kab-dz" => return "θɐqβæjlɪθ (Asenǧaq n Dzayer)",
            //   Chakma language, an Indo-Aryan language spoken in Bangladesh
            //   TODO: How is written "Bangladesh" in Chakma language?
            "cpp-bd" => return "𑄌𑄋𑄴𑄟𑄳𑄦 𑄞𑄌𑄴 (Bangladesh)",

            // lang (3 letter) -> country (3 letter)
            "kab-dza" => return "θɐqβæjlɪθ (Asenǧaq n Dzayer)",
            "cpp-bgd" => return "𑄌𑄋𑄴𑄟𑄳𑄦 𑄞𑄌𑄴 (Bangladesh)",

            // Followed this table: https://www.fincher.org/Utilities/CountryLanguageList.shtml
            //
            // TODO:
            // - Next dialects are iso639-3 codes. Investigate them:
            //    + 'agq-CM' and 'agq-CMR'
            //    + 'ksf-CM' and 'ksf-CMR'
            //    + 'bas-CM' and 'bas-CMR'
            //    + 'dua-CM' and 'dua-CMR'
            //    + 'ewo-CM' and 'ewo-CMR'
            //    + 'kkj-CM' and 'kkj-CMR'
            //    + 'nmg-CM' and 'nmg-CMR'
            //    + 'mgo-CM' and 'mgo-CMR'
            //    + 'mua-CM' and 'mua-CMR'
            //    + 'nnh-CM' and 'nnh-CMR'
            //    + 'jgo-CM' and 'jgo-CMR'
            //    + 'yav-CM' and 'yav-CMR'
            //    + 'moh-CA' and 'moh-CAN'
            //    + 'kea-CV' and 'kea-CPV'
            //    + 'arn-CL' and 'arn-CHL'
            //    + 'yue-CN' and 'yue-CHN'
            //    + 'byn-ER' and 'byn-ERI'
            //    + 'gez-ER' and 'gez-ERI'
            //    + 'tig-ER' and 'tig-ERI'
            //    + 'gez-ET' and 'gez-ETH'
            //    + 'wal-ET' and 'wal-ETH'
            //    + 'smn-FI' and 'smn-FIN'
            //    + 'gsw-FR' and 'gsw-FRA'
            //    + 'ksh-DE' and 'ksh-DEU'
            //    + 'nds-DE' and 'nds-DEU'
            //    + 'dsb-DE' and 'dsb-DEU'
            //    + 'hsb-DE' and 'hsb-DEU'
            //    + 'gaa-GH' and 'gaa-GHA'
            //    + 'kpe-GN' and 'kpe-GIN'
            //    + 'nqo-GN' and 'nqo-GIN'
            //    + 'yue-HK' and 'yue-HKG'
            //    + 'brx-IN' and 'brx-IND'
            //    + 'ccp-IN' and 'ccp-IND'
            //    + 'kok-IN' and 'kok-IND'
            //    + 'mni-IN' and 'mni-IND'
            //    + 'sat-IN' and 'sat-IND'
            //    + 'ckb-IR' and 'ckb-IRN'
            //    + 'mzn-IR' and 'mzn-IRN'
            //    + 'lrc-IR' and 'lrc-IRN'
            //    + 'ckb-IQ' and 'ckb-IRQ'
            //    + 'lrc-IQ' and 'lrc-IRQ'
            //    + 'syr-IQ' and 'syr-IRQ'
            //    + 'fur-IT' and 'fur-ITA'
            //    + 'scn-IT' and 'scn-ITA'
            //    + 'ebu-KE' and 'ebu-KEN'
            //    + 'guz-KE' and 'guz-KEN'
            //    + 'kln-KE' and 'kln-KEN'
            //    + 'kam-KE' and 'kam-KEN'
            //    + 'luo-KE' and 'luo-KEN'
            //    + 'luy-KE' and 'luy-KEN'
            //    + 'mas-KE' and 'mas-KEN'
            //    + 'mer-KE' and 'mer-KEN'
            //    + 'saq-KE' and 'saq-KEN'
            //    + 'dav-KE' and 'dav-KEN'
            //    + 'teo-KE' and 'teo-KEN'
            //    + 'kpe-LR' and 'kpe-LBR'
            //    + 'vai-LR' and 'vai-LBR'
            //    + 'gsw-LI' and 'gsw-LIE'
            //    + 'khq-ML' and 'khq-MLI'
            //    + 'ses-ML' and 'ses-MLI'
            //    + 'mfe-MU' and 'mfe-MUS'
            //    + 'tzm-MA' and 'tzm-MAR'
            //    + 'zgh-MA' and 'zgh-MAR'
            //    + 'shi-MA' and 'shi-MAR'
            //    + 'mgh-MZ' and 'mgh-MOZ'
            //    + 'seh-MZ' and 'seh-MOZ'
            //    + 'naq-NA' and 'naq-NAM'
            //    + 'nds-NL' and 'nds-NLD'
            //    + 'twq-NE' and 'twq-NER'
            //    + 'dje-NE' and 'dje-NER'
            //    + 'kaj-NG' and 'kaj-NGA'
            //    + 'kcg-NG' and 'kcg-NGA'
            //    + 'ceb-PH' and 'ceb-PHL'
            //    + 'fil-PH' and 'fil-PHL'
            //    + 'myv-RU' and 'myv-RUS'
            //    + 'sah-RU' and 'sah-RUS'
            //    + 'dyo-SN' and 'dyo-SEN'
            //    + 'nso-ZA' and 'nso-ZAF'
            //    + 'nus-SS' and 'nus-SSD'
            //    + 'ast-ES' and 'ast-ESP'
            //    + 'gsw-CH' and 'gsw-CHE'
            //    + 'wae-CH' and 'wae-CHE'
            //    + 'syr-SY' and 'syr-SYR'
            //    + 'trv-TW' and 'trv-TWN'
            //    + 'asa-TZ' and 'asa-TZA'
            //    + 'bez-TZ' and 'bez-TZA'
            //    + 'lag-TZ' and 'lag-TZA'
            //    + 'jmc-TZ' and 'jmc-TZA'
            //    + 'kde-TZ' and 'kde-TZA'
            //    + 'mas-TZ' and 'mas-TZA'
            //    + 'rof-TZ' and 'rof-TZA'
            //    + 'rwk-TZ' and 'rwk-TZA'
            //    + 'sbp-TZ' and 'sbp-TZA'
            //    + 'ksb-TZ' and 'ksb-TZA'
            //    + 'vun-TZ' and 'vun-TZA'
            //    + 'cgg-UG' and 'cgg-UGA'
            //    + 'nyn-UG' and 'nyn-UGA'
            //    + 'xog-UG' and 'xog-UGA'
            //    + 'teo-UG' and 'teo-UGA'
            //    + 'chr-US' and 'chr-USA'
            //    + 'haw-US' and 'haw-USA'
            //    + 'lkt-US' and 'lkt-USA'
            //    + 'jbo-001'
            //    + 'bem-ZM' and 'bem-ZMB'
            _ => {}
        }
    }

    let c = code_to_iso639(code);
    match c.as_str() {
        "aa" => "’Afar Af",
        "ab" => "Аҧсуа бызшәа",
        "ae" => "Avestan",
        "af" => "Afrikaans",
        "ak" => "Akan",
        "am" => "አማርኛ",
        "an" => "Aragonés",
        "ar" => "العربية",
        "as" => "অসমীয়া",
        "av" => "Магӏарул мацӏ",
        "ay" => "Aymar aru",
        "az" => "Azərbaycan dili",
        "ba" => "Башҡорт теле",
        "be" => "Беларуская мова",
        "bg" => "български език",
        "bi" => "Bislama",
        "bm" => "ߓߊߡߊߣߊߣߞߊߣ",
        "bn" => "বাংলা",
        "bo" => "བོད་སྐད་",
        "br" => "Brezhoneg",
        "bs" => "Bosanski",
        "ca" => "Català",
        "ce" => "Нохчийн мотт",
        "ch" => "Finu' Chamoru",
        "co" => "Corsu",
        "cr" => "Cree",
        "cs" => "čeština",
        "cu" => "Славе́нскїй ѧ҆зы́къ",
        "cv" => "Чӑвашла",
        "cy" => "Cymraeg",
        "da" => "Dansk",
        "de" => "Deutsch",
        "dv" => "ދިވެހި",
        "dz" => "རྫོང་ཁ་",
        "ee" => "Èʋegbe",
        "el" => "Νέα Ελληνικά",
        "en" => "English",
        "eo" => "Esperanto",
        "es" => "Español",
        "et" => "Eesti keel",
        "eu" => "Euskara",
        "fa" => "فارسی",
        "ff" => "Fulfulde",
        "fi" => "Suomen kieli",
        "fj" => "Na Vosa Vakaviti",
        "fo" => "Føroyskt",
        "fr" => "Français",
        "fy" => "Frysk",
        "ga" => "Gaeilge",
        "gd" => "Gàidhlig",
        "gl" => "Galego",
        "gn" => "Avañe'ẽ",
        "gu" => "ગુજરાતી",
        "gv" => "Gaelg",
        "ha" => "Harshen Hausa",
        "he" => "עברית",
        "hi" => "हिन्दी",
        "ho" => "Hiri Motu",
        "hr" => "Hrvatski",
        "ht" => "Kreyòl Ayisyen",
        "hu" => "Magyar nyelv",
        "hy" => "Հայերէն",
        "hz" => "Otjiherero",
        "ia" => "Interlingua",
        "id" => "Bahasa Indonesia",
        "ie" => "Interlingue",
        "ig" => "Asụsụ Igbo",
        "ii" => "ꆈꌠꉙ",
        "ik" => "Iñupiaq",
        "io" => "Ido",
        "is" => "Íslenska",
        "it" => "Italiano",
        "iu" => "ᐃᓄᒃᑎᑐᑦ",
        "ja" => "日本語",
        "jv" => "ꦧꦱꦗꦮ",
        "ka" => "ქართული",
        "kg" => "Kikongo",
        "ki" => "Gĩkũyũ",
        "kj" => "Kuanyama",
        "kk" => "Қазақ тілі",
        "kl" => "Kalaallisut",
        "km" => "ភាសាខ្មែរ",
        "kn" => "ಕನ್ನಡ",
        "ko" => "한국어",
        "kr" => "Kanuri",
        "ks" => "कॉशुर",
        "ku" => "Kurdî",
        "kv" => "Коми кыв",
        "kw" => "Kernowek",
        "ky" => "Кыргызстандык",
        "la" => "Lingua latīna",
        "lb" => "Lëtzebuergesch",
        "lg" => "Luganda",
        "li" => "Lèmburgs",
        "ln" => "Lingala",
        "lo" => "ພາສາລາວ",
        "lt" => "Lietuvių kalba",
        "lu" => "Kiluba",
        "lv" => "Latviešu valoda",
        "mg" => "Malagasy",
        "mh" => "Kajin M̧ajeļ",
        "mi" => "Te Reo Māori",
        "mk" => "Македонски јазик",
        "ml" => "മലയാളം",
        "mn" => "Монгол хэл",
        "mr" => "मराठी",
        "ms" => "Bahasa Melayu",
        "mt" => "Malti",
        "my" => "မြန်မာစာ",
        "na" => "Dorerin Naoero",
        "nb" => "Norsk Bokmål",
        "nd" => "SiNdebele saSeNyakatho",
        "ne" => "नेपाली भाषा",
        "ng" => "Ndonga",
        "nl" => "Nederlands",
        "nn" => "Norsk Nynorsk",
        "no" => "Norsk",
        "nr" => "SiNdebele saSewula",
        "nv" => "Diné bizaad",
        "ny" => "Chichewa",
        "oc" => "Occitan",
        "oj" => "Ojibwe",
        "om" => "Afaan Oromoo",
        "or" => "ଓଡ଼ିଆ",
        "os" => "Ирон ӕвзаг",
        "pa" => "ਪੰਜਾਬੀ",
        "pi" => "Pāli",
        "pl" => "Język polski",
        "ps" => "پښتو",
        "pt" => "Português",
        "qu" => "Runa simi",
        "rm" => "Rumantsch",
        "rn" => "Ikirundi",
        "ro" => "Limba română",
        "ru" => "Русский язык",
        "rw" => "Ikinyarwanda",
        "sa" => "संस्कृतम्",
        "sc" => "Sardu",
        "sd" => "سنڌي",
        "se" => "Davvisámegiella",
        "sg" => "Yângâ tî Sängö",
        "si" => "සිංහල",
        "sk" => "Slovenčina",
        "sl" => "Slovenščina",
        "sm" => "Gagana faʻa Sāmoa",
        "sn" => "ChiShona",
        "so" => "Af Soomaali",
        "sq" => "Shqip",
        "sr" => "Српски",
        "ss" => "SiSwati",
        "st" => "Sesotho",
        "su" => "ᮘᮞ ᮞᮥᮔ᮪ᮓ",
        "sv" => "Svenska",
        "sw" => "Kiswahili",
        "ta" => "தமிழ்",
        "te" => "తెలుగు",
        "tg" => "Тоҷикӣ",
        "th" => "ภาษาไทย",
        "ti" => "ትግርኛ",
        "tk" => "Türkmençe",
        "tl" => "Wikang Tagalog",
        "tn" => "Setswana",
        "to" => "Lea faka-Tonga",
        "tr" => "Türkçe",
        "ts" => "Xitsonga",
        "tt" => "Татар теле",
        "tw" => "Twi",
        "ty" => "Reo Tahiti",
        "ug" => "ئۇيغۇرچە",
        "uk" => "Українська",
        "ur" => "اردو",
        "uz" => "Oʻzbekcha",
        "ve" => "Tshivenḓa",
        "vi" => "Tiếng Việt",
        "vo" => "Volapük",
        "wa" => "Walon",
        "wo" => "Wollof",
        "xh" => "isiXhosa",
        "yi" => "ייִדיש",
        "yo" => "Yorùbá",
        "za" => "Saɯ cueŋƅ",
        "zh" => "中文",
        "zu" => "isiZulu",
        _ => panic!("Language name for language code '{code}' not found",),
    }
}
