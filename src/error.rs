use std::path::PathBuf;

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        SendError(::std::sync::mpsc::SendError<PathBuf>);
    }
}
