use std::fs;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let input = read_input_from_file("day01/input.txt")?;
    println!("Input: {:?}", input);

    Ok(())
}

fn read_input_from_file(path: impl AsRef<Path>) -> anyhow::Result<String> {
    let input = fs::read_to_string(path)?;

    Ok(input)
}

#[cfg(test)]
mod test {

}
