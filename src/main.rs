
use rustypath::RPath;

fn main() {
    // to create a new RPath
    let rpath = RPath::new_from("/abc");
    println!("path created using RPath::new_from() -> {}", rpath.convert_to_string());
    
    // convert to pathbuf
    let _rpath_pbuf = rpath.convert_to_pathbuf();

    // convert to string
    let _rpath_string = rpath.convert_to_string();

    // clone
    let _rpath_clone = rpath.clone();

    // expand relative path
    let rpath_new = RPath::new_from("~/abc");
    println!("Expanded using RPath : {}", rpath_new.expand().convert_to_string());
}