use anyhow::{Error, Result};
use rclrs::*;

fn main() -> Result<(), Error> {
    // 从环境变量中创建一个默认的上下文
    let context = Context::default_from_env()?;
    // 创建一个基本的执行器
    let executor = context.create_basic_executor();

    // 创建一个节点
    let node = executor.create_node("minimal_publisher")?;

    // 创建一个发布者，发布主题为"topic"，QOS_PROFILE_DEFAULT
    let publisher = node.create_publisher::<std_msgs::msg::String>("topic", QOS_PROFILE_DEFAULT)?;

    // 创建一个默认的消息
    let mut message = std_msgs::msg::String::default();

    // 发布次数
    let mut publish_count: u32 = 1;

    // 当上下文正常时，循环发布消息
    while context.ok() {
        // 设置消息内容
        message.data = format!("Hello, world! {}", publish_count);
        // 打印发布内容
        println!("Publishing: [{}]", message.data);
        // 发布消息
        publisher.publish(&message)?;
        // 发布次数加一
        publish_count += 1;
        // 等待500毫秒
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    // 返回成功
    Ok(())
}
