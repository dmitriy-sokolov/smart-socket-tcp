pub enum Command {
    Unknown,
    TurnOff,
    TurnOn,
    IsEnabled,
    GetPower,
}

impl From<u8> for Command {
    fn from(val: u8) -> Self {
        match val {
            1 => Self::TurnOff,
            2 => Self::TurnOn,
            3 => Self::IsEnabled,
            4 => Self::GetPower,
            _ => Self::Unknown,
        }
    }
}

impl From<Command> for u8 {
    fn from(cmd: Command) -> Self {
        match cmd {
            Command::Unknown => 0,
            Command::TurnOff => 1,
            Command::TurnOn => 2,
            Command::IsEnabled => 3,
            Command::GetPower => 4,
        }
    }
}
