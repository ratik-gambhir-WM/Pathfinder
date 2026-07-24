pub mod deal;
pub mod research;
pub mod users;

pub type CommandResult<T> = Result<T, String>;

pub trait CommandResultExt<T> {
    fn command_context(self, command_name: &str) -> CommandResult<T>;
}

impl<T, E> CommandResultExt<T> for Result<T, E>
where
    E: std::fmt::Display,
{
    fn command_context(self, command_name: &str) -> CommandResult<T> {
        self.map_err(|err| format!("{command_name} failed: {err}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_context_preserves_success_values() {
        assert_eq!(Ok::<_, String>(42).command_context("example"), Ok(42));
    }

    #[test]
    fn command_context_prefixes_errors_with_command_name() {
        assert_eq!(
            Err::<(), _>("bad input").command_context("example"),
            Err("example failed: bad input".to_string())
        );
    }
}
