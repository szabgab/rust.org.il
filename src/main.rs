use std::path::PathBuf;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Person {
    name: String,
    linkedin: String,
    github: String,
    home: String,
}

fn main() {
    let people = load_people();
    for person in people {
        println!("{:?}", person);
    }
}

fn load_people() -> Vec<Person> {
    let mut people = Vec::new();
    let paths = std::fs::read_dir("people").unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let (front_matter, _body) = read_md_file_separate_front_matter(&path);
        let person: Person = serde_yaml::from_str(&front_matter).unwrap();
        people.push(person);
    }
    people
}

fn read_md_file_separate_front_matter(path: &PathBuf) -> (String, String) {
    let content =
        std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Could not read file {path:?}"));
    let parts = content.split("---").collect::<Vec<_>>();
    assert!(parts.len() == 3, "File {path:?} does not have front matter");
    (parts[1].to_string(), parts[2].to_string())
}
