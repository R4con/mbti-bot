extern crate reqwest;
extern crate selectors;

use scraper::{Html, Selector};
use selectors::attr::CaseSensitivity;
use std::borrow::Cow;

fn get_document(url: &String) -> Result<scraper::html::Html, Box<dyn std::error::Error>> {
    //pull html from url
    let response = reqwest::blocking::get(url)?;
    let html_string = response.text()?;

    //parse html
    let document = Html::parse_document(&html_string);

    //check if a token is required
    if document.errors.contains(&Cow::from("Unexpected token")) {
        println!("youre fucked!");
        //add the token to the html header, and load the website again
    }

    Ok(document)
}

fn main() {
    let mut imgur_post_link_list: Vec<String> = Vec::new();
    let mut meme_url_collection: Vec<String> = Vec::new();

    let document = get_document(&"https://imgur.com/search?q=intj".to_string()).unwrap();

    //filter domcument and find urls, of the posts
    let selector = Selector::parse("a").unwrap();
    
    for element in document.select(&selector) {
        if element.value().has_class("image-list-link", CaseSensitivity::CaseSensitive) {
            let link = element.value().attr("href").unwrap();
            imgur_post_link_list.push(link.to_string());
        }
    }

    //go threw every post, and find the original image link
    // ! DOES NOT WORK - Privacy consent needs to be accepted 
    // for url_piece in imgur_post_link_list {
    //     let url = String::from("https://imgur.com")+ &url_piece.to_string();
    //     let document = get_document(&url).unwrap();

    //     let selector = Selector::parse("img").unwrap();

    //     println!("{:#?}", document);
    //     for element in document.select(&selector) {
    //         print!(".");
    //         if element.value().has_class("image-placeholder", CaseSensitivity::CaseSensitive) {
    //             println!("found something! {:#?}", element.value());
    //             let meme_url = scraper::element_ref::ElementRef::wrap(
    //                     element
    //                     .next_sibling()
    //                     .unwrap()
    //                 )
    //                 .unwrap()
    //                 .value()
    //                 .attr("src")
    //                 .unwrap();
    //             meme_url_collection.push(meme_url.to_string());
    //             println!("{}", meme_url);
    //             break;
    //         }
    //     }
    // }

    for post_link in imgur_post_link_list {
        println!("https://imgur.com{}", post_link);
    }
}   