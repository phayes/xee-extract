use ahash::HashMap;

use crate::repl::RunContext;

type Execute = Box<dyn Fn(&[&str], &mut RunContext, &CommandDefinitions)>;

pub(crate) struct CommandDefinition {
    name: &'static str,
    short_name: Option<&'static str>,
    about: &'static str,
    args: Vec<ArgumentDefinition>,
    execute: Execute,
}

#[derive(Default)]
pub(crate) struct CommandDefinitions {
    pub(crate) definitions: Vec<CommandDefinition>,
    by_name: HashMap<&'static str, usize>,
    by_short_name: HashMap<&'static str, usize>,
}

pub(crate) struct ArgumentDefinition {
    name: &'static str,
    default: Option<&'static str>,
}

impl ArgumentDefinition {
    pub fn new(name: &'static str, default: Option<&'static str>) -> Self {
        Self { name, default }
    }
}

impl CommandDefinitions {
    pub(crate) fn new(defitions: Vec<CommandDefinition>) -> Self {
        let mut definitions = Self {
            definitions: Vec::new(),
            by_name: HashMap::default(),
            by_short_name: HashMap::default(),
        };
        for definition in defitions {
            definitions.add(definition);
        }
        definitions
    }

    pub(crate) fn add(&mut self, definition: CommandDefinition) {
        let index = self.definitions.len();
        self.by_name.insert(definition.name, index);
        if let Some(short_name) = definition.short_name {
            self.by_short_name.insert(short_name, index);
        }
        self.definitions.push(definition);
    }

    pub(crate) fn execute(&self, command: &str, run_context: &mut RunContext) {
        let parts = command.split_whitespace().collect::<Vec<_>>();
        let command_s = parts[0];
        let args = &parts[1..];
        let command = self.get(command_s);
        if let Some(command) = command {
            if args.len() > command.args.len() {
                println!("Too many arguments for command: {}", command_s);
                return;
            }
            let args = command.preprocess_arguments(args);

            if args.len() < command.args.len() {
                println!("Too few arguments for command: {}", command_s);
                return;
            }
            (command.execute)(&args, run_context, self);
        } else {
            println!("Unknown command: {}", command_s);
        }
    }

    fn get(&self, command: &str) -> Option<&CommandDefinition> {
        self.by_name
            .get(command)
            .or_else(|| self.by_short_name.get(command))
            .map(|&i| &self.definitions[i])
    }
}

impl CommandDefinition {
    pub(crate) fn new(
        name: &'static str,
        short_name: Option<&'static str>,
        about: &'static str,
        args: Vec<ArgumentDefinition>,
        execute: Execute,
    ) -> Self {
        Self {
            name,
            short_name,
            about,
            args,
            execute,
        }
    }

    pub(crate) fn help(&self) -> String {
        let description = self.arg_description();
        let main = if description.is_empty() {
            format!("!{} - {}", self.name, self.about)
        } else {
            format!("!{} {} - {}", self.name, description, self.about)
        };
        if let Some(short_name) = self.short_name {
            format!("{} (!{})", main, short_name)
        } else {
            main
        }
    }

    fn arg_description(&self) -> String {
        self.args
            .iter()
            .map(|arg| {
                if let Some(default) = arg.default {
                    format!("<{}>={}", arg.name, default)
                } else {
                    format!("<{}>", arg.name)
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn preprocess_arguments<'a>(&self, args: &[&'a str]) -> Vec<&'a str> {
        let mut result = Vec::with_capacity(self.args.len());
        let mut i = 0;
        for arg in &self.args {
            if i < args.len() {
                result.push(args[i]);
                i += 1;
            } else if let Some(default) = arg.default {
                result.push(default);
            }
        }
        result
    }
}
