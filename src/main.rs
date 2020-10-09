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

impl fmt::Debug for PlanckState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", "Locked") // fixme - display enum state
    }
}

#[derive(Copy, Clone)]
struct Planck {
    state: PlanckState
}

impl fmt::Debug for Planck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Plank {{\n\t\tstate: {:?}\n\t}}", self.state)
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

impl fmt::Debug for Plancks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        for (i, plank) in self.values.iter().enumerate() {
            out += &format!("\n\t{}: {:?}", i, plank);
        }
        write!(f, "Plancks [{}]", out)
    }
}

fn main() {
    // todo - need workers for parallelism
    // todo - benchmarks (parallelism, read, writes, etc)
    // todo - memory allocation details/tests
    // todo - overflow panic (cleanup/dump state?)

    let planks = Plancks::new();

    println!("{:?}", planks);
}
