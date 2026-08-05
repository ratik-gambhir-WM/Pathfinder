pub mod data_room;
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
#[path = "../../tests/commands/mod_tests.rs"]
mod tests;
