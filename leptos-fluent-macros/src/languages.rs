use std::fs;
use std::path::PathBuf;

pub(crate) fn read_languages_file(path: &PathBuf) -> Vec<(String, String)> {
    let file_extension = path.extension().unwrap_or_default();
    if file_extension == "json" {
        serde_json::from_str::<Vec<Vec<String>>>(
            fs::read_to_string(path)
                .expect("Couldn't read languages file")
                .as_str(),
        )
        .expect("Invalid JSON")
        .iter()
        .map(|lang| (lang[0].clone(), lang[1].clone()))
        .collect::<Vec<(String, String)>>()
    } else {
        panic!("The languages file should be a JSON file. Found file extension {:?}", file_extension);
    }
}

pub(crate) fn read_locales_folder(path: &PathBuf) -> Vec<(String, String)> {
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
    for entry in entries.iter() {
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
        let lang_name = language_name_from_language_code(
            lang_code.as_str(),
            use_country_code,
        );
        locales.push((lang_code, lang_name.to_string()));
    }
    locales.sort_by(|a, b| a.1.cmp(&b.1));
    locales
}

pub(crate) fn generate_code_for_static_language(
    id: &str,
    name: &str,
) -> String {
    format!(
        concat!(
            "&::leptos_fluent::Language{{",
            "id: ::unic_langid::langid!(\"{}\"),",
            "name: \"{}\"",
            "}}",
        ),
        id, name
    )
}

fn code_to_iso639(code: &str) -> String {
    let mut c = code.to_string();
    if code.contains('_') {
        c = code.split('_').collect::<Vec<&str>>()[0].to_string()
    } else if code.contains('-') {
        c = code.split('-').collect::<Vec<&str>>()[0].to_string()
    }
    c.to_lowercase()
}

fn language_name_from_language_code(
    code: &str,
    use_country_code: bool,
) -> &'static str {
    if use_country_code {
        let c = code.to_string().to_lowercase();
        match c.as_str() {
            "en-us" => return "English (United States)",
            "en-gb" => return "English (United Kingdom)",
            "en-ca" => return "English (Canada)",
            "en-au" => return "English (Australia)",
            "en-nz" => return "English (New Zealand)",
            "en-ie" => return "English (Ireland)",
            "en-za" => return "English (South Africa)",
            "en-jm" => return "English (Jamaica)",
            "en-bz" => return "English (Belize)",
            "en-tt" => return "English (Trinidad and Tobago)",
            "es-mx" => return "Español (México)",
            "es-es" => return "Español (España)",
            "es-co" => return "Español (Colombia)",
            "es-ar" => return "Español (Argentina)",
            "es-cl" => return "Español (Chile)",
            "es-pe" => return "Español (Perú)",
            "es-ve" => return "Español (Venezuela)",
            "es-ec" => return "Español (Ecuador)",
            "es-gt" => return "Español (Guatemala)",
            "es-cu" => return "Español (Cuba)",
            "es-bo" => return "Español (Bolivia)",
            "es-do" => return "Español (República Dominicana)",
            "es-hn" => return "Español (Honduras)",
            "es-py" => return "Español (Paraguay)",
            "es-sv" => return "Español (El Salvador)",
            "es-ni" => return "Español (Nicaragua)",
            "es-pr" => return "Español (Puerto Rico)",
            "es-uy" => return "Español (Uruguay)",
            "es-pa" => return "Español (Panamá)",
            "es-cr" => return "Español (Costa Rica)",
            "pt-br" => return "Português (Brasil)",
            "pt-pt" => return "Português (Portugal)",
            "zh-cn" => return "中文 (简体)",
            "zh-hk" => return "中文 (香港)",
            "zh-tw" => return "中文 (繁體)",
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
        "ky" => "Кыргызча",
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
        _ => panic!("Language name for language code '{}' not found", code,),
    }
}
