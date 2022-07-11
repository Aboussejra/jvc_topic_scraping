use scraper::{Html, Selector};
use std::process;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "https://www.jeuxvideo.com";
    let list_topax = base_url.to_owned() + "/forums/0-51-0-1-0-1-0-blabla-18-25-ans.htm";
    let resp = match reqwest::get(list_topax).await {
        Ok(x) => x,
        Err(_) => {
            println!("error on request");
            process::exit(0x0100); //exit if error expected during request to /random
        }
    };
    let text = resp.text().await?;
    let document = Html::parse_document(&text);
    let topax_selector = Selector::parse("a.lien-jv").unwrap();
    let topax_count_selector = Selector::parse("span.topic-count").unwrap();
    let mut pinned_topics_count = 0;
    let mut topax_titles = vec![];
    let mut topax_counts = vec![];
    'outer: for elem in document.select(&topax_selector) {
        if let Some(title) = elem.value().attr("title") {
            for classification_elem_topax in elem.prev_siblings() {
                let element = format!("{:?}", classification_elem_topax.value());
                if element.contains("topic-pin") {
                    pinned_topics_count += 1;
                    continue 'outer;
                }
            }
            topax_titles.push(title);
        }
    }
    'outer: for elem in document.select(&topax_count_selector).skip(pinned_topics_count + 1){
        let topic_count = elem.inner_html().trim_end().trim_start().parse::<usize>().unwrap();
        topax_counts.push(topic_count);
    }
    for (topax,count) in topax_titles.iter().zip(topax_counts.iter()){
        println!("Topax {} has count {}",topax,count);
    }
    Ok(())
}
