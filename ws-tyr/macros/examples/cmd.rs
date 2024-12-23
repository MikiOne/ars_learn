#[derive(Default, Debug)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: Option<String>,
}
#[derive(Default, Debug)]
pub struct CommandBuilder {
    executable: Option<String>,
    args: Option<Vec<String>>,
    env: Option<Vec<String>>,
    current_dir: Option<String>,
}
impl CommandBuilder {
    fn executable(mut self, executable: String) -> Self {
        self.executable = Some(executable);
        self
    }
    fn args(mut self, args: Vec<String>) -> Self {
        self.args = Some(args);
        self
    }
    fn env(mut self, env: Vec<String>) -> Self {
        self.env = Some(env);
        self
    }
    fn current_dir(mut self, current_dir: String) -> Self {
        self.current_dir = Some(current_dir);
        self
    }
    fn build(mut self) -> Result<Command, &'static str> {
        Ok(Command {
            executable: self.executable.take().ok_or("executable must be set")?,
            args: self.args.take().ok_or("args must be set")?,
            env: self.env.take().ok_or("env must be set")?,
            current_dir: self.current_dir.take(),
        })
    }
}

impl Command {
    fn builder() -> CommandBuilder {
        CommandBuilder::default()
    }
}

fn main() {
    let command = Command::builder()
        .executable("cargo".to_owned())
        .args(vec!["build".to_owned(), "--release".to_owned()])
        .env(vec![])
        .build()
        .unwrap();
    assert!(command.current_dir.is_none());

    let command = Command::builder()
        .executable("cargo".to_owned())
        .args(vec!["build".to_owned(), "--release".to_owned()])
        .env(vec![])
        .current_dir("..".to_owned())
        .build()
        .unwrap();
    assert!(command.current_dir.is_some());
}
