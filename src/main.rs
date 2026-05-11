use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};

pub type Partials = liquid::partials::EagerCompiler<liquid::partials::InMemorySource>;

struct Community {
    data_root: PathBuf,
    pages: Vec<Page>,
    people: HashMap<String, Person>,
    companies: HashMap<String, Company>,
    jobs: HashMap<String, Job>,
    presentations: HashMap<String, Presentation>,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
struct Company {
    name: String,
    url: String,
    linkedin: Option<String>,
    links: Option<Vec<String>>,
    github: Option<String>,

    #[serde(default = "get_default_empty_string")]
    slug: String,

    #[serde(default = "get_default_empty_string")]
    body: String,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
struct Job {
    title: String,
    company: String,
    posted: String,  // YYYY.MM.DD
    updated: String, // YYYY.MM.DD
    contacts: Vec<String>,
    #[serde(default = "get_default_empty_vector_of_people")]
    people: Vec<Person>,

    #[serde(default = "get_default_empty_string")]
    slug: String,

    #[serde(default = "get_default_empty_string")]
    body: String,
}

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
    phone: Option<String>,

    #[serde(default = "get_default_empty_string")]
    slug: String,

    #[serde(default = "get_default_empty_string")]
    body: String,

    img: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
struct Presentation {
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

    video_he: Option<String>,
    video_en: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
struct Break {
    title: String,

    length: u8,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
enum ScheduleItem {
    Presentation(Presentation),
    Break(Break),
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
    time: String,
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

    #[serde(default = "get_default_empty_vector_of_schedule_items")]
    schedule_items: Vec<ScheduleItem>,
}

fn get_default_empty_vector_of_people() -> Vec<Person> {
    Vec::new()
}

fn get_default_empty_vector_of_strings() -> Vec<String> {
    Vec::new()
}
fn get_default_empty_vector_of_schedule_items() -> Vec<ScheduleItem> {
    Vec::new()
}

fn get_default_false() -> bool {
    false
}

fn get_default_empty_string() -> String {
    String::new()
}

fn main() -> Result<(), Box<dyn Error>> {
    // let utc: DateTime<Utc> = Utc::now();
    // let today = utc.format("%Y.%m.%d");

    let args: Vec<String> = std::env::args().collect();
    println!("args: {args:?}");

    let root = if args.len() > 1 {
        std::path::PathBuf::from(&args[1])
    } else {
        std::env::current_dir()?
    };
    let mut community = Community {
        data_root: root.clone(),
        pages: Vec::new(),
        people: HashMap::new(),
        companies: HashMap::new(),
        jobs: HashMap::new(),
        presentations: HashMap::new(),
    };

    validate_root(&root)?;

    let path = std::path::PathBuf::from("_site");
    community.load_pages();
    community.load_people();
    community.load_companies();
    community.load_jobs();
    community.load_presentations();
    let events = load_events(&root, &community.presentations.clone())?;
    let projects = load_projects(&root, &community.people.clone());

    generate_people_pages(
        &community.people,
        &community.presentations,
        &projects,
        &path,
    );

    generate_event_pages(&events, &path);

    generate_presentation_pages(&community.presentations, &events, &path);

    generate_videos_page(&community.presentations, &path);

    generate_markdown_pages(community.pages, events, &path);

    generate_project_pages(projects, &community.people, &path);
    generate_job_pages(community.jobs, &community.people, &path);
    copy_static_files(&path);
    generate_companies_pages(&community.companies, &path);

    Ok(())
}

fn validate_root(root: &Path) -> Result<(), Box<dyn Error>> {
    if !root.exists() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Data root '{}' does not exist", root.display()),
        )));
    }

    if !root.is_dir() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Data root '{}' is not a directory", root.display()),
        )));
    }

    for folder in [
        "pages",
        "people",
        "companies",
        "jobs",
        "presentations",
        "events",
        "projects",
        "img",
        "slides",
    ] {
        let path = root.join(folder);
        if !path.is_dir() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!(
                    "Data root '{}' is missing required folder '{}'",
                    root.display(),
                    folder
                ),
            )));
        }
    }

    Ok(())
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

