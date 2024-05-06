#[derive(Default)]
pub struct List<T> {
    head: Link<T>,
}

// option 型にしたことで冗長な記述が減らせる
// たとえば、mem::replace(&mut list.head(), Link::None) -> list.head.take() に簡略化できる
// https://rust-lang.github.io/rust-clippy/master/index.html#/mem_replace_option_with_none
// あるいは、 list.head.is_some() とか
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: Link::None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            next: self.head.take(),
            elem,
        });
        self.head = Link::Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    fn pop_node(&mut self) -> Link<T> {
        match self.head.take() {
            Link::None => Link::None,
            Link::Some(mut node) => {
                self.head = node.next.take();
                Link::Some(node)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // let mut cur_link = self.head.take();
        // while let Link::Some(mut boxed_node) = cur_link {
        //     cur_link = boxed_node.next.take();
        // }
        while self.pop_node().is_some() {}
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // IntoIter は tuple 型の struct なので、 0 でアクセスする
        self.0.pop()
    }
}

#[cfg(test)]
mod test {
    // #[cfg(test)] を入れないと、これが unused 扱いされてしまう
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        if let Some(value) = list.peek_mut() {
            *value = 42
        }
        // clippy の警告が出る
        // https://rust-lang.github.io/rust-clippy/master/index.html#option_map_unit_fn
        // list.peek_mut().map(|value| *value = 42);

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.peek_mut(), Some(&mut 42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
