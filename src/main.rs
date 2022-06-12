use compiler::Compiler;

mod compiler;

fn main() {
    let mut args = std::env::args();
    args.next();

    let cmd = match args.next() {
        Some(str) => str,
        None => {
            eprintln!("Expected command");
            return;
        }
    };

    match cmd.as_str() {
        "compile" => {
            let path = match args.next() {
                Some(str) => str,
                None => {
                    eprintln!("Expected path");
                    return;
                }
            };
            let compiler = match Compiler::new(path.as_str()) {
                Ok(comp) => comp,
                Err(e) => {
                    eprintln!("{:?}", e);
                    return;
                }
            };

            compiler.compile();
        }
        "help" => println!(
            "Help:
    compile [path] compile the given source"
        ),
        _ => eprintln!("Wrong command. Type help for more info"),
    }
}
