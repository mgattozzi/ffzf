#![feature(try_from)]

#[macro_use]
extern crate error_chain;
extern crate crossbeam;

mod error;
mod entry;

use std::fs::read_dir;
use std::path::{ PathBuf, Path };
use std::convert::TryInto;
use std::sync::mpsc::{ Sender, channel };
use std::process;
use error::*;
use entry::*;
use std::io::{ Write, stderr, stdout };

fn main() {
    if let Err(ref e) = run() {
        let stderr = &mut stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "Error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "Caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this with
        // `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "Backtrace: {:?}", backtrace).expect(errmsg);
        }

        process::exit(1);
    }
}

fn run() -> Result<()> {
    let test_dir = "/home/michael/Code";
    let (tx, rx) = channel();

    file_search(test_dir, tx)?;

    let stdout = stdout();
    let mut lock = stdout.lock();
    loop {
        match rx.try_recv() {
            Ok(rec) => {
                print!("{}\n", rec.display());
            },
            Err(_) => break,
        }
    }
    lock.flush()?;

    Ok(())
}

fn file_search<P>(path: P, tx: Sender<PathBuf>) -> Result<()>
    where P: AsRef<Path>
{
    for entry in read_dir(path)? {
        let entry: Entry = entry?.try_into()?;
        let dir = entry.is_dir();
        let symlink = entry.is_symlink();
        if dir && !symlink {
            let tx_clone = tx.clone();
            crossbeam::scope(|scope| {
                scope.spawn(move || {
                    let _ = file_search(entry.path(), tx_clone).unwrap();
                });
            });
        } else if !dir && !symlink {
            tx.send(entry.path())?;
        }
    }

    Ok(())
}
