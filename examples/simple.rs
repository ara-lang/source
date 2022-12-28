use ara_source::loader;

fn main() {
    let root = format!(
        "{}/examples/fixture/",
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    );

    let map = loader::load_directories(
        root.clone(),
        vec![
            format!("{}src", root),
            format!("{}vendor/foo", root),
            format!("{}vendor/bar", root),
        ],
    )
    .unwrap();

    println!("{:#?}", map);
}
