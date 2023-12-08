use tokio::net::UdpSocket;
use std::net::{SocketAddr, ToSocketAddrs};
use std::str::FromStr;
use std::vec;
use std::collections::HashMap;
use std::process::exit;
use std::sync::Arc;
extern crate csv;
use csv::Writer;
use csv::ReaderBuilder;
use std::error::Error;
use std::fs::OpenOptions;
use std::mem;
use image::{ImageBuffer, DynamicImage, Rgba};
use steganography::encoder::Encoder;
use steganography::decoder::Decoder;
use std::time::Duration;
use std::fs;
use std::convert::TryInto;
use tokio::io;
use std::thread;
use std::io::prelude::*;
use std::fs::File;


async fn read_image_as_bytes(image_path: &str) -> Result<Vec<u8>, io::Error> {
    tokio::fs::read(image_path).await
}
fn image_to_bytes(img: &DynamicImage) -> Vec<u8> {
    let mut bytes = Vec::new();
    img.write_to(&mut bytes, image::ImageOutputFormat::PNG).unwrap();
    bytes
 }
 
 fn bytes_to_image(bytes: Vec<u8>) -> DynamicImage {
    image::load_from_memory(&bytes).unwrap()
 }

fn read_file(user_data: &mut HashMap<String, SocketAddr> ) -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::open("users.csv")?;
    let mut read = ReaderBuilder::new().from_reader(file);

    for result in read.records() {
        let record = result?;
        if record.len() >= 2 {
            let name = record[0].to_string();
            let add = record[1].to_string();


            let socket_addr = add.parse::<SocketAddr>()?;
            
            user_data.insert(name, socket_addr);
        }
    }

    Ok(())
}


fn write_file( name: &str, socket_addr: &SocketAddr) -> Result<(), Box<dyn Error>> {

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("users.csv")?;

    let mut write = Writer::from_writer(file);


    write.write_record(&[name, &socket_addr.to_string()])?;
    write.flush()?;

    Ok(())
}


async fn send_counter(counter : i32, socket:&UdpSocket)  -> Result<(), Box<dyn std::error::Error>>
{

    socket.send_to(counter.to_string().as_bytes(), "127.0.0.1:1027".to_string()).await.expect("Failed to send response"); 

    socket.send_to(counter.to_string().as_bytes(), "127.0.0.1:1007".to_string()).await.expect("Failed to send response");

    Ok(())

}


