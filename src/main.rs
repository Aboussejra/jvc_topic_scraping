use scraper::{Html, Selector};
use std::process;
#[derive(Debug)]
struct TitleLink {
    title: String,
    link: String,
}

#[derive(Debug)]
struct TopaxInfo {
    title: String,
    link: String,
    count: usize,
    messages_info: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "https://www.jeuxvideo.com";
    let list_topax = base_url.to_owned() + "/forums/0-51-0-1-0-1-0-blabla-18-25-ans.htm";
    let get_messages_on_topax = false;
    let mut topax_info_list = vec![];
    let mut temp_struct_vec_messages = vec![];
    let resp = match reqwest::get(list_topax).await {
        Ok(x) => x,
        Err(_) => {
            println!("error on request");
            process::exit(0x0100); //exit if error expected during request to /random
        }
    };
    let text = resp.text().await?;
    let document = Html::parse_document(&text);
    let topax_selector = Selector::parse("li").unwrap();
    let title_link_selector = Selector::parse("a.topic-title").unwrap();
    let count_selector = Selector::parse("span.topic-count").unwrap();
    'outer: for elem in document.select(&topax_selector) {
        let mut title_link_html = elem.select(&title_link_selector);
        let mut count = elem.select(&count_selector);
        if elem.inner_html().contains("topic-pin") {
            continue 'outer;
        }
        let title_link = match title_link_html.next() {
            Some(title_link) => {
                let title = title_link
                    .value()
                    .attr("title")
                    .expect("Title not found")
                    .to_string();
                let link = title_link
                    .value()
                    .attr("href")
                    .expect("Link not found")
                    .to_string();
                Some(TitleLink { title, link })
            }
            None => None,
        };
        let count_str = match count.next() {
            Some(count) => {
                let count: String = count.inner_html().to_string();
                Some(count)
            }
            None => None,
        };
        if let Some(title_and_link) = title_link {
            if let Some(count_to_parse) = count_str {
                let count_parsed = count_to_parse.trim().to_string().parse();
                match count_parsed {
                    Ok(count) => topax_info_list.push(TopaxInfo {
                        title: title_and_link.title,
                        link: base_url.to_owned() + &title_and_link.link,
                        count,
                        messages_info: vec![],
                    }),
                    Err(e) => println!("Err {} occured", e),
                }
            }
        }
    }
    if get_messages_on_topax {
        for topax_info in topax_info_list.iter() {
            match extract_message_topax(topax_info.link.clone()).await {
                Ok(vec_messages) => temp_struct_vec_messages.push(vec_messages),
                Err(e) => println!("Err {} occured", e),
            };
        }
        for (temp_vec_messages, topax_info) in temp_struct_vec_messages
            .iter_mut()
            .zip(topax_info_list.iter_mut())
        {
            topax_info.messages_info = temp_vec_messages.to_vec();
        }
    }
    println!("Topax infos are {:?}", topax_info_list);
    Ok(())
}

async fn extract_message_topax(link: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut messages_topax = vec![];
    let resp = match reqwest::get(link).await {
        Ok(x) => x,
        Err(_) => {
            println!("error on request");
            process::exit(0x0100); //exit if error expected during request to /random
        }
    };
    let text = resp.text().await?;
    let document = Html::parse_document(&text);
    let bloc_message_selector = Selector::parse("div.bloc-message-forum").unwrap();
    let message_in_block_selector = Selector::parse("div.txt-msg").unwrap();
    if let Some(elem) = document.select(&bloc_message_selector).next() {
        let mut message_selected = elem.select(&message_in_block_selector);
        if let Some(message) = message_selected
            .next()
            .map(|message| message.inner_html())
        {
            messages_topax.push(message);
        }
    }
    Ok(messages_topax)
}

//fn write_into_db(topax_info: TopaxInfo) {}
