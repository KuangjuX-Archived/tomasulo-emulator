use std::fs::File;
pub struct Trace {
    pub file: File
}

impl Trace {
    pub fn new<S>(filename: S) -> Self
        where S: Into<String> + Clone + Copy
    {
        let file = File::create(filename.into()).unwrap();
        Self{ file }
    } 
}