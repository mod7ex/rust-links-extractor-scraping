use error_chain::error_chain;
use select::document::Document;
use select::predicate::Name;

error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()>{
    let response = reqwest::get("https://www.rust-lang.org/en-US/")
                            .await?
                            .text()
                            .await?;

    Document::from(response.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("[link] {}", x));


    Ok(())
}
