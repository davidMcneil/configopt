use std::{collections::BTreeMap, convert::Infallible, env, fmt, mem, str::FromStr};

mod parser;
#[cfg(test)]
mod tests;

use parser::{CliComponent, LONG_PREFIX, POSITIONAL_DELIMITER, SHORT_PREFIX};

static EMPTY: &[String] = &[];

pub type Short = char;
pub type Long = String;

pub enum Arg {
    Short { name: Short, values: Vec<String> },
    Long { name: String, values: Vec<String> },
}

enum More {
    None,
    Positionals(Vec<String>),
    Subcommand(Box<Command>),
}

impl Default for More {
    fn default() -> Self {
        More::None
    }
}

pub struct Command {
    name: String,
    short_args: BTreeMap<Short, Vec<String>>,
    long_args: BTreeMap<Long, Vec<String>>,
    more: More,
}

impl Command {
    pub fn new(name: String) -> Self {
        Self {
            name,
            short_args: BTreeMap::new(),
            long_args: BTreeMap::new(),
            more: More::None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        let iter = parser::parse(iter);
        // Peek at the first component. If it is of type Command use that as the name of the
        // initial command otherwise use the empty string.
        let mut iter = iter.into_iter().peekable();
        let name = if let Some(CliComponent::Command(command)) = iter.peek() {
            let name = String::from(command);
            iter.next();
            name
        } else {
            String::new()
        };
        Self::from_component_iter(name, iter)
    }

    fn from_component_iter(name: String, mut iter: impl Iterator<Item = CliComponent>) -> Self {
        let mut command = Command::new(name);
        let subcommand_name = loop {
            if let Some(component) = iter.next() {
                match component {
                    CliComponent::Short(name, value) => command.add_short(name, value),
                    CliComponent::Long(name, value) => command.add_long(&name, value),
                    CliComponent::Positionals(positional) => command.add_positionals(positional),
                    CliComponent::Command(subcommand_name) => {
                        break subcommand_name;
                    }
                }
            } else {
                return command;
            }
        };
        command.add_subcommand(Command::from_component_iter(subcommand_name, iter));
        command
    }

    pub fn from_env() -> Self {
        Self::from_iter(env::args())
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn subcommand(&self) -> Option<&Command> {
        match &self.more {
            More::None => None,
            More::Positionals(_) => None,
            More::Subcommand(subcommand) => Some(subcommand),
        }
    }

    pub fn subcommand_mut(&mut self) -> Option<&mut Command> {
        match &mut self.more {
            More::None => None,
            More::Positionals(_) => None,
            More::Subcommand(subcommand) => Some(subcommand),
        }
    }

    pub fn add_subcommand(&mut self, subcommand: Command) {
        self.more = More::Subcommand(Box::new(subcommand));
    }

    pub fn remove_subcommand(&mut self) -> Option<Command> {
        match &mut self.more {
            More::None => None,
            More::Positionals(_) => None,
            More::Subcommand(_) => match mem::take(&mut self.more) {
                More::None => None,
                More::Positionals(_) => None,
                More::Subcommand(subcommand) => Some(*subcommand),
            },
        }
    }

    pub fn positionals(&self) -> Option<impl Iterator<Item = &String>> {
        match &self.more {
            More::None => None,
            More::Positionals(positional) => Some(positional.iter()),
            More::Subcommand(_) => None,
        }
    }

    pub fn positionals_mut(&mut self) -> Option<impl Iterator<Item = &mut String>> {
        match &mut self.more {
            More::None => None,
            More::Positionals(positional) => Some(positional.iter_mut()),
            More::Subcommand(_) => None,
        }
    }

    pub fn add_positionals(&mut self, positional: Vec<String>) {
        self.more = More::Positionals(positional);
    }

    pub fn remove_positionals(&mut self) -> Option<Vec<String>> {
        match &mut self.more {
            More::None => None,
            More::Positionals(_) => match mem::take(&mut self.more) {
                More::None => None,
                More::Positionals(positional) => Some(positional),
                More::Subcommand(_) => None,
            },
            More::Subcommand(_) => None,
        }
    }

    pub fn find_subcommand(&self, subcommand_path: &[&str]) -> Option<&Command> {
        if subcommand_path.is_empty() {
            return Some(self);
        }
        self.subcommand().and_then(|sub| {
            if sub.name == subcommand_path[0] {
                sub.find_subcommand(&subcommand_path[1..])
            } else {
                None
            }
        })
    }

    pub fn find_subcommand_mut(&mut self, subcommand_path: &[&str]) -> Option<&mut Command> {
        if subcommand_path.is_empty() {
            return Some(self);
        }
        self.subcommand_mut().and_then(|sub| {
            if sub.name == subcommand_path[0] {
                sub.find_subcommand_mut(&subcommand_path[1..])
            } else {
                None
            }
        })
    }

    pub fn remove_subcommand_path(&mut self, subcommand_path: &[&str]) -> Option<Command> {
        self.find_subcommand_mut(subcommand_path)
            .and_then(|sub| sub.remove_subcommand())
    }

    pub fn short_occurrences(&self, name: Short) -> usize {
        self.short_args
            .get(&name)
            .map(|values| values.len())
            .unwrap_or(0)
    }

    pub fn short_exists(&self, name: Short) -> bool {
        self.short_occurrences(name) > 0
    }

    pub fn short_values(&self, name: Short) -> impl Iterator<Item = &String> {
        self.short_args
            .get(&name)
            .map(|values| values.iter())
            .unwrap_or_else(|| EMPTY.iter())
    }

    pub fn short_value(&self, name: Short) -> Option<&String> {
        self.short_args.get(&name).and_then(|values| values.get(0))
    }

    pub fn short_value_mut(&mut self, name: Short) -> Option<&mut String> {
        self.short_args
            .get_mut(&name)
            .and_then(|values| values.get_mut(0))
    }

    pub fn short_retain_first_value(&mut self, name: Short) -> Option<Vec<String>> {
        self.short_args
            .get_mut(&name)
            .map(|values| values.drain(1..).collect())
    }

    pub fn short_retain_last_value(&mut self, name: Short) -> Option<Vec<String>> {
        let len = self.short_args.len();
        self.short_args
            .get_mut(&name)
            .map(|values| values.drain(..len - 1).collect())
    }

    pub fn add_short(&mut self, name: Short, value: String) {
        self.short_args.entry(name).or_default().push(value)
    }

    pub fn remove_short(&mut self, name: Short) -> Option<Vec<String>> {
        self.short_args.remove(&name)
    }

    pub fn long_occurrences(&self, name: &str) -> usize {
        self.long_args
            .get(name)
            .map(|values| values.len())
            .unwrap_or(0)
    }

    pub fn long_exists(&self, name: &str) -> bool {
        self.long_occurrences(name) > 0
    }

    pub fn long_values(&self, name: &str) -> impl Iterator<Item = &String> {
        self.long_args
            .get(name)
            .map(|values| values.iter())
            .unwrap_or_else(|| EMPTY.iter())
    }

    pub fn long_value(&self, name: &str) -> Option<&String> {
        self.long_args.get(name).and_then(|values| values.get(0))
    }

    pub fn long_value_mut(&mut self, name: &str) -> Option<&mut String> {
        self.long_args
            .get_mut(name)
            .and_then(|values| values.get_mut(0))
    }

    pub fn long_retain_first_value(&mut self, name: &str) -> Option<Vec<String>> {
        self.long_args
            .get_mut(name)
            .map(|values| values.drain(1..).collect())
    }

    pub fn long_retain_last_value(&mut self, name: &str) -> Option<Vec<String>> {
        let len = self.long_args.len();
        self.long_args
            .get_mut(name)
            .map(|values| values.drain(..len - 1).collect())
    }

    pub fn add_long(&mut self, name: &str, value: String) {
        self.long_args
            .entry(name.to_string())
            .or_default()
            .push(value)
    }

    pub fn remove_long(&mut self, name: &str) -> Option<Vec<String>> {
        self.long_args.remove(name)
    }

    pub fn arg_occurrences(&self, short: Short, long: &str) -> usize {
        self.short_occurrences(short) + self.long_occurrences(long)
    }

    pub fn arg_exists(&self, short: Short, long: &str) -> bool {
        self.arg_occurrences(short, long) > 0
    }

    pub fn arg_values(&self, short: Short, long: &str) -> impl Iterator<Item = &String> {
        self.short_values(short).chain(self.long_values(long))
    }

    pub fn arg_value(&self, short: Short, long: &str) -> Option<&String> {
        self.short_value(short).or_else(|| self.long_value(long))
    }

    pub fn arg_value_mut(&mut self, short: Short, long: &str) -> Option<&mut String> {
        if self.short_exists(short) {
            self.short_value_mut(short)
        } else {
            self.long_value_mut(long)
        }
    }

    pub fn remove_arg(&mut self, short: Short, long: &str) -> Option<Vec<String>> {
        let short = self.remove_short(short);
        let long = self.remove_long(long);
        match (short, long) {
            (None, None) => None,
            (short, None) => short,
            (None, long) => long,
            (Some(mut short), Some(long)) => {
                short.extend(long);
                Some(short)
            }
        }
    }

    pub fn remove_all_arg(&mut self) {
        self.short_args.clear();
        self.long_args.clear();
    }
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        Self::from_iter(String::from(s).split_whitespace())
    }
}

impl FromStr for Command {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        for (short, values) in &self.short_args {
            for value in values {
                write!(f, " {}{}{}", SHORT_PREFIX, short, value)?;
            }
        }
        for (long, values) in &self.long_args {
            for value in values {
                write!(f, " {}{}={}", LONG_PREFIX, long, value)?;
            }
        }
        if let Some(subcommand) = self.subcommand() {
            write!(f, " {}", subcommand)?;
        }
        if let Some(positionals) = self.positionals() {
            write!(f, " {}", POSITIONAL_DELIMITER,)?;
            for positional in positionals {
                write!(f, " {}", positional)?;
            }
        }
        Ok(())
    }
}
