use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::process;

mod cli;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        println!("Use: ./pico (compiles all .pico files in the current directory)");
        process::exit(0);
    }

    let pasta_saida = Path::new("dist");
    if !pasta_saida.exists() {
        fs::create_dir(pasta_saida)?;
    }

    let caminhos = fs::read_dir(".")?;

    for entrada in caminhos {
        let entry = entrada?;
        let path_archive = entry.path();

        if path_archive.is_file() && path_archive.extension().and_then(|s| s.to_str()) == Some("pico") {
            let path_str = path_archive.to_string_lossy().into_owned();
            let archive = File::open(&path_str)?;
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

            let nome_arquivo = path_archive.file_name().unwrap().to_string_lossy();
            let nome_js = nome_arquivo.replace(".pico", ".js");
            let caminho_final_js = pasta_saida.join(nome_js);
            let caminho_final_str = caminho_final_js.to_string_lossy().into_owned();

            fs::write(&caminho_final_str, codeJs)?;
            cli::runJs(&caminho_final_str);
        }
    }

    Ok(())
}
