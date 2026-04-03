use comrak::{Options, ResolvedReference, markdown_to_html};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::Read, sync::Arc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub user: String,
    pub social: Vec<String>,
    pub pfp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category(pub u64, pub Option<u64>, String, String); // [id, parent_id, Path, Description]

#[derive(Debug, Serialize, Deserialize)]
pub struct Text {
    pub authors: Vec<u64>,
    pub desc: Option<String>,
    pub image: Option<String>,
    pub category_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Details {
    #[serde(rename = "StartPath")]
    pub start_path: String,
    #[serde(rename = "Authors")]
    pub authors: HashMap<String, Author>,

    pub categories: HashMap<String, Category>,

    pub texts: HashMap<String, Text>,

    pub dependencies: Vec<String>,
}

pub struct Structure(
    pub u64,
    pub HashMap<u64, Structure>,
    pub Vec<(String, String, Vec<u64>, String)>,
    pub String,
    pub String,
);
pub fn parse_markdown(input: &str) -> String {
    let mut options = Options::default();
    options.extension.table = true;
    options.parse.broken_link_callback = Some(Arc::new(
        |link: comrak::options::BrokenLinkReference<'_>| {
            let label = link.original.to_string();
            let url = label.to_lowercase();

            Some(ResolvedReference {
                url,
                title: String::new(),
            })
        },
    ));

    markdown_to_html(input, &options)
}
fn graph_into(input: &mut Structure, details: &Details, curr: String) {
    for (k, i) in details.texts.iter() {
        if i.category_id == input.0 {
            let path = format!("{}/{}/{}", details.start_path, curr, k);
            let mut f = File::open(path).unwrap();
            let mut txt = String::new();
            f.read_to_string(&mut txt).unwrap();
            txt = parse_markdown(&txt);
            input.2.push((
                k.to_string(),
                txt,
                i.authors.clone(),
                i.image.clone().unwrap_or(String::new()),
            ));
        }
    }

    for (k, i) in details.categories.iter() {
        if let Some(parent) = i.1
            && parent == input.0
        {
            let mut my_self =
                Structure(i.0, HashMap::new(), Vec::new(), k.to_string(), i.3.clone());
            graph_into(&mut my_self, details, i.2.clone());
            input.1.insert(i.0, my_self);
        }
    }
}

pub fn derive_structure(details: &Details) -> Option<Structure> {
    let mut root: Structure =
        Structure(0, HashMap::new(), Vec::new(), String::new(), String::new());
    let mut found_root = false;
    let mut curr: String = String::new();
    for (k, i) in details.categories.iter() {
        if i.1.is_none() {
            root = Structure(i.0, HashMap::new(), Vec::new(), k.to_string(), i.3.clone());
            found_root = true;
            curr = i.2.clone();
        }
    }

    graph_into(&mut root, details, curr);

    if !found_root {
        return None;
    }

    Some(root)
}
