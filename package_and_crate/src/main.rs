fn main() {
    // 这里 Vec<T> 在调用 iter() 时被解引用成 &[T]，所以可以访问 iter()
    let result = vec![1, 2, 3, 4]
        // return a iterator
        .iter()
        // return a type which impl Iterator
        .map(|v| v * v)
        // return a type which impl Iterator
        .filter(|v| *v < 16)
        // return a type which impl Iterator
        .take(1)
        // operate a sequence of iterators to get the data
        .collect::<Vec<_>>();
    println!("{:?}", result);
}
