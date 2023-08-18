use std::{collections::HashMap, fs};

fn main() {
    let vals = all_of_each_type();
    for i in &vals["N"] {
        println!("{i}");
    }
}

fn all_of_each_type<'a>() -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let file_data = fs::read_to_string("source_data/JCL_lemmas.txt").unwrap();
    let mut last_word = "";
    map.insert(String::from("MULT"), Vec::new());

    for line in file_data.lines() {
        let mut values = line.split('\t');
        let word = values.next().unwrap();
        if word == last_word {
            map.entry(String::from("MULT"))
                .and_modify(|x| x.push(word.to_owned()));
            continue;
        } else {
            last_word = word;
        }
        let pos = values.next().unwrap();
        map.entry(pos.to_owned())
            .and_modify(|x| x.push(word.to_owned()))
            .or_insert(vec![word.to_owned()]);
    }
    map
}

fn get_all_is_words<'a>() -> Vec<(String, &'a str)> {
    let specific_is = [
        ("io", "1"),
        ("iu", "1"),
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

    let file_data = fs::read_to_string("source_data/dazninis").unwrap();
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
