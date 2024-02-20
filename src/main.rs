include!(concat!(env!("OUT_DIR"), "/version.rs"));

fn env(mut args: std::env::Args) -> Result<(), Box<dyn std::error::Error>> {
    let me = args.next().unwrap();
    let mut debug = false;
    while let Some(arg) = args.next() {
        let field: Vec<&str> = arg.splitn(2, '=').collect();
        if field.len() == 2 {
            if debug {
                println!("setenv {}={}", field[0], field[1]);
            }
            std::env::set_var(field[0].clone(), field[1].clone())
        } else if field[0] == "-v" || field[0] == "--debug" {
            debug = true
        } else if field[0] == "--version" {
            println!("{} {}", &me, VERSION);
            return Ok(());
        } else if field[0] == "-h" || field[0] == "--help" {
            println!("{} {}", &me, VERSION);
            println!("Usage:");
            println!(
                "  {} {{options}}... {{NAME=VALUE}}...  COMMAND ARGS...",
                &me
            );
            println!("Options:");
            println!("  -v --debug   print verbose information");
            println!("     --version output version information");
            println!("  -h --help    display this help");
            return Ok(());
        } else {
            if debug {
                println!("call {}", arg);
            }
            let param: Vec<String> = args.collect();
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
