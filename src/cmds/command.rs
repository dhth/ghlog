use crate::domain::events::EventLimit;
use crate::domain::user::Username;

pub enum Command {
    Run {
        username: Username,
        limit: EventLimit,
    },
}

impl TryFrom<crate::cli::Command> for Command {
    type Error = anyhow::Error;

    fn try_from(command: crate::cli::Command) -> Result<Self, Self::Error> {
        match command {
            crate::cli::Command::Run { username, limit } => Ok(Self::Run {
                username: Username::try_from(username)?,
                limit: EventLimit::try_from(limit)?,
            }),
        }
    }
}

impl Command {
    pub fn handle(self) -> anyhow::Result<()> {
        match self {
            Self::Run { username, limit } => super::run::handle(&username, limit),
        }
    }
}
