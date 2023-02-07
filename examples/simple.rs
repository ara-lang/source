use ara_source::loader;

fn main() {
    let root = format!(
        "{}/examples/fixture/",
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    );

    let map = loader::load_directories(
        root.clone(),
        vec![
            format!("{root}src"),
            format!("{root}vendor/foo"),
            format!("{root}vendor/bar"),
        ],
    )
    .unwrap();

    println!("{map:#?}");
}
