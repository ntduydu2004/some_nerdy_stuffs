use std::{default, fs::File, io::{BufWriter, Read, Write}};

const INPUT_DIRECTORY: &str = "/coding products/testRust/test_code/src/data/input.txt";
const OUTPUT_DIRECTORY: &str = "/coding products/testRust/test_code/src/data/output.txt";
#[derive(Default)]
pub struct Scanner{
    buffer: Vec<String>
}
impl Scanner{
    pub fn input(&mut self) -> String{
        let mut open_file_result = File::open(INPUT_DIRECTORY);
        match open_file_result {
            Ok(mut file) =>{
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("fail to read contents of file");
                println!("{}", contents.clone());
                contents
            },
            Err(_) => {
                println!("Failed to open the input file");
                String::new()
            }
        }
    }

    pub fn output(&self, results: &Vec<i32>){
        let mut f = BufWriter::new(File::create(OUTPUT_DIRECTORY).expect("failed to create the output file"));
        for result in results{
            write!(f, "{0}\n", result);
        }
        
    }
}