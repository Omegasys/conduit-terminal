use std::env;
use std::io;

use conduit::core::{
    Session,
    SessionConfig,
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let shell = if args.len() > 1 {
        args[1].clone()
    } else {
        default_shell()
    };

    let config = SessionConfig {
        shell,
        working_directory: None,
        environment: Vec::new(),
        rows: 24,
        columns: 80,
    };

    let mut session = Session::new(config)?;

    session.start()?;

    session.run()?;

    Ok(())
}

fn default_shell() -> String {
    if let Ok(shell) = env::var("SHELL") {
        if !shell.trim().is_empty() {
            return shell;
        }
    }

    #[cfg(unix)]
    {
        "/bin/sh".to_string()
    }

    #[cfg(windows)]
    {
        "cmd.exe".to_string()
    }
}
