fn main() {
    let url = "156.67.224.173";
    let path = "/record/current.jpg";

    let mut file = std::fs::File::create("current.jpg").unwrap();
    reqwest::blocking::get(format!("http://{}{}", url, path))
        .unwrap()
        .copy_to(&mut file)
        .unwrap();
}