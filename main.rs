use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;

mod cli;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Err: this program need for one archive.pico");
        println!("Use: ./pico <archive.pico> [--help]");
        process::exit(1);
    }

    let path_archive = &args[1];
    let archive = File::open(path_archive)?;
    let lay = BufReader::new(archive);
    let mut codeJs = String::new(); 
    let mut boxs = String::new();

    // lay execute
    for linha in lay.lines() {
        let lay_cont = linha?;
        let layp = lay_cont.trim();
        
        // print indentifier
        if layp.starts_with("print(") && layp.ends_with(")") { 
            let part: Vec<&str> = layp.split('"').collect();
            if part.len() >= 3 {
                //if line is print("hello world"), part[0] = 'print(', part[1] = 'hello world', part[2] = ')'
                let lineJs = format!("console.log(\"{}\");\n", part[1]);
                codeJs.push_str(&lineJs);
            }
        }// end print

        // box
        if layp.contains('=') && !layp.starts_with("print(") {
            let part: Vec<&str> = layp.split('=').collect();
            let NameOfBox = part[0].trim();
            let ValueOfBox = part[1].trim();
            
            if boxs.contains(NameOfBox) {
                let lineJs = format!("{}={};\n", NameOfBox, ValueOfBox);
                codeJs.push_str(&lineJs);
            } else {
                boxs.push_str(&format!(" {} ", NameOfBox));
                let lineJs = format!("let {} = {};\n", NameOfBox, ValueOfBox);
                codeJs.push_str(&lineJs); 
            }
        } // end
    }

    let archive_output = path_archive.replace(".pico", ".js");
    std::fs::write(&archive_output, codeJs)?;
    cli::runJs(&archive_output);
    Ok(())
}
