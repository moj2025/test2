pub mod api_test2 {
    pub mod common {
        pub mod query {
            tonic::include_proto!("cn.pecs.api.common.query");
        }
        tonic::include_proto!("cn.pecs.api.common.heartbeat");
    }
    pub mod test2 {
        pub mod oauth {
            tonic::include_proto!("cn.pecs.api.test2.oauth");
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    Ok(())
}
