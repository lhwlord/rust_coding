extern crate communicator;

fn main() {
    communicator::client::connect();
    crate::communicator::client::connect(); // 둘 다 된다. 자기 크레이트 안에서 작업하는 것이므로.
}