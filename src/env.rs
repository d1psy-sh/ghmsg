use dotenvy::dotenv;

pub fn get_token() -> String {
    dotenv().ok();
    dotenvy::var("GITHUB_TOKEN").expect("There was an error while loading the token for GitHub")
}
