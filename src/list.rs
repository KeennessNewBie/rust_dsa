#[derive(Debug)]
enum List<T: std::fmt::Display> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T: std::fmt::Display> List<T> {
    // 创建一个空链表
    fn new() -> List<T> {
        // `Nil` 是 `List`类型的。因为前面我们使用了 `use List::*;`
        // 所以不需要 List::Nil 这样使用
        List::Nil
    }

    // 在前面加一个元素节点，并且链接旧的链表和返回新的链表
    fn prepend(self, elem: T) -> List<T> {
        // `Cons` 也是 List 类型的
        List::Cons(elem, Box::new(self))
    }

    // 返回链表的长度
    fn len(&self) -> u32 {
        // `self` 的类型是 `&List`, `*self` 的类型是 `List`,
        // 匹配一个类型 `T` 好过匹配一个引用 `&T`
        match *self {
            // 因为`self`是借用的，所以不能转移 tail 的所有权
            // 因此使用 tail 的引用
            List::Cons(_, ref tail) => 1 + tail.len(),
            // 基本规则：所以空的链表长度都是0
            List::Nil => 0,
        }
    }

    // 返回连链表的字符串表达形式
    fn stringify(&self) -> String {
        match self {
            List::Cons(head, ref tail) => {
                // `format!` 和 `print!` 很像
                // 但是返回一个堆上的字符串去替代打印到控制台
                format!("{}, {}", head, tail.stringify())
            }
            List::Nil => {
                format!("Nil")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_list() {
        let mut list = List::new();

        list = list.prepend(1);
        list = list.prepend(2);
        list = list.prepend(3);

        println!("linked list has length: {}", list.len());
        println!("{}", list.stringify());
    }
}
