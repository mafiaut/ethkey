extern crate docopt;
extern crate rustc_serialize;
extern crate ethkey;

use std::{env, fmt};
use std::num::ParseIntError;
use docopt::Docopt;
use rustc_serialize::hex::{ToHex, FromHex, FromHexError};
use ethkey::{KeyPair, Random, Brain, Prefix, Error as EthkeyError, Generator};

pub const USAGE: &'static str = r#"
Ethereum ABI coder.
  Copyright 2016 Ethcore (UK) Limited

Usage:
    ethkey generate random [options]
    ethkey generate prefix <prefix> <iterations> [options]
    ethkey generate brain <seed> [options]
    ethkey [-h | --help]

Options:
    -h, --help         Display this message and exit.
    -s, --secret       Display only the secret.
    -p, --public       Display only the public.
    -a, --address      Display only the address.

Commands:
    generate           Generates new ethereum key.
    random             Random generation.
    prefix             Random generation, but address must start with a prefix
    brain              Generate new key from string seed.
"#;

#[derive(Debug, RustcDecodable)]
struct Args {
	cmd_generate: bool,
	cmd_random: bool,
	cmd_prefix: bool,
	cmd_brain: bool,
	arg_prefix: String,
	arg_iterations: String,
	arg_seed: String,
	flag_secret: bool,
	flag_public: bool,
	flag_address: bool,
}

#[derive(Debug)]
enum Error {
	Ethkey(EthkeyError),
	FromHex(FromHexError),
	ParseInt(ParseIntError),
}

impl From<EthkeyError> for Error {
	fn from(err: EthkeyError) -> Self {
		Error::Ethkey(err)
	}
}

impl From<FromHexError> for Error {
	fn from(err: FromHexError) -> Self {
		Error::FromHex(err)
	}
}

impl From<ParseIntError> for Error {
	fn from(err: ParseIntError) -> Self {
		Error::ParseInt(err)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match *self {
			Error::Ethkey(ref e) => write!(f, "{}", e),
			Error::FromHex(ref e) => write!(f, "{}", e),
			Error::ParseInt(ref e) => write!(f, "{}", e),
		}
	}
}

enum DisplayMode {
	KeyPair,
	Secret,
	Public,
	Address,
}

impl DisplayMode {
	fn new(args: &Args) -> Self {
		if args.flag_secret {
			DisplayMode::Secret
		} else if args.flag_public {
			DisplayMode::Public
		} else if args.flag_address {
			DisplayMode::Address
		} else {
			DisplayMode::KeyPair
		}
	}
}

fn main() {
	match execute(env::args()) {
		Ok(ok) => println!("{}", ok),
		Err(err) => println!("{}", err),
	}
}

fn display(keypair: KeyPair, mode: DisplayMode) -> String {
	match mode {
		DisplayMode::KeyPair => format!("{}", keypair),
		DisplayMode::Secret => format!("{}", keypair.secret().to_hex()),
		DisplayMode::Public => format!("{}", keypair.public().to_hex()),
		DisplayMode::Address => format!("{}", keypair.address().to_hex()),
	}
}

fn execute<S, I>(command: I) -> Result<String, Error> where I: IntoIterator<Item=S>, S: AsRef<str> {
	let args: Args = Docopt::new(USAGE)
		.and_then(|d| d.argv(command).decode())
		.unwrap_or_else(|e| e.exit());

	return if args.cmd_generate {
		let display_mode = DisplayMode::new(&args);
		let keypair = if args.cmd_random {
			Random.generate()
		} else if args.cmd_prefix {
			let prefix = try!(args.arg_prefix.from_hex());
			let iterations = try!(usize::from_str_radix(&args.arg_iterations, 10));
			Prefix::new(prefix, iterations).generate()
		} else if args.cmd_brain {
			Brain::new(args.arg_seed).generate()
		} else {
			unreachable!();
		};
		Ok(display(try!(keypair), display_mode))
	} else {
		unreachable!();
	}
}


