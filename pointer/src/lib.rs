mod allocate;
pub struct LinkedList<T> {
    pub next: Option<Box<LinkedList<T>>>,
    pub key: T
}

#[repr(C, packed)]
pub struct DoublelyLinkedList<T> {
    pub prev: Option<Box<DoublelyLinkedList<T>>>,
    pub next: Option<Box<DoublelyLinkedList<T>>>,
    pub key: T
}

#[cfg(test)]
mod tests {
    use super::*;
}
