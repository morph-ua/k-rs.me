use std::{env, process};
use base64::{Engine as _, engine::{general_purpose}};
extern crate pretty_env_logger;
#[macro_use] extern crate log;

#[tokio::main]
async  fn main() {
    pretty_env_logger::init();

    if env::var("ACTION").is_ok() {
        let pr_number = {
            if env::var("PR_NUMBER").is_ok() {
                env::var("PR_NUMBER").unwrap()
            } else {
                error!("Failed to get the Pull Request ID from Github Actions.");
                process::exit(1);
            }
        };
        trace!("Got PR ID from GitHub Action: {pr_number}");

        let pr = octocrab::instance()
            .pulls("kyee-rs", "k-rs.me")
            .get(pr_number.parse().unwrap())
            .await
            .unwrap();
        let pr_files = octocrab::instance()
            .pulls("kyee-rs", "k-rs.me")
            .list_files(pr_number.parse().unwrap())
            .await;

        if let Ok(files) = pr_files {
            for file in files {
                trace!("Looking if Pull Request has new files in `domains`...");
                if file.filename.starts_with("domains/") && file.filename.ends_with(".domain.json") {
                    let content = octocrab::instance()
                        .repos("kyee-rs", "k-rs.me")
                        .get_content()
                        .path(&file.filename)
                        .r#ref(&pr.head.ref_field)
                        .send()
                        .await;

                    if let Ok(content) = content {
                        let innercontent = content.items[0].clone();
                        let bytes = general_purpose::STANDARD
                            .decode(innercontent.content.unwrap().replace('\n', "")).unwrap();
                        info!("Found the content...");
                        let json: serde_json::Value = match serde_json::from_str(String::from_utf8_lossy(&bytes).as_ref()) {
                            Ok(s) => s,
                            Err(_) => {
                                error!("File content has an invalid JSON. Exiting...");
                                process::exit(1);
                            }
                        };

                        // Check the author username
                        if let Some(author) = json.get("author") {
                            if let Some(username) = author.get("username") {
                                println!("Username: {}", username);
                            }
                        }
                    }
                } else {
                    error!("No new files in `domains` were found.");
                    process::exit(1);
                }
            }
        }

    }
}
