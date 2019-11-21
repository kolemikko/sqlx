pub mod com_stmt_close;
pub mod com_stmt_exec;
pub mod com_stmt_fetch;
pub mod com_stmt_prepare;
pub mod com_stmt_prepare_ok;
pub mod com_stmt_reset;

pub use com_stmt_close::ComStmtClose;
pub use com_stmt_exec::{ComStmtExecute, StmtExecFlag};
pub use com_stmt_fetch::ComStmtFetch;
pub use com_stmt_prepare::ComStmtPrepare;
pub use com_stmt_prepare_ok::ComStmtPrepareOk;
pub use com_stmt_reset::ComStmtReset;

pub enum BinaryProtocol {
    ComStmtPrepare = 0x16,
    ComStmtExec = 0x17,
    ComStmtClose = 0x19,
    ComStmtReset = 0x1A,
    ComStmtFetch = 0x1C,
}