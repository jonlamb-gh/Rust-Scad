use crate::ScadObject;
use std::{fs, io, path::Path};

/**
    Object that stores scad objects along with global parameters for
    the objects. Also has methods for writing the  data to files.
*/
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct ScadFile {
    objects: Vec<ScadObject>,
    detail: i32,
}

impl ScadFile {
    pub fn new() -> ScadFile {
        ScadFile {
            objects: Vec::new(),

            detail: 0,
        }
    }

    /**
        Returns the code for the global parameters as well as all the
        children in the file
    */
    pub fn get_code(&self) -> String {
        let mut result = String::from("");

        if self.detail != 0 {
            result = result + "$fn=" + &self.detail.to_string() + ";\n";
        }

        for object in &self.objects {
            result = result + &object.get_code() + "\n";
        }

        result
    }

    pub fn add_object(&mut self, object: ScadObject) {
        self.objects.push(object);
    }

    /**
     Sets the $fn variable for the whole file. This varibale defines  the detail
     amount for cylindrical objects
    */
    pub fn set_detail(&mut self, detail: i32) {
        self.detail = detail;
    }

    /**
     Writes the resulting code to a file

     ##Arguments

       path: The path to the file where we want to write relative to the current
       working directory.

     ##Returns
     The function will return false and print a message to the console if
     writing fails.
    */
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        fs::write(&path, self.get_code().as_bytes())
    }
}

impl Default for ScadFile {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod file_tests {
    use super::*;
    use crate::ScadElement;

    #[test]
    fn detail_test() {
        let mut sfile = ScadFile::new();

        sfile.detail = 30;

        assert_eq!(sfile.get_code(), "$fn=30;\n");

        let obj = ScadObject::new(ScadElement::Union);
        sfile.add_object(obj);

        let obj = ScadObject::new(ScadElement::Difference);
        sfile.add_object(obj);

        assert_eq!(sfile.get_code(), "$fn=30;\nunion();\ndifference();\n")
    }

    #[test]
    fn file_test() {
        let out_dir = tempfile::tempdir().unwrap();
        let file_path = out_dir.path().join("test.scad");
        let mut sfile = ScadFile::new();

        sfile.detail = 30;

        sfile.write_to_file(&file_path).unwrap();

        let file_content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(file_content, sfile.get_code());
    }
}
