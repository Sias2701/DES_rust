use std::fmt;

pub struct DES{
    key : u64,
    sub_keys : [u64;16]
}

const IP : [u8; 64] = [
    58 ,50 ,42 ,34 ,26 ,18 ,10,	2,
    60 ,52 ,44 ,36 ,28 ,20 ,12,	4,
    62 ,54 ,46 ,38 ,30 ,22 ,14,	6,
    64 ,56 ,48 ,40 ,32 ,24 ,16,	8,
    57 ,49 ,41 ,33 ,25 ,17 ,9 , 1,
    59 ,51 ,43 ,35 ,27 ,19 ,11,	3,
    61 ,53 ,45 ,37 ,29 ,21 ,13,	5,
    63 ,55 ,47 ,39 ,31 ,23 ,15,	7
];

const IP_INV : [u8; 64] = [
    40, 8 ,48 ,16, 56, 24, 64, 32,
    39, 7 ,47 ,15, 55, 23, 63, 31,
    38, 6 ,46 ,14, 54, 22, 62, 30,
    37, 5 ,45 ,13, 53, 21, 61, 29,
    36, 4 ,44 ,12, 52, 20, 60, 28,
    35, 3 ,43 ,11, 51, 19, 59, 27,
    34, 2 ,42 ,10, 50, 18, 58, 26,
    33, 1 ,41 ,9 , 49, 17, 57, 25,
];

const E : [u8; 48] = [
    32,  1,  2,  3,  4 ,  5,
    4 ,  5,  6,  7,  8 ,  9,
    8 ,  9, 10,	11, 12 , 13,
    12, 13, 14,	15, 16 , 17,
    16, 17, 18,	19, 20 , 21,
    20, 21, 22,	23, 24 , 25,
    24, 25, 26,	27, 28 , 29,
    28, 29, 30,	31, 32 ,  1,
];

const PC1 : [u8; 56] = [
    57, 49, 41, 33, 25, 17,  9,
     1, 58, 50, 42, 34, 26, 18,
    10,  2, 59, 51, 43, 35, 27,
    19, 11,  3, 60, 52, 44, 36,
    63, 55, 47, 39, 31, 23, 15,
     7, 62, 54, 46, 38, 30, 22,
    14,  6, 61, 53, 45, 37, 29,
    21, 13,  5, 28, 20, 12,  4,
];

const PC2 : [u8; 48] =[
    14, 17, 11, 24,  1,  5,
     3, 28, 15,  6, 21, 10,
    23, 19, 12,  4, 26,  8,
    16,  7, 27, 20, 13,  2,
    41, 52, 31, 37, 47, 55,
    30, 40, 51, 45, 33, 48,
    44, 49, 39, 56, 34, 53,
    46, 42, 50, 36, 29, 32
];

const S1 : [[u8; 16]; 4]= [
[14, 4, 13, 1, 2, 15, 11, 8, 3, 10, 6, 12, 5, 9, 0, 7 ],
[0, 15, 7, 4, 14, 2, 13, 1, 10, 6, 12, 11, 9, 5, 3, 8],
[4, 1, 14, 8, 13, 6, 2, 11, 15, 12, 9, 7, 3, 10, 5, 0],
[15, 12, 8, 2, 4, 9, 1, 7, 5, 11, 3, 14, 10, 0, 6, 13 ]
];

const S2 : [[u8; 16]; 4]= [
[15, 1, 8, 14, 6, 11, 3, 4, 9, 7, 2, 13, 12, 0, 5, 10],
[3, 13, 4, 7, 15, 2, 8, 14, 12, 0, 1, 10, 6, 9, 11, 5],
[0, 14, 7, 11, 10, 4, 13, 1, 5, 8, 12, 6, 9, 3, 2, 15],
[13, 8, 10, 1, 3, 15, 4, 2, 11, 6, 7, 12, 0, 5, 14, 9]
];

const S3 : [[u8; 16]; 4]= [
[10, 0, 9, 14, 6, 3, 15, 5, 1, 13, 12, 7, 11, 4, 2, 8],
[13, 7, 0, 9, 3, 4, 6, 10, 2, 8, 5, 14, 12, 11, 15, 1],
[13, 6, 4, 9, 8, 15, 3, 0, 11, 1, 2, 12, 5, 10, 14, 7],
[1, 10, 13, 0, 6, 9, 8, 7, 4, 15, 14, 3, 11, 5, 2, 12]
];

