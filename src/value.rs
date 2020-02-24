#[derive(Clone)]
pub enum Value {
    Int(i32),
    Float(f64),
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
        }
    }
}
