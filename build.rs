use vergen_git2::{BuildBuilder, Emitter, Git2Builder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Emitter::default()
        .add_instructions(&BuildBuilder::all_build()?)?
        .add_instructions(&Git2Builder::all_git()?)?
        .emit()?;
    Ok(())
}
