pub fn hex_to_base64(input: &str) -> Option<String> {
    if !input.is_ascii() {
        None
    } else {
        let bits = input.chars()
            .map(|hex| hex_to_number(&hex.to_string()))
            .map(|number| to_binary(number))
            .fold(String::new(), |mut acc, ref x| {
                acc.push_str(x);
                acc
            });
        let bit_groups = to_groups(&bits);
        let base64 = bit_groups.iter()
            .map(|ref bits| binary_to_number(bits))
            .map(|number| to_base64(number as usize))
            .fold(String::new(), |mut acc, x| {
                acc.push(x);
                acc
            });

        Some(base64)
    }
}

#[allow(dead_code)] // The base64-encoded string of the challenge is not padded with '='
fn pad(input: &str) -> String {
    let length = input.len();
    let padding = 4 - length % 4;
    format!("{:=<width$}", input, width = length + padding)
}

fn to_groups(input: &str) -> Vec<String> {
    let mut groups = Vec::new();
    let mut rest = input;
    while !rest.is_empty() {
        if rest.len() <= 6 {
            groups.push(format!("{:0<width$}", rest, width = 6));
            rest = "";
        } else {
            let (chunk, tail) = rest.split_at(6);
            groups.push(chunk.to_string());
            rest = tail;
        }
    }
    groups
}

fn hex_to_number(input: &str) -> u8 {
    u8::from_str_radix(input, 16).unwrap()
}

fn to_binary(input: u8) -> String {
    format!("{:0width$b}", input, width = 4)
}

fn binary_to_number(input: &str) -> u8 {
    u8::from_str_radix(input, 2).unwrap()
}

fn to_base64(input: usize) -> char {
    let table = vec!['A','B','C','D','E','F','G','H','I','J','K','L','M',
                     'N','O','P','Q','R','S','T','U','V','W','X','Y','Z',
                     'a','b','c','d','e','f','g','h','i','j','k','l','m',
                     'n','o','p','q','r','s','t','u','v','w','x','y','z',
                     '0','1','2','3','4','5','6','7','8','9','+','/'];
    table[input]
}