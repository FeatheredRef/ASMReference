use crate::{
    rendering::{self, style::STYLE},
    structuring::logic::{Details, Structure},
};

macro_rules! PRESET {
    () => {
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <link href="/statics/style.css" rel="stylesheet">
        <script type="module" src="/statics/index.js"></script>
        <title>asm/reference | {}</title>
        <meta property="og:title" content="{}">
        <meta property="og:description" content="{}">
</head>
<body>
        <header>
                <canvas class="noise"></canvas>
                <p><a href="/">asm/reference</a></p>
        </header>
        <article class="txt">
        {}
        </article>
        <footer>
                <canvas class="noise"></canvas>
                <div id="d1">
                        <p>asm/reference</p>
                        <p>Licensed under GPL-2.0</p>
                </div>
                <div id="d2">
                        <a href="https://github.com/FeatheredRef/ASMReference">Github</a>
                </div>
        </footer>
        <script>
        function applyNoise(canvas) {{
                const ctx = canvas.getContext("2d");
                const rect = canvas.getBoundingClientRect();
                canvas.width = rect.width;
                canvas.height = rect.height;
                const imageData = ctx.createImageData(canvas.width, canvas.height);
                const data = imageData.data;

                for (let i = 0; i < data.length; i += 4) {{
                        const value = Math.random() * 255;
                        data[i] = data[i+1] = data[i+2] = value;
                        data[i+3] = 255;
                }}

                ctx.putImageData(imageData, 0, 0);
        }}
        document.querySelectorAll(".noise").forEach(applyNoise);
        </script>
</body>
</html>
"#
    };
}

const INDEX_PAGE: &str = r#"
<html lang="en"><head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <link href="/statics/style.css" rel="stylesheet">
        <title>asm/reference | %0</title>
        <meta property="og:title" content="%0">
        <meta property="og:description" content="%$">
        <script type="module" src="/statics/index.js"></script>
</head>
<body>
        <header>
                <canvas class="noise" width="775" height="50"></canvas>
                <p><a href="/">asm/reference</a></p>
        </header>
        <article class="txt">
        <h1>%2</h1>
        <hr>
        <main class="index">
        %1
        </main>
        </article>
        <footer>
                <canvas class="noise" width="775" height="50"></canvas>
                <div id="d1">
                        <p>asm/reference</p>
                        <p>Licensed under GPL-2.0</p>
                </div>
                <div id="d2">
                        <a href="https://github.com/FeatheredRef/ASMReference">Github</a>
                </div>
        </footer>
        <script>
        function applyNoise(canvas) {
                const ctx = canvas.getContext("2d");
                const rect = canvas.getBoundingClientRect();
                canvas.width = rect.width;
                canvas.height = rect.height;
                const imageData = ctx.createImageData(canvas.width, canvas.height);
                const data = imageData.data;

                for (let i = 0; i < data.length; i += 4) {
                        const value = Math.random() * 255;
                        data[i] = data[i+1] = data[i+2] = value;
                        data[i+3] = 255;
                }

                ctx.putImageData(imageData, 0, 0);
        }
        document.querySelectorAll(".noise").forEach(applyNoise);
        </script>

</body></html>
"#;

const LANDING_PAGE: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <link href="/statics/style.css" rel="stylesheet">
        <title>asm/reference</title>
</head>
<body>
        <header>
                <canvas class="noise"></canvas>
                <p>asm/reference</p>
        </header>
        <section id="s1">
                <h1>Low-level knowledge,<br><span id="text-thing">accessible.</span></h1>
        </section>
        <hr>
        <section id="s2">
                <canvas class="noise"></canvas>
                <br>
                <h2>Categories</h2>
                <br>
                <nav>
                       {} 
                </nav>
        </section>
        <hr>
        <footer>
                <canvas class="noise"></canvas>
                <div id="d1">
                        <p>asm/reference</p>
                        <p>Licensed under GPL-2.0</p>
                </div>
                <div id="d2">
                        <a href="https://github.com/FeatheredRef/ASMReference">Github</a>
                </div>
        </footer>
        <script>
        const texts = [
                "accessible.","simple.","free.", "open."
        ];
        const el = document.getElementById("text-thing");

        let textIndex = 0;
        let charIndex = 0;
        let deleting = false;

        const TYPE_SPEED = 80;
        const DELETE_SPEED = 20;
        const PAUSE_TIME = 2200;

        function tick() {
            const current = texts[textIndex];

            if (!deleting) {
                charIndex++;
                el.textContent = current.slice(0, charIndex);

                if (charIndex === current.length) {
                    deleting = true;
                    return setTimeout(tick, PAUSE_TIME);
                }
            } else {
                charIndex--;
                el.textContent = current.slice(0, charIndex);

                if (charIndex === 0) {
                    deleting = false;
                    textIndex = (textIndex + 1) % texts.length;
                }
            }

            setTimeout(tick, deleting ? DELETE_SPEED : TYPE_SPEED);
        }

        tick();
        function applyNoise(canvas) {
                const ctx = canvas.getContext("2d");
                const rect = canvas.getBoundingClientRect();
                canvas.width = rect.width;
                canvas.height = rect.height;
                const imageData = ctx.createImageData(canvas.width, canvas.height);
                const data = imageData.data;

                for (let i = 0; i < data.length; i += 4) {
                        const value = Math.random() * 255;
                        data[i] = data[i+1] = data[i+2] = value;
                        data[i+3] = 255;
                }

                ctx.putImageData(imageData, 0, 0);
        }
        document.querySelectorAll(".noise").forEach(applyNoise);
        </script>
