pub mod solution {
    pub fn alphabet_position(text: &str) -> String {
        let mut res = String::new();
        for c in text.chars() {
            let pos = match c {
                x if x.is_ascii() && x.is_alphabetic() => ((x.to_ascii_lowercase() as u8) - 96).to_string() ,
                _ => "".to_string()
            };
            if pos.len() != 0 && res.len() != 0{
                res.push(' ');
            }
            res.push_str(&pos)
        }
        res
    }
}

fn main() -> ()
{
   let res = solution::alphabet_position("The narwhal bacons at midnight.");
   println!("{res}")
}