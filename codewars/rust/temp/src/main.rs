fn main() {
    println!("Hello, world!");
}

fn is_valid_ip(ip: &str) -> bool {
    let nums = ip.split('.').collect::<Vec<&str>>();
    if nums.len() != 4 {
        return false;
    }
    for num in nums {
        if num.len() > 1 && num.chars().nth(0).unwrap() == '0' {
            return false;
        }
        if let Ok(n) = num.parse::<u32>() {
            if n > 255 {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

