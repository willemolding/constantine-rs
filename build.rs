use std::io::{self, Write};
use std::process::Command;

fn main() {
    let output = Command::new("nim")
        .arg("c")
        .arg("--noMain")
        .arg("--noLinking")
        .arg("--nimcache:nimcache")
        .arg("constantine/constantine/blssig_pop_on_bls12381_g2.nim")
        .output()
        .unwrap();
    if !output.status.success() {
        let msg = String::from_utf8_lossy(output.stderr.as_slice());
        let _ = writeln!(io::stderr(), "\nerror occurred: {}\n", msg);
        std::process::exit(1);
    }

    let files = std::fs::read_dir("nimcache").unwrap()
        .filter_map(Result::ok)
        .filter(|d| if let Some(e) = d.path().extension() { e == "c" } else {false})
        .map(|d| d.path());
    
    cc::Build::new()
        .include("/usr/lib/nim")
        .include("nimcache")
        .files(files)
        .warnings(false)
        .compile("bls_nim");
}