//! flows network这个是以来wasmedge云函数进行开发llm大模型的， 我们首先来实现一个云函数的流程
//! 首先取github建立一个项目
//! 现在我们可以开始写代码了
//!
//! 我们同步完毕代码了，  需要在flows中创建一个flow, 我们在创建的flow的同时会自动
//! 进行编译和部署的，同时我们还会查看编译结果

use std::collections::HashMap;

use flowsnet_platform_sdk::logger;
use serde_json::Value;
use webhook_flows::{create_endpoint, request_handler, send_response};

// flows.network部署
#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    // 这个函数是创建了flows的webhook链接
    create_endpoint().await;
}

// 下面是具体的逻辑实现, 这个处理会被调用
#[request_handler]
async fn handler(headers: Vec<(String, String)>, qry: HashMap<String, Value>, _body: Vec<u8>) {
    logger::init();
    log::info!("Headers--{:?}", header);

    // 接受消息
    let msg = qry.get("msg").unwrap();
    // 设置响应消息
    let resp = format!(
        "欢迎来到flows.network, 
    您刚才发送了一个消息:{}",
        msg
    );
    // 发送消息
    send_response(
        200,
        vec![(String::from("content-type"), String::from("text/html"))],
        resp.as_bytes().to_vec(),
    );
}
