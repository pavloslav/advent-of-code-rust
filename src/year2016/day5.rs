use crypto::md5::Md5;
use crypto::digest::Digest;

pub fn parse_input(input: &str) -> &str {
    input
}

pub fn task1(input:&str)->String {

    let mut result = String::with_capacity(8);
    for i in 0.. {
        let mut hasher = Md5::new();
        hasher.input(input.as_bytes());
        hasher.input(i.to_string().as_bytes());
        let mut md5 = [0; 16];
        hasher.result(&mut md5);
        if (md5[0]|md5[1]|(md5[2]>>4))==0  {
            result.push( format!("{:x}",(md5[2]& 0b1111)).chars().next().unwrap() );
        }
        if result.len()==8 {
            break;
        }
        if i%1000000==0 {
            println!("i={}, {} found",i,result.len());
        }
    }
    result
}

pub fn task2(input:&str)->String {
    let mut result = ['X';8];
    for i in 0.. {
        let mut hasher = Md5::new();
        hasher.input(input.as_bytes());
        hasher.input(i.to_string().as_bytes());
        let mut md5 = [0u8; 16];
        hasher.result(&mut md5);
        if (md5[0]|md5[1]|(md5[2]>>4))==0  {
            let pos = (md5[2]& 0b1111) as usize;
            let symb = format!("{:x}",(md5[3] >> 4)).chars().next().unwrap();
            if pos<result.len() && result[pos]=='X' {
                result[pos] = symb;
            }            
        }
        if result.iter().all(|c|*c!='X') {
            break;
        }
        if i%1000000==0 {
            println!("i={}, now have {}",i, md5.iter().map(|x|format!("{:2X} ",x)).collect::<String>() );
        }
        
    }
    result.iter().cloned().collect::<String>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn test_password() {
        assert_eq!(task1("abc"), "18f47a30");
        assert_eq!(task2("abc"), "05ace8e3");
    }
}