use rhai::{Engine, EvalAltResult};

fn main() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    engine.run_file("scripts/fib.qb".into())?;

    Ok(())
}
