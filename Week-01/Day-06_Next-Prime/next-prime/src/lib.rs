use num_bigint::RandBigInt;
use num_bigint::{BigInt, BigUint};
use num_integer::Integer;
use num_prime::PrimalityUtils;
use num_traits::{One, ToPrimitive, one, zero};
use std::ops::{Div, Sub};

pub fn next_prime(n: u128) -> u128 {
    for i in n..u128::MAX {
        if is_prime(&i) {
            return i;
        }
    }
    0
}

// Do a Miller-Rabin test or test if the integer is a (Fermat) probable prime
// or Test if the integer is an extra strong Lucas probable prime If p is not specified,
// then first p starting from 3 such that Jacobi symbol is -1 will be chosen,
// which is sometimes refered as “Method C”
pub fn is_prime(p: &u128) -> bool {
    match p {
        0..2 => false,
        _ => _is_prime(p),
    }
}

// Тест Миллера-Рабина на простоту
// O(k·log³(n))
pub fn is_prime_miller_rabin_probable(p: &u128) -> bool {
    let num = &BigUint::from(*p);
    if num <= &one() || num == &BigUint::from(4u8) {
        return false;
    }
    if num <= &BigUint::from(3u8) {
        return true;
    }

    let mut d = num - 1u8;
    
    while &d % 2u8 == zero() {
        d /= 2u8
    }
    for _ in 0..10 {
        if miller_probable_test(d.clone(), num) == false {
            return false;
        }
    }
    true
}

// функция вычисляет (a·b) % c, принимая во внимание, что a*b может переполниться
pub fn mul_mod(a: &u128, b: &u128, c: &u128) -> u128 {
    let (mut d, mut x, mut y) = (*b, 0, a % c);

    while d > 0 {
        if d % 2 == 1 {
            x = (x + y) % c;
        }
        y = (y * 2) % c;
        d /= 2;
    }
    x % c
}

fn _is_prime(p: &u128) -> bool {
    is_prime_miller_rabin_probable(p)
        || u128::is_prp(p, 128)
        || u128::is_eslprp(p, Some(2))
        || is_prime_miller_test(p)
}

// Тест Миллера-Рабина на простоту, итерация показывает точность теста
fn miller_probable_test(mut d: BigUint, n: &BigUint) -> bool {
    let mut rng = rand::thread_rng();
    let mut random_num = BigUint::from(1u8);
    
    if n != &BigUint::from(5u8) {
        random_num = rng.gen_biguint_range(&one(), &(n - 4u8));
    }
    
    let a = BigUint::from(2u8) + random_num;
    let mut x = BigUint::modpow(&a, &d, &n);

    if x == one() || x == n - 1u8 {
        return true;
    }
    while d != n - 1u8 {
        x = (&x * &x) % n;
        d *= 2u8;

        if x == one() {
            return false;
        }
        if x == n - 1u8 {
            return true;
        }
    }
    false
}

// Тест Миллера
/*
Ввод: n > 2, нечётное натуральное число, которое необходимо проверить на простоту;
Вывод: составное, означает, что n является составным числом;
       простое, означает, что n является простым числом.
(1) Проверить, является ли n степенью какого-либо числа.
    Если является, то вернуть составное
(2) Найти первые m простых чисел p₁, ..., pₘ, где m такое, что pₘ ≤ ƒ(n) ≤ pₘ₊₁
    Вычислить s и q такие, что n-1 = q·2ˢ и q - нечётное
    Положить i = 1 перейти на шаг (4)
(3) если i ≤ m, то i = i + 1
    если i > m, то вернуть простое
(4) если pᵢ|n, то вернуть составное
    Вычислить pᵢ^q mod n, pᵢ^(q·2) mod n, ..., pᵢ^(q·2ˢ)
(5) если pᵢ^(q·2ˢ) ≠ 1, то вернуть составное
(6) если pᵢ^q = 1, то перейти на шаг (3)
    Положить j = max(j : pᵢ^(q·2ʲ) mod n ≠ 1)
(7) если pᵢ^(q·2ʲ) mod n = n - 1, то перейти на шаг (3)
(8) вернуть составное
 */
