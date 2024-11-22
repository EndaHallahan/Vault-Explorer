use regex::{Captures, Regex};
use html_escape::encode_text;
use comrak::{markdown_to_html, Options};

use vault_dweller::{ VaultIndex, QueryOutput};

fn convert_links(in_md: String, vault_name: &String) -> String {
    let re = Regex::new(r"\[\[([^\]]+?)(\|[^\]]+)?\]\]").unwrap();
    return re.replace_all(&in_md, |caps: &Captures| {
        let mut l_text = &caps[1];
        if caps.get(2) != None {
            l_text = &caps[2];
        }
        return format!(
            "[{}](/vault/{}/note/{})", 
            l_text.replacen('|', "", 1), 
            encode_text(&vault_name.replace(' ', "_").replace('/', "%2F")), 
            encode_text(&caps[1].replace(' ', "_").replace('/', "%2F"))
        );
    }).to_string();
}

fn convert_dataview(in_md: String, vault: &VaultIndex) -> String {
    let re = Regex::new(r"```dataview[\n\r]([\w\W]*?)[\n\r]```").unwrap();
     return re.replace_all(&in_md, |caps: &Captures| {
        let mut out_string: String = Default::default();
        let query_out = vault.query(&caps[1]);
        match query_out {
            QueryOutput::List(list) => {
                if list.len() == 0 {
                    out_string.push_str("\n> `Dataview: No results to show for list query.`\n");
                } else {
                    for li in list {
                        let mut name: String = Default::default();
                        let mut add_info: String = Default::default();
                        if let Some(note_name) = li.note_name {
                            name = format!(
                                "[{}](/vault/{}/note/{})",
                                note_name,
                                encode_text(&vault.name.replace(' ', "_").replace('/', "%2F")), 
                                encode_text(&note_name.replace(' ', "_").replace('/', "%2F"))
                            );
                        }
                        if let Some(additional_info) = li.additional_info {
                            add_info = format!(": {}", additional_info);
                        }
                        out_string.push_str(&format!("- {}{}\n", name, add_info));
                    }
                }
            },
            QueryOutput::Err(err_vec) => {
                for err in err_vec {
                    out_string.push_str(&format!("\n> `Dataview Error: {}`", err));
                }
                out_string.push_str("\n");
            },
            _ => todo!("Not implemented yet!"),
        }

        out_string
    }).to_string();
}

fn  markdown_options<'a>() -> Options<'a> {
    let mut options = Options::default();
    options.extension.strikethrough = true;
    options.extension.tagfilter = true;
    options.render.unsafe_ = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.superscript = true;
    options.extension.header_ids = Some("".to_string());
    options.extension.footnotes = true;
    options.extension.front_matter_delimiter = Some("---".to_owned());
    options.extension.underline = true;
    options.extension.spoiler = true;
    options.render.hardbreaks = true;

    options
}

pub fn parse_md(in_md: String, vault: &VaultIndex) -> String {
	markdown_to_html(&convert_dataview(convert_links(in_md, &vault.name), &vault), &markdown_options())
}