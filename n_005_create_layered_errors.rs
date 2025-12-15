use std::{fmt, fs};

#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Parse(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(err) => write!(f, "IO Error: {}", err),
            AppError::Parse(str) => write!(f, "Parsed error: {}", str),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError::Io(value)
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Io(e) => Some(e),
            AppError::Parse(_) => None,
        }
    }
}

fn read_number() -> Result<i32, AppError> {
    let s = fs::read_to_string("number.txt")?;
    let n = s
        .trim()
        .parse::<i32>()
        .map_err(|_| AppError::Parse("Invalid number".into()))?;
    Ok(n)
}

pub fn run() {
    println!("Create layered errors");

    match read_number() {
        Ok(n) => println!("Parsed number: {}", n),
        Err(err) => eprintln!("{}", err),
    }

    println!("_____________________");
}
