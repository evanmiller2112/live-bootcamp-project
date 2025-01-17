use std::ffi::CString;
use validator::Validate;
struct Email {
    #[validate(email)]
    email: String,
}