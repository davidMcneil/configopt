pub(crate) const SHORT_PREFIX: &str = "-";
pub(crate) const LONG_PREFIX: &str = "--";
pub(crate) const LONG_DELIMITER: &str = "=";
pub(crate) const POSITIONAL_DELIMITER: &str = "--";

fn split1<'a>(s: &'a str, p: &str) -> (&'a str, Option<&'a str>) {
    let mut split = s.splitn(2, p);
    let first = split.next().unwrap();
    let second = split.next();
    (first, second)
}

fn long(s: &str) -> Option<(&str, Option<&str>)> {
    match split1(s, LONG_PREFIX) {
        (_, None) => None,
        ("", Some("")) => None,
        ("", Some(long)) => {
            return match split1(long, LONG_DELIMITER) {
                (_, None) => Some((long, None)),
                (name, Some(value)) => Some((name, Some(value))),
            };
        }
        (_, Some(_)) => None,
    }
}

fn short(s: &str) -> Option<(char, Option<&str>)> {
    match split1(s, SHORT_PREFIX) {
        (_, None) => None,
        ("", Some("")) => None,
        ("", Some(short)) => {
            let name = short
                .chars()
                .next()
                .expect("expected at least one char in short");
            if short.len() == 1 {
                Some((name, None))
            } else {
                Some((name, Some(&short[1..])))
            }
        }
        (_, Some(_)) => None,
    }
}

enum ParserComponent<'a> {
    Short(char, Option<&'a str>),
    Long(&'a str, Option<&'a str>),
    PositionalDelimiter,
    Symbol,
}

impl<'a> ParserComponent<'a> {
    fn new(s: &'a str) -> Self {
        if s == POSITIONAL_DELIMITER {
            return ParserComponent::PositionalDelimiter;
        }
        if let Some(long) = long(s) {
            return ParserComponent::Long(long.0, long.1);
        }
        if let Some(short) = short(s) {
            return ParserComponent::Short(short.0, short.1);
        }
        ParserComponent::Symbol
    }
}

enum State {
    Start,
    WaitingPositionals(Vec<String>),
    WaitingShortValue(char),
    WaitingLongValue(String),
}

pub enum CliComponent {
    Short(char, String),
    Long(String, String),
    Positionals(Vec<String>),
    Command(String),
}

fn next_state(state: State, current: String, next: &str) -> (State, Option<CliComponent>) {
    match state {
        State::Start => match (ParserComponent::new(&current), ParserComponent::new(next)) {
            (ParserComponent::Short(name, None), ParserComponent::Symbol) => {
                (State::WaitingShortValue(name), None)
            }
            (ParserComponent::Long(name, None), ParserComponent::Symbol) => {
                (State::WaitingLongValue(String::from(name)), None)
            }
            (ParserComponent::Short(name, None), _) => {
                (State::Start, Some(CliComponent::Short(name, String::new())))
            }
            (ParserComponent::Long(name, None), _) => (
                State::Start,
                Some(CliComponent::Long(String::from(name), String::new())),
            ),
            (ParserComponent::Short(name, Some(value)), _) => (
                State::Start,
                Some(CliComponent::Short(name, String::from(value))),
            ),
            (ParserComponent::Long(name, Some(value)), _) => (
                State::Start,
                Some(CliComponent::Long(String::from(name), String::from(value))),
            ),
            (ParserComponent::PositionalDelimiter, _) => {
                (State::WaitingPositionals(Vec::new()), None)
            }
            (ParserComponent::Symbol, _) => (State::Start, Some(CliComponent::Command(current))),
        },
        State::WaitingPositionals(mut positionals) => {
            positionals.push(current);
            if next.is_empty() {
                (State::Start, Some(CliComponent::Positionals(positionals)))
            } else {
                (State::WaitingPositionals(positionals), None)
            }
        }
        State::WaitingShortValue(name) => (State::Start, Some(CliComponent::Short(name, current))),
        State::WaitingLongValue(name) => (State::Start, Some(CliComponent::Long(name, current))),
    }
}

pub fn parse<I>(iter: I) -> Vec<CliComponent>
where
    I: IntoIterator,
    I::Item: Into<String>,
{
    let mut iter = iter.into_iter().map(I::Item::into).peekable();
    let mut result = Vec::new();
    let mut state = State::Start;
    let empty = String::new();
    loop {
        let (current, next) = match (iter.next(), iter.peek()) {
            (Some(current), Some(next)) => (current, next),
            (Some(current), None) => (current, &empty),
            (None, _) => {
                return result;
            }
        };
        let state_and_component = next_state(state, current, next);
        state = state_and_component.0;
        if let Some(component) = state_and_component.1 {
            result.push(component);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split1() {
        assert_eq!(split1("testing:123", ":"), ("testing", Some("123")));
        assert_eq!(
            split1("testing:123:more", ":"),
            ("testing", Some("123:more"))
        );
        assert_eq!(split1("testing", ":"), ("testing", None));
        assert_eq!(split1("testing", ":"), ("testing", None));
        assert_eq!(split1("", ":"), ("", None));
        assert_eq!(split1(":testing", ":"), ("", Some("testing")));
        assert_eq!(split1(":", ":"), ("", Some("")));
        assert_eq!(split1("testing123", "123"), ("testing", Some("")));
    }

    #[test]
    fn test_long() {
        assert_eq!(long(""), None);
        assert_eq!(long("test"), None);
        assert_eq!(long("test--more"), None);
        assert_eq!(long("--"), None);
        assert_eq!(long("--arg_one"), Some(("arg_one", None)));
        assert_eq!(
            long("--arg_one=the_value="),
            Some(("arg_one", Some("the_value=")))
        );
    }

    #[test]
    fn test_short() {
        assert_eq!(short(""), None);
        assert_eq!(short("t"), None);
        assert_eq!(short("test--more"), None);
        assert_eq!(short("-"), None);
        assert_eq!(short("-a"), Some(('a', None)));
        assert_eq!(short("-Athe_value="), Some(('A', Some("the_value="))));
    }
}