</body>
</html>
"#;

const NOT_FOUND: (&str, &str) = ("404.html", "<h1>404</h1>");

#[derive(Default)]
pub struct Dir {
    pub sub: Vec<Dir>,
    pub name: String,
    pub files: Vec<(String, String)>,
    pub desc: String,
}

#[allow(clippy::useless_conversion)]
fn first_upper(s: &str) -> String {
    let mut a = false;
    let b: String = s
        .chars()
        .into_iter()
        .map(|c| {
            if !a {
                a = true;
                return c.to_ascii_uppercase();
            }
            c
        })
        .collect();
    b
}
pub fn generate_sitemap(root: &Dir, base_url: &str) -> String {
    let mut xml = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
    );

    fn walk(dir: &Dir, base_url: &str, current_path: &str, xml: &mut String) {
        // Add files in current directory
        for (file_name, _) in &dir.files {
            if file_name == "index.html" {
                xml.push_str(&format!(
                    "\n  <url><loc>{}{}/</loc></url>",
                    base_url, current_path
                ));
            } else {
                let name = file_name.strip_suffix(".html").unwrap_or(file_name);
                xml.push_str(&format!(
                    "\n  <url><loc>{}{}/{}</loc></url>",
                    base_url, current_path, name
                ));
            }
        }
        // Recurse into subdirectories
        for sub in &dir.sub {
            let new_path = format!("{}/{}", current_path, sub.name.to_lowercase());
            walk(sub, base_url, &new_path, xml);
        }
    }

    walk(root, base_url, "", &mut xml);
    xml.push_str("\n</urlset>");
    xml
}
fn render(se: &Structure, directory: &mut Dir, details: &Details) {
    directory.name = se.3.clone();
    let index_title = se.3.clone();
    let mut index_template = INDEX_PAGE.to_string();
    let mut index_content: String = String::from("");
    index_template = index_template
        .replace("%0", &index_title)
        .replace("%2", &first_upper(&se.3))
        .replace("%$", &directory.desc);
    index_content.push_str(&se.3);
    index_content.push_str("<a href=\"..\">..</a>");
    let se1: Vec<(bool, String)> =
        se.1.values()
            .map(|a| (a.2.is_empty(), a.3.clone()))
            .collect();
    se1.into_iter().for_each(|b| {
        if b.0 {
            index_content.push_str(&format!(
                "<a class=\"empty\" href=\"{}\">{}</a>",
                b.1.to_lowercase(),
                b.1
            ));
        } else {
            index_content.push_str(&format!("<a href=\"{}\">{}</a>", b.1.to_lowercase(), b.1));
        }
    });

    let mut se2 = se.2.clone();
    se2.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
    se2.iter().for_each(|b| {
        let mut authors_line: String = String::from("");
        for i in &b.2 {
            if let Some(a) = details.authors.get(&i.to_string())
                && let Some(lll) = a.social.first()
            {
                authors_line.push_str(&format!(
                    "<a href=\"{}\" class=\"author-line\"><img alt=\"an image\" src=\"{}\"></img><p>{}</p></a>",
                    lll, a.pfp, a.user
                ));
            }
        }

        let title: String = first_upper(
            &b.0.clone()
                .strip_suffix(".md")
                .unwrap_or(&b.0)
                .to_string()
                .replace(" ", "-")
                .to_ascii_lowercase(),
        );
        let content = format!(PRESET!(), title, title, b.4, b.1)
            .replacen(
                "</h1>",
                r#"</h1>
                    <hr>
                    <img src="%img"></img><br>
                    <section>
                        <div class="author-div">
                            %authors
                        </div>
                    </section>
                "#,
                1,
            )
            .replacen("%img", &b.3, 1)
            .replacen("%authors", &authors_line, 1);

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

    let p = index_template
        .replacen("%0", &index_title, 1)
        .replacen("%1", &index_content, 1);
    directory.files.push(("index.html".to_string(), p));

    let mut se1: Vec<u64> = se.1.keys().copied().collect();
    se1.sort_by(|a, b| {
        se.1.get(a)
            .unwrap()
            .3
            .to_lowercase()
            .cmp(&se.1.get(b).unwrap().3.to_lowercase())
    });
    se1.iter().for_each(|b| {
        let mut dir = rendering::logic::Dir {
            desc: se.1.get(b).unwrap().4.clone(),
            ..Default::default()
        };
        render(se.1.get(b).unwrap(), &mut dir, details);
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

    let _ = root.files.extract_if(0.., |a| a.0 == "index.html");

    let mut something = String::new();

    for i in root.sub.iter() {
        let mut txts = 0;
        txts += i.files.len();
        txts -= 1;
        if txts == 0 {
            continue;
        }
        something.push_str(&format!(
            r#"
            <a href="/{}">
                <h3>{}</h3>
                <p>{}<br><sub>{} Articles</sub></p>
            </a>
            "#,
            &i.name, &i.name, i.desc, txts
        ));
    }

    root.files.push((
        "index.html".to_string(),
        LANDING_PAGE.replace("{}", &something).to_string(),
    ));
    let xml = generate_sitemap(&root, "/");
    root.files.push(("sitemap.xml".to_string(), xml));
    let robots_content =
        "User-agent: *\nAllow: /\nSitemap: https://asm-reference.pages.dev/sitemap.xml";
    root.files
        .push((String::from("robots.txt"), String::from(robots_content)));
    root
}
