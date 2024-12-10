//! 这个模块提供了一个二分查找算法的实现，可以同时处理升序和降序排列的数组。
//! 二分查找函数在找到目标元素时返回其索引，如果目标不在数组中则返回 `None`。

use std::cmp::Ordering;

/// 在已排序的数组中执行二分查找。
///
/// 这个函数可以处理升序和降序排列的数组。它接收一个要搜索的目标值的引用
/// 和一个数组切片作为参数。如果找到目标值，则返回该值在数组中的索引。
/// 如果未找到目标值，则返回 `None`。
///
/// # 参数
///
/// - `item`: 要搜索的目标值的引用
/// - `arr`: 已排序数组的切片
///
/// # 返回值
///
/// 返回 `Option<usize>` 类型：
/// - 如果找到目标值，返回 `Some(index)`，其中 index 为目标值的索引
/// - 如果未找到目标值，返回 `None`
pub fn binary_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    let is_asc = is_asc_arr(arr);

    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        if match_compare(item, arr, &mut left, &mut right, is_asc) {
            return Some(left);
        }
    }

    None
}

/// 将目标值与当前搜索范围的中间元素进行比较，并相应地更新搜索边界。
/// 这个函数可以处理升序和降序排列的数组。它计算当前搜索范围的中间索引，
/// 并将目标值与该位置的元素进行比较。然后根据比较结果更新搜索边界
/// （`left` 和 `right`）。如果找到目标值，它会将 `left` 更新为找到的
/// 项的索引并返回 `true`。
///
/// # 参数
///
/// - `item`: 要搜索的目标值的引用
/// - `arr`: 要搜索的数组切片
/// - `left`: 左边界的可变引用
/// - `right`: 右边界的可变引用
/// - `is_asc`: 布尔值，表示数组是否按升序排列
///
/// # 返回值
///
/// 返回 `bool` 类型：
/// - 如果找到目标值，返回 `true`
/// - 如果未找到目标值，返回 `false`
fn match_compare<T: Ord>(
    item: &T,
    arr: &[T],
    left: &mut usize,
    right: &mut usize,
    is_asc: bool,
) -> bool {
    let mid = *left + (*right - *left) / 2;
    let cmp_result = item.cmp(&arr[mid]);

    match (is_asc, cmp_result) {
        (true, Ordering::Less) | (false, Ordering::Greater) => {
            *right = mid;
        }
        (true, Ordering::Greater) | (false, Ordering::Less) => {
            *left = mid + 1;
        }
        (_, Ordering::Equal) => {
            *left = mid;
            return true;
        }
    }

    false
}

/// 判断给定数组是否按升序排列。
///
/// 这个辅助函数通过检查数组的第一个元素是否小于最后一个元素来判断
/// 是否为升序排列。如果数组长度小于2，则返回 `false`。
///
/// # 参数
///
/// - `arr`: 要检查的数组切片
///
/// # 返回值
///
/// 返回 `bool` 类型，表示数组是否按升序排列
fn is_asc_arr<T: Ord>(arr: &[T]) -> bool {
    arr.len() > 1 && arr[0] < arr[arr.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_cases {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (item, arr, expected) = $test_case;
                    assert_eq!(binary_search(&item, arr), expected);
                }
            )*
        };
    }

    test_cases! {
        empty: ("a", &[] as &[&str], None),
        one_item_found: ("a", &["a"], Some(0)),
        one_item_not_found: ("b", &["a"], None),
        search_strings_asc_start: ("a", &["a", "b", "c", "d", "google", "zoo"], Some(0)),
        search_strings_asc_middle: ("google", &["a", "b", "c", "d", "google", "zoo"], Some(4)),
        search_strings_asc_last: ("zoo", &["a", "b", "c", "d", "google", "zoo"], Some(5)),
        search_strings_asc_not_found: ("x", &["a", "b", "c", "d", "google", "zoo"], None),
        search_strings_desc_start: ("zoo", &["zoo", "google", "d", "c", "b", "a"], Some(0)),
        search_strings_desc_middle: ("google", &["zoo", "google", "d", "c", "b", "a"], Some(1)),
        search_strings_desc_last: ("a", &["zoo", "google", "d", "c", "b", "a"], Some(5)),
        search_strings_desc_not_found: ("x", &["zoo", "google", "d", "c", "b", "a"], None),
        search_ints_asc_start: (1, &[1, 2, 3, 4], Some(0)),
        search_ints_asc_middle: (3, &[1, 2, 3, 4], Some(2)),
        search_ints_asc_end: (4, &[1, 2, 3, 4], Some(3)),
        search_ints_asc_not_found: (5, &[1, 2, 3, 4], None),
        search_ints_desc_start: (4, &[4, 3, 2, 1], Some(0)),
        search_ints_desc_middle: (3, &[4, 3, 2, 1], Some(1)),
        search_ints_desc_end: (1, &[4, 3, 2, 1], Some(3)),
        search_ints_desc_not_found: (5, &[4, 3, 2, 1], None),
        with_gaps_0: (0, &[1, 3, 8, 11], None),
        with_gaps_1: (1, &[1, 3, 8, 11], Some(0)),
        with_gaps_2: (2, &[1, 3, 8, 11], None),
        with_gaps_3: (3, &[1, 3, 8, 11], Some(1)),
        with_gaps_4: (4, &[1, 3, 8, 10], None),
        with_gaps_5: (5, &[1, 3, 8, 10], None),
        with_gaps_6: (6, &[1, 3, 8, 10], None),
        with_gaps_7: (7, &[1, 3, 8, 11], None),
        with_gaps_8: (8, &[1, 3, 8, 11], Some(2)),
        with_gaps_9: (9, &[1, 3, 8, 11], None),
        with_gaps_10: (10, &[1, 3, 8, 11], None),
        with_gaps_11: (11, &[1, 3, 8, 11], Some(3)),
        with_gaps_12: (12, &[1, 3, 8, 11], None),
        with_gaps_13: (13, &[1, 3, 8, 11], None),
    }
}
