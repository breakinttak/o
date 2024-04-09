use std::io::{self, BufWriter, Read, Write};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufRead};

use bumpalo::Bump;


fn main() {

    //reading line for tokenization

    let file = File::open("src/abc.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    // let n = File::open("src/efg.txt").expect("Cannot open n file");


    // let mut writer = OpenOptions::new().append(true).open("src/efg.txt").expect("Cannot open fiel to append");
    let mut i = 0;
    let b = Bump::new();
    for line in reader.lines(){
       i += 1;
      //getting each line as "line"
      let mut line1 = format!("({} : {})," , &line.unwrap().to_lowercase() , i); 
      // writer.write_all( line1.as_bytes() ).expect("Cannot write in the new file");   
      let s = b.alloc(line1);
      println!("{:?}",s);
    }
  }
    

