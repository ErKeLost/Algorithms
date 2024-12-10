use super::binary_search;

#[test]
fn test_non_consecutive_numbers() {
    // 不连续的数字数组
    let numbers = vec![2, 5, 8, 12, 16, 23, 38, 56, 72, 91];
    
    // 测试存在的数字
    assert_eq!(binary_search(&23, &numbers), Some(5));  // 23在索引5的位置
    
    // 测试不存在的数字
    assert_eq!(binary_search(&24, &numbers), None);     // 24不在数组中
    
    // 测试边界值
    assert_eq!(binary_search(&2, &numbers), Some(0));   // 第一个元素
    assert_eq!(binary_search(&91, &numbers), Some(9));  // 最后一个元素
    assert_eq!(binary_search(&1, &numbers), None);      // 比最小值还小
    assert_eq!(binary_search(&100, &numbers), None);    // 比最大值还大
}

#[test]
fn test_strings() {
    // 字符串数组
    let words = vec!["apple", "banana", "orange", "pear", "watermelon"];
    
    // 测试存在的单词
    assert_eq!(binary_search(&"orange", &words), Some(2));
    
    // 测试不存在的单词
    assert_eq!(binary_search(&"grape", &words), None);
}

#[test]
fn test_duplicates() {
    // 包含重复元素的数组
    let numbers = vec![1, 2, 2, 2, 3, 4, 4, 5, 5, 5];
    
    // 测试重复的数字
    assert_eq!(binary_search(&2, &numbers), Some(1)); // 返回其中一个2的位置
    assert_eq!(binary_search(&5, &numbers), Some(7)); // 返回其中一个5的位置
}
