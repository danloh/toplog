// web server bin

use srv::init_server;

fn main() -> std::io::Result<()> {

    /*
    // to test sth
    use srv::bot::spider::{WebPage};
    let r = WebPage::new("https://seanmonstar.com/");
    let links = r.clean_links();
    println!("{:#?}", links);
    let item = WebPage::new(&links[0]).into_item();
    println!("{:#?}", item);
    // end
    */
    
    init_server()
}
