use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

pub type Partials = liquid::partials::EagerCompiler<liquid::partials::InMemorySource>;

#[derive(Deserialize, Debug, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
struct Page {
    title: String,
    description: String,

    #[serde(default = "get_default_empty_string")]
    slug: String,

    #[serde(default = "get_default_empty_string")]
    content: String,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
struct Person {
    name: String,
    linkedin: String,
    github: String,
    home: String,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
struct Presentaton {
    title: String,

    length: u8,

    language: Language,

    #[serde(default = "get_default_empty_vector_of_strings")]
    speakers: Vec<String>,

    #[serde(default = "get_default_empty_string")]
    slug: String,

    #[serde(default = "get_default_empty_string")]
    body: String,
}

#[derive(Deserialize, Debug, Serialize)]
enum Language {
    English,
    Hebrew,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
struct Event {
    date: String,
    title: String,
    online: bool,

    register: String,

    #[serde(default = "get_default_empty_string")]
    slug: String,

    #[serde(default = "get_default_empty_string")]
    body: String,

    #[serde(default = "get_default_false")]
    future: bool,

    language: Language,

    #[serde(default = "get_default_empty_vector_of_strings")]
    schedule: Vec<String>,

    #[serde(default = "get_default_empty_vector_of_presentations")]
    presentations: Vec<Presentaton>,
}

fn get_default_empty_vector_of_strings() -> Vec<String> {
    Vec::new()
}
fn get_default_empty_vector_of_presentations() -> Vec<Presentaton> {
    Vec::new()
}

fn get_default_false() -> bool {
    false
}

fn get_default_empty_string() -> String {
    String::new()
}

fn main() {
    // let utc: DateTime<Utc> = Utc::now();
    // let today = utc.format("%Y.%m.%d");

    let path = std::path::PathBuf::from("_site");
    let people = load_people();
    let presentations = load_presentations();
    let events = load_events();
    let pages = load_pages();

    let template = include_str!("../templates/people.html");
    let globals = liquid::object!({
        "title": "People",
        "people": people,
    });
    let people_path = path.join("people");
    std::fs::create_dir_all(&people_path).unwrap();
    render_page(globals, template, people_path.join("index.html")).unwrap();

    let template = include_str!("../templates/events.html");
    let globals = liquid::object!({
        "title": "Events",
        "events": events,
    });
    let events_path = path.join("events");
    std::fs::create_dir_all(&events_path).unwrap();
    render_page(globals, template, events_path.join("index.html")).unwrap();

    let template = include_str!("../templates/presentations.html");
    let globals = liquid::object!({
        "title": "Presentations",
        "presentations": presentations.values().collect::<Vec<&Presentaton>>(),
    });
    let presentations_path = path.join("presentations");
    std::fs::create_dir_all(&presentations_path).unwrap();
    render_page(globals, template, presentations_path.join("index.html")).unwrap();

    for page in &pages {
        if page.slug == "index" {
            let template = include_str!("../templates/index.html");
            let globals = liquid::object!({
                "title": page.title,
                "events": events,
                "content": page.content,
            });
            render_page(globals, template, path.join("index.html")).unwrap();
        } else {
            let template = include_str!("../templates/page.html");
            let globals = liquid::object!({
                "title": page.title,
                "content": page.content,
            });
            render_page(globals, template, path.join(format!("{}.html", page.slug))).unwrap();
        }
    }
}

fn load_pages() -> Vec<Page> {
    let mut pages = Vec::new();
    let paths = std::fs::read_dir("pages").unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let (front_matter, body) = read_md_file_separate_front_matter(&path);
        let mut page: Page = serde_yaml::from_str(&front_matter).unwrap();
        page.content = markdown2html(&body);
        page.slug = path.file_stem().unwrap().to_str().unwrap().to_string();

        pages.push(page);
    }
    pages
}

fn load_presentations() -> HashMap<String, Presentaton> {
    let mut presentations = HashMap::new();
    let paths = std::fs::read_dir("presentations").unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let (front_matter, body) = read_md_file_separate_front_matter(&path);
        let mut presentation: Presentaton = serde_yaml::from_str(&front_matter).unwrap();
        presentation.body = markdown2html(&body);
        presentation.slug = path.file_stem().unwrap().to_str().unwrap().to_string();

        let path_str = path.as_os_str().to_str().unwrap().to_string();
        presentations.insert(path_str, presentation);
    }
    presentations
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

fn load_events() -> Vec<Event> {
    let utc: DateTime<Utc> = Utc::now();
    let today = utc.format("%Y.%m.%d").to_string();

    let mut events = Vec::new();
    let paths = std::fs::read_dir("events").unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let (front_matter, body) = read_md_file_separate_front_matter(&path);
        let mut event: Event = serde_yaml::from_str(&front_matter)
            .unwrap_or_else(|err| panic!("Could not parse front matter in {path:?} {err}"));
        event.slug = path.file_stem().unwrap().to_str().unwrap().to_string();
        event.body = markdown2html(&body);
        event.future = event.date >= today;
        events.push(event);
    }
    events.sort_by(|a, b| b.date.cmp(&a.date));
    events
}

fn read_md_file_separate_front_matter(path: &PathBuf) -> (String, String) {
    let content =
        std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Could not read file {path:?}"));
    let parts = content.split("---").collect::<Vec<_>>();
    assert!(parts.len() == 3, "File {path:?} does not have front matter");
    (parts[1].to_string(), parts[2].to_string())
}

fn render_page(
    globals: liquid::Object,
    template: &str,
    path: PathBuf,
) -> Result<(), Box<dyn Error>> {
    let partials = load_templates()?;

    let template = liquid::ParserBuilder::with_stdlib()
        .partials(partials)
        .build()?
        .parse(template)?;

    let content = template.render(&globals)?;

    std::fs::write(path, content)?;

    Ok(())
}

pub fn load_templates() -> Result<Partials, Box<dyn Error>> {
    let mut partials = Partials::empty();
    partials.add(
        "templates/incl/header.html",
        include_str!("../templates/incl/header.html"),
    );
    partials.add(
        "templates/incl/footer.html",
        include_str!("../templates/incl/footer.html"),
    );
    partials.add(
        "templates/incl/navigation.html",
        include_str!("../templates/incl/navigation.html"),
    );
    Ok(partials)
}

fn markdown2html(content: &str) -> String {
    markdown::to_html_with_options(
        content,
        &markdown::Options {
            compile: markdown::CompileOptions {
                allow_dangerous_html: true,
                //allow_dangerous_protocol: true,
                ..markdown::CompileOptions::default()
            },
            ..markdown::Options::gfm()
        },
    )
    .unwrap()
}
