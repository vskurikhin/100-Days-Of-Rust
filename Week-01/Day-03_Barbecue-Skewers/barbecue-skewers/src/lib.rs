
pub fn barbecue_skewers(inp: Vec<&str>) -> (u32, u32) {
    let mut vegetarian_skewers: u32 = 0;
    let mut non_vegetarian_skewers: u32 = 0;

    for item in inp {
        if search_x(&item) {
            non_vegetarian_skewers += 1
        } else {
            vegetarian_skewers += 1
        }
    }
    (vegetarian_skewers, non_vegetarian_skewers)
}

fn search_x(item: &str) -> bool {
    for c in item.chars() {
        if c.eq_ignore_ascii_case(&'x') {
            return true;
        }
    }
    false
}

pub fn count_skewers(buffer: &str) -> [usize; 2] {
    let mut skewers = skewer_utils::parse_skewers_to_vec(&buffer);
    let mut skewers_clone = skewers.clone();
    [skewer_utils::count_vegetarians(&mut skewers), skewer_utils::count_meats(&mut skewers_clone)]
}

mod skewer_utils {
    pub fn parse_skewers_to_vec(buffer: &str) -> Vec<&str> {
        Vec::from_iter(buffer.split(','))
    }

    pub fn count_vegetarians(skewers: &mut Vec<&str>) -> usize {
        skewers.retain(|&skewer| !skewer.contains("x"));
        skewers.len()
    }

    pub fn count_meats(skewers: &mut Vec<&str>) -> usize {
        skewers.retain(|&skewer| skewer.contains("x"));
        skewers.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        assert_eq!(barbecue_skewers(vec![
            "--oooo-ooo--",
            "--xx--x--xx--",
            "--o---o--oo--",
            "--xx--x--ox--",
            "--xx--x--ox--"

        ]), (2,3));
    }

    #[test]
    fn test_case2() {
        assert_eq!(barbecue_skewers(vec![
            "--oooo-ooo--",
            "--xxxxxxxx--",
            "--o---",
            "-o-----o---x--",
            "--o---o-----"
        ]), (3,2));
    }
}