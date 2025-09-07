type Password = Vec<u8>;

pub fn parse_input(input: &str) -> anyhow::Result<Password> {
    Ok(input.trim().chars().map(|c| c as u8).rev().collect())
}

fn is_invalid_symbol(c: u8) -> bool {
    b"iol".contains(&c)
}

fn next_string(password: &Password) -> Password {
    let mut arr = password.clone();
    for idx in 0..arr.len() {
        if is_invalid_symbol(arr[idx]) {
            arr[idx] += 1;
            for letter in &mut arr[0..idx] {
                *letter = b'a';
            }
            return arr;
        }
    }
    arr[0] += 1;
    for idx in 0..arr.len() {
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

fn is_valid_password(password: &[u8]) -> bool {
    if password.iter().all(|&c| !is_invalid_symbol(c))
        && password
            .windows(3)
            .any(|part| part[2] + 1 == part[1] && part[1] + 1 == part[0])
    {
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

fn next_password(password: &[u8]) -> Password {
    let mut password = password.to_vec();
    loop {
        password = next_string(&password);
        if is_valid_password(&password) {
            return password;
        }
    }
}

pub fn task1(password: &Password) -> anyhow::Result<String> {
    Ok(next_password(password)
        .into_iter()
        .rev()
        .map(|c| c as char)
        .collect())
}

pub fn task2(password: &Password) -> anyhow::Result<String> {
    Ok(next_password(&next_password(password))
        .into_iter()
        .rev()
        .map(|c| c as char)
        .collect())
}

#[cfg(test)]
mod test {
    use super::{parse_input, task1};

    #[test]
    fn test_next_password() {
        assert_eq!(
            task1(&parse_input("abcdefgh").unwrap()).unwrap(),
            "abcdffaa"
        );
        assert_eq!(
            task1(&parse_input("ghijklmn").unwrap()).unwrap(),
            "ghjaabcc"
        );
    }
}
