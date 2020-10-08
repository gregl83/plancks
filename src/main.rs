const DEFAULT_BUFFER_SIZE: usize = 10_000_000;

// todo - control-flow operations applied (series/parallel); state MUST reflect status there
// requirements:
//  - when to consume
//  - when consumed
//  - when to overwrite
#[derive(Debug, Clone)]
enum PlankState {
    Locked(bool),
}

#[derive(Debug, Clone)]
struct Planck {
    state: PlankState
}

#[derive(Debug, Clone)]
struct Plancks {
    values: Vec<Planck>,
}

impl Plancks {
    pub fn new() -> Plancks {
        Plancks {
            // fixme - vectors will store on heap, consider using array to persist on stack
            values: vec![
                Planck {
                    state: PlankState::Locked(false)
                }; 2 // fixme - BUFFER_SIZE
            ]
        }
    }
}

fn main() {
    // todo - need workers for parallelism
    // todo - benchmarks (parallelism, read, writes, etc)
    // todo - memory allocation details/tests
    // todo - overflow panic (cleanup/dump state?)

    let planks = Plancks::new();
    println!("{:?}", planks);
    println!("max size: {}", DEFAULT_BUFFER_SIZE);

    let test = [0; 100];
}
