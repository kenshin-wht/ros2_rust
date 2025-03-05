use std::convert::TryInto;

use anyhow::{Error, Result};
use rosidl_runtime_rs::{seq, BoundedSequence, Message, Sequence};

use rclrs::*;

fn check_default_values() {
    // 创建一个默认的VariousTypes消息
    let msg = rclrs_example_msgs::msg::rmw::VariousTypes::default();
    // 断言bool_member为true
    assert!(msg.bool_member);
    // 断言int8_member为1
    assert_eq!(msg.int8_member, 1i8);
    // 断言uint8_member为2
    assert_eq!(msg.uint8_member, 2u8);
    // 断言byte_member为3
    assert_eq!(msg.byte_member, 3u8);
    // 断言float32_member为1e-2
    assert_eq!(msg.float32_member, 1e-2f32);
    // 断言float_array为[1.0, 2.0, 3.0]
    assert_eq!(msg.float_array, [1.0, 2.0, 3.0]);
    // 断言float_seq_bounded为seq![3 # 4.0, 5.0]
    assert_eq!(msg.float_seq_bounded, seq![3 # 4.0, 5.0]);
    // 断言float_seq_unbounded为seq![6.0]
    assert_eq!(msg.float_seq_unbounded, seq![6.0]);
    // 断言string_member为"Χαίρετε 你好"
    assert_eq!(msg.string_member.to_string(), "Χαίρετε 你好");
    // 断言wstring_member为"αντίο σου 再见"
    assert_eq!(msg.wstring_member.to_string(), "αντίο σου 再见");
    // 断言bounded_string_member为"aou"
    assert_eq!(msg.bounded_string_member.to_string(), "aou");
    // 断言bounded_wstring_member为"äöü"
    assert_eq!(msg.bounded_wstring_member.to_string(), "äöü");
    // 断言string_array为["R", "O", "S", "2"]
    assert_eq!(
        msg.string_array.clone().map(|s| s.to_string()),
        ["R", "O", "S", "2"].map(String::from)
    );
    // 断言string_seq_bounded为seq![4 # "R".into(), "O".into(), "S".into(), "2".into()]
    assert_eq!(
        msg.string_seq_bounded,
        seq![4 # "R".into(), "O".into(), "S".into(), "2".into()]
    );
    // 断言string_seq_unbounded为seq!["R".into(), "O".into(), "S".into(), "2".into()]
    assert_eq!(
        msg.string_seq_unbounded,
        seq!["R".into(), "O".into(), "S".into(), "2".into()]
    );
    // 断言bounded_string_array为["R", "O", "S", "2"]
    assert_eq!(
        msg.bounded_string_array.clone().map(|s| s.to_string()),
        ["R", "O", "S", "2"].map(String::from)
    );
    // 断言bounded_string_seq_bounded为["R", "O", "S", "2"]
    assert_eq!(
        msg.bounded_string_seq_bounded,
        ["R", "O", "S", "2"]
            .into_iter()
            .map(|s| s.try_into().unwrap())
            .collect()
    );
    assert_eq!(
        msg.bounded_string_seq_unbounded,
        ["R", "O", "S", "2"]
            .into_iter()
            .map(|s| s.try_into().unwrap())
            .collect()
    );
    assert_eq!(msg.nested_member.effect.to_string(), "discombobulate");
    assert_eq!(
        msg.nested_array,
        [msg.nested_member.clone(), msg.nested_member.clone()]
    );
    assert_eq!(msg.nested_seq_bounded, seq![3 #]);
    assert_eq!(msg.nested_seq_unbounded, seq![]);

    // The default instance for the idiomatic type also has the defaults set
    let idiomatic_msg = rclrs_example_msgs::msg::VariousTypes::default();
    assert_eq!(
        rclrs_example_msgs::msg::VariousTypes::into_rmw_message(std::borrow::Cow::Owned(
            idiomatic_msg
        ))
        .into_owned(),
        msg
    );
}

// 检查默认IDL值
fn check_default_idl_values() {
    // 创建一个默认的idiomatic消息
    let idiomatic_msg = rclrs_example_msgs::msg::MyMessage::default();
    // 创建一个默认的rmw消息
    let rmw_msg = rclrs_example_msgs::msg::rmw::MyMessage::default();

    // 断言idiomatic消息的wchar_value字段值为0
    assert_eq!(idiomatic_msg.wchar_value, 0u16);
    // 断言rmw消息的wchar_value字段值为0
    assert_eq!(rmw_msg.wchar_value, 0u16);
}

fn demonstrate_printing() {
    // 创建一个默认的VariousTypes消息
    let default_msg = rclrs_example_msgs::msg::VariousTypes::default();
    // 打印Compact debug representation
    println!("================== Compact debug representation ==================");
    println!("{:?}", default_msg);
    // 打印Pretty debug representation
    println!("================== Pretty debug representation ===================");
    println!("{:#?}", default_msg);
    // The RMW-native message type has the same output
    let default_rmw_msg = rclrs_example_msgs::msg::rmw::VariousTypes::default();
    assert_eq!(
        format!("{:?}", default_msg),
        format!("{:?}", default_rmw_msg)
    );
    assert_eq!(
        format!("{:#?}", default_msg),
        format!("{:#?}", default_rmw_msg)
    );
}

fn demonstrate_serde() -> Result<(), Error> {
    // When the serde feature is turned on, messages are able to be serialized
    // to and deserialized from a variety of formats. Here JSON is used as an
    // example.
    // Works with RMW-native and idiomatic messages.
    let idiomatic_msg = rclrs_example_msgs::msg::VariousTypes::default();
    let rmw_msg = rclrs_example_msgs::msg::rmw::VariousTypes::default();
    println!("================= JSON serialization with Serde ==================");
    let idiomatic_serialized = serde_json::to_string_pretty(&idiomatic_msg)?;
    let rmw_serialized = serde_json::to_string_pretty(&rmw_msg)?;
    assert_eq!(idiomatic_serialized, rmw_serialized);
    println!("{}", rmw_serialized);
    let idiomatic_deserialized = serde_json::from_str(&idiomatic_serialized)?;
    let rmw_deserialized = serde_json::from_str(&rmw_serialized)?;
    assert_eq!(idiomatic_msg, idiomatic_deserialized);
    assert_eq!(rmw_msg, rmw_deserialized);
    Ok(())
}

fn demonstrate_sequences() {
    // Convenient creation of (bounded) sequences with the seq! macro
    // This one has three items and a length bound of 5
    let mut float_seq_bounded = seq![5 # 1.0, 2.0, 3.0];
    // Sequences and bounded sequences have iter(), iter_mut(), and into_iter()
    float_seq_bounded
        .iter_mut()
        .for_each(|n: &mut f32| *n += 1.0);
    let float_vec_1: Vec<_> = float_seq_bounded.iter().copied().collect();
    let float_vec_2: Vec<_> = float_seq_bounded.into_iter().collect();
    assert_eq!(float_vec_1, float_vec_2);
    // Sequences also implement FromIterator.
    let mut int_seq_unbounded: Sequence<i32> = [42; 4].into_iter().collect();
    // Bounded sequences will ignore remaining items once the length bound is reached
    let mut int_seq_bounded: BoundedSequence<i32, 3> = [42; 4].into_iter().collect();
    // Sequences deref to slices
    int_seq_bounded[2] = 24;
    assert_eq!(int_seq_bounded.last(), Some(&24));
    int_seq_unbounded[2..].copy_from_slice(&int_seq_bounded[1..]);
    // New sequences will contain default values – and 0 for primitive types
    let seq_with_default_values = Sequence::<rclrs_example_msgs::msg::rmw::NestedType>::new(1);
    assert_eq!(seq_with_default_values[0].effect, "discombobulate".into());
}

fn demonstrate_pubsub() -> Result<(), Error> {
    println!("================== Interoperability demo ==================");
    // Demonstrate interoperability between idiomatic and RMW-native message types
    let mut executor = Context::default_from_env()?.create_basic_executor();
    let node = executor.create_node("message_demo")?;

    let idiomatic_publisher = node
        .create_publisher::<rclrs_example_msgs::msg::VariousTypes>("topic", QOS_PROFILE_DEFAULT)?;
    let direct_publisher = node.create_publisher::<rclrs_example_msgs::msg::rmw::VariousTypes>(
        "topic",
        QOS_PROFILE_DEFAULT,
    )?;

    let _idiomatic_subscription = node
        .create_subscription::<rclrs_example_msgs::msg::VariousTypes, _>(
            "topic",
            QOS_PROFILE_DEFAULT,
            move |_msg: rclrs_example_msgs::msg::VariousTypes| println!("Got idiomatic message!"),
        )?;
    let _direct_subscription = node
        .create_subscription::<rclrs_example_msgs::msg::rmw::VariousTypes, _>(
            "topic",
            QOS_PROFILE_DEFAULT,
            move |_msg: rclrs_example_msgs::msg::rmw::VariousTypes| {
                println!("Got RMW-native message!")
            },
        )?;
    println!("Sending idiomatic message.");
    idiomatic_publisher.publish(rclrs_example_msgs::msg::VariousTypes::default())?;
    executor.spin(SpinOptions::spin_once()).first_error()?;
    println!("Sending RMW-native message.");
    direct_publisher.publish(rclrs_example_msgs::msg::rmw::VariousTypes::default())?;
    executor.spin(SpinOptions::spin_once()).first_error()?;

    Ok(())
}

fn main() -> Result<(), Error> {
    check_default_values();
    check_default_idl_values();
    demonstrate_printing();
    demonstrate_serde()?;
    demonstrate_sequences();
    demonstrate_pubsub()?;
    Ok(())
}
