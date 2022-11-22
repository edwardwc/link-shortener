mod slugs;

use std::borrow::Borrow;
use salvo::__private::tracing::error;
use salvo::prelude::{Request, Response, StatusCode, Router, Server, TcpListener, handler};
use serde::{Serialize, Deserialize};

#[handler]
async fn link_shortener(req: &mut Request, mut res: &mut Response) {
    let vec: Vec<&str> = req.uri_mut().path().clone().split("/").collect(); // get the URI path, clone & split then turn it into a vector
    println!("{}", vec[vec.len()-1]);
    let slug_response = slugs::get_slug(vec[vec.len()-1]).await;
    if slug_response != "" {
        res.set_status_code(StatusCode::TEMPORARY_REDIRECT);
        let result = res.with_header("Location", slug_response, true);
        match result {
            Err(e) => error!("Failed to set headers: {e}"),
            Ok(t) => res = t
        }
    } else {
        res.set_status_code(StatusCode::NOT_FOUND);
        res.render("Couldn't find that slug!")
    }
}
#[handler]
async fn homepage(req: &mut Request, mut res: &mut Response) {
    res.render("Homepage!");
    match res.with_header("Content-Type", "text/html; charset=utf-8", true) {
        Ok(t) => res = t,
        Err(e) => error!("Failed to set headers: {e}")
    }
}

#[handler]
async fn set_shortener(req: &mut Request, res: &mut Response) {
    let parsed_link_shortener = req.parse_json::<LinkShortener>().await;
    match parsed_link_shortener {
        Ok(t) => {
            res.render(t.domain)
        }
        Err(e) => {
            res.render("Couldn't get domain");
            error!("Error: {e}")
        }
    }
    res.render("This shouldn't be hit!")
}

#[derive(Serialize, Deserialize, Extractible, Debug)]
#[extract(
default_source(from = "query"),
default_source(from = "param"),
default_source(from = "body")
)]
struct LinkShortener<'a> {
    slug: &'a str,
    domain: &'a str
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .push(
            Router::with_path("<*>")
                .get(link_shortener)
        )
        .push(
            Router::with_path("/")
                .get(homepage)
        )
        .push(
            Router::with_path("/add-shortener")
                .post(set_shortener)
        );
    //slugs::set_slug("lol", "https://edward.engineer");
    Server::new(TcpListener::bind("127.0.0.1:7878")).serve(router).await;
}