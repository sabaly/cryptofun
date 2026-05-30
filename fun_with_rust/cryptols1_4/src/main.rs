// challenge 1
mod challenge_1 {


    fn fromhex(ch: char) -> u32{
        if ch=='a' { 
            return 10 as u32;
        } else if ch=='b' {
            return 11 as u32;
        } else if ch=='c' {
            return 12 as u32;
        }   else if ch=='d' {
            return 13 as u32;
        }   else if ch=='e' {
            return 14 as u32;
        }   else if ch=='f' {
            return 15 as u32;
        } else {
            let a = (ch as u32) - ('0' as u32);
            return a;
        }
    }
    pub fn hex_to_b64() {
        let input = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
        //let mut b64 = String::new();
        let mut b64: [char; 65] = ['0'; 65];
        let _i: i8 = 0;
        for i in 0..64 {
            if i < 26 {
                b64[i] =  ((i + 65) as u8)  as char;
            } else if i < 52 {
                b64[i] = char::from_u32((i+71) as u32).expect("REASON");
            }else if i < 62{
                b64[i] = char::from_u32((i-4) as u32).expect("REASON");
            } else if i==62 {
                b64[i] = '+';
            } else 
            {
                b64[i] = '/';
            }
        }
        let mut tmp = String::with_capacity(150);
        let mut i = 0; //input.len()-1;
        loop {
            if i>input.len()-3 {
                break;
            }
            let m2 = fromhex(input.as_bytes()[i] as char);
            let m1 = fromhex(input.as_bytes()[i+1] as char);
            let m0 = fromhex(input.as_bytes()[i+2] as char);
            let m: u32 = ((((m2 as u32)<<4)|(m1 as u32))<<4)|(m0 as u32) & 0xfff;
            //println!(">>> {}->{}-{}--{}", i, m2, m, (m>>5)&0x3f);
            let c1 = ((m>>6)) as usize;
            let c2 = ((m&0x3f)) as usize;
            tmp.push(b64[c1]);
            tmp.push(b64[c2]);
            //println!("{}-[{}] / {}-[{}]", m>>6, c1, m&0x3f, c2);
            i += 3;
        }
        println!("b64:  \t{}", tmp);
        let res = String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
        println!("Exp.: \t{}", res);
    }
}


fn main() {
    challenge_1::hex_to_b64();
}
