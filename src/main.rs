use std::fs::File;
use std::io::Read;

use serde::Deserialize;
#[derive(Deserialize, Debug)]
struct Person {
    name: String,
    linkedin: String,
    github: String,
    home: String,
}

fn main() {
    let filename = "speakers.yaml";
    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            //let data: serde_yaml::Value = serde_yaml::from_str(&content).expect("YAML parsing error");
            //dbg!(data);

            let data: Vec<Person> = serde_yaml::from_str(&content).expect("YAML parsing error");
            dbg!(data);
        },
        Err(error) => {
            println!("Error opening file {}: {}", filename, error);
        },
    }

}
