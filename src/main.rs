use std::error::Error;

use isahc::ResponseExt;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://crates.io/api/v1/crates/url";
    let mut res = isahc::get(url)?;

    dbg!(&res);

    dbg!(res.text());

    Ok(())
}
