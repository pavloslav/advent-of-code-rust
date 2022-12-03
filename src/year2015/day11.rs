type Password = Vec<u8>;

pub fn parse_input(input: &str) -> Password {
    input.trim().chars().map(|c| c as u8).rev().collect()
}

fn is_invalid_symbol(c: u8) -> bool {
    "iol".chars().any(|wrong| wrong as u8 == c)
}

fn next_string(password: &Password) -> Password {
    let mut arr = password.clone();
    arr[0] += 1;
    for idx in 0..arr.len() {
        if is_invalid_symbol(arr[idx]) {
            arr[idx] += 1;
        }
        if arr[idx] > b'z' {
            arr[idx] = b'a';
            if idx + 1 < arr.len() {
                arr[idx + 1] += 1;
            } else {
                arr.push(b'a');
            }
        } else {
            break;
        }
    }
    arr
}

fn is_valid_password(password: &Password) -> bool {
    if password.windows(3).any(|part| {
        part[2] as u8 + 1 == part[1] as u8 && part[1] as u8 + 1 == part[0] as u8
    }) {
        let mut found: i32 = -1;
        for i in 0..password.len() - 1 {
            if password[i] == password[i + 1] && found != i as i32 {
                if found == -1 {
                    found = i as i32 + 1;
                } else {
                    return true;
                }
            }
        }
    }
    false
}

fn next_password(password: &Password) -> Password {
    let mut password = password.clone();
    loop {
        password = next_string(&password);
        if is_valid_password(&password) {
            return password;
        }
    }
}

pub fn task1(password: &Password) -> String {
    next_password(password)
        .into_iter()
        .rev()
        .map(|c| c as char)
        .collect()
}

pub fn task2(password: &Password) -> String {
    next_password(&next_password(password))
        .into_iter()
        .rev()
        .map(|c| c as char)
        .collect()
}
