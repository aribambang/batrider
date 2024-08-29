fn next_language<'a>(languages: &'a[String], current: &str) -> &'a str {
    let mut found = false;

    for lang in languages {
        if found {
            return lang;
        }

        if lang == current {
            found = true
        };
    }

    languages.last().unwrap()
}

fn last_language(languages: &[String]) -> &str {
    languages.last().unwrap()
}

fn longest<'a>(lang_a: &'a str, lang_b: &'a str) -> &'a str {
    if lang_a.len() > lang_b.len() {
        lang_a
    } else {
        lang_b
    }
}

fn main() {
    let languages = vec![
        String::from("Node.JS"),
        String::from("Go"),
        String::from("Rust"),
        String::from("Java"),
    ];

    // let result = next_language(&languages, "Go");
    // let result = last_language(&languages);
    let result = longest("go", "typescript");
    println!("{}", result);
}
