use ghost::phantom;

const fn main() {
    #[phantom]
    struct MyPhantom<T: ?Sized>;
}
