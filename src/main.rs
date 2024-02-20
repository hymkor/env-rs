mod version;

fn env(args: std::env::Args) -> Result<(), Box<dyn std::error::Error>> {
    let mut args_ = args.skip(1);
    let mut debug = false;
    while let Some(arg) = args_.next() {
        let field: Vec<&str> = arg.splitn(2, '=').collect();
        if field.len() == 2 {
            if debug {
                println!("setenv {}={}", field[0], field[1]);
            }
            std::env::set_var(field[0].clone(), field[1].clone())
        } else if field[0] == "-v" || field[0] == "--debug" {
            debug = true
        } else if field[0] == "--version" {
            println!("{}", version::VERSION);
            return Ok(());
        } else {
            if debug {
                println!("call {}", arg);
            }
            let param: Vec<String> = args_.collect();
            if let Err(err) = std::process::Command::new(arg).args(param).spawn() {
                return Err(Box::new(err));
            } else {
                return Ok(());
            }
        }
    }
    for (key, value) in std::env::vars() {
        println!("{}={}", key, value)
    }
    Ok(())
}

fn main() {
    if let Err(err) = env(std::env::args()) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
