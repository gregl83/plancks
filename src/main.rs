use std::fmt;

const DEFAULT_BUFFER_SIZE: usize = 33;

// todo - control-flow operations applied (series/parallel); state MUST reflect status there
// requirements:
//  - when to consume
//  - when consumed
//  - when to overwrite
#[derive(Copy, Clone)]
enum PlanckState {
    Locked(bool),
}

impl fmt::Display for PlanckState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", "Locked") // fixme - display enum state
    }
}

#[derive(Copy, Clone)]
struct Planck {
    state: PlanckState
}

impl fmt::Display for Planck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Plank State: {}", self.state)
    }
}

#[derive(Copy, Clone)]
struct Plancks {
    values: [Planck; DEFAULT_BUFFER_SIZE],
}

impl Plancks {
    pub fn new() -> Plancks {
        Plancks {
            values: [
                Planck {
                    state: PlanckState::Locked(false)
                }; DEFAULT_BUFFER_SIZE
            ],
        }
    }
}

impl fmt::Display for Plancks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        for plank in self.values.iter() {
            out += &format!("\n{}", plank);
        }
        write!(f, "{}", out)
    }
}

fn main() {
    // todo - need workers for parallelism
    // todo - benchmarks (parallelism, read, writes, etc)
    // todo - memory allocation details/tests
    // todo - overflow panic (cleanup/dump state?)

    let planks = Plancks::new();

    println!("{}", planks);
}