pub fn is_prime_miller_test(n: &u128) -> bool {
    let checking_num = &BigInt::from(*n);
    // (если является степенью другого числа)
    if is_power_of_number(checking_num) {
        return false;
    }
    let log_n = f64::log(*n as f64, std::f64::consts::E);
    let log_log_n = f64::log(log_n, std::f64::consts::E);
    let max_checked = BigInt::from(log_log_n.ceil() as u128);
    let mut base_current = BigInt::from(2);
    let mut is_prime = true;

    while base_current.le(&max_checked) {
        // (если не сильно псевдопростое по этому основанию)
        if is_strong_pseudo_prime(checking_num, &base_current) {
            // (тогда число не простое)
            is_prime = false;
            break;
        }
        base_current = BigInt::from(next_prime(base_current.to_u128().unwrap()));
    }
    is_prime
}

fn is_strong_pseudo_prime(checking_num: &BigInt, base_current: &BigInt) -> bool {
    let two = &BigInt::from(2);
    let mut exp = &checking_num.sub(BigInt::one());
    // (exp будет меняться, а проверка остатка -1 эквивалентна проверке остатка (checkingNum - 1))
    let ost = exp.clone();
    let mut res = BigInt::modpow(base_current, &exp, checking_num);

    if res.ne(&BigInt::one()) {
        return false;
    }
    // (тест Ферма пройден)
    loop {
        // (чётное; при первом попадании всегда будет чётным, далее цикл до тех пор пока снова станет нечётным)
        let exp = &exp.clone().div(two);
        // (остаток -1 всегда должны проверить)
        res = BigInt::modpow(base_current, exp, checking_num);

        if res.eq(&ost) {
            return true;
        }
        // (снова стало нечётным — нужно проверить ещё на 1)
        if exp.div(two).eq(&BigInt::one()) {
            let res = BigInt::modpow(base_current, exp, checking_num);
            if res.eq(&BigInt::one()) {
                return true;
            }
            break;
        }
    }
    false
}

// Функция, немного более сложная, которая определяет, является ли передаваемое число,
// степенью другого, простого числа. Нужно найти максимально простую реализацию этой функции.
fn is_power_of_number(n: &BigInt) -> bool {
    let mp = BigInt::modpow(&BigInt::from(2), &n.sub(BigInt::one()), n);
    mp.gcd(n).ge(&BigInt::one())
}

#[cfg(test)]
mod tests {
    use crate::{
        is_power_of_number, is_prime as next_prime_is_prime, is_prime_miller_rabin_probable, mul_mod,
        next_prime,
    };
    use miller_rabin::is_prime;
    use num_bigint::BigInt;

    #[test]
    fn test_empty_case() {}

    #[test]
    fn test_case_extra_primes() {
        assert_eq!(next_prime(0), 2);
        assert_eq!(next_prime(u128::MAX), 0);
        assert_eq!(next_prime(u128::MAX - 1), 0);
        assert_eq!(
            next_prime(u128::MAX - 2),
            340282366920938463463374607431768211453
        );
    }

