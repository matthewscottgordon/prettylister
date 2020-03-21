use rustop::opts;

use std::error::Error;
use std::fs;

fn autodetect_column_widths(input: &str, num_header_lines: usize) -> Vec<usize> {
    let mut column_is_all_whitespace = vec![
        true;
        input
            .lines()
            .skip(num_header_lines)
            .map(|l| l.len())
            .max()
            .unwrap_or(0)
    ];
    for line in input.lines().skip(num_header_lines) {
        for (i, c) in line.char_indices() {
            column_is_all_whitespace[i] &= c.is_whitespace();
        }
    }
    /*izip!(
            1..,
            column_is_all_whitespace.iter(),
            column_is_all_whitespace.iter().skip(1)
    )*/
    let column_starts: Vec<usize> = column_is_all_whitespace
        .windows(2)
        .zip(1..)
        .flat_map(|(cs, i)| if !cs[0] && cs[1] { Some(i) } else { None })
        .collect();
    column_starts
        .iter()
        .cloned()
        .take(1)
        .chain(column_starts.windows(2).map(|w| w[1] - w[0]))
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let (args, _) = opts! {
        opt input:String, desc:"Input filename";
    }
    .parse_or_exit();

    let num_header_lines = 7;

    let input_contents = fs::read_to_string(args.input)?;

    let column_widths = autodetect_column_widths(&input_contents, num_header_lines);

    println!("<table>");
    for line in input_contents.lines().skip(num_header_lines) {
        let (percent_str, rest) = line.split_at(column_widths[0].min(line.len()));
        let (address_str, code_str) = rest.split_at(column_widths[1].min(rest.len()));

        println!(
            "<tr><td>{}</td><td>{}</td><td>{}</td></tr>",
            percent_str, address_str, code_str
        );
    }
    println!("</table>");

    Ok(())
}
