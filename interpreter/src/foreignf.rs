use crate::{callable::LoxCallable, result::IResult};

struct ForeignFunctions {}
impl ForeignFunctions {
    fn clock() -> LoxCallable {
        LoxCallable {
            arity: 0,
            call: Box::new(|_, _| {
                IResult::Number(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as f64,
                )
            }),
        }
    }
}
