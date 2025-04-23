pub struct Queue {
    stack: Vec<i64>
}

impl Queue {
    pub fn enqueue(&mut self, x: i64) {
        let mut buff: Vec<i64> = vec![];
        swap(&mut self.stack, &mut buff);
        self.stack.push(x);
        swap(&mut buff, &mut self.stack);
    }

    pub fn dequeue(&mut self) -> Option<i64> {
        return self.stack.pop()
    }
}

fn swap(from: &mut Vec<i64>, to: &mut Vec<i64>) {
    while ! from.is_empty() {
        to.push(from.pop().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enqueue_works() {
        let mut q = Queue {
            stack: vec![3, 2, 1],
        };
        q.enqueue(4);
        assert_eq!(q.stack, vec![4,3,2,1])
    }
    #[test]
    fn dequeue_works() {
        let mut q = Queue {
            stack: vec![3, 2, 1],
        };
        q.dequeue();
        assert_eq!(q.stack, vec![3,2])
    }
}
