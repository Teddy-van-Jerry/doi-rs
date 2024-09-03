use doi::Doi;
// use doi::DoiBuilder;

fn main() {
    let zhao2024flexible = Doi::new("10.1109/TCSII.2024.3366282");
    // let zhao2024flexible = DoiBuilder::new().doi("10.1109/TCSII.2024.3366282").proxy("http://127.0.0.1:7890").unwrap().build();
    // let zhao2024flexible = DoiBuilder::new().doi("10.1109/TCSII.2024.3366282").env_proxy(false).build();
    println!("Is DOI set? {}", zhao2024flexible.is_set());
    println!("DOI Link: {}", zhao2024flexible.https_url());
    match zhao2024flexible.resolve() {
        Ok(resolved) => println!("Resolved Link: {}", resolved),
        Err(e) => eprintln!("Error: {}", e),
    }
    // dbg!(&zhao2024flexible);

    #[cfg(feature = "metadata")]
    {
        let metadata = zhao2024flexible.metadata().unwrap();
        println!("Paper Title: {}", metadata.title.unwrap());
        if let Some(authors) = metadata.authors {
            for author in authors {
                let name = author.full_name().unwrap();
                println!("Author: {}", name);
            }
        }
        if let Some(r#type) = metadata.r#type {
            println!("Type: {}", r#type.as_str());
        }
        dbg!(&zhao2024flexible.metadata_json());
    }
}