    #[test]
    fn test_case_primes() {
        let primes: Vec<u128> = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
            181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271,
            263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359,
            367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461,
            463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577,
            587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677,
            683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809,
            811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919,
            929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997, 1009, 1013, 1019, 1021, 1031,
            1033, 1039, 1049, 1051, 1061, 1063, 1069, 1087, 1091, 1093, 1097, 1103, 1109, 1117,
            1123, 1129, 1151, 1153, 1163, 1171, 1181, 1187, 1193, 1201, 1213, 1217, 1223, 1229,
            1231, 1237, 1249, 1259, 1277, 1279, 1283, 1289, 1291, 1297, 1301, 1303, 1307, 1319,
            1321, 1327, 1361, 1367, 1373, 1381, 1399, 1409, 1423, 1427, 1429, 1433, 1439, 1447,
            1451, 1453, 1459, 1471, 1481, 1483, 1487, 1489, 1493, 1499, 1511, 1523, 1531, 1543,
            1549, 1553, 1559, 1567, 1571, 1579, 1583, 1597, 1601, 1607, 1609, 1613, 1619, 1621,
            1627, 1637, 1657, 1663, 1667, 1669, 1693, 1697, 1699, 1709, 1721, 1723, 1733, 1741,
            1747, 1753, 1759, 1777, 1783, 1787, 1789, 1801, 1811, 1823, 1831, 1847, 1861, 1867,
            1871, 1873, 1877, 1879, 1889, 1901, 1907, 1913, 1931, 1933, 1949, 1951, 1973, 1979,
            1987, 1993, 1997, 1999, 2003, 2011, 2017, 2027, 2029, 2039, 2053, 2063, 2069, 2081,
            2083, 2087, 2089, 2099, 2111, 2113, 2129, 2131, 2137, 2141, 2143, 2153, 2161, 2179,
            2203, 2207, 2213, 2221, 2237, 2239, 2243, 2251, 2267, 2269, 2273, 2281, 2287, 2293,
            2297, 2309, 2311, 2333, 2339, 2341, 2347, 2351, 2357, 2371, 2377, 2381, 2383, 2389,
            2393, 2399, 2411, 2417, 2423, 2437, 2441, 2447, 2459, 2467, 2473, 2477, 2503, 2521,
            2531, 2539, 2543, 2549, 2551, 2557, 2579, 2591, 2593, 2609, 2617, 2621, 2633, 2647,
            2657, 2659, 2663, 2671, 2677, 2683, 2687, 2689, 2693, 2699, 2707, 2711, 2713, 2719,
            2729, 2731, 2741, 2749, 2753, 2767, 2777, 2789, 2791, 2797, 2801, 2803, 2819, 2833,
            2837, 2843, 2851, 2857, 2861, 2879, 2887, 2897, 2903, 2909, 2917, 2927, 2939, 2953,
            2957, 2963, 2969, 2971, 2999, 3001, 3011, 3019, 3023, 3037, 3041, 3049, 3061, 3067,
            3079, 3083, 3089, 3109, 3119, 3121, 3137, 3163, 3167, 3169, 3181, 3187, 3191, 3203,
            3209, 3217, 3221, 3229, 3251, 3253, 3257, 3259, 3271, 3299, 3301, 3307, 3313, 3319,
            3323, 3329, 3331, 3343, 3347, 3359, 3361, 3371, 3373, 3389, 3391, 3407, 3413, 3433,
            3449, 3457, 3461, 3463, 3467, 3469, 3491, 3499, 3511, 3517, 3527, 3529, 3533, 3539,
            3541, 3547, 3557, 3559, 3571,
        ];
        for prim in primes {
            let cp: &u128 = &prim;
            assert_eq!(is_prime_miller_rabin_probable(cp), is_prime(cp, 96));
            assert!(next_prime_is_prime(cp));
        }
    }

    #[test]
    fn test_case_carol_primes() {
        for n in 2..65 {
            let c = u128::pow(2, n) - 1;
            let cp: &u128 = &(c * c - 2);
            assert_eq!(is_prime_miller_rabin_probable(cp), is_prime(cp, 96));
            assert!(next_prime_is_prime(cp));
        }
    }

    #[test]
    fn test_case_cullen_primes() {
        for n in 2..122 {
            let cp: &u128 = &((n as u128) * u128::pow(2, n) + 1);
            assert_eq!(is_prime_miller_rabin_probable(cp), is_prime(cp, 96));
            assert!(next_prime_is_prime(cp));
        }
    }

    #[test]
    fn test_case_mersenne_primes() {
        for n in 2..128 {
            let cp: &u128 = &(u128::pow(2, n) - 1);
            assert_eq!(is_prime_miller_rabin_probable(cp), is_prime(cp, 96));
            assert!(next_prime_is_prime(cp));
        }
    }

    #[test]
    fn test_case_fermat_primes() {
        for n in 1..7 {
            let cp: &u128 = &(u128::pow(2, u32::pow(2, n)) + 1);
            assert_eq!(is_prime_miller_rabin_probable(cp), is_prime(cp, 96));
            assert!(next_prime_is_prime(cp));
        }
    }

    #[test]
    fn test_mul_mod_case_1() {
        assert_eq!(0, mul_mod(&2, &2, &2));
    }

    #[test]
    fn test_mul_mod_case_2() {
        assert_eq!(
            18446744073709551614u128,
            mul_mod(
                &340282366920938463463374607431768211455,
                &2,
                &18446744073709551616
            )
        );
    }

    #[test]
    fn test_mul_mod_case_3() {
        assert_eq!(
            24u128,
            mul_mod(
                &340282366920938463463374607431768211455,
                &3,
                &18446744073709551613
            )
        );
    }

    #[test]
    fn test_mul_mod_case_max() {
        assert_eq!(
            0u128,
            mul_mod(&(u128::MAX), &(u32::MAX as u128), &(u64::MAX as u128))
        );
    }

    #[test]
    fn test_is_power_of_number() {
        let mut i = 1u128;
        while i < 64 {
            println!("is_power_of_number({}) => {}", i, is_power_of_number(&BigInt::from(i)));
            i = i * 2;
        }
    }
}
