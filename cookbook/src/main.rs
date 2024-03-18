use std::fs;
use std::io::{self};
use regex::Regex;

fn main() -> io::Result<()> {

    let header_html = r#"
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
            <label class="switch">
                <input type="checkbox" checked id="checkbox" onchange="switchcolors(event)">
                <span class="slider round"></span>
            </label>
            <button onclick="show_hide_notes()" id="notes-button">show my notes</button>
        </div>
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
                    make_recipe_pages(entry_name, entry_contents, header_html, footer_html);
                }
                Err(err) => {
                    eprintln!("Error reading {}: {}", file_path.display(), err);
                }
            }
        }
    }

    Ok(())
}

fn make_recipe_pages(markdown_name: String, mut markdown_contents: String, header_html: &str, footer_html: &str) {
    let start_index = markdown_contents.find("---").unwrap_or(0);
    let end_index = markdown_contents.rfind("---").unwrap_or(markdown_contents.len());
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
    // write html_contents to file
    let final_html_contents = format!("{}{}{}", header_html, modified_html_contents, footer_html);
    fs::write(format!("{}.html", markdown_name), final_html_contents.as_bytes()).unwrap();
}
