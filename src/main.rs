fn env(args: std::env::Args) -> Result<(), Box<dyn std::error::Error>> {
    let mut args_ = args.skip(1);
    while let Some(arg) = args_.next() {
        let field: Vec<&str> = arg.splitn(2, '=').collect();
        if field.len() == 2 {
            println!("{}={}",field[0],field[1]);
            std::env::set_var(field[0].clone(), field[1].clone())
        } else {
            println!("call {}",arg);
            let param: Vec<String> = args_.collect();
            if let Err(err) = std::process::Command::new(arg).args(param).spawn() {
                return Err(Box::new(err))
            } else {
                return Ok(())
            }
        }
    }
    Ok(())
}

fn main() {
    if let Err(err) = env(std::env::args()) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
