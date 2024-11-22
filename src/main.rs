use std::env;
use std::process::ExitCode;

fn usage(program: &str) {
    eprintln!("Usage: {program} [SUBCOMMAND] [OPTIONS]");
    eprintln!("Subcommands:");
    eprintln!("     hash <password>");
}

fn hash_password(password: String) -> String {
    todo!();
}

fn entry() -> Result<(), ()> {
    let mut args = env::args();
    let program = args.next().expect("Program path needs to be provided");
    let subcommand = args.next().ok_or_else(|| {
        usage(&program);
        eprint!("ERROR: no subcommand is provided");
    })?;

    match subcommand.as_str() {
        "hash" => {
            let password = args.next().ok_or_else(|| {
                usage(&program);
                eprint!("ERROR: no password provided");
            })?;
            let hashed_password = hash_password(password);
        }
        _ => {
            usage(&program);
            eprintln!("ERROR: unknown subcommand: {subcommand}");
            return Err(());
        }
    }

    Ok(())
}

fn main() -> ExitCode {
    match entry() {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}
