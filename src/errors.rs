#[allow(dead_code)]
pub mod error_handling {
    pub struct ErrorState {
        pub error_occured: bool,
    }

    impl ErrorState {
        pub fn error(&self, line: u32, msg: &str) {
            self.report(line, "", msg);
        }

        fn report(&self, line: u32, location: &str, msg: &str) {
            println!("[line: {}, where: {}] Error: {} ", line, location, msg);
        }
    }
}
