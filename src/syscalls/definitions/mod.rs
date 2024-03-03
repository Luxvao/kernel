// Add a system call entry here
pub enum SystemCall {
    Test,
}

// Add a system call response here
pub enum SystemCallResponse {
    TestResponse(i32),
    SystemCallError(SystemCallError),
}

// Everything that could go wrong
pub enum SystemCallError {}
