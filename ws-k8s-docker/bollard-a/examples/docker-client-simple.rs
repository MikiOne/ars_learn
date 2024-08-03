use std::collections::HashMap;
/// 创建一个简单的 Docker 客户端
use bollard::Docker;
use bollard::container::ListContainersOptions;
use tokio;

#[tokio::main]
async fn main() {
    // 创建 Docker 客户端
    let docker = Docker::connect_with_local_defaults().unwrap();

    // 列出所有运行中的容器
    // let containers = docker.list_containers(Some(ListContainersOptions::default())).await.unwrap();
    // let options = Some(ListContainersOptions::default());

    // filter
    let mut filters = HashMap::new();
    filters.insert("health", vec!["unhealthy"]);

    let options = Some(ListContainersOptions {
        all: true,
        filters,
        ..Default::default()
    });

    let containers = docker.list_containers(options).await.unwrap();

    // 打印容器信息
    for container in containers {
        println!("Container ID: {:?}", container.id);
        println!("Container Names: {:?}", container.names);
        println!("Container Status: {:?}", container.status);
        println!("-------------------------");
    }
}
