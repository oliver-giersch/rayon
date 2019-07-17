use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    let mut args: Vec<_> = env::args().skip(1).collect();
    args.sort();
    args.dedup();

    let parent = env::current_dir()
        .expect("failed to read current dir")
        .join("results");

    let epoch = parent.join("bench_w_epoch");
    let debra = parent.join("bench_w_debra");
    let hazptr = parent.join("bench_w_hazptr");

    for arg in args {
        match arg.as_ref() {
            "epoch" => {
                println!("running rayon benchmarks with epoch...");
                run_bench(&epoch, None);
            }
            "debra" => {
                println!("running rayon benchmarks with debra...");
                run_bench(&debra, Some(&["use-debra"]));
            }
            "hazptr" => {
                println!("running rayon benchmarks with hazptr...");
                run_bench(&hazptr, Some(&["use-hazptr"]));
            }
            _ => panic!("invalid argument"),
        }
    }
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
        //.arg("dj10")
        .output()
        .expect("failed to run benchmarks");

    let mut file = File::create(outfile).expect("could not create file");
    file.write_all(&out.stdout)
        .expect("could not write to file");
}
