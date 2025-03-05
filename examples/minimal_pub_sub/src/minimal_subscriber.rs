use anyhow::{Error, Result};
use rclrs::*;

fn main() -> Result<(), Error> {
    // 从环境变量中创建一个默认的上下文
    let context = Context::default_from_env()?;
    // 创建一个基本的执行器
    let mut executor = context.create_basic_executor();

    // 创建一个节点
    let node = executor.create_node("minimal_subscriber")?;

    // 初始化消息数量
    let mut num_messages: usize = 0;

    // 创建一个订阅
    let _subscription = node.create_subscription::<std_msgs::msg::String, _>(
        "topic",
        QOS_PROFILE_DEFAULT,
        move |msg: std_msgs::msg::String| {
            // 每次收到消息，消息数量加一
            num_messages += 1;
            // 打印收到的消息
            println!("I heard: '{}'", msg.data);
            // 打印到目前为止收到的消息数量
            println!("(Got {} messages so far)", num_messages);
        },
    )?;

    // 执行器开始运行
    executor
        .spin(SpinOptions::default())
        // 返回第一个错误
        .first_error()
        // 将错误转换为Error类型
        .map_err(|err| err.into())
}
