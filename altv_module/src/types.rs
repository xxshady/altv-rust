use std::io::BufReader;

use wasmer_wasix::Pipe;

pub(crate) type StdoutReader = BufReader<Pipe>;
