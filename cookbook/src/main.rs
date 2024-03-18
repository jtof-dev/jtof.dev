use regex::Regex;
use std::fs;
use std::io::{self};

fn main() -> io::Result<()> {
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
                    );
                }
                Err(err) => {
                    eprintln!("Error reading {}: {}", file_path.display(), err);
                }
            }
        }
    }
    make_index_page();
    Ok(())
}

fn make_recipe_pages(
    markdown_name: String,
    markdown_name_with_spaces: String,
    mut markdown_contents: String,
) {
    let footer_html = fs::read_to_string("src/footer_html.html").unwrap();
    let first_header = fs::read_to_string("src/first_header.html").unwrap();
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
    if let Ok(_) = fs::read_dir(&markdown_name) {
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

fn make_index_page() {

    let index_header_html = fs::read_to_string("src/index_header_html.html").unwrap();
    let footer_html = fs::read_to_string("src/footer_html.html").unwrap();

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
