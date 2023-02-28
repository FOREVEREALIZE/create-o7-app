mod create;
mod input;
mod utils;

use anyhow::Result;

use crate::create::create;

fn main() -> Result<()> {
	let input = input::prompt()?;

	create(input)?;

	// Scaffold (copy files)
	// git?
	// install deps?

	Ok(())
}
