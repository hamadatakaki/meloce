pub struct NotBoundError;

pub struct Environment<T: Clone> {
    records: Vec<(String, T)>,
}

impl<T: Clone> Environment<T> {
    pub fn empty() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    pub fn extend(&mut self, x: &str, v: T) {
        self.records.push((String::from(x), v));
    }

    pub fn lookup(&self, x: &str) -> Result<&T, NotBoundError> {
        match self.records.iter().find(|&(key, _)| key == x).iter().next() {
            Some(&(_, v)) => Ok(v),
            None => Err(NotBoundError {}),
        }
    }

    pub fn map<U, F>(&self, f: F) -> Environment<U>
    where
        U: Clone,
        F: Fn(T) -> U,
    {
        let records = self
            .records
            .iter()
            .map(|(x, v)| (x.clone(), f(v.to_owned())))
            .collect();
        Environment { records }
    }

    pub fn fold_right<U, F>(&self, f: F, a: U) -> U
    where
        F: FnMut(U, T) -> U,
    {
        self.records
            .iter()
            .rev()
            .map(|(_, v)| v.to_owned())
            .fold(a, f)
    }
}

#[test]
fn test_initialize_environment() {
    use crate::eval::{apply_prim, ExpressedValue};
    use crate::syntax::BinOp;

    let mut env = Environment::empty();
    env.extend("x", ExpressedValue::IntValue(10));
    env.extend("v", ExpressedValue::IntValue(5));
    env.extend("i", ExpressedValue::IntValue(1));

    let u = env.fold_right(
        |acc, x| apply_prim(BinOp::Plus, acc, x).unwrap(),
        ExpressedValue::IntValue(0),
    );
    assert_eq!(u.to_int().unwrap(), 16)
}
