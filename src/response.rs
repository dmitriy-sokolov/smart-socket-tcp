use std::fmt;

pub enum Response {
    Unknown,
    Ok,
    Enabled,
    Disabled,
    Power(f32),
}

impl From<[u8; 5]> for Response {
    fn from(bytes: [u8; 5]) -> Self {
        match bytes {
            [1, ..] => Self::Ok,
            [2, ..] => Self::Enabled,
            [3, ..] => Self::Disabled,
            [4, ..] => {
                let mut buf = [0u8; 4];
                buf.copy_from_slice(&bytes[1..]);
                Self::Power(f32::from_be_bytes(buf))
            }
            _ => Self::Unknown,
        }
    }
}

impl From<Response> for [u8; 5] {
    fn from(resp: Response) -> Self {
        let mut buffer = [0u8; 5];
        match resp {
            Response::Ok => buffer[0] = 1,
            Response::Enabled => buffer[0] = 2,
            Response::Disabled => buffer[0] = 3,
            Response::Power(pwr) => {
                buffer[0] = 4;
                buffer[1..].copy_from_slice(&pwr.to_be_bytes())
            }
            Response::Unknown => {}
        };
        buffer
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Response::Ok => write!(f, "Ok"),
            Response::Enabled => write!(f, "Enabled"),
            Response::Disabled => write!(f, "Disabled"),
            Response::Power(power) => write!(f, "Power: {}", power),
            Response::Unknown => write!(f, "Unknown"),
        }
    }
}
