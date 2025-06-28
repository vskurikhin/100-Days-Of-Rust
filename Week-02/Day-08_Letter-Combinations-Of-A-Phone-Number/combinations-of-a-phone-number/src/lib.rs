use factorial::*;
use phf::phf_map;

pub static KEYWORDS: phf::Map<&'static str, &[char]> = phf_map! {
    "0" => &[' '],
    "1" => &['\0'],
    "2" => &['a', 'b', 'c'],
    "3" => &['d', 'e', 'f'],
    "4" => &['g', 'h', 'i'],
    "5" => &['j', 'k', 'l'],
    "6" => &['m', 'n', 'o'],
    "7" => &['p', 'q', 'r', 's'],
    "8" => &['s', 't', 'u', 'v'],
    "9" => &['x', 'y', 'z'],
};

pub fn all_combinations(m: &[&[char]]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new(); // Текущая комбинация
    let mc = number_of_combinations(m);

    for j in 0..mc {
        let cr = run_through_chars(j, m);
        result.push(cr);
    }
    result.sort();
    result
}

/// run_through_sets - Бежим по множествам
pub fn run_through_chars(j: usize, m: &[&[char]]) -> String {
    let mut result: String = String::new(); // Текущая комбинация
    let mut nc: usize = 1; // Сдвиг разрядов

    // Бежим по множествам
    for i in 0..m.len() {
        if m[i].len() < 1 {
            continue;
        }
        // Номер комбинации, сдвинутый на i разрядов влево.
        let shifted = ((j as f64) / (nc as f64)).floor() as usize;
        // Индекс элемента в i-том разряде
        let idx = shifted % m[i].len();
        // Заносим в комбинацию элемент по полученному индексу
        result.push(m[i][idx]);
        // Двигаем разряд влево дальше
        nc *= m[i].len();
    }
    result
}

/// Массив всех комбинаций
pub fn vec_of_all_combinations(m: &[&[char]]) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new(); // Текущая комбинация
    let mc = number_of_combinations(m);

    for j in 0..mc {
        let cr = run_through_sets(j, m);
        result.push(cr);
    }
    result
}

/// number_of_combinations - Число комбинаций
pub fn number_of_combinations(slice: &[&[char]]) -> usize {
    let mut combinations = 1;
    for i in 0..slice.len() {
        combinations *= slice[i].len();
    }
    combinations
}

/// run_through_sets - Бежим по множествам
pub fn run_through_sets(j: usize, m: &[&[char]]) -> Vec<char> {
    let mut result: Vec<char> = Vec::new(); // Текущая комбинация
    let mut nc: usize = 1; // Сдвиг разрядов

    // Бежим по множествам
    for i in 0..m.len() {
        if m[i].len() < 1 {
            continue;
        }
        // Номер комбинации, сдвинутый на i разрядов влево.
        let shifted = ((j as f64) / (nc as f64)).floor() as usize;
        // Индекс элемента в i-том разряде
        let idx = shifted % m[i].len();
        // Заносим в комбинацию элемент по полученному индексу
        result.push(m[i][idx]);
        // Двигаем разряд влево дальше
        nc *= m[i].len();
    }
    result
}

fn permutation(n: i8, k: i8) -> Vec<Vec<usize>> {
    let elm = n as u128;
    let elm_checked_factorial = elm.checked_factorial();
    let elm_factorial = match elm_checked_factorial {
        Some(elm_factorial) => elm_factorial,
        None => return Vec::new(),
    };
    let cells = k as u128;
    let elm_cells_checked_factorial = (elm - cells).checked_factorial();
    let elm_cells_factorial = match elm_cells_checked_factorial {
        Some(elm_cells_factorial) => elm_cells_factorial,
        None => return Vec::new(),
    };
    let mut res: Vec<Vec<usize>> = Vec::new();
    let arrang = elm_factorial / elm_cells_factorial;

    for i in 0..arrang {
        res.push(Vec::from([]));
        let mut source: Vec<u128> = (0..elm).collect();

        for j in 0..cells {
            let fact = (cells - 1 - j).checked_factorial().unwrap();
            let p = i / fact % source.len() as u128;
            let n = res.len() - 1;
            res[n].push(source[p as usize] as usize);
            source.remove(p as usize);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        permutation(15, 5);
    }

    #[test]
    fn test_combinations_case_0() {
        let result = all_combinations(&[&[]]);
        assert_eq!(result.len(), 0);
        let expected: Vec<String> = Vec::new();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_all_combinations_case_1() {
        let result = all_combinations(&[&['a', 'b', 'c']]);
        assert_eq!(vec!["a", "b", "c"], result);
    }

    #[test]
    fn test_all_combinations_case_2() {
        let result = all_combinations(&[&['a', 'b', 'c'], &['d', 'e', 'f']]);
        assert_eq!(
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"],
            result
        );
    }

    #[test]
    fn test_all_combinations_case_3() {
        let result = all_combinations(&[&['a', 'b', 'c'], &['d', 'e', 'f'], &['p', 'q', 'r', 's']]);
        assert_eq!(
            vec![
                "adp", "adq", "adr", "ads", "aep", "aeq", "aer", "aes", "afp", "afq", "afr", "afs",
                "bdp", "bdq", "bdr", "bds", "bep", "beq", "ber", "bes", "bfp", "bfq", "bfr", "bfs",
                "cdp", "cdq", "cdr", "cds", "cep", "ceq", "cer", "ces", "cfp", "cfq", "cfr", "cfs"
            ],
            result
        );
    }

    #[test]
    fn test_run_through_sets_case_1() {
        for char in run_through_sets(0, &[&['a', 'b', 'c']]) {
            assert_eq!(char, 'a');
        }
        for char in run_through_sets(1, &[&['a', 'b', 'c']]) {
            assert_eq!(char, 'b');
        }
        for char in run_through_sets(2, &[&['a', 'b', 'c']]) {
            assert_eq!(char, 'c');
        }
        for char in run_through_sets(3, &[&['a', 'b', 'c']]) {
            assert_eq!(char, 'a');
        }
    }

    #[test]
    fn test_vec_of_all_combinations_case_0() {
        let result = vec_of_all_combinations(&[&[]]);
        assert_eq!(result.len(), 0);
        let expected: Vec<Vec<char>> = Vec::new();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_vec_of_all_combinations_case_1() {
        let mut s = String::new();
        let expected = "adbdcdaebeceafbfcf";
        let result = vec_of_all_combinations(&[&['a', 'b', 'c'], &['d', 'e', 'f']]);
        for v in result {
            for e in v {
                s.push(e);
            }
        }
        assert_eq!(expected, s);
    }

    #[test]
    fn test_number_of_combinations_case_0() {
        assert_eq!(0, number_of_combinations(&[&[]]));
    }

    #[test]
    fn test_number_of_combinations_case_1() {
        assert_eq!(3, number_of_combinations(&[&['a', 'b', 'c']]));
    }

    #[test]
    fn test_number_of_combinations_case_2() {
        assert_eq!(9, number_of_combinations(&[&['a', 'b', 'c'], &['d', 'e', 'f']]));
    }

    #[test]
    fn test_number_of_combinations_case_3() {
        assert_eq!(36, number_of_combinations(&[&['a', 'b', 'c'], &['d', 'e', 'f'], &['p', 'q', 'r', 's']]));
    }
}