const S4 : [[u8; 16]; 4]= [
[7, 13, 14, 3, 0, 6, 9, 10, 1, 2, 8, 5, 11, 12, 4, 15],
[13, 8, 11, 5, 6, 15, 0, 3, 4, 7, 2, 12, 1, 10, 14, 9],
[10, 6, 9, 0, 12, 11, 7, 13, 15, 1, 3, 14, 5, 2, 8, 4],
[3, 15, 0, 6, 10, 1, 13, 8, 9, 4, 5, 11, 12, 7, 2, 14]
];

const S5 : [[u8; 16]; 4]= [
[2, 12, 4, 1, 7, 10, 11, 6, 8, 5, 3, 15, 13, 0, 14, 9 ],
[14, 11, 2, 12, 4, 7, 13, 1, 5, 0, 15, 10, 3, 9, 8, 6],
[4, 2, 1, 11, 10, 13, 7, 8, 15, 9, 12, 5, 6, 3, 0, 14],
[11, 8, 12, 7, 1, 14, 2, 13, 6, 15, 0, 9, 10, 4, 5, 3]
];

const S6 : [[u8; 16]; 4]= [
[12, 1, 10, 15, 9, 2, 6, 8, 0, 13, 3, 4, 14, 7, 5, 11],
[10, 15, 4, 2, 7, 12, 9, 5, 6, 1, 13, 14, 0, 11, 3, 8],
[9, 14, 15, 5, 2, 8, 12, 3, 7, 0, 4, 10, 1, 13, 11, 6],
[4, 3, 2, 12, 9, 5, 15, 10, 11, 14, 1, 7, 6, 0, 8, 13]
];

const S7 : [[u8; 16]; 4]= [
[4, 11, 2, 14, 15, 0, 8, 13, 3, 12, 9, 7, 5, 10, 6, 1],
[13, 0, 11, 7, 4, 9, 1, 10, 14, 3, 5, 12, 2, 15, 8, 6],
[1, 4, 11, 13, 12, 3, 7, 14, 10, 15, 6, 8, 0, 5, 9, 2],
[6, 11, 13, 8, 1, 4, 10, 7, 9, 5, 0, 15, 14, 2, 3, 12]
];

const S8 : [[u8; 16]; 4]= [
[13, 2, 8, 4, 6, 15, 11, 1, 10, 9, 3, 14, 5, 0, 12, 7],
[1, 15, 13, 8, 10, 3, 7, 4, 12, 5, 6, 11, 0, 14, 9, 2],
[7, 11, 4, 1, 9, 12, 14, 2, 0, 6, 10, 13, 15, 3, 5, 8],
[2, 1, 14, 7, 4, 10, 8, 13, 15, 12, 9, 0, 3, 5, 6, 11]
];

const P : [u8; 32] = [
    16,  7, 20, 21,
    29, 12, 28, 17,
     1, 15, 23, 26,
     5, 18, 31, 10,
     2,  8, 24, 14,
    32, 27,  3,  9,
    19, 13, 30,  6,
    22, 11,  4, 25
];

const SBOX : [[[u8; 16]; 4]; 8] = [S1, S2, S3, S4, S5, S6, S7, S8];

const ROTATE_TABLE : [u8; 16] = [1, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1];

#[allow(dead_code)]
impl DES {
    pub fn new(key : &u64) -> Self
    {
        DES { key: *key, sub_keys : Self::key_schedule(key)}
    }

    pub fn set_key(&mut self, key : &u64)
    {
        self.key = *key;
        self.sub_keys = Self::key_schedule(key);
    }

    pub fn get_key(&mut self) -> u64
    {
        self.key
    }

    pub fn encrypt_block(&mut self, block: &u64) -> u64 {
        println!("Plaintext : {:064b}", block);

        let intermediate_block : u64 = Self::apply_permutation(block, 64, &IP);
        println!("IP : {:064b}", intermediate_block);

        let mut l: u64 = intermediate_block >> 32;
        let mut r: u64 = intermediate_block & 0xffffffff;
        println!("L0 = {:032b}", l);
        println!("R0 = {:032b}", r);
        for i in 0..15 {
            (l, r) = (r, l ^ Self::f(&r, &self.sub_keys[i]));
            println!("L{} : {:032b}", i + 1, l);
            println!("R{} : {:032b}", i + 1, r);
        }

        // final round 
        let tmp = r;
        l = l ^ Self::f(&r, &self.sub_keys[15]);
        r = tmp;
        println!("L{} : {:032b}", 16, r);
        println!("R{} : {:032b}", 16, l);
        let inv_ip = Self::apply_permutation(&((l << 32) | r), 64, &IP_INV);
        println!("INV_IP : {:064b}", inv_ip);
        inv_ip
    }

