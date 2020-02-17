use criterion::{criterion_group, criterion_main, Criterion};
use git2::Repository;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process,
};

const TEMP_DIR: &'static str = "/tmp/minirepo";
const MINIREPO_REMOTE: &'static str = "/Users/pchevalier/Documents/code/minirepo";

struct InMiniRepo {
    old_dir: PathBuf,
}

impl Default for InMiniRepo {
    fn default() -> Self {
        let in_mini_repo = InMiniRepo {
            old_dir: env::current_dir().unwrap(),
        };
        Repository::clone(MINIREPO_REMOTE, TEMP_DIR).expect("Couldn't clone minirepo");
        env::set_current_dir(&Path::new(TEMP_DIR))
            .expect(&format!("Couldn't chdir to {}", TEMP_DIR));
        in_mini_repo
    }
}

impl Drop for InMiniRepo {
    fn drop(&mut self) {
        env::set_current_dir(&self.old_dir).unwrap();
        fs::remove_dir_all(TEMP_DIR).unwrap();
    }
}

impl InMiniRepo {
    fn run_pants(args: &[&str]) -> Result<process::Output, std::io::Error> {
        process::Command::new("./pants").args(args).output()
    }
    fn run_pants_pex(args: &[&str]) -> Result<process::Output, std::io::Error> {
        process::Command::new("./pants-1.24.0rc1-git2481dab2-md59a1c1708.pex").args(args).output()

    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let _guard = InMiniRepo::default();
    c.bench_function("version", |b| {
        b.iter(|| InMiniRepo::run_pants(&["version"]).unwrap())
    });
    c.bench_function("version (pex)", |b| {
        b.iter(|| InMiniRepo::run_pants_pex(&["version"]).unwrap())
    });
    c.bench_function("goals", |b| {
        b.iter(|| InMiniRepo::run_pants(&["goals"]).unwrap())
    });
    c.bench_function("help", |b| {
        b.iter(|| InMiniRepo::run_pants(&["help"]).unwrap())
    });
    c.bench_function("binary (v2)", |b| {
        b.iter(|| InMiniRepo::run_pants(&["binary", "hello:", "--v2", "--no-v1"]).unwrap())
    });
    c.bench_function("binary (v1)", |b| {
        b.iter(|| InMiniRepo::run_pants(&["binary", "hello:"]).unwrap())
    });
}

fn small_sample_size() -> Criterion {
    Criterion::default().sample_size(10)
}

criterion_group! {
    name = benches;
    config = small_sample_size();
    targets = criterion_benchmark
}
criterion_main!(benches);
