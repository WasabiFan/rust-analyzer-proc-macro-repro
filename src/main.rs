#[cfg(test)]
mod mocks;

fn main() {

}

#[cfg(test)]
mod tests {
    use crate::mocks::MockBarImpl;

    #[test]
    fn my_test() {
        let bar = MockBarImpl::new();
    }
}
