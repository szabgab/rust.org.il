use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};

pub type Partials = liquid::partials::EagerCompiler<liquid::partials::InMemorySource>;

#[derive(Deserialize, Debug, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
struct Project {
    name: String,
    home: Option<String>,
    source: String,
    authors: Vec<String>,

    #[serde(default = "get_default_empty_string")]
    slug: String,

    #[serde(default = "get_default_empty_vector_of_people")]
    people: Vec<Person>,

    #[serde(default = "get_default_empty_string")]
    body: String,
}

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

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
struct Person {
    name: String,
    linkedin: Option<String>,
    github: Option<String>,
    crates: Option<String>,
    home: Option<String>,

    #[serde(default = "get_default_empty_string")]
    slug: String,

    #[serde(default = "get_default_empty_string")]
    body: String,

    img: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
struct Presentaton {
    title: String,

    length: u8,

    language: Language,

    #[serde(default = "get_default_empty_vector_of_strings")]
    speakers: Vec<String>,

    #[serde(default = "get_default_empty_vector_of_people")]
    people: Vec<Person>,

    #[serde(default = "get_default_empty_string")]
    slug: String,

    #[serde(default = "get_default_empty_string")]
    body: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
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

fn get_default_empty_vector_of_people() -> Vec<Person> {
    Vec::new()
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
    let pages = load_pages();
    let people = load_people();
    let presentations = load_presentations(&people.clone());
    let events = load_events(&presentations.clone());
    let projects = load_projects(&people.clone());

    generate_people_pages(&people, &path);

    generate_event_pages(&events, &path);

    generate_presentation_pages(presentations, &path);

    generate_markdown_pages(pages, events, &path);

    generate_project_pages(projects, &people, &path);

    copy_static_files(&path);
}

fn copy_static_files(path: &Path) {
    for folder in &["img", "slides"] {
        let src = PathBuf::from(folder);
        let dest = path.join(folder);
        std::fs::create_dir_all(&dest).unwrap();
        for entry in
            std::fs::read_dir(&src).unwrap_or_else(|err| panic!("Could not read {src:?} {err}"))
        {
            let entry = entry.unwrap();
            let src = entry.path();
            let dest = dest.join(entry.file_name());
            std::fs::copy(&src, &dest).unwrap();
        }
    }
}

fn generate_project_pages(
    projects: HashMap<String, Project>,
    people: &HashMap<String, Person>,
    path: &Path,
) {
    let template = include_str!("../templates/projects.html");
    let globals = liquid::object!({
        "title": "Open Source Rust Projects developed by Israelis",
        "projects": projects.values().collect::<Vec<&Project>>(),
        "people": people.values().collect::<Vec<&Person>>(),
    });
    let projects_path = path.join("projects");
    std::fs::create_dir_all(&projects_path).unwrap();
    render_page(globals, template, projects_path.join("index.html")).unwrap();
}

fn generate_markdown_pages(pages: Vec<Page>, events: HashMap<String, Event>, path: &Path) {
    let mut future_events = events
        .values()
        .filter(|event| event.future)
        .collect::<Vec<&Event>>();
    future_events.sort_by(|a, b| a.date.cmp(&b.date));

    for page in &pages {
        if page.slug == "index" {
            let template = include_str!("../templates/index.html");
            let globals = liquid::object!({
                "title": page.title,
                "events": future_events,
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

fn generate_presentation_pages(presentations: HashMap<String, Presentaton>, path: &Path) {
    let template = include_str!("../templates/presentations.html");
    let globals = liquid::object!({
        "title": "Presentations",
        "presentations": presentations.values().collect::<Vec<&Presentaton>>(),
    });
    let presentations_path = path.join("presentations");
    std::fs::create_dir_all(&presentations_path).unwrap();
    render_page(globals, template, presentations_path.join("index.html")).unwrap();

    let template = include_str!("../templates/presentation.html");
    for presentation in presentations.values() {
        let globals = liquid::object!({
            "title": presentation.title,
            "presentation": presentation,
        });
        render_page(
            globals,
            template,
            presentations_path.join(format!("{}.html", presentation.slug)),
        )
        .unwrap();
    }
}

fn generate_event_pages(events: &HashMap<String, Event>, path: &Path) {
    let template = include_str!("../templates/events.html");
    let mut values = events.values().collect::<Vec<&Event>>();
    values.sort_by(|a, b| b.date.cmp(&a.date));
    let globals = liquid::object!({
        "title": "Events",
        "events": values,
    });
    let events_path = path.join("events");
    std::fs::create_dir_all(&events_path).unwrap();
    render_page(globals, template, events_path.join("index.html")).unwrap();

    let template = include_str!("../templates/event.html");
    for event in events.values() {
        let globals = liquid::object!({
            "title": event.title,
            "event": event,
        });
        render_page(
            globals,
            template,
            events_path.join(format!("{}.html", event.slug)),
        )
        .unwrap();
    }
}

fn generate_people_pages(people: &HashMap<String, Person>, path: &Path) {
    let template = include_str!("../templates/people.html");
    let mut values = people.values().collect::<Vec<&Person>>();
    values.sort_by(|a, b| a.name.cmp(&b.name));
    let globals = liquid::object!({
        "title": "People",
        "people": values,
    });
    let people_path = path.join("people");
    std::fs::create_dir_all(&people_path).unwrap();
    render_page(globals, template, people_path.join("index.html")).unwrap();

    let template = include_str!("../templates/person.html");
    for person in people.values() {
        let globals = liquid::object!({
            "title": person.name,
            "person": person,
        });
        render_page(
            globals,
            template,
            people_path.join(format!("{}.html", person.slug)),
        )
        .unwrap();
    }
}

fn load_projects(people: &HashMap<String, Person>) -> HashMap<String, Project> {
    let mut projects = HashMap::new();
    let paths = std::fs::read_dir("projects").unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.extension().unwrap() == "swp" {
            continue;
        }
        if path.file_name().unwrap() == "skeleton.md" {
            continue;
        }

        let (front_matter, body) = read_md_file_separate_front_matter(&path);
        let mut project: Project = serde_yaml::from_str(&front_matter)
            .unwrap_or_else(|err| panic!("Could not parse front matter in {path:?} {err}"));
        project.body = markdown2html(&body);
        project.slug = path.file_stem().unwrap().to_str().unwrap().to_string();
        let authors = project
            .authors
            .iter()
            .map(|speaker| people[speaker].clone())
            .collect::<Vec<_>>();
        //println!("{:?}", speakers);
        project.people = authors;

        let path_str = path.as_os_str().to_str().unwrap().to_string();
        projects.insert(path_str, project);
    }

    projects
}

fn load_pages() -> Vec<Page> {
    let mut pages = Vec::new();
    let paths = std::fs::read_dir("pages").unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.extension().unwrap() == "swp" {
            continue;
        }

        let (front_matter, body) = read_md_file_separate_front_matter(&path);
        let mut page: Page = serde_yaml::from_str(&front_matter)
            .unwrap_or_else(|err| panic!("Could not parse front matter in {path:?} {err}"));
        page.content = markdown2html(&body);
        page.slug = path.file_stem().unwrap().to_str().unwrap().to_string();

        pages.push(page);
    }
    pages
}

fn load_people() -> HashMap<String, Person> {
    let mut people = HashMap::new();
    let paths = std::fs::read_dir("people").unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.extension().unwrap() == "swp" {
            continue;
        }
        if path.file_name().unwrap() == "skeleton.md" {
            continue;
        }
        let (front_matter, body) = read_md_file_separate_front_matter(&path);
        let mut person: Person = serde_yaml::from_str(&front_matter).unwrap();
        person.slug = path.file_stem().unwrap().to_str().unwrap().to_string();
        person.body = markdown2html(&body);
        if let Some(img) = &person.img {
            let file = PathBuf::from(format!("img/{}", img));
            if !file.exists() {
                panic!("File '{file:?}' used in '{path:?}' does not exist");
            }
        }

        let path_str = path.as_os_str().to_str().unwrap().to_string();
        people.insert(path_str, person);
    }
    people
}

fn load_presentations(people: &HashMap<String, Person>) -> HashMap<String, Presentaton> {
    let mut presentations = HashMap::new();
    let paths = std::fs::read_dir("presentations").unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.extension().unwrap() == "swp" {
            continue;
        }
        if path.file_name().unwrap() == "skeleton.md" {
            continue;
        }

        let (front_matter, body) = read_md_file_separate_front_matter(&path);
        let mut presentation: Presentaton = serde_yaml::from_str(&front_matter).unwrap();
        presentation.body = markdown2html(&body);
        presentation.slug = path.file_stem().unwrap().to_str().unwrap().to_string();
        for speaker in &presentation.speakers {
            if !people.contains_key(speaker) {
                panic!("Speaker '{speaker}' in '{path:?}' does not exist");
            }
        }
        let speakers = presentation
            .speakers
            .iter()
            .map(|speaker| people[speaker].clone())
            .collect::<Vec<_>>();
        //println!("{:?}", speakers);
        presentation.people = speakers;

        let path_str = path.as_os_str().to_str().unwrap().to_string();
        presentations.insert(path_str, presentation);
    }
    presentations
}

fn load_events(presentatons: &HashMap<String, Presentaton>) -> HashMap<String, Event> {
    let utc: DateTime<Utc> = Utc::now();
    let today = utc.format("%Y.%m.%d").to_string();

    let mut events = HashMap::new();
    let paths = std::fs::read_dir("events").unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let (front_matter, body) = read_md_file_separate_front_matter(&path);
        let mut event: Event = serde_yaml::from_str(&front_matter)
            .unwrap_or_else(|err| panic!("Could not parse front matter in {path:?} {err}"));
        event.slug = path.file_stem().unwrap().to_str().unwrap().to_string();
        event.body = markdown2html(&body);
        event.future = event.date >= today;

        let presentations = event
            .schedule
            .iter()
            .map(|presentation_slug| presentatons[presentation_slug].clone())
            .collect::<Vec<_>>();
        event.presentations = presentations;

        let path_str = path.as_os_str().to_str().unwrap().to_string();
        events.insert(path_str, event);
    }

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