fn generate_companies_pages(companies: &HashMap<String, Company>, path: &Path) {
    let template = include_str!("../templates/companies.html");

    let mut companies = companies.values().collect::<Vec<&Company>>();
    companies.sort_by(|a, b| a.name.cmp(&b.name));

    let globals = liquid::object!({
        "title": "Companies using Rust in Israel",
        "companies": companies,
    });
    let companies_path = path.join("companies");
    std::fs::create_dir_all(&companies_path).unwrap();
    render_page(globals, template, companies_path.join("index.html")).unwrap();
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

fn generate_job_pages(jobs: HashMap<String, Job>, people: &HashMap<String, Person>, path: &Path) {
    let template = include_str!("../templates/jobs.html");
    let globals = liquid::object!({
        "title": "Rust Jobs in Israel",
        "jobs": jobs.values().collect::<Vec<&Job>>(),
        "people": people.values().collect::<Vec<&Person>>(),
    });
    let jobs_path = path.join("jobs");
    std::fs::create_dir_all(&jobs_path).unwrap();
    render_page(globals, template, jobs_path.join("index.html")).unwrap();
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

fn generate_videos_page(presentations: &HashMap<String, Presentation>, path: &Path) {
    let mut presentations_in_hebrew = presentations
        .values()
        .filter(|presentation| presentation.video_he.is_some())
        .collect::<Vec<&Presentation>>();
    presentations_in_hebrew.sort_by_key(|presentation| &presentation.title);

    let mut presentations_in_english = presentations
        .values()
        .filter(|presentation| presentation.video_en.is_some())
        .collect::<Vec<&Presentation>>();
    presentations_in_english.sort_by_key(|presentation| &presentation.title);

    let template = include_str!("../templates/videos.html");
    let globals = liquid::object!({
        "title": "Videos",
        "presentations_in_english": presentations_in_english,
        "presentations_in_hebrew": presentations_in_hebrew,
    });
    render_page(globals, template, path.join("videos.html")).unwrap();
}

fn generate_presentation_pages(
    presentations: &HashMap<String, Presentation>,
    events: &HashMap<String, Event>,
    path: &Path,
) {
    let template = include_str!("../templates/presentations.html");
    let mut all_presentations = presentations.values().collect::<Vec<&Presentation>>();
    all_presentations.sort_by_key(|presentation| &presentation.title);

    let globals = liquid::object!({
        "title": "Presentations",
        "presentations": all_presentations,
    });
    let presentations_path = path.join("presentations");
    std::fs::create_dir_all(&presentations_path).unwrap();
    render_page(globals, template, presentations_path.join("index.html")).unwrap();

    let template = include_str!("../templates/presentation.html");
    for presentation in presentations.values() {
        let filename = format!("presentations/{}.md", presentation.slug);
        let event = events
            .values()
            .find(|event| event.schedule.contains(&filename))
            .unwrap();

        let globals = liquid::object!({
            "title": presentation.title,
            "presentation": presentation,
            "event": event,
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
    let mut future_events = events
        .values()
        .filter(|event| event.future)
        .collect::<Vec<&Event>>();
    future_events.sort_by(|a, b| a.date.cmp(&b.date));

    let template = include_str!("../templates/events.html");
    let mut values = events.values().collect::<Vec<&Event>>();
    values.sort_by(|a, b| b.date.cmp(&a.date));
    let globals = liquid::object!({
        "title": "Events",
        "events": values,
        "future_events": future_events,
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

fn generate_people_pages(
    people: &HashMap<String, Person>,
    presentations: &HashMap<String, Presentation>,
    projects: &HashMap<String, Project>,
    path: &Path,
) {
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

    let template = include_str!("../templates/people_missing.html");
    let mut values = people.values().collect::<Vec<&Person>>();
    values.sort_by(|a, b| a.name.cmp(&b.name));
    let globals = liquid::object!({
        "title": "People Missing data",
        "people": values,
    });
    let people_path = path.join("people");
    std::fs::create_dir_all(&people_path).unwrap();
    render_page(globals, template, people_path.join("missing.html")).unwrap();

    let template = include_str!("../templates/person.html");

    for person in people.values() {
        let filename = format!("people/{}.md", person.slug);

        let mut presentations_by_this_person = presentations
            .values()
            .filter(|presentation| presentation.speakers.contains(&filename))
            .collect::<Vec<&Presentation>>();
        presentations_by_this_person.sort_by_key(|presentation| &presentation.title);

        let mut projects_by_this_person = projects
            .values()
            .filter(|project| project.authors.contains(&filename))
            .collect::<Vec<&Project>>();
        projects_by_this_person.sort_by_key(|project| &project.name);

        let globals = liquid::object!({
            "title": person.name,
            "person": person,
            "presentations": presentations_by_this_person,
            "projects": projects_by_this_person,
        });
        render_page(
            globals,
            template,
            people_path.join(format!("{}.html", person.slug)),
        )
        .unwrap();
    }
}

fn load_projects(
    root: &std::path::Path,
    people: &HashMap<String, Person>,
) -> HashMap<String, Project> {
    let mut projects = HashMap::new();
    let paths = std::fs::read_dir(root.join("projects")).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.extension().unwrap() == "swp" {
            continue;
        }
        if path.file_name().unwrap() == "skeleton.md" {
            continue;
        }

        let (front_matter, body) = read_md_file_separate_front_matter(&path);
        let mut project: Project = serde_yml::from_str(&front_matter)
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

        let path_str = path_to_root_relative_key(root, &path);
        projects.insert(path_str, project);
    }

    projects
}

fn path_to_root_relative_key(root: &std::path::Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

impl Community {
    fn load_pages(&mut self) {
        let mut pages = Vec::new();
        let paths = std::fs::read_dir(self.data_root.join("pages")).unwrap();
        for path in paths {
            let path = path.unwrap().path();
            if path.extension().unwrap() == "swp" {
                continue;
            }

            let (front_matter, body) = read_md_file_separate_front_matter(&path);
            let mut page: Page = serde_yml::from_str(&front_matter)
                .unwrap_or_else(|err| panic!("Could not parse front matter in {path:?} {err}"));
            page.content = markdown2html(&body);
            page.slug = path.file_stem().unwrap().to_str().unwrap().to_string();

            pages.push(page);
        }
        self.pages = pages;
    }

    fn load_people(&mut self) {
        let mut people = HashMap::new();
        let paths = std::fs::read_dir(self.data_root.join("people")).unwrap();
        for path in paths {
            let path = path.unwrap().path();
            if path.extension().unwrap() == "swp" {
                continue;
            }
            if path.file_name().unwrap() == "skeleton.md" {
                continue;
            }
            let (front_matter, body) = read_md_file_separate_front_matter(&path);
            let mut person: Person = serde_yml::from_str(&front_matter).unwrap();
            person.slug = path.file_stem().unwrap().to_str().unwrap().to_string();
            person.body = markdown2html(&body);
            if let Some(img) = &person.img {
                let file = self.data_root.join(format!("img/{img}"));
                if !file.exists() {
                    panic!("File '{file:?}' used in '{path:?}' does not exist");
                }
            }

            let path_str = path_to_root_relative_key(&self.data_root, &path);
            people.insert(path_str, person);
        }
        self.people = people;
    }

    fn load_companies(&mut self) {
        let mut companies = HashMap::new();
        let paths = std::fs::read_dir(self.data_root.join("companies")).unwrap();
        for path in paths {
            let path = path.unwrap().path();
            if path.extension().unwrap() == "swp" {
                continue;
            }
            if path.file_name().unwrap() == "skeleton.md" {
                continue;
            }
            let (front_matter, body) = read_md_file_separate_front_matter(&path);
            let mut company: Company = serde_yml::from_str(&front_matter).unwrap();
            company.slug = path.file_stem().unwrap().to_str().unwrap().to_string();
            company.body = markdown2html(&body);

            let path_str = path_to_root_relative_key(&self.data_root, &path);
            companies.insert(path_str, company);
        }
        self.companies = companies;
    }

    fn load_jobs(&mut self) {
        let mut jobs = HashMap::new();
        let paths = std::fs::read_dir(self.data_root.join("jobs")).unwrap();
        for path in paths {
            let path = path.unwrap().path();
            if path.extension().unwrap() == "swp" {
                continue;
            }
            if path.file_name().unwrap() == "skeleton.md" {
                continue;
            }
            let (front_matter, body) = read_md_file_separate_front_matter(&path);
            let mut job: Job = serde_yml::from_str(&front_matter).unwrap();
            job.slug = path.file_stem().unwrap().to_str().unwrap().to_string();
            job.body = markdown2html(&body);
            job.people = get_people(&self.people, &job.contacts, &path);

            let path_str = path_to_root_relative_key(&self.data_root, &path);
            jobs.insert(path_str, job);
        }
        self.jobs = jobs;
    }

    fn load_presentations(&mut self) {
        let mut presentations = HashMap::new();
        let paths = std::fs::read_dir(self.data_root.join("presentations")).unwrap();
        for path in paths {
            let path = path.unwrap().path();
            if path.extension().unwrap() == "swp" {
                continue;
            }
            if path.file_name().unwrap() == "skeleton.md" {
                continue;
            }

            let (front_matter, body) = read_md_file_separate_front_matter(&path);
            let mut presentation: Presentation = serde_yml::from_str(&front_matter).unwrap();
            presentation.body = markdown2html(&body);
            presentation.slug = path.file_stem().unwrap().to_str().unwrap().to_string();
            presentation.people = get_people(&self.people, &presentation.speakers, &path);

            let path_str = path_to_root_relative_key(&self.data_root, &path);
            presentations.insert(path_str, presentation);
        }
        self.presentations = presentations;
    }
}

fn get_people(
    people: &HashMap<String, Person>,
    people_files: &[String],
    path: &PathBuf,
) -> Vec<Person> {
    for speaker in people_files.iter() {
        if !people.contains_key(speaker) {
            panic!("Speaker '{speaker}' in '{path:?}' does not exist");
        }
    }

    people_files
        .iter()
        .map(|speaker| people[speaker].clone())
        .collect::<Vec<_>>()
}

fn load_events(
    root: &std::path::Path,
    presentatons: &HashMap<String, Presentation>,
) -> Result<HashMap<String, Event>, Box<dyn Error>> {
    let utc: DateTime<Utc> = Utc::now();
    let today = utc.format("%Y.%m.%d").to_string();

    let mut events = HashMap::new();
    let paths = std::fs::read_dir(root.join("events")).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.extension().unwrap() == "swp" {
            continue;
        }
        if path.file_name().unwrap() == "skeleton.md" {
            continue;
        }

        let (front_matter, body) = read_md_file_separate_front_matter(&path);
        let mut event: Event = serde_yml::from_str(&front_matter)
            .unwrap_or_else(|err| panic!("Could not parse front matter in {path:?} {err}"));
        event.slug = path.file_stem().unwrap().to_str().unwrap().to_string();
        event.body = markdown2html(&body);
        event.future = event.date >= today;

        //let schedule_items = [String::from("mingling"), String::from("break")];
        let special_items = HashMap::from([
            (
                String::from("mingling"),
                Break {
                    title: String::from("Mingling"),
                    length: 30,
                },
            ),
            (
                String::from("break"),
                Break {
                    title: String::from("Break"),
                    length: 15,
                },
            ),
        ]);
        let special_names = special_items.keys().collect::<Vec<_>>();

        for presentation_slug in &event.schedule {
            if !special_names.contains(&presentation_slug)
                && !presentatons.contains_key(presentation_slug)
            {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Presentation '{presentation_slug}' in '{path:?}' does not exist"),
                )));
            }
        }

        let schedule_items = event
            .schedule
            .iter()
            .map(|presentation_slug| {
                if special_names.contains(&presentation_slug) {
                    return ScheduleItem::Break(special_items[presentation_slug].clone());
                }
                ScheduleItem::Presentation(presentatons[presentation_slug].clone())
            })
            .collect::<Vec<_>>();
        event.schedule_items = schedule_items;

        let path_str = path_to_root_relative_key(root, &path);
        events.insert(path_str, event);
    }

    Ok(events)
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
        .filter(TypeStr)
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
    partials.add(
        "templates/incl/event_presentations_short_list.html",
        include_str!("../templates/incl/event_presentations_short_list.html"),
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

use liquid_core::{
    Display_filter, Filter, FilterReflection, ParseFilter, Result, Runtime, Value, ValueView,
};

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(name = "type", description = "Type", parsed(TypeFilter))]
pub struct TypeStr;

#[derive(Debug, Default, Display_filter)]
#[name = "typestr"]
pub struct TypeFilter;

impl Filter for TypeFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let keys = ["Presentation", "Break"];
        match input.as_object() {
            Some(obj) => {
                for key in keys {
                    if obj.contains_key(key) {
                        return Ok(Value::scalar(key.to_string()));
                    }
                }
                Ok(Value::scalar("Unknown Item"))
            }
            None => Ok(Value::scalar("Not an object")),
        }
    }
}
