use std::fmt;

enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T: fmt::Display, E: fmt::Display> fmt::Display for MyResult<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyResult::Ok(value) => write!(f, "{}", value),
            MyResult::Err(err) => write!(f, "{}", err),
        }
    }
}

impl<T: Clone, E: Clone> Clone for MyResult<T, E> {
    fn clone(&self) -> Self {
        match self {
            MyResult::Ok(value) => MyResult::Ok(value.clone()),
            MyResult::Err(err) => MyResult::Err(err.clone()),
        }
    }
}

impl<T, E> MyResult<T, E> {
    // Pure Transform
    fn map<U, F>(self, f: F) -> MyResult<U, E>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            MyResult::Ok(value) => MyResult::Ok(f(value)),
            MyResult::Err(err) => MyResult::Err(err),
        }
    }

    // Control Flow
    fn and_then<U, F>(self, f: F) -> MyResult<U, E>
    where
        F: FnOnce(T) -> MyResult<U, E>,
    {
        match self {
            MyResult::Ok(value) => f(value),
            MyResult::Err(err) => MyResult::Err(err),
        }
    }

    // Escape Hatch
    fn unwrap_or(self, default: T) -> T {
        match self {
            MyResult::Ok(value) => value,
            MyResult::Err(_) => default,
        }
    }
}

pub fn run() {
    println!("Build result type");

    let result = MyResult::<i32, String>::Ok(2);
    assert_eq!(result.clone().map(|v| v * 20).unwrap_or(0), 40);
    assert_eq!(
        result
            .map(|v| v * 20 + 1)
            .and_then(|v| {
                if v & 1 == 0 {
                    MyResult::Ok(v)
                } else {
                    MyResult::Err("NOT_EVENT".into())
                }
            })
            .unwrap_or(0),
        0
    );

    println!("_________________");
}
