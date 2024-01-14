pub enum Operation<T: Operational> {
    Add(T, T),
    Sub(T, T),
    Mult(T, T),
    Div(T, T),
}

pub trait Operational {
    fn add(&self, other: Self) -> Result<Self, String>
    where
        Self: Sized;
    fn sub(&self, other: Self) -> Result<Self, String>
    where
        Self: Sized;
    fn div(&self, other: Self) -> Result<Self, String>
    where
        Self: Sized;
    fn mult(&self, oter: Self) -> Result<Self, String>
    where
        Self: Sized;
}

impl Operational for i32 {
    fn add(&self, a: i32) -> Result<i32, String> {
        Ok(*self + a)
    }

    fn sub(&self, a: i32) -> Result<i32, String> {
        Ok(*self - a)
    }

    fn div(&self, a: i32) -> Result<i32, String> {
        if a == 0 {
            Err("Cannot devide by zero".to_string())
        } else {
            Ok(self / a)
        }
    }

    fn mult(&self, a: i32) -> Result<i32, String> {
        Ok(self * a)
    }
}

impl<T: Operational> Operation<T> {
    pub fn build(operation: &str, param1: T, param2: T) -> Result<Operation<T>, String> {
        match operation.to_lowercase().as_str() {
            "add" => Ok(Operation::Add(param1, param2)),
            "sub" => Ok(Operation::Sub(param1, param2)),
            "div" => Ok(Operation::Div(param1, param2)),
            "mult" => Ok(Operation::Mult(param1, param2)),
            unknown => Err(format!("Unknown operation - {}", unknown)),
        }
    }
}

pub fn perform_operation<T: Operational>(operation: Operation<T>) -> Result<T, String> {
    match operation {
        Operation::Add(a, b) => a.add(b),
        Operation::Sub(a, b) => a.sub(b),
        Operation::Mult(a, b) => a.mult(b),
        Operation::Div(a, b) => a.div(b),
    }
}
