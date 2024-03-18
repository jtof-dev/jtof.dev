use regex::Regex;
use std::fs;
use std::io::{self};

fn main() -> io::Result<()> {
    let index_header_html = r#"
    <!DOCTYPE html>
    <html lang="en">
    
    <head>
        <meta charset="UTF-8">
        <title>jtof.dev</title>
        <link rel="stylesheet" href="/styles.css">
        <link rel="stylesheet" href="notes.css">
        <link rel="icon" href="/assets/ferris the crab.png">
        <script src="/script.js"></script>
    </head>
    
    <body class="dark-mode">
        <div class="switch-and-notes">
        <div class="left-actions">
        <a href="/">
            <svg xmlns="http://www.w3.org/2000/svg" version="1.1" viewBox="-5.0 -10.0 110.0 120.0"
                class="homepage-svg">
                <path
                    d="m87.5 44.844c-0.15625-1.0938-0.78125-2.0312-1.5625-2.6562l-9.6875-7.1875v-14.844c0-2.3438-1.875-4.0625-4.0625-4.0625h-5.9375c-2.3438 0-4.0625 1.875-4.0625 4.0625v4.375l-10.938-8.125c-0.625-0.46875-1.25-0.46875-1.875 0l-35.156 25.625c-0.9375 0.625-1.4062 1.5625-1.5625 2.6562s0.15625 2.1875 0.78125 3.125l3.9062 5.3125c1.4062 1.875 3.9062 2.1875 5.7812 0.78125l0.78125-0.625v26.562c0 2.3438 1.875 4.0625 4.0625 4.0625h44.062c2.3438 0 4.0625-1.875 4.0625-4.0625v-26.562l0.78125 0.625c0.78125 0.625 1.5625 0.78125 2.5 0.78125 1.25 0 2.5-0.625 3.2812-1.7188l3.9062-5.3125c0.78125-0.625 1.0938-1.7188 0.9375-2.8125zm-22.5-24.844c0-0.625 0.46875-0.9375 0.9375-0.9375h5.9375c0.625 0 0.9375 0.46875 0.9375 0.9375v12.656l-7.8125-6.0938zm-6.875 61.094h-16.25v-21.406h16.406v21.406zm14.844-1.0938c0 0.625-0.46875 0.9375-0.9375 0.9375h-10.625v-22.812c0-0.9375-0.625-1.5625-1.5625-1.5625h-19.531c-0.9375 0-1.5625 0.625-1.5625 1.5625v22.969h-10.625c-0.625 0-0.9375-0.46875-0.9375-0.9375v-29.062l22.812-17.031 22.969 17.031zm11.25-34.062-3.9062 5.3125c-0.3125 0.46875-0.9375 0.46875-1.4062 0.15625l-27.969-20.625c-0.3125-0.15625-0.625-0.3125-0.9375-0.3125s-0.625 0.15625-0.9375 0.3125l-27.812 20.781c-0.46875 0.3125-1.0938 0.3125-1.4062-0.15625l-3.9062-5.3125c-0.3125-0.3125-0.3125-0.625-0.3125-0.78125 0-0.3125 0.15625-0.46875 0.46875-0.625l33.906-25.312 33.906 25.312c0.15625 0.15625 0.3125 0.46875 0.46875 0.625 0 0.15625 0 0.46875-0.15625 0.625z" />
                <title>go back to the homepage</title>
            </svg>
        </a>
        <label class="switch">
            <input type="checkbox" checked id="checkbox" onchange="switchcolors(event)">
            <span class="slider round"></span>
        </label>
    </div>
            <h2 class="recipe-name">cookbook search</h2>
            <div style="width: 60px"></div>
        </div>
        <input type="text" onkeyup="search_recipes(event)" placeholder="search recipes" class="search-bar">
        <div id="recipe-links">
"#;

    let footer_html = r#"
<footer>
<br>
<br>
<hr>
<p class="footer">this <a href="https://github.com/jtof-dev/jtof.dev" target="_blank">website</a> is
    open-source, and is licensed under <a href="https://www.gnu.org/licenses/gpl-3.0.html"
        target="_blank">gpl-3</a><br>powered by <a href="https://pages.github.com" target="_blank">github
        pages</a><br>developed by <a href="https://github.com/jtof-dev" target="_blank">jtof</a></p>
</footer>
</body>

</html>
"#;

    // Specify the directory path
    let directory_path = "../../../obsidian vault/recipes/";
    // let directory_path = "recipes/";

    // Read the contents of the directory
    let dir_entries = fs::read_dir(directory_path)?;

    // Iterate over each entry in the directory
    for entry in dir_entries {
        let entry = entry?;

        // Check if the entry is a file
        if entry.file_type()?.is_file() {
            // Get the file path
            let file_path = entry.path();

            // Read the contents of the file
            match fs::read_to_string(&file_path) {
                Ok(content) => {
                    let entry_name_full = entry.file_name().into_string().unwrap();
                    let entry_name_with_spaces: String = entry_name_full
                        .chars()
                        .take(entry_name_full.len() - 3)
                        .collect();
                    let entry_name = entry_name_with_spaces.replace(" ", "-").to_lowercase();
                    let entry_contents = content;
                    make_recipe_pages(
                        entry_name,
                        entry_name_with_spaces,
                        entry_contents,
                        footer_html,
                    );
                }
                Err(err) => {
                    eprintln!("Error reading {}: {}", file_path.display(), err);
                }
            }
        }
    }
    make_index_page(index_header_html, footer_html);
    Ok(())
}

