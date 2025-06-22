use num_bigint::RandBigInt;
use num_bigint::{BigInt, BigUint};
use num_integer::Integer;
use num_prime::nt_funcs::is_prime as nt_funcs_nt_prime;
use num_prime::{Primality, PrimalityTestConfig, PrimalityUtils};
use num_traits::{One, ToPrimitive, one, zero};

const EPSILON: f64 = 1e-13;

pub fn next_prime(n: &u128) -> u128 {
    match *n {
        0..2 => 2,
        u128::MAX => 0,
        _ => _next_prime(n),
    }
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

fn _is_prime(p: &u128) -> bool {
    is_prime_miller_rabin_probable(p) || _is_prime_probable(p) || is_prime_miller_test(p)
}

fn _is_prime_probable(p: &u128) -> bool {
    match nt_funcs_nt_prime(p, Some(PrimalityTestConfig::strict())) {
        Primality::Yes => true,
        Primality::No => false,
        Primality::Probable(f) => {
            println!("Probable({})", f);
            u128::is_prp(p, 128) || u128::is_eslprp(p, Some(2))
        }
    }
}

// Тест Миллера-Рабина на простоту
// O(k·log³(n))
fn is_prime_miller_rabin_probable(p: &u128) -> bool {
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

fn _next_prime(n: &u128) -> u128 {
    let mut i = if n % 2 == 0 { n + 1 } else { n + 2 };

    while i < u128::MAX {
        if is_prime(&i) {
            return i;
        }
        i += 2;
    }
    0
}

fn __next_prime(n: &u128) -> u128 {
    let mut i = n + 1;

    while i < u128::MAX {
        if is_prime(&i) {
            return i;
        }
        i += 1;
    }
    0
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
    // (если является степенью другого числа)
    if is_power_of_number(n) {
        return false;
    }
    let checking_num = &BigInt::from(*n);
    let log_n = f64::log(n.clone() as f64, std::f64::consts::E);
    let log_log_n = f64::log(log_n, std::f64::consts::E);
    let max_checked =
        BigInt::from((log_n * log_log_n / f64::log(2f64, std::f64::consts::E)).round() as u128);
    let mut base_current = BigInt::from(2);
    let mut is_prime = true;

    while base_current.le(&max_checked) {
        // (если не сильно псевдопростое по этому основанию)
        if !is_strong_pseudo_prime(checking_num, &base_current) {
            // (тогда число не простое)
            is_prime = false;
            break;
        }
        let np = __next_prime(&base_current.to_u128().unwrap());
        base_current = BigInt::from(np);
    }
    is_prime
}

fn is_strong_pseudo_prime(checking_num: &BigInt, base_current: &BigInt) -> bool {
    let mut exp = checking_num - BigInt::one();
    // (exp будет меняться, а проверка остатка -1 эквивалентна проверке остатка (checkingNum - 1))
    let ost = exp.clone();
    let mut res = base_current.modpow(&exp, checking_num);

    if res.ne(&BigInt::one()) {
        return false;
    }
    // (тест Ферма пройден)
    loop {
        // (чётное; при первом попадании всегда будет чётным, далее цикл до тех пор, пока снова станет нечётным)
        exp = exp / 2;
        // (остаток -1 всегда должны проверить)
        res = base_current.modpow(&exp, checking_num);

        if res.eq(&ost) {
            return true;
        }
        // (снова стало нечётным — нужно проверить ещё на 1)
        if exp.is_odd() {
            res = base_current.modpow(&exp, checking_num);

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
/*
  Любое число является степенью любого другого (за исключением особых случаев: 0, 1, отрицательные)
Нужно установить: является ли данное число целой степенью. Для этого есть логарифмы.

x = p^n
n = log(x)/log(p)
Осталось только проверить, целое ли число n.
 */
fn is_power_of_number(n: &u128) -> bool {
    let x = *n;
    for p in 2..(x / 2) + 1 {
        let log_x = f64::log(x as f64, std::f64::consts::E);
        let log_p = f64::log(p as f64, std::f64::consts::E);
        let abs = (log_x / log_p).abs();
        let delta = (abs - abs.round()).abs();
        if delta < f64::EPSILON {
            return true;
        } else if delta < EPSILON {
            if _is_power_of_number(&p, &x) {
                return true;
            }
        }
    }
    false
}

fn _is_power_of_number(a: &u128, b: &u128) -> bool {
    let x = *a;
    let mut y = *b;

    'outer: while x <= y {
        let mut p = x;
        'inner: loop {
            let p2 = p * p;
            if p2 > y {
                break 'inner;
            }
            p = p2;
        }
        if y % p != 0 {
            break 'outer;
        }
        y = y / p;
    }
    y == 1
}

#[cfg(test)]
mod tests {
    use crate::{
        _is_prime as next_prime_is_prime, is_power_of_number, is_prime_miller_rabin_probable,
        mul_mod, next_prime,
    };
    use miller_rabin::is_prime;

    #[test]
    fn test_empty_case() {}

    #[test]
    fn test_case_extra_primes() {
        assert_eq!(next_prime(&0), 2);
        assert_eq!(next_prime(&1), 2);
        assert_eq!(next_prime(&2), 3);
        assert_eq!(next_prime(&4), 5);
        assert_eq!(next_prime(&u128::MAX), 0);
        assert_eq!(next_prime(&(u128::MAX - 1)), 0);
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
        ];
        for prim in &primes {
            let cp: &u128 = &prim;
            assert_eq!(is_prime_miller_rabin_probable(cp), is_prime(cp, 96));
            assert!(next_prime_is_prime(cp));
            assert_eq!(next_prime(&(cp - 1)), *cp);
            assert!(!is_power_of_number(cp));
        }
        for i in 0..primes.len() - 1 {
            let a = &primes[i];
            let b = &primes[i + 1];
            for j in *a..*b {
                let n = next_prime(&j);
                assert_eq!(n, *b);
            }
        }
    }

    // Начиная с 7 каждое третье число Кэрола делится на 7.
    #[test]
    fn test_case_carol_primes() {
        let ns: Vec<u128> = vec![
            7,
            47,
            223,
            3967,
            16127,
            1046527,
            16769023,
            1073676287,
            68718952447,
            274876858367,
            4398042316799,
            1125899839733759,
            18014398241046527,
            1298074214633706835075030044377087,
        ];
        for n in ns {
            assert_eq!(is_prime_miller_rabin_probable(&n), is_prime(&n, 96));
            assert!(next_prime_is_prime(&n));
        }
    }

    #[test]
    fn test_case_cullen_primes() {
        let ns: Vec<u128> = vec![3, 257, 65537];
        for n in ns {
            assert_eq!(is_prime_miller_rabin_probable(&n), is_prime(&n, 96));
            assert!(next_prime_is_prime(&n));
        }
    }

    #[test]
    fn test_case_mersenne_primes() {
        let ps: Vec<u32> = vec![2, 3, 5, 7, 13, 17, 19, 31, 61, 89, 107, 127];
        for p in ps {
            if next_prime_is_prime(&(p as u128)) {
                let cp: &u128 = &(u128::pow(2, p) - 1);
                assert_eq!(is_prime_miller_rabin_probable(cp), is_prime(cp, 96));
                assert!(next_prime_is_prime(cp));
            }
        }
    }

    #[test]
    fn test_case_fermat_primes() {
        for n in 1..5 {
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
        let power_of_primes: Vec<u128> = vec![
            4, 8, 16, 32, 64, 128, 256, 512, 1024, 9, 27, 81, 243, 729, 2187, 6561, 19683, 59049,
            25, 125, 625, 3125, 15625, 78125, 390625, 1953125, 9765625, 49, 343, 2401, 16807,
            117649, 823543, 5764801, 40353607, 282475249,
        ];
        for power in &power_of_primes {
            assert!(is_power_of_number(power));
        }
        assert!(!is_power_of_number(&13));
    }
}