async fn send_image_over_udp(
    image_path: &str,
    server_ip: &str,
    socket: &UdpSocket,
) -> Result<(), Box<dyn std::error::Error>> {

    let image_data = fs::read(image_path)?;


    let expected_image_size = image_data.len() as u64;


    socket.send_to(&expected_image_size.to_ne_bytes(), server_ip).await?;


    let max_fragment_size = 1024; // Adjust this according to your needs

    println!("Expected sent image size: {} bytes",expected_image_size);

    for chunk in image_data.chunks(max_fragment_size) {

         socket.send_to(chunk, server_ip).await.expect("Failed to receive data");
    }

    println!("Image sent successfully!");

    Ok(())
}
static mut servers: i32 = 3;
static mut counter1: i32 =0;
static mut request_num:i32=0;
static mut turn:i32 = 1;
static  mut flag1:i32=0;
static mut flag2:i32=0;
static mut flag3:i32 = 0;
static mut down_flag:i32 = 0;

 async fn handle_udp_socket(bind_address: &str) {
    let socket = UdpSocket::bind(bind_address).await.expect("Failed to bind to address");
    println!("Bound to {}", bind_address);

    let mut buffer_Size=0;
    let mut access: Vec<(String,i32,String)> = Vec::new();

 
    let mut users_data:HashMap<String, SocketAddr> = HashMap::new();
    

    let mut request_queue: Vec<(String, String)> = Vec::new();
   
    loop {
   

        
        /*let (num_bytes, sender) = socket.recv_from(&mut buf).await.expect("Failed to receive data");

        let data = &buf[..num_bytes];
        let data_string = String::from_utf8_lossy(data).to_string(); // Convert the data to a String

        let mut flag3=0;*/

        
        unsafe{

        match bind_address {
           
            "127.0.0.1:1012" => {
             
            let mut buf = [0; 1024];
            let (num_bytes, sender) = socket.recv_from(&mut buf).await.expect("Failed to receive data");
            println!("received");
            let data = &buf[..num_bytes];
            let ip = String::from_utf8_lossy(data).to_string(); // Convert the data to a String
           
            println!("Received message: {}", ip);
               
            let add = ip.parse::<SocketAddr>().expect("Failed to parse address");

            let mut flagg=0;
         
            socket.send_to("ACK".as_bytes(), add).await.expect("Failed to send response");
            println!("connected address {}",ip);
            read_file(&mut users_data);
            if let Some(result) = Add_Search(&add, &users_data) {
                println!("Name already registered as: {}", result);
                socket.send_to("Name Registered".to_string().as_bytes(), ip).await.expect("Failed to send response");


            } else {
                println!("Address not registered.");
                socket.send_to("Address not registered.".to_string().as_bytes(), ip).await.expect("Failed to send response");
                let (num_bytes, sender) = socket.recv_from(&mut buf).await.expect("Failed to receive data");
                let data = &buf[..num_bytes];
                let data_string = String::from_utf8_lossy(data).to_string(); // Convert the data to a String
                if let Some(result) = Name_Search(& data_string.clone(), &users_data)
                {
                    socket.send_to("Name is taken".to_string().as_bytes(), sender).await.expect("Failed to send response");

                    println!("Name is taken");
                    while flagg==0 {
                        let (num_bytes, sender) = socket.recv_from(&mut buf).await.expect("Failed to receive data");
                        let data = &buf[..num_bytes];
                        let data_string = String::from_utf8_lossy(data).to_string(); // Convert the data to a String

                        if let Some(result) = Name_Search(& data_string.clone(), &users_data)
                        {
                            socket.send_to("Name is taken".to_string().as_bytes(), sender).await.expect("Failed to send response");
                            println!("Name is taken");
                        }
                        else{
                             println!("Registered {}", data_string);

                             write_file(&data_string, &sender);

                             read_file(&mut users_data);
                             socket.send_to(data_string.as_bytes(), "127.0.0.1:1025".to_string()).await.expect("Failed to send response");
    
                             socket.send_to(sender.to_string().as_bytes(), "127.0.0.1:1025".to_string()).await.expect("Failed to send response");
    
                         
                             socket.send_to(data_string.as_bytes(), "127.0.0.1:1005".to_string()).await.expect("Failed to send response");
    
                             socket.send_to(sender.to_string().as_bytes(), "127.0.0.1:1005".to_string()).await.expect("Failed to send response");
    
                            
                           
                            flagg=1;
                            socket.send_to("Name Registered".to_string().as_bytes(), sender).await.expect("Failed to send response");
    
                  }
        
                    }
        
                }
                else {
                  
                    //socket.send_to("Name Registered".to_string().as_bytes(), sender).await.expect("Failed to send response");
                    println!("{} is registered as {} ", sender, data_string);

                    write_file(&data_string, &sender);

                    read_file(&mut users_data);
                    socket.send_to(data_string.as_bytes(), "127.0.0.1:1025".to_string()).await.expect("Failed to send response");
    
                    socket.send_to(sender.to_string().as_bytes(), "127.0.0.1:1025".to_string()).await.expect("Failed to send response");
    
                         
                    socket.send_to(data_string.as_bytes(), "127.0.0.1:1005".to_string()).await.expect("Failed to send response");
    
                    socket.send_to(sender.to_string().as_bytes(), "127.0.0.1:1005".to_string()).await.expect("Failed to send response");
    
                }
        
            }
            flag1=0;
   
        }
            "127.0.0.1:1013" => {
            
                let mut buf = [0; 1024];
                let (num_bytes, sender) = socket.recv_from(&mut buf).await.expect("Failed to receive data");
        
                let data = &buf[..num_bytes];
                let ip = String::from_utf8_lossy(data).to_string(); // Convert the data to a String
                let add = ip.parse::<SocketAddr>().expect("Failed to parse address");

                
                socket.send_to("ACK".to_string().as_bytes(), add).await.expect("Failed to send response");
                

          
         

                 
                



               let mut flag =0;
          
               read_file(&mut users_data);
               while flag ==0{
                let (num_bytes, sender) = socket.recv_from(&mut buf).await.expect("Failed to receive data");

                let data = &buf[..num_bytes];
                let data_string = String::from_utf8_lossy(data).to_string(); // Convert the data to a String
                if let Some(add) = Name_Search(&data_string.to_string(), &users_data)
                {
                    println!("Address = {:?}", add);
                    println!("sender address: {:?}",sender);
                    socket.send_to(add.to_string().as_bytes(), sender).await.expect("Failed to send response");
                    let flag =1;
                }
                else{
                    let mut notf = "Name not found";
                    println!("{}",notf);
                    println!("sender address: {:?}",sender);
                    socket.send_to(notf.to_string().as_bytes(), sender).await.expect("Failed to send response");
            
                }
               }
             
         flag2=0;
        }
              
            
            "127.0.0.1:1014" => {
                let mut buf = [0; 1024];
                let (num_bytes, sender) = socket.recv_from(&mut buf).await.expect("Failed to receive data");
                let data = &buf[..num_bytes];
                let mess = String::from_utf8_lossy(data).to_string(); 
                if(mess =="count_handled_requests" )
                {
                    
                    socket.send_to(request_num.to_string().as_bytes(), sender).await.expect("Failed to send response");

                }
                if(mess =="read_buffer_size" )
                {
                    let buff = request_queue.len();
                    socket.send_to(buff.to_string().as_bytes(), sender).await.expect("Failed to send response");

                }
                if(mess =="pew pew" )
                {
                 
                  
                     down_flag = 1;
                    socket.send_to("i am down".to_string().as_bytes(), "127.0.0.1:1004").await.expect("Failed to send response");
                    socket.send_to("i am down".to_string().as_bytes(), "127.0.0.1:1024").await.expect("Failed to send response");
                    let sleep_duration = Duration::from_secs(15);
                    thread::sleep(sleep_duration);
                    socket.send_to("i am back".to_string().as_bytes(), "127.0.0.1:1004").await.expect("Failed to send response");
                    socket.send_to("i am back".to_string().as_bytes(), "127.0.0.1:1024").await.expect("Failed to send response");
                    unsafe{turn = 2;}
                    unsafe{counter1 =0;}
                    down_flag = 0;
                    let mut file = File::create("users.csv").expect(" ");


                    let mut buffer = [0; 1024];
                    let mut total_received = 0;
                
                    loop {
                        match socket.recv_from(&mut buffer).await {
                            Ok((bytes_received, _)) => {
                                if bytes_received == 0 {

                                    break;
                                }
                                file.write_all(&buffer[..bytes_received]);
                                total_received += bytes_received;
                            }
                            Err(err) => {
                                eprintln!("Error receiving data: {}", err);
                                break;
                            }
                        }
                    }

                }
                if( mess== "i am down")
                {
                    unsafe{ servers=2;}
                    unsafe{counter1 =0;}

                    let temp_sender = "127.0.0.1:1004".to_string();
                    let sock_add = temp_sender.parse::<SocketAddr>().unwrap();

                    if(sender == sock_add)
                    {
                        unsafe{turn = 0;}
                    }


                    println!(" my turn: {} ", turn);
                }
                if( mess== "i am back")
                {
                    unsafe{ servers=3;}
                    unsafe{counter1 =0;}
                    println!(" my turn: {} ", turn);
                    read_file(&mut users_data);

                   
                    let mut file = File::open("users.csv").expect(" ");

                

                    let mut buffer = [0; 1024];
                    let mut total_sent = 0;
                
                    while let Ok(bytes_read) = file.read(&mut buffer) {
                        if bytes_read == 0 {

                            break;
                        }
                
                        let _ = socket.send_to(&buffer[..bytes_read], sender).await.expect(" ");
                        total_sent += bytes_read;
                    }

                }
                
                
            }
            "127.0.0.1:1015" => {
                let mut buf = [0; 1024];
                let (num_bytes, sender) = socket.recv_from(&mut buf).await.expect("Failed to receive data");
                let data = &buf[..num_bytes];
                let name = String::from_utf8_lossy(data).to_string(); // Convert the data to a String
                
                let mut buf2 = [0; 1024];
                let (num_bytes, sender) = socket.recv_from(&mut buf2).await.expect("Failed to receive data");
                let data2 = &buf2[..num_bytes];
                let address = String::from_utf8_lossy(data2).to_string(); // Convert the data to a String
                let sock_add = address.parse::<SocketAddr>().unwrap();
                write_file(&name, &sock_add);
                read_file(&mut users_data);
                
            }
            "127.0.0.1:1016" => {
                let mut buf = [0; 1024];
                let (num_bytes, sender) = socket.recv_from(&mut buf).await.expect("Failed to receive data");
        
                let data = &buf[..num_bytes];
                let ip = String::from_utf8_lossy(data).to_string(); // Convert the data to a String

                socket.send_to("ACK".to_string().as_bytes(), ip).await.expect("Failed to send response");

                let mut received_data = Vec::new(); // Store received fragments
                let mut buf2 = [0; 1024];
                let (num_bytes, sender) = socket.recv_from(&mut buf2).await.expect("Failed to receive data");


                let expected_image_size = u64::from_ne_bytes(buf2[..num_bytes].try_into().unwrap());
                
          
            
                println!("Expected image size: {} bytes", expected_image_size);
                let mut buffer = vec![0; 1024];
                while let Ok(bytes) = socket.recv(&mut buffer).await {

                    received_data.extend_from_slice(&buffer[..bytes]);
            

                    if received_data.len() as u64 == expected_image_size {

                        fs::write("received_image.jpg", &received_data);
            
                        println!("Received image and saved it as 'received_image.jpg'");
                       /* let imagePath = "received_image.jpg".to_string();
                        let inputImage = steganography::util::file_as_dynamic_image(imagePath);
                        let inputImageBytes = image_to_bytes(&inputImage);

                         
                        let coverPath = "src/low.jpg".to_string();
                        let coverImage = steganography::util::file_as_dynamic_image(coverPath);
                       
                        let encoder = Encoder::new(&inputImageBytes, coverImage);
                     
                        let alpha_encoded_image = encoder.encode_alpha();
                      
                        let outputPath = "test.jpg".to_string();
                        let outputPath2 = outputPath.clone(); 
                        steganography::util::save_image_buffer(alpha_encoded_image,outputPath);
                     

                        let encoded_image = steganography::util::file_as_image_buffer(outputPath2);
                     */

                        let imagePath = "received_image.jpg".to_string();
                        let inputImage = steganography::util::file_as_dynamic_image(imagePath);
                        let inputImageBytes = image_to_bytes(&inputImage);

                         
                        let coverPath = "src/low.jpg".to_string();
                        let coverImage = steganography::util::file_as_dynamic_image(coverPath);
                       
                        let encoder = Encoder::new(&inputImageBytes, coverImage);
                     
                        let alpha_encoded_image = encoder.encode_alpha();
                      
                        let outputPath = "output.png".to_string();
                        let outputPath2 = outputPath.clone(); 
                        steganography::util::save_image_buffer(alpha_encoded_image,outputPath);
                     

                        let encoded_image = steganography::util::file_as_image_buffer(outputPath2);
                     
                        let file_path = "output.png";
                        print!("reached");
                        send_image_over_udp(file_path,&sender.to_string(),&socket).await.expect("msg");

                        /*
                        let image_data = fs::read(file_path).await?;
                         let image_data = fs::read("output.png").await.expect("Failed to receive data");
                         let expected_image_size = image_data.len() as u64; // Get the size of the image

                         socket.send_to(&expected_image_size.to_ne_bytes(), server_ip)?;
                     

                         let max_fragment_size = 1024; // Adjust this according to your needs
                     

                         for chunk in image_data.chunks(max_fragment_size) {
                             socket.send_to(chunk,  server_ip)?;
                         }
                     
                         println!("Image sent successfully!");
                     */
                        break; // Exit the loop once the entire image is received
                    }
                } 
               
       flag3=0;
            }
         
        
            
           
        "127.0.0.1:1017" => {
            let mut buf = [0; 1024];

            let (num_bytes, sender) = socket.recv_from(&mut buf).await.expect("Failed to receive data");
            let data = &buf[..num_bytes];
            let data_string = String::from_utf8_lossy(data).to_string(); // Convert the data to a String

            let count=data_string.parse().unwrap();
           counter1 = count;
              
                         
            
        }

        "127.0.0.1:1018"=> {
            let mut buf = [0; 1024];
            let (num_bytes, sender) = socket.recv_from(&mut buf).await.expect("Failed to receive data");
            let data = &buf[..num_bytes];
            let data_string = String::from_utf8_lossy(data).to_string(); // Convert the data to a String
            println!("{}",data_string);
            if data_string == "update"
            {
                let mut buf2 = [0; 1024];
                let (num_bytes2, sender) = socket.recv_from(&mut buf2).await.expect("Failed to receive data");
                let data2 = &buf2[..num_bytes2];
                let path = String::from_utf8_lossy(data2).to_string(); // Convert the data to a String

                let mut buf2 = [0; 1024];
                let (num_bytes2, sender) = socket.recv_from(&mut buf2).await.expect("Failed to receive data");
                let data2 = &buf2[..num_bytes2];
                let accessnum = i32::from_ne_bytes(buf2[..num_bytes2].try_into().unwrap());
                println!("new access: {}", accessnum);
                let mut buf2 = [0; 1024];
                let (num_bytes2, sender) = socket.recv_from(&mut buf2).await.expect("Failed to receive data");
                let data2 = &buf2[..num_bytes2];
                let addrto = String::from_utf8_lossy(data2).to_string(); // Convert the data to a String
                access.push((path,accessnum,addrto));
                println!("updated");

            }
            request_num+=1;
            println!("requests = {}",request_num );
            let mut mes = data_string.clone();
            let mut addr_str = sender.to_string();
            counter1 = counter1 % servers;
      
            if(counter1 == turn && down_flag == 0){
                println! ("counter  {} turn {}", counter1,turn);
            counter1+=1;
            
            send_counter(counter1, &socket).await.expect("could not update counter");       
           
        
            if flag1 == 0 && data_string =="Register" {
                let mut impath: String ;
                let mut acc: i32;
              
                if let Some(index) = access.iter().position(|&(_,_, ref ad)| ad == &sender.to_string()) {
                    impath = access[index].0.to_string();
                    acc = access[index].1;
                    access.remove(index);
                 
                    
                    println!("The vector contains a tuple with the role '{}'", sender);
                    socket.send_to(impath.as_bytes(), sender).await.expect("Failed to send response");
                    socket.send_to(&acc.to_ne_bytes(), sender).await.expect("Failed to send response");
                    socket.send_to(addr_str.as_bytes(), "127.0.0.1:1002".to_string()).await.expect("Failed to send response");;
                    flag1 = 1;
                    println!("sent to register {} ", addr_str);

                } else {
                    socket.send_to("Ack".as_bytes(), sender).await.expect("Failed to send response");
                  
                    socket.send_to(addr_str.as_bytes(), "127.0.0.1:1002".to_string()).await.expect("Failed to send response");;
                    flag1 = 1;
                    println!("sent to register {} ", addr_str);
                }
                              

            } 
        
        
            else if flag2 == 0 && data_string == "Ask"{ 
                socket.send_to("ACK".as_bytes(), sender).await.expect("Failed to send response");
                socket.send_to(addr_str.as_bytes(), "127.0.0.1:1013".to_string()).await.expect("Failed to send response");;
                flag2 = 1;
            }
            else if flag3 == 0 && data_string == "Encrypt"{ 
                socket.send_to("ACK".as_bytes(), sender).await.expect("Failed to send response");
                socket.send_to(addr_str.as_bytes(), "127.0.0.1:1016".to_string()).await.expect("Failed to send response");;
                flag3 = 1; //unsetting where??
            }
            else{ // Otherwise, push the data to the queue
                request_queue.push((data_string, sender.to_string()));

            }
          
            if mes == "Available"{ //server sent it is available
                let addr: &str = addr_str.as_str();
                match addr {
                    "127.0.0.1:1012" => { //Register
                        let mut i = 0;
                        let mut handled_once = 0;
                        while i != request_queue.len() {
                            if request_queue[i].0.as_str() == "Register" {
                                socket.send_to(request_queue[i].1.as_bytes(), "127.0.0.1:1012".to_string()).await.expect("Failed to send response");;
                                request_queue.remove(i);
                                handled_once = 1;
                                break;
                            } else {
                                i += 1;
                            }
                        }

                        if handled_once == 0 {
                            flag1 = 0;
                        }
                    }

                    "127.0.0.1:1013" => { //Ask
                        let mut i = 0;
                        let mut handled_once_buf2 = 0;
                        while i != request_queue.len() {
                            if request_queue[i].0.as_str() == "Ask" {
                                socket.send_to(request_queue[i].1.as_bytes(), "127.0.0.1:1013".to_string()).await.expect("Failed to send response");;
                                request_queue.remove(i);
                                handled_once_buf2 = 1;
                                break;
                            } else {
                                i += 1;
                            }
                        }
                        if handled_once_buf2 == 0 {
                            flag2 = 0;
                        }
                    }
                    "127.0.0.1:1016" => { //Encrypt
                        let mut i = 0;
                        let mut handled_once_buf3 = 0;
                        while i != request_queue.len() {
                            if request_queue[i].0.as_str() == "Encrypt" {
                                socket.send_to(request_queue[i].1.as_bytes(), "127.0.0.1:1016".to_string()).await.expect("Failed to send response");;
                                request_queue.remove(i);
                                handled_once_buf3 = 1;
                                break;
                            } else {
                                i += 1;
                            }
                        }
                        
                        if handled_once_buf3 == 0 {
                            flag3 = 0;
                        }
                    }
                    _ => {
                        println!("Received data on an unknown port");

                    }
            }
           
            
            






    }
}
        }
            _ => {
                println!("Received data on an unknown port");

            }
    
    
    
    
    }
    }
}

 }

#[tokio::main]
async fn main() {
    /*let socket = UdpSocket::bind("127.0.0.1:1002").await.expect("Failed to bind to address");
    println!("Bound to {}", "127.0.0.1:1002");

    let mut buf = [0; 1024];
*/


    
    let ports_to_bind = vec!["127.0.0.1:1012","127.0.0.1:1013","127.0.0.1:1014","127.0.0.1:1015","127.0.0.1:1016","127.0.0.1:1017","127.0.0.1:1018" ];


  
    let mut handles = vec![];

    for &bind_address in &ports_to_bind {
        

        let handle = tokio::spawn(handle_udp_socket(bind_address));
        handles.push(handle);
    
       
    }

    for handle in handles {
        println!("here");
        handle.await.expect("Task panicked");
    }
}






pub fn iterative_Add(a: &[SocketAddr], len: usize, target_value: SocketAddr, ite: usize) -> Option<usize> {
    let mut low: i8 = 0;
    let mut high: i8 = len as i8 - 1;

    while low <= high {
        let mid = ((high - low) / 2) + low;
        let mid_index = mid as usize;
        let val: SocketAddr = a[mid_index];

        if val == target_value {
            return Some(mid_index);
        }


        if val < target_value {
            low = mid + 1;
        }


        if val > target_value {
            high = mid - 1;
        }
    }

    None
}
pub fn get_name_index(name: &String, array: &mut Vec<String>) -> usize {
    let mut v: &mut Vec<String> =array;
    println!("{:?}", &v);

    v.sort_unstable();
    println!("{:?}", &v);
    println!("{}",name);

    println!("{}",name);
    match array.binary_search(name) {
        Ok(index) => index,
        
        Err(_) => {
            println!("Error : variable {} not found in name array", name);
            std::process::exit(1)
        }
    }
}

pub fn Name_Search(name: &String, addresses: &HashMap<String, SocketAddr>) -> Option<SocketAddr> {


    for (key,val) in addresses.into_iter() {
        println!(" Address {}  Name {} ", val,key);
        if (key == name){
                return Some(*val);
        };
        
     }
None
}

pub fn Add_Search(add: &SocketAddr, addresses: &HashMap<String, SocketAddr>) -> Option<String> {

    for (key,val) in addresses.into_iter() {
        println!(" Address {}  Name {} ", val,key);
        if (val == add){
                return Some(key.to_string());
        };
        
     }
None
}
