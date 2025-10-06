pub fn modpow(mut base: u64, mut exp: u64, n: u64) -> u64 {
    if exp == 0 {
        return 1;
    }

    let mut res = 1;
    base %= n;

    loop {
        if exp & 1 == 1 {
            res *= base;
            res %= n;
        }

        if exp == 1 {
            return res;
        }

        exp >>= 1;
        base *= base;
        base %= n;
    }
}

fn main() {
	for i in 1..10000{
		for p in 1..i{
			match modpow(10,p,i){
				0=>break,// no repeating decimal
				1=>{
					println!("1/{i} has {p} repeating digits");
					break;
				},
				_=>{}
			}
		}
	}
}
