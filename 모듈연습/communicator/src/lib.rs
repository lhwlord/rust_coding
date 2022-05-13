pub mod client;

pub mod network;


pub mod outermost {
    pub fn middle_function() {}

    pub fn middle_secret_function() {}

    pub mod inside {
        pub fn inner_function() {
            crate::outermost::middle_secret_function();  // 절대경로 : 루트 경로에서부터 시작
            super::middle_function();   // 상대경로 : 자기 기준 시작. 여기선 super, 즉 상위에서부터 시작했음
        }

        pub fn secret_function() {}
    }
}

pub fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        crate::client::connect();
        super::client::connect();
    }
}
