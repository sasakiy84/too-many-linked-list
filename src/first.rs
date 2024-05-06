use std::mem;

#[derive(Default)]
pub struct List<T> {
    head: Link<T>,
}

// これはメモリ上ではポインタの値として配置され、固定長である
// しかも、 null pointer optimization により enum のタグの値のためのメモリ領域が必要ない
// なぜなら、`More(Box<Node>)` は null pointer でなく、そのため 0 を Empty の場合と見なせるから
#[derive(Default)]
enum Link<T> {
    #[default]
    Empty,
    More(Box<Node<T>>),
}

// enum は複数の異なる値の中のうちの一つを表す
// struct は複数の異なる値を持った一つの値を表す
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    // self が引数に無いので static method
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: T) {
        // 悲しいことに、一度 0 で self.head を埋めなければいけない
        let new_node = Box::new(Node {
            next: mem::replace(&mut self.head, Link::Empty),
            elem,
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => Option::None,
            Link::More(node) => {
                self.head = node.next;
                Option::Some(node.elem)
            }
        }
    }

    // drop 用のチャレンジ問題として実装したメソッド
    // https://rust-unofficial.github.io/too-many-lists/first-drop.html#bonus-section-for-premature-optimization
    // pop を drop で使うときの問題点であった elem が stack にコピーされてしまうという問題を
    // ポインタ（Box<Node>） を返すことで回避している
    fn pop_node(&mut self) -> Link<T> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => Link::Empty,
            Link::More(mut node) => {
                self.head = mem::replace(&mut node.next, Link::Empty);
                Link::More(node)
            }
        }
    }
}

// tail recursive なかたちで drop させることが自動実装ではできないので、明示的にやり方を指定する
// 具体的には、Box<Node> に対する Drop が以下のように参照先を drop したあとに自分自身を deallocate するため、
// 最初の再帰元の状態を最後の再帰先の drop が終わるまで保持しなければならない（スタックに積んでおかなければならない？）
// impl Drop for Box<Node> {
//     fn drop(&mut self) {
//         self.ptr.drop(); // uh oh, not tail recursive!
//         deallocate(self.ptr);
//     }
// }
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // while let Link::More(mut boxed_node) = cur_link {
        //     cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        // }
        while let Link::More(_) = self.pop_node() {}
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
}