    pub fn decrypt_block(&mut self, block: &u64) -> u64{
        println!("Plaintext : {:064b}", block);

        let intermediate_block : u64 = Self::apply_permutation(block, 64, &IP);
        println!("IP : {:064b}", intermediate_block);

        let mut l: u64 = intermediate_block >> 32;
        let mut r: u64 = intermediate_block & 0xffffffff;
        println!("L0 : {:032b}", l);
        println!("R0 : {:032b}", r);
        for i in (1..16).rev() {
            (l, r) = (r, l ^ Self::f(&r, &self.sub_keys[i]));
            println!("L{} : {:032b}", i + 1, l);
            println!("R{} : {:032b}", i + 1, r);
        }

        // final round 
        let tmp = r;
        l = l ^ Self::f(&r, &self.sub_keys[0]);
        r = tmp;
        println!("L{} : {:032b}", 16, r);
        println!("R{} : {:032b}", 16, l);
        let inv_ip = Self::apply_permutation(&((l << 32) | r), 64, &IP_INV);
        println!("INV_IP : {:064b}", inv_ip);
        inv_ip
    }

    fn apply_permutation(target : &u64, input_length: usize, perm_box: &[u8]) -> u64 {
        let mut ret : u64 = 0;
        let output_length: usize = perm_box.len();
        for (p1, &p2) in perm_box.iter().enumerate() {
            ret |= ((target >> (input_length - p2 as usize)) & 1) << (output_length - p1 - 1);
        }
        ret
    }
    
    fn left_rotate_28(op : u64, step: u8) -> u64{
        ((op << step) | (op >> (28-step))) & 0xfffffff
    }

    fn key_schedule(key : &u64) -> [u64; 16]
    {
        println!("DES key schedule procedure");
        let mut ret : [u64 ; 16] = [0; 16];
        println!("Key : {:064b}", key);
        let cd: u64 = Self::apply_permutation(key, 64, &PC1);
        println!("PC1 : {:064b}", cd);
        let mut c: u64 = (cd >> 28) & 0xfffffff;
        let mut d: u64 = cd & 0xfffffff;
        println!("C0 : {:028b}\nD0 : {:028b}", c, d);
        for i in 0..16 {

            c = Self::left_rotate_28(c, ROTATE_TABLE[i]);
            d = Self::left_rotate_28(d, ROTATE_TABLE[i]);
            println!("C{} : {:028b}\nD{} : {:028b}", i+1, c, i+1, d);
            ret[i] = Self::apply_permutation(&((c << 28) | d),56, &PC2);
            println!("Subkey K{} : {:048b}", i+1, ret[i]);
        }
        ret
    }

    fn substitution(block: &u64, sbox: &[[[u8; 16]; 4]; 8]) -> u64 {
        let mut ret : u64 = 0;

        for i in 0..8{
            let part = (block >> (6*i)) & 0b111111;
            let index1 = ((((part) >> 4) & 2) | (part & 1)) as usize;
            let index2 = ((part & 0b011110) >> 1) as usize;
            ret |= (sbox[7-i][index1][index2] as u64) << (4*i);
        }
        ret
    }

    fn f(block: &u64, key : &u64) -> u64{
        println!("Function F procedure");
        
        println!("Input : {:032b}", block);
        let e: u64 = Self::apply_permutation(&block, 32, &E);
        println!("Expansion : {:048b}", e);
        println!("Subkey : {:048b}", key);
        let ct: u64 = e ^ key;
        println!("Subkey addition : {:048b}", ct);
        let s : u64 = Self::substitution(&ct,&SBOX);
        println!("Substitution : {:032b}", s);
        let p = Self::apply_permutation(&s, 32, &P);
        println!("Permutation : {:032b}", p);
        p
    }

}

impl fmt::Display for DES {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Current DES key: {:064b}\n", self.key).unwrap();
        for (pos, sk) in self.sub_keys.iter().enumerate(){
            write!(f, "Current Subkey {} : {:048b}\n", pos, sk).unwrap();
        }
        fmt::Result::Ok(())
    }
}