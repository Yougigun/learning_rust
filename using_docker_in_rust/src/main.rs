use bollard::Docker;
#[tokio::main]
async fn main() {
    let docker = Docker::connect_with_local_defaults().unwrap();
    let containers = docker.list_containers::<String>(None).await.unwrap();
    for container in containers {
        println!("Container: {:?}", container.id);
    }
}
