pub mod api_test2 {
    tonic::include_proto!("cn.pecs.api.common.query");
    tonic::include_proto!("cn.pecs.api.common.heartbeat");
    tonic::include_proto!("cn.pecs.api.test2.oauth");
}


#[tokio::main]
async fn main() -> std::io::Result<()> {
    // api_test2::oauth_service_server::OauthServiceServer
    Ok(())
}
