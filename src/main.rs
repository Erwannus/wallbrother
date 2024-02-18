use std::io::Write;
use std::path::Path;
use std::env;

fn main() {

    creat_congig();


    let url = "156.67.224.173";
    let path = "/record/current.jpg";

    let mut file = std::fs::File::create("current.jpg").unwrap();
    reqwest::blocking::get(format!("http://{}{}", url, path))
        .unwrap()
        .copy_to(&mut file)
        .unwrap();
}

fn creat_congig(){
    let mire = "https://upload.wikimedia.org/wikipedia/commons/1/1b/RCA_Indian_Head_test_pattern.JPG";
    
    let key = "HOME";
    let home = env::var(key).expect("Variable HOME indefinie");
    let path = format!("{}{}", home, "/.config/wallbrother/");
    let full_path = format!("{}{}", home, "/.config/wallbrother/wallbrother.conf");
    
    std::fs::create_dir_all(path).unwrap();
    if !Path::new("does_not_exist.txt").exists() {
        let mut conf = std::fs::File::create(full_path).unwrap();
        conf.write_all(format!("mire    0   {}", mire).as_bytes()).unwrap();
    }
}