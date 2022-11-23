mod slugs;

use std::borrow::{Borrow, BorrowMut};
use salvo::__private::tracing::error;
use salvo::prelude::{Request, Response, StatusCode, Router, Server, TcpListener, handler};
use salvo::cors::Cors;
use salvo::http::header::{self, HeaderValue};
use salvo::prelude::*;
use serde::{Serialize, Deserialize};

#[handler]
async fn link_shortener(req: &mut Request, mut res: &mut Response) {
    let vec: Vec<&str> = req.uri_mut().path().clone().split("/").collect(); // get the URI path, clone & split then turn it into a vector
    let mut slug_response = slugs::get_slug(vec[vec.len()-1]).await;
    if slug_response != "" {
        res.set_status_code(StatusCode::TEMPORARY_REDIRECT);
        if !slug_response.contains("http") {
            slug_response = "http://".to_owned() + &slug_response; // http for compatibility
        }
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
async fn homepage(mut res: &mut Response) {
    res.render("Homepage!");
    match res.with_header("Content-Type", "text/html; charset=utf-8", true) {
        Ok(t) => res = t,
        Err(e) => error!("Failed to set headers: {e}")
    }
}

#[handler]
async fn set_shortener(req: &mut Request, mut res: &mut Response) {
    match req.parse_json::<LinkShortener>().await {
        Ok(t) => {
            if t.domain.contains(".") {
                let domain = slugs::set_slug(t.domain).await;
                res.render("https://api.swath.cc/".to_owned() + &domain)
            } else {
                res.set_status_code(StatusCode::BAD_REQUEST);
                res.render("That doesn't look like a domain!")
            }
        }
        Err(e) => {
            res.set_status_code(StatusCode::BAD_REQUEST);
            res.render("Couldn't parse JSON data into a link shortener!");
            error!("Error: {e}");
        }
    }
    match res.with_header("Access-Control-Allow-Origin", "*", true) {
        Ok(t) => res = t,
        Err(e) => error!("Failed to set headers: {e}")
    }
}

#[handler]
async fn get_options(mut res: &mut Response) {
    res.set_status_code(StatusCode::OK);
    match res.with_header("Access-Control-Allow-Origin", "*", true) {
        Ok(t) => res = t,
        Err(e) => error!("Failed to set headers: {e}")
    }
}

#[handler]
async fn add_header(res: &mut Response) {
    res.headers_mut()
        .insert(header::ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("Content-Type"));
}

#[derive(Serialize, Deserialize, Debug)]
struct LinkShortener {
    domain: String
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
                .hoop(add_header)
                .options(get_options)
                .post(set_shortener)
        );
    //slugs::set_slug("lol", "https://edward.engineer");
    Server::new(TcpListener::bind("0.0.0.0:7878")).serve(router).await;
}