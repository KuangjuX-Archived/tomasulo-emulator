use std::fs::File;
pub struct Trace {
    pub file: File
}

impl Trace {
    pub fn new<S>(filename: S) -> Self
        where S: Into<String> 
    {
        Self{
            file: File::open(filename.into()).unwrap()
        }
    }

    
}