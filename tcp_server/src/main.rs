//引入标准库net模块的TcpListener、TcpStream方法
use std::net::{TcpListener, TcpStream};
//引入标准库io模块的Read、Write特性
use std::io::{Read,Write};
//引入标准库fs模块
use std::fs;

//定义一个handle_client函数，用于处理客户端的请求，输入为可变的TcpStream类型的流
fn handle_client(mut stream: TcpStream) {
    //初始化一个可变数组buffer，512个0
    let mut buffer = [0;512];
    //使用read方法将流写入buffer，并通过unwrap进行模式匹配
    stream.read(&mut buffer).unwrap();
    //在服务端打印转码后的buffer
    println!("Request: {}" , String::from_utf8_lossy(&buffer[..]));

    //初始化变量get，值为转为字节码的"GET / HTTP/1.1\r\n"
    let get = b"GET / HTTP/1.1\r\n";
    //条件判断，如果buffer开头和get相同
    if buffer.starts_with(get){
        //初始化变量contents，值为hello.html文件转为字符串，并通过unwrap进行模式匹配
        let contents = fs::read_to_string("hello.html").unwrap();
        //使用format宏初始化变量response，包含状态码为200的响应头和contents
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
        //使用write方法将response转为字节码写入流，并通过unwrap进行模式匹配
        stream.write(response.as_bytes()).unwrap();
        //冲刷出流,将所有的缓存数据发送到客户端，并通过unwrap进行模式匹配
        stream.flush().unwrap();
    //条件判断，如果buffer开头和get不同
    } else {
        //同上，略
        let contents = fs::read_to_string("404.html").unwrap();
        let response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}", contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    
}

//定义一个入口函数，返回值为Result类型
fn main() -> std::io::Result<()> {
    //开始监听TCP连接，bind函数返回一个Result类型的实例，返回后通过unwrap进行模式匹配
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    //使用incoming方法返回TcpStream类型的迭代器，通过for循环取出
    for stream in listener.incoming() {
        //使用handle_client函数处理流，处理前通过unwrap进行模式匹配
        handle_client(stream?);
    }
    //main函数的返回值
    Ok(())
} 