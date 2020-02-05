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
    fn run_pants(&self, args: &[&str]) -> Result<process::Output, std::io::Error> {
        process::Command::new("./pants").args(args).output()
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let in_mini_repo = InMiniRepo::default();
    c.bench_function("./pants goals", move |b| {
        b.iter(|| in_mini_repo.run_pants(&["goals"]).unwrap())
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
