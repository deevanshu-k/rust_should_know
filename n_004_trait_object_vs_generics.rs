trait Job {
    fn run(&self);
}

// For every concrete type T, rust generates seperate machine code for run_all_concrete.
// run_all_concrete<Foo1>
// run_all_concrete<Foo2>
fn run_all_concrete<T: Job>(jobs: Vec<T>) {
    for job in jobs {
        job.run();
    }
}

// Only one version of run_all
fn run_all(jobs: Vec<Box<dyn Job>>) {
    for job in jobs {
        job.run();
    }
}

struct Print {
    s: String,
}

impl Job for Print {
    fn run(&self) {
        println!("Print: {}", self.s);
    }
}

pub fn run() {
    println!("Trait Object vs Generics");

    println!("> run_all_concrete");
    run_all_concrete(vec![
        Print { s: "job1".into() },
        Print { s: "job2".into() },
        Print { s: "job3".into() },
    ]);

    println!("> run_all");
    run_all(vec![
        Box::new(Print { s: "job1".into() }),
        Box::new(Print { s: "job2".into() }),
        Box::new(Print { s: "job3".into() }),
    ]);

    println!("________________________");
}
