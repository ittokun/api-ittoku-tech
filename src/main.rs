fn main() {
    let result = api_ittoku_tech::main();

    if let Some(err) = result.err() {
        eprint!("error: {}", err);
    }
}
