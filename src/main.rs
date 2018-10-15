extern crate futures;
extern crate http;
extern crate hyper;
extern crate hyper_tls;
extern crate select;
extern crate tokio;

use std::error::Error;

use futures::{future, Future, Stream};
use http::Request;
use hyper::Client;
use hyper_tls::HttpsConnector;
use select::document::Document;
use select::predicate::Name;
use tokio::runtime::Runtime;

fn main() -> Result<(), Box<Error>> {
    let mut rt = Runtime::new().expect("failed to initialize runtime");
    let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new(4).unwrap());
    let polish = client
        .get("https://github.com/softprops/envy".parse().unwrap())
        .and_then(|response| {
            response.into_body().concat2().map(|bytes| {
                let body = String::from_utf8_lossy(bytes.as_ref()).to_owned();
                Document::from(body.as_ref())
                    .find(Name("img"))
                    .map(|node| node.attr("src").unwrap_or_default().to_owned())
                    .into_iter()
                    .filter(|link| link.starts_with("https://camo.githubusercontent.com"))
                    .collect::<Vec<_>>()
            })
        }).and_then(move |links| {
            future::join_all(links.into_iter().map(move |link| {
                client
                    .request(
                        Request::builder()
                            .header("User-Agent", "camo-polish")
                            .method("PURGE")
                            .uri(&link)
                            .body(Default::default())
                            .unwrap(),
                    ).map(|response| (response.status(), link))
            }))
        });
    println!("{:#?}", rt.block_on(polish));
    Ok(())
}
