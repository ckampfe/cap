fn main() {
    std::process::Command::new("cc")
        .arg("-dynamiclib")
        .arg("shathree.c")
        .arg("-o")
        .arg("/usr/local/lib/shathree.dylib")
        .output()
        .expect("Unable to compile and install shathree.c to /usr/local/lib/shathree.dylib");
}
