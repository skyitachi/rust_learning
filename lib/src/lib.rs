#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod network;

pub mod client;

mod outermost {
    pub fn middle_function() {
        inside::inner_function();
        // inside::secret_function();
    }
    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {
            secret_function();
            super::middle_secret_function();
        }       
        fn secret_function() {}
    }
}