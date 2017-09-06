quick_error! {
    #[derive(Debug)]
    pub enum CommonError {
        CommonError(text: &'static str) {
            description(text)
        }
    }
}
