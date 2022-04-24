use ghost::phantom;

fn main() {
    #[phantom]
    struct MyPhantom<T: ?Sized>;
}
