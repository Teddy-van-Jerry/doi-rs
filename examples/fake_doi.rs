use doi::Doi;

fn main() {
    let fake_doi = Doi::new("10.1109/TCSII.2030.fake");
    match fake_doi.resolve() {
        Err(e) => eprintln!("Error: {}", e),
        _ => unreachable!(),
    }
    #[cfg(feature = "metadata")]
    {
        match fake_doi.metadata() {
            Err(e) => eprintln!("Error: {}", e),
            _ => unreachable!(),
        }
    }
}