fn make_recipe_pages(
    markdown_name: String,
    markdown_name_with_spaces: String,
    mut markdown_contents: String,
    footer_html: &str,
) {
    let start_index = markdown_contents.find("---").unwrap_or(0);
    let end_index = markdown_contents
        .rfind("---")
        .unwrap_or(markdown_contents.len());
    markdown_contents.replace_range(start_index..=end_index + 2, "");

    // convert markdown to html
    let parser = pulldown_cmark::Parser::new(&markdown_contents);
    let mut html_contents = String::new();
    pulldown_cmark::html::push_html(&mut html_contents, parser);

    // wrap notes in div's
    let snippet_pattern = r"-&gt; <strong>.*?</strong>";

    // Compile the regular expression
    let re = Regex::new(snippet_pattern).unwrap();

    // Replace each occurrence of the snippet with the wrapped version
    let modified_html_contents = re.replace_all(&html_contents, |captures: &regex::Captures| {
        let matched_text = captures.get(0).unwrap().as_str();
        format!("<span class=\"notes\">{}</span>", matched_text)
    });

    let first_header = r#"
    <!DOCTYPE html>
    <html lang="en">
    
    <head>
        <meta charset="UTF-8">
        <title>jtof.dev</title>
        <link rel="stylesheet" href="/styles.css">
        <link rel="stylesheet" href="/cookbook/notes.css">
        <link rel="icon" href="/assets/ferris the crab.png">
        <script src="/script.js"></script>
    </head>
    
    <body class="dark-mode">
        <div class="switch-and-notes">
            <div class="left-actions">
                <a href="/cookbook/">
                    <svg xmlns="http://www.w3.org/2000/svg" version="1.1" viewBox="-5.0 -10.0 110.0 120.0"
                        class="homepage-svg">
                        <path
                            d="m87.5 44.844c-0.15625-1.0938-0.78125-2.0312-1.5625-2.6562l-9.6875-7.1875v-14.844c0-2.3438-1.875-4.0625-4.0625-4.0625h-5.9375c-2.3438 0-4.0625 1.875-4.0625 4.0625v4.375l-10.938-8.125c-0.625-0.46875-1.25-0.46875-1.875 0l-35.156 25.625c-0.9375 0.625-1.4062 1.5625-1.5625 2.6562s0.15625 2.1875 0.78125 3.125l3.9062 5.3125c1.4062 1.875 3.9062 2.1875 5.7812 0.78125l0.78125-0.625v26.562c0 2.3438 1.875 4.0625 4.0625 4.0625h44.062c2.3438 0 4.0625-1.875 4.0625-4.0625v-26.562l0.78125 0.625c0.78125 0.625 1.5625 0.78125 2.5 0.78125 1.25 0 2.5-0.625 3.2812-1.7188l3.9062-5.3125c0.78125-0.625 1.0938-1.7188 0.9375-2.8125zm-22.5-24.844c0-0.625 0.46875-0.9375 0.9375-0.9375h5.9375c0.625 0 0.9375 0.46875 0.9375 0.9375v12.656l-7.8125-6.0938zm-6.875 61.094h-16.25v-21.406h16.406v21.406zm14.844-1.0938c0 0.625-0.46875 0.9375-0.9375 0.9375h-10.625v-22.812c0-0.9375-0.625-1.5625-1.5625-1.5625h-19.531c-0.9375 0-1.5625 0.625-1.5625 1.5625v22.969h-10.625c-0.625 0-0.9375-0.46875-0.9375-0.9375v-29.062l22.812-17.031 22.969 17.031zm11.25-34.062-3.9062 5.3125c-0.3125 0.46875-0.9375 0.46875-1.4062 0.15625l-27.969-20.625c-0.3125-0.15625-0.625-0.3125-0.9375-0.3125s-0.625 0.15625-0.9375 0.3125l-27.812 20.781c-0.46875 0.3125-1.0938 0.3125-1.4062-0.15625l-3.9062-5.3125c-0.3125-0.3125-0.3125-0.625-0.3125-0.78125 0-0.3125 0.15625-0.46875 0.46875-0.625l33.906-25.312 33.906 25.312c0.15625 0.15625 0.3125 0.46875 0.46875 0.625 0 0.15625 0 0.46875-0.15625 0.625z" />
                        <title>go back to the cookbook search page</title>
                    </svg>
                </a>
                <label class="switch">
                    <input type="checkbox" checked id="checkbox" onchange="switchcolors(event)">
                    <span class="slider round"></span>
                </label>
            </div>
    "#;
    let second_header = format!(
        "<h2 class=\"recipe-name\">{}</h2>        
        <button onclick=\"show_hide_notes()\" id=\"notes-button\">hide my notes</button>
        </div>",
        markdown_name_with_spaces
    );
    // write html_contents to file
    let final_html_contents = format!(
        "{}{}{}{}",
        first_header, second_header, modified_html_contents, footer_html
    );
    if let Ok(dir) = fs::read_dir(&markdown_name) {
        fs::write(
            format!("{}/index.html", &markdown_name),
            final_html_contents.as_bytes(),
        )
        .unwrap();
    } else {
        fs::create_dir(&markdown_name).unwrap();
        fs::write(
            format!("{}/index.html", &markdown_name),
            final_html_contents.as_bytes(),
        )
        .unwrap();
    }
}

fn make_index_page(index_header_html: &str, footer_html: &str) {

    let mut list_of_recipes = String::new();

    let directory_path = "../../../obsidian vault/recipes/";
    // let directory_path = "recipes/";

    // Read the contents of the directory
    let dir_entries = fs::read_dir(directory_path).unwrap();

    // Iterate over each entry in the directory
    for entry in dir_entries {
        let entry = entry.unwrap();

        // Check if the entry is a file
        if entry.file_type().unwrap().is_file() {
            // Get the file path
            let file_path = entry.path();
            let entry_name_full = entry.file_name().into_string().unwrap();
            let entry_name_with_spaces: String = entry_name_full
                .chars()
                .take(entry_name_full.len() - 3)
                .collect();
            let entry_name = entry_name_with_spaces.replace(" ", "-").to_lowercase();
            let recipe_name_contents = format!("<a href=\"{}\" class=\"recipe-link\">{}</a><br>\n", entry_name, entry_name_with_spaces);
            list_of_recipes.push_str(&recipe_name_contents);
        }
    }

    let final_html_contents = format!("{}{}</div>{}", index_header_html, list_of_recipes, footer_html);
    fs::write("index.html", final_html_contents.as_bytes()).unwrap();
}
