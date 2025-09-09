pub mod api_proto {
    tonic::include_proto!("cn.pecs.api");
    tonic::include_proto!("cn.pecs.api.common");
    pub use test2::*;
    pub mod test2 {
        pub use oauth::*;
        pub mod oauth {
            tonic::include_proto!("cn.pecs.api.test2.oauth");
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // api_test2::oauth_service_server
    println!("Hello, world!");
    Ok(())
}
