use crate::{
    rendering::style::STYLE,
    structuring::logic::{Details, Structure},
};

const STATICS: &str = r#"
<link rel="stylesheet" href="/statics/style.css">
"#;

macro_rules! PRESET {
    () => {
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<link rel="icon" href="/assets/icon.png">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
{}
<title>{}</title>
</head>
<body>
{}
</body>
</html>
"#
    };
}

const NOT_FOUND: (&str, &str) = ("404.html", "<h1>404</h1>");

#[derive(Default)]
pub struct Dir {
    pub sub: Vec<Dir>,
    pub name: String,
    pub files: Vec<(String, String)>,
}

fn render(se: &Structure, directory: &mut Dir, details: &Details) {
    directory.name = se.3.clone();
    let index_title = format!("ASMReference | {}/index", se.3);
    let mut index_content: String = String::from("<h1>");

    index_content.push_str(&se.3);
    index_content.push_str("</h1><div class=\"index\">");

    index_content.push_str("<a href=\"..\">..</a>");
    se.1.values().for_each(|b| {
        index_content.push_str(&format!("<a href=\"{}\">{}</a>", b.3.to_lowercase(), b.3));
    });

    se.2.iter().for_each(|b| {
        let mut authors_line: String = String::from("</h1><br><div class=\"author-div\">");
        for i in &b.2 {
            if let Some(a) = details.authors.get(&i.to_string())
                && let Some(lll) = a.social.first()
            {
                authors_line.push_str(&format!(
                    "<a href=\"{}\" class=\"author-line\"><img src=\"{}\"></img><p>{}</p></a>",
                    lll, a.pfp, a.user
                ));
            }
        }
        authors_line.push_str("</div><br>");

        let title: String =
            b.0.clone()
                .strip_suffix(".md")
                .unwrap_or(&b.0)
                .to_string()
                .replace(" ", "-")
                .to_ascii_lowercase();
        let content = format!(PRESET!(), STATICS, title, b.1).replacen("</h1>", &authors_line, 1);

        index_content.push_str(&format!(
            "<a href=\"{}\">{}</a>",
            b.0.to_lowercase().strip_suffix(".md").unwrap_or(""),
            b.0.strip_suffix(".md").unwrap_or("")
        ));
        directory.files.push((
            b.0.clone()
                .replace(" ", "")
                .replace(".md", ".html")
                .to_lowercase(),
            content,
        ));
    });

    index_content.push_str("</div>");
    directory.files.push((
        "index.html".to_string(),
        format!(PRESET!(), STATICS, index_title, index_content),
    ));
    se.1.iter().for_each(|(_, b)| {
        let mut dir = Dir::default();
        render(b, &mut dir, details);
        directory.sub.push(dir);
    });
}
#[allow(clippy::field_reassign_with_default)]
pub fn pre_render(se: &Structure, details: &Details) -> Dir {
    let mut root = Dir::default();
    render(se, &mut root, details);

    let mut statics = Dir::default();
    statics.name = String::from("statics");
    statics
        .files
        .push((String::from("style.css"), String::from(STYLE)));
    root.files
        .push((String::from(NOT_FOUND.0), String::from(NOT_FOUND.1)));
    root.sub.push(statics);

    root
}
