#[derive(Debug)]
pub struct List<T> {
    head: Option<Box<Node<T>>>,
    curr: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct Node<T> {
    next: Option<Box<Node<T>>>,
    value: T,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List {
            head: None,
            curr: None,
        }
    }

    pub fn push_head(&mut self, value: T) {
        let new_node = Node {
            next: self.head.take(),
            value,
        };
        self.head = Some(Box::new(new_node));
    }

    pub fn pop_head(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.value
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.value
        })
    }

    pub fn len(&self) -> i64 {
        let mut count = 0;
        let mut current_node = &self.head;
        while let Some(node) = current_node {
            current_node = &node.next;
            count += 1;
        };
        count
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_head().is_some() {}
    }
}

impl<T> Iterator for List<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self.curr {
            Some(node) => {

            },
            None => {
                self.curr = &self.head;
            }
        }
    }
}


fn main() {
    let mut list: List<i32> = List::new();
    list.push_head(1);
    list.push_head(3);
    let val = list.pop_head();
    let val2 = list.peek();
    println!("{:?}", list);
    println!("{:?}", val);
    println!("{:?}", val2);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_head(), None);

        // Populate list
        list.push_head(1);
        list.push_head(2);
        list.push_head(3);

        // Check normal removal
        assert_eq!(list.pop_head(), Some(3));
        assert_eq!(list.pop_head(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_head(4);
        list.push_head(5);

        // Check normal removal
        assert_eq!(list.pop_head(), Some(5));
        assert_eq!(list.pop_head(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_head(), Some(1));
        assert_eq!(list.pop_head(), None);
    }

    #[test]
    fn len() {
        let mut list = List::new();

        assert_eq!(list.len(), 0);

        list.push_head(1);
        list.push_head(2);
        list.push_head(3);

        assert_eq!(list.len(), 3);

        list.pop_head();
        list.pop_head();

        assert_eq!(list.len(), 1);

        list.pop_head();
        assert_eq!(list.len(), 0);
        list.pop_head();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push_head(1);
        list.push_head(2);
        list.push_head(3);

        assert_eq!(list.peek(), Some(&3));
        list.peek_mut().map(|num| {
            *num = 5;
        });
        assert_eq!(list.peek(), Some(&5));
    }
}
