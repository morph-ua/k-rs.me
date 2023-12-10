use std::env;

#[tokio::main]
async  fn main() {
    if env::var("ACTION").is_ok() {
        let pr_number = {
            if env::var("PR_NUMBER").is_ok() {
                env::var("PR_NUMBER").unwrap()
            } else {
                panic!("Failed to get the Pull Request ID from Github Actions.")
            }
        };
        let pr = octocrab::instance()
            .pulls("kyee-rs", "k-rs.me")
            .get(pr_number.parse().unwrap())
            .await;
        println!("{pr:?}")
    }
}
