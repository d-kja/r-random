use dotenv::dotenv;
use std::env;

fn main() {
    // Reading .env file
    dotenv().ok();

    // CLI Arguments when running the binary
    let mut args_iterator = env::args();

    // Skipping the first argument as it'd give us the reference to the binary being executed and we don't need it.
    args_iterator.next();

    // collection converts any Iterator into something else, for example a String the only downside is that I printed the result and it was concatenating every string and it wasn't using any separator to separate those
    let cli_args: String = args_iterator.collect();

    get_user_github_details(&cli_args);
}

fn get_user_github_details(query: &str) {
    // similat to process.env
    let api_url = env::var("API_URL")
        .expect("Missing API_URL, consider setting the variable in the .env file"); // .unwrap()

    let formatted_request_url = format!("{api_url}/users/{query}");
    let api_client = reqwest::blocking::Client::new();

    let response = api_client
        // Request method
        .get(formatted_request_url)
        // Updaing with custom header
        .header(reqwest::header::USER_AGENT, "foo")
        // execute the request (blocking/non-async)
        .send()
        .expect("a successful response")
        // Json method needs the "serde_json::Value" to understand how it's supposed to convert the data received
        .json::<serde_json::Value>()
        .expect("expect to be a valid json");

    // debug output
    dbg!(response);
}
