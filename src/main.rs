fn main() {
    let mut sortable = [64, 34, 25, 12, -18, 22, 11, 18, 26, 69, 9, 90];
    sortable::bubble_sort(&mut sortable);
    println!("Sorted: {:?}", sortable);
}

mod sortable {
    pub(crate) fn bubble_sort<T: Ord>(sortable: &mut [T]) {
        let n = sortable.len();
        for i in 0..n {
            for j in 0..n - i - 1 {
                if sortable[j] > sortable[j + 1] {
                    sortable.swap(j, j + 1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut sortable = [64, 34, 25, 12, -18, 22, 11, 18, 26, 69, 9, 90];
        sortable::bubble_sort(&mut sortable);
        assert_eq!(sortable, [-18, 9, 11, 12, 18, 22, 25, 26, 34, 64, 69, 90]);
    }
}
