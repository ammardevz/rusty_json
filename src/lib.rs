use std::error::Error;

mod base;
mod extra;

fn main() -> Result<(), Box<dyn Error>> {



    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        main().unwrap();
    }
}
