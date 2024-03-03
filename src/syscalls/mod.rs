pub mod definitions;

use definitions::{SystemCall, SystemCallResponse};

// System call handler
pub fn handler(syscall: &SystemCall) -> &SystemCallResponse {
    match syscall {
        SystemCall::Test => &SystemCallResponse::TestResponse(69),
    }
}
