use crate::state::application_state::GraphRootLocation;
use std::path::PathBuf;
use std::process::{Command, Output};

pub struct GitCommandExecutor {
    cmd_description: &'static str,
    working_directory: PathBuf,
}

pub struct GitCommandExecutable {
    args: Vec<String>,
    cmd_description: &'static str,
    working_directory: PathBuf,
}

impl GitCommandExecutor {
    pub fn new(
        cmd_description: &'static str,
        graph_root_location: &GraphRootLocation,
    ) -> GitCommandExecutor {
        GitCommandExecutor {
            cmd_description,
            working_directory: graph_root_location.path.clone(),
        }
    }

    pub fn args_str(self, args: &'static [&'static str]) -> GitCommandExecutable {
        GitCommandExecutable {
            args: args.iter().map(|&arg| arg.to_string()).collect(),
            cmd_description: self.cmd_description,
            working_directory: self.working_directory,
        }
    }

    pub fn args(self, args: &[String]) -> GitCommandExecutable {
        GitCommandExecutable {
            args: args.iter().map(|arg| arg.to_string()).collect(),
            cmd_description: self.cmd_description,
            working_directory: self.working_directory,
        }
    }
}

impl GitCommandExecutable {
    pub fn execute(self) -> Result<Output, String> {
        let mut cmd = Command::new("git");
        for arg in self.args {
            cmd.arg(arg);
        }
        cmd.env("LANG", "en_US.UTF-8");
        cmd.env("GIT_TERMINAL_PROMPT", "0");

        println!("Executing {} with command {cmd:?}", self.cmd_description);
        println!("Current working directory: {:?}", self.working_directory);

        cmd.current_dir(self.working_directory);
        let result = cmd
            .output()
            .map_err(|e| format!("Failed to {}: {}", self.cmd_description, e));
        println!("Command success? {:?}", result.is_ok());
        result
    }
}
