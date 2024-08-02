use clap::{Parser, ValueEnum};

fn main() {
    let app = StackExhaustTestApp::parse();

    let use_stacker = match app.use_stacker {
        UseStackerOpt::Always => UseStacker::Always {
            red_zone: app.red_zone,
            stack_size: app.stack_size,
        },
        UseStackerOpt::Once => UseStacker::Once {
            red_zone: app.red_zone,
            stack_size: app.stack_size,
        },
        UseStackerOpt::No => UseStacker::No,
    };

    let inner = if app.make_large_local {
        recurse_with_large_local
    } else {
        recurse_without_local
    };

    eprintln!(
        "running with use_stacker = {:?}, new_thread = {}, make_large_local = {}",
        use_stacker, app.new_thread, app.make_large_local,
    );

    if app.new_thread {
        std::thread::spawn(move || recurse(app.stack_depth, use_stacker, inner))
            .join()
            .unwrap();
    } else {
        recurse(app.stack_depth, use_stacker, inner);
    }
}

fn recurse(remaining: usize, use_stacker: UseStacker, inner: fn(usize, UseStacker)) {
    match use_stacker {
        UseStacker::Always {
            red_zone,
            stack_size,
        }
        | UseStacker::Once {
            red_zone,
            stack_size,
        } => stacker::maybe_grow(red_zone, stack_size, || {
            recurse_with_large_local(remaining, use_stacker.next())
        }),
        UseStacker::No => inner(remaining, use_stacker.next()),
    }
}

fn recurse_without_local(remaining: usize, use_stacker: UseStacker) {
    if remaining > 0 {
        recurse(remaining - 1, use_stacker, recurse_without_local);
    }
}

fn recurse_with_large_local(remaining: usize, use_stacker: UseStacker) {
    let array = [0u8; 1024];
    // Attempt to ensure that array doesn't get optimized out.
    std::hint::black_box(array);
    if remaining > 0 {
        recurse(remaining - 1, use_stacker, recurse_with_large_local);
    }
}

#[derive(Debug, Parser)]
struct StackExhaustTestApp {
    #[clap(long, short = 's', default_value_t, value_enum)]
    use_stacker: UseStackerOpt,

    #[clap(long, short = 'l')]
    make_large_local: bool,

    #[clap(long, short = 't', default_value_t)]
    new_thread: bool,

    #[clap(long, short = 'r', default_value_t = 100 * 1024, requires = "use_stacker")]
    red_zone: usize,

    #[clap(long, short = 'z', default_value_t = 1 * 1024 * 1024, requires = "use_stacker")]
    stack_size: usize,

    #[clap(long, short = 'n', default_value_t = 100000)]
    stack_depth: usize,
}

#[derive(Clone, Copy, Debug, Default, ValueEnum)]
enum UseStackerOpt {
    Always,
    Once,
    #[default]
    No,
}

#[derive(Clone, Copy, Debug)]
enum UseStacker {
    Always { red_zone: usize, stack_size: usize },
    Once { red_zone: usize, stack_size: usize },
    No,
}

impl UseStacker {
    fn next(self) -> Self {
        match self {
            UseStacker::Always {
                red_zone,
                stack_size,
            } => UseStacker::Always {
                red_zone,
                stack_size,
            },
            UseStacker::Once { .. } | UseStacker::No => UseStacker::No,
        }
    }
}
