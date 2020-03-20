use rustop::opts;

use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let (args, _) = opts! {
        opt input:String, desc:"Input filename";
    }
    .parse_or_exit();

    let input_contents = fs::read_to_string(args.input)?;

    let num_header_lines = 8;
    let percent_column_width = 8;
    let address_column_width = 6;

    println!("<table>");
    for line in input_contents.lines().skip(num_header_lines) {
        let (percent_str, rest) = line.split_at(percent_column_width.min(line.len()));
        let (address_str, code_str) = rest.split_at(address_column_width.min(rest.len()));

        println!(
            "<tr><td>{}</td><td>{}</td><td>{}</td></tr>",
            percent_str, address_str, code_str
        );
    }
    println!("</table>");

    Ok(())
}
