1. catching unwinds with `std::panic::catch_unwind`; whereas catch panics and inspect the error messges by changing hooks `set_hook` & `take_hook`

```rust
        let prev = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            if let Some(s) = info.payload().downcast_ref::<&str>() {
                if s.starts_with(PARSER_ERR_TAG) {
                    eprintln!("parser error: {s:?}");
                    return;
                }
            }
            prev(info);
        }));

        let result = panic::catch_unwind(AssertUnwindSafe(|| self.expression()));
        return if let Ok(exp) = result {
            Ok(exp)
        } else {
            Err("parser error".to_string())
        };

```

