use std::time::Duration;

use clap::{builder::Str, Parser};

use fetcher::{Fetcher, FetcherError};
use next_page::NextPage;
use parser::{PaginatedBody, Parser as DataParser};
use repository::Repository;

mod entity;
mod fetcher;
mod next_page;
mod parser;
mod repository;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Api key with debug default value
    #[arg(short, long, default_value_t = String::from("2/1210066817168544/1210066924811437:4751cc07fd5a98597a1c10db54d5c89a"))]
    api_key: String,
    /// Save path with debug default value
    #[arg(short, long, default_value_t = String::from("jsons"))]
    save_path: String,
    /// Short polling interval (default true)
    #[arg(long, default_value_t = true)]
    short_polling: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let fetcher = Fetcher::new(args.api_key)?;
    let parser = DataParser {};
    let repo = Repository::new(args.save_path.into())?;

    let polling_interval = if args.short_polling {
        Duration::from_secs(30)
    } else {
        Duration::from_secs(300)
    };

    // Done in the simplest way possible.
    // Ideally pagination should be involved and async waiting instead of thread sleep
    loop {
        println!("Polling");

        let users_result = fetch_with_retry(|| fetcher.fetch_users());
        process(users_result?, &parser, &repo)?;

        let projects_result = fetch_with_retry(|| fetcher.fetch_projects());
        process(projects_result?, &parser, &repo)?;

        std::thread::sleep(polling_interval);
    }
}

// experimental
fn main_paginated() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let fetcher = Fetcher::new(args.api_key)?;
    let parser = DataParser {};
    let repo = Repository::new(args.save_path.into())?;

    let polling_interval = if args.short_polling {
        Duration::from_secs(30)
    } else {
        Duration::from_secs(300)
    };

    let limit = 2;
    let mut offset: Option<String> = None;
    let offset_ref = &mut offset;
    loop {
        println!("Polling");

        let users_result = fetch_with_retry(|| fetcher.fetch_users_paginated(limit, offset_ref.clone()));
        let page = process_paginated(users_result?, &parser, &repo)?;

        // let projects_result = fetch_with_retry(|| fetcher.fetch_projects_paginated(limit, offset_ref.clone()));
        // process_paginated(projects_result?, &parser, &repo)?;

        // TODO: fix below
        // offset = Some(page.offset);
        std::thread::sleep(polling_interval);
    }
}

fn fetch_with_retry<F, T>(fetch_closure: F) -> Result<T, FetcherError>
where
    F: Fn() -> Result<T, FetcherError>,
{
    loop {
        let result = (fetch_closure)();

        // Retry on `Too Many Requests`
        if let Err(FetcherError::Http { code: 429, retry_after }) = result {
            std::thread::sleep(retry_after.unwrap_or(Duration::from_secs(300)));
            continue;
        }

        return result;
    }
}

fn process_paginated(
    body: String,
    parser: &DataParser,
    repo: &Repository,
) -> Result<NextPage, Box<dyn std::error::Error>> {
    let paginated_body = parser.parse_entities_paginated(body)?;

    for entity in paginated_body.data {
        repo.save(entity)?;
    }

    Ok(paginated_body.next_page)
}

fn process(
    body: String,
    parser: &DataParser,
    repo: &Repository,
) -> Result<(), Box<dyn std::error::Error>> {
    let entites = parser.parse_entities(body)?;

    for entity in entites {
        repo.save(entity)?;
    }

    Ok(())
}
