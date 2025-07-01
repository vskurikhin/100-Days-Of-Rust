/// ## Решение
/// Для решения этой задачи можно использовать два указателя на каждом конце массива,
/// а также две переменные для запоминания максимальных значений слева и справа от указателей.
/// 
/// ```text
/// 1. Инициализируем переменные left_max и right_max со значениями 0.
/// 2. Инициализируем переменную water со значением 0.
/// 3. Инициализируем переменную left со значением 0 и переменную right со значением n-1.
/// 4. Пока left меньше right:
///     - Если elevation[left] меньше elevation[right], то:
///         - Если elevation[left] больше left_max, то left_max принимает значение elevation[left].
///         - Иначе water увеличивается на left_max - elevation[left].
///         - left увеличивается на 1.
///     - Иначе:
///         - Если elevation[right] больше right_max, то right_max принимает значение elevation[right].
///         - Иначе water увеличивается на right_max - elevation[right].
///         - right уменьшается на 1.
/// 5. Возвращаем значение переменной water.
/// ```
/// 
/// ## Объяснение
/// 
/// Для решения задачи мы используем два указателя, которые последовательно сдвигаются внутрь массива.
/// На каждом шаге мы запоминаем максимальное значение слева и справа от указателей.
/// Если значение слева меньше значения справа, то мы сравниваем значение слева с левым максимумом,
/// который мы уже запомнили. Если значение слева больше левого максимума, то левый максимум
/// принимает это значение. Иначе мы добавляем воду, которая может быть задержана на данной позиции.
/// Аналогично мы поступаем, если значение слева больше значения справа. На каждом шаге мы сдвигаем
/// указатели внутрь массива и повторяем процесс.
pub fn trapping_rain_water(elevation: &[i32]) -> i32 {
    let (mut left_max, mut right_max) = (0, 0);
    let (mut left, mut right) = (0, eval_right(elevation));
    let mut water = 0;

    while left < right {
        if elevation[left] < elevation[right] {
            if elevation[left] > left_max {
                left_max = elevation[left];
            } else {
                water += left_max - elevation[left];
            }
            left += 1;
        } else {
            if elevation[right] > right_max {
                right_max = elevation[right];
            } else {
                water += right_max - elevation[right];
            }
            right -= 1;
        }
    }
    water
}

fn eval_right(elevation: &[i32]) -> usize {
    if elevation.len() == 0 {
        return 0;
    }
    elevation.len() - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {}

    #[test]
    fn trapping_rain_water_test_case_0() {
        assert_eq!(trapping_rain_water(&[]), 0);
    }

    #[test]
    fn trapping_rain_water_test_case_21() {
        assert_eq!(trapping_rain_water(&[2, 1]), 0);
    }

    #[test]
    fn trapping_rain_water_test_case_010() {
        assert_eq!(trapping_rain_water(&[0, 1, 0]), 0);
    }

    #[test]
    fn trapping_rain_water_test_case_201() {
        assert_eq!(trapping_rain_water(&[2, 0, 1]), 1);
    }

    #[test]
    fn trapping_rain_water_test_case_010210132121() {
        assert_eq!(
            trapping_rain_water(&[0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]),
            6
        );
    }

    #[test]
    fn trapping_rain_water_test_case_420325() {
        assert_eq!(trapping_rain_water(&[4, 2, 0, 3, 2, 5]), 9);
    }
}
