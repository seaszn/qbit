use qbit_lang::parser::Parser;

fn main() -> Result<(), String> {
    let result = Parser::parse_src(
        r#"let S  = "";

fn test() {
   let tt = "";
}"#,
    );

    match result {
        Ok(res) => {
            println!("{:#?}", res.statements());
            println!("{:#?}", res.diagnositcs());
        }
        Err(err) => println!("{err:?}"),
    }

    // let engine = qbit_lang::parser::Parser::builder::::new()?;

    // // engine.run_file("scripts/core/fib.qb".into())?;
    // engine.run("print(40 + 2);")?;
    // engine.run("series ffff; print(ffff);")?;

    Ok(())
}
