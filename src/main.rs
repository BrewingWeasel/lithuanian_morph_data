use std::fs;

fn main() {
    println!("Getting declension of all nouns that end with is.");
    let values = get_all_is_words();
    let only_third = values.iter().filter(|(_, t)| *t == "3");
    for (word, _) in only_third.collect::<Vec<&(String, &str)>>() {
        println!("{word}")
    }
}

fn get_all_is_words<'a>() -> Vec<(String, &'a str)> {
    let specific_is = [
        ("io", "1"),
        ("ies", "3"),
        ("imi", "3"),
        ("ie", "3"),
        ("ys", "3"),
        ("iams", "1"),
        ("ims", "3"),
        ("imis", "3"),
        ("iais", "1"),
        ("iuose", "1"),
        ("yse", "3"),
    ];
    let mut word_data = Vec::new();
    let mut last_lemma = "";

    let file_data = fs::read_to_string("word_data/dazninis").unwrap();
    let mut skip_future = false;

    for line in file_data.lines() {
        let mut values = line.split('\t');
        let new_lemma = values.next().unwrap();
        if values.next().unwrap() != "dkt" {
            continue;
        }
        if new_lemma == last_lemma {
            if new_lemma.ends_with("is") && !skip_future {
                let declined = values.next().unwrap();
                for (ending, declension_num) in specific_is {
                    if declined.ends_with(ending) {
                        word_data.push((new_lemma.to_owned(), declension_num));
                        // println!("{} {}", new_lemma.to_owned(), declension_num);
                        skip_future = true;
                        break;
                    }
                }
            }
        } else {
            last_lemma = new_lemma;
            skip_future = false;
        }
    }
    word_data
}
