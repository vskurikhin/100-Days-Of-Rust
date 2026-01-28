// Сколько существует различных по структуре бинарных деревьев поиска (БДП),
// состоящих из n вершин? Вершины пронумерованы числами 1...n.
// 
// Например, для n = 3 имеется 5 различных БДП.
// 
//    1           3      3       2       1
//      \         /     /       /  \       \
//       3       2     1       1    3       2
//      /      /        \                    \
//     2      1          2                    3
//
// РЕШЕНИЕ
// числа Каталана
//
// Анализ алгоритма
// Количество искомых бинарных деревьев равно числам Каталана.
// 
// Корень бинарного дерева содержит одну вершину. Если левое поддерево содержит k вершин
// (0 £ k £ n – 1), то правое поддерево содержит (n – k – 1) вершину.
// Обозначим через f(n) количество бинарных деревьев с n вершинами. Тогда
// 
// f(n) = f(0) * f(n – 1) + f(1) * f(n – 2) + … + f(n – 1) * f(0),
// 
// то есть f(n) = cn.

pub fn num_trees(n: usize) -> i32 {
    let mut cat: Vec<i32> = vec![0; n as usize + 1];
    let slice: &mut [i32] = &mut cat.as_mut_slice()[0..];
    _num_trees(slice)
}

fn _num_trees(cat: &mut [i32]) -> i32 {
    if cat.len() < 2 {
        return 1;
    }
    cat[0] = 1;
    cat[1] = 1;
    for i in 2..cat.len() {
        cat[i] = 0;
        for j in 0..i {
            cat[i] = cat[i] + cat[j] * cat[i-j-1];
        }
    }
    cat[cat.len()-1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {}

    #[test]
    fn num_trees_0() {
        assert_eq!(num_trees(0), 1);
    }

    #[test]
    fn num_trees_1() {
        assert_eq!(num_trees(1), 1);
    }

    #[test]
    fn num_trees_2() {
        assert_eq!(num_trees(2), 2);
    }

    #[test]
    fn num_trees_3() {
        assert_eq!(num_trees(3), 5);
    }

    #[test]
    fn num_trees_for() {
        let cat: [i32; 9] = [1, 1, 2, 5, 14, 42, 132, 429, 1430];
        for i in 0..9 {
            assert_eq!(num_trees(i), cat[i as usize]);
        }
    }
}