# GitHub Message

GitHub Message is an application that uses the GitHub API to display the messages one user received on GitHub b
in the terminal. You can set different filters or search for a specific term.

## Authentication for GitHub user

Documentation [here](https://docs.github.com/en/developers/apps/building-github-apps/authenticating-with-github-apps#generating-a-private-key)...

## Documentation for the REST API

The documentation is [here](https://docs.github.com/en/rest/activity/notifications?apiVersion=2022-11-28#about-github-notifications)...

## How to list notifications

The info for that is [here](https://docs.github.com/en/rest/activity/notifications?apiVersion=2022-11-28#list-notifications-for-the-authenticated-user)...

## API Key is saved in dotenv file

For this project I used the lib `dotenvy` for `.env` environment variables.

```rust
use dotenvy::dotenv;

fn get_vars() {
    // load vars from .env file
    dotenv().ok();
    // get one var by name
    let test = dotenvy::var("TEST");
    // match the result
    match test {
        Ok(test) => println!(test),
        Err(_) => println!("test not set")
    }
}
```

## Contribution

If you have an idea or see a bug or a best practice missed feel free to make a PR, I am eager to learn :D!
