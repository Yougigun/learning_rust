pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn test() {
    #[derive(Debug)]
    struct Node {
        value: i32,
        next: Option<Box<Node>>,
    }
    let mut n = Node {
        value: 1,
        next: Some(Box::new(Node {
            value: 2,
            next: None,
        })),
    };

    n.value = 3;

    if let Some(ref mut next) = n.next {
        next.as_mut().value = 4;
    }
    println!("n = {:?}", n);

    n.next = Some(Box::new(Node {
        value: 5,
        next: None,
    }));

    println!("n = {:?}", n);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
