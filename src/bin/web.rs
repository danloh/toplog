// web server bin

use srv::init_server;

fn main() -> std::io::Result<()> {

    /*
    // to test spider works
    use srv::bot::spider::{WebPage};
    let r = WebPage::new("https://rustacean-station.org/");
    let links = r.unwrap_or_default().clean_links();
    println!("{:#?}", links);
    let item = WebPage::new(&links[0]).unwrap_or_default().into_item();
    println!("{:#?}", item);
    // end
    */
    
    init_server()
}
