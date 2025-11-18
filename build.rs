fn main() {
    let mut res = winres::WindowsResource::new();
    res.set("FileDescription", "Take notes in the background");
    res.set("ProductName", "Background Notes");
    res.set("LegalCopyright", "JJayRex");
    res.set("FileVersion", "0.1.0.0");
    res.set("ProductVersion", "0.1.0.0");
    res.compile().unwrap();
}