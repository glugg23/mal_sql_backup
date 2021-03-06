use reqwest::Error;
use scraper::{Html, Selector};
use std::collections::HashMap;

use crate::request::Request;

const LOGIN_URL: &'static str = "https://myanimelist.net/login.php";

fn get_csrf_token(request: &Request) -> Result<String, Error> {
    let res = request.client.get(LOGIN_URL).send()?;

    let html = Html::parse_document(res.text()?.as_str());
    let selector = Selector::parse("meta[name=csrf_token]").unwrap();
    let elem = html.select(&selector).next().unwrap().value();

    Ok(elem.attr("content").unwrap().to_owned())
}

pub fn set_session_cookie(request: &Request, username: &str, password: &str) -> Result<(), Error> {
    let csrf_token = get_csrf_token(request)?;

    let mut form = HashMap::with_capacity(6);
    form.insert("user_name", username);
    form.insert("password", password);
    form.insert("csrf_token", &csrf_token);
    form.insert("cookie", "1");
    form.insert("sublogin", "Login");
    form.insert("submit", "1");

    request.client.post(LOGIN_URL).form(&form).send()?;
    Ok(())
}
