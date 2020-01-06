// web server bin

use srv::init_server;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    /*
    // to test spider works
    use srv::bot::spider::{WebPage};
    let r = WebPage::new("https://www.fluentcpp.com/");
    let links = r.unwrap_or_default().clean_links();
    println!("{:#?}", links);
    let item = WebPage::new(&links[0]).unwrap_or_default().into_item();
    println!("{:#?}", item);
    // end
    */
    
    init_server().await
}
