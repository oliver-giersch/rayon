use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    let parent = env::current_dir()
        .expect("failed to read current dir")
        .join("results");

    let epoch = parent.join("bench_w_epoch");
    let debra = parent.join("bench_w_debra");
    let hazptr = parent.join("bench_w_hazptr");

    println!("running rayon benchmarks with epoch...");
    run_bench(&epoch, None);
    println!("running rayon benchmarks with debra...");
    run_bench(&debra, Some(&["use-debra"]));
    println!("running rayon benchmarks with hazptr...");
    run_bench(&hazptr, Some(&["use-hazptr"]));
}

fn run_bench(outfile: &Path, args: Option<&[&str]>) {
    let mut cmd = Command::new("cargo");

    cmd.arg("+nightly").arg("bench");

    if let Some(slice) = args {
        cmd.arg("--features").args(slice);
    }

    let out = cmd
        .arg("--package")
        .arg("rayon-demo")
        .output()
        .expect("failed to run benchmarks");

    let mut file = File::create(outfile).expect("could not create file");
    file.write_all(&out.stdout)
        .expect("could not write to file");
}
