pub mod solution {
    pub fn to_camel_case(text: &str) -> String {
        let mut res = String::new();
        let mut preceded : bool = false;
        for l in text.chars(){
            let new_l =  match l {
                modifier if "_-".contains(modifier) => {
                    preceded = true;
                    '\0'
                },
                to_uppercase if to_uppercase.is_ascii_lowercase() && preceded => to_uppercase.to_ascii_uppercase(),
                _ => l
            };
            res.push(new_l)
        }
        return  res;
    }
}

fn main() -> ()
{

}