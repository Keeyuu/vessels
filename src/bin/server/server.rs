use vessels::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    start_server("0.0.0.0:8080");
    Ok(())
}