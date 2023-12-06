use tokio::net::UdpSocket;
use std::net::{SocketAddr, ToSocketAddrs};
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
use std::thread;
use std::fs;
use std::convert::TryInto;
use tokio::io;
use std::time::Duration;
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
            //println!(" Address {}  Name {} ", add,name);
            // Parse the SocketAddr from the CSV data.
            let socket_addr = add.parse::<SocketAddr>()?;
            
            user_data.insert(name, socket_addr);
        }
    }
   // mem::drop(&file);
    Ok(())
}


fn write_file( name: &str, socket_addr: &SocketAddr) -> Result<(), Box<dyn Error>> {
    // Open the CSV file in append mode
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("users.csv")?;

    let mut write = Writer::from_writer(file);

    // Write the new entry to the CSV file
    write.write_record(&[name, &socket_addr.to_string()])?;
    write.flush()?;

    Ok(())
}


async fn send_counter(counter : i32, socket:&UdpSocket)  -> Result<(), Box<dyn std::error::Error>>
{
   // let socket = UdpSocket::bind("127.0.0.1:1017").await.expect("Failed to bind to address");
    socket.send_to(counter.to_string().as_bytes(), "127.0.0.1:1017".to_string()).await.expect("Failed to send response"); 

    socket.send_to(counter.to_string().as_bytes(), "127.0.0.1:1007".to_string()).await.expect("Failed to send response");

    Ok(())

}

async fn send_image_over_udp(
    image_path: &str,
    server_ip: &str,
    socket: &UdpSocket,
) -> Result<(), Box<dyn std::error::Error>> {
    // Read the image file asynchronously
    let image_data = fs::read(image_path)?;

    // Get the size of the image
    let expected_image_size = image_data.len() as u64;

    // Send the expected image size to the server
    socket.send_to(&expected_image_size.to_ne_bytes(), server_ip).await?;

    // Define the maximum size of each fragment
    let max_fragment_size = 1024; // Adjust this according to your needs

    println!("Expected sent image size: {} bytes",expected_image_size);
    // Split the image data into smaller fragments and send them
    for chunk in image_data.chunks(max_fragment_size) {
       // println!("sent");
         socket.send_to(chunk, server_ip).await.expect("Failed to receive data");
    }

    println!("Image sent successfully!");

    Ok(())
}
static mut servers: i32 = 3;
static mut counter1: i32 =0;
static mut request_num:i32=0;
static mut turn:i32 = 2;
static  mut flag1:i32=0;
static mut flag2:i32=0;
static mut flag3:i32 = 0;
static mut down_flag:i32 = 0;

 async fn handle_udp_socket(bind_address: &str) {
    let socket = UdpSocket::bind(bind_address).await.expect("Failed to bind to address");
    println!("Bound to {}", bind_address);
  
    let mut buffer_Size=0;

    //let mut servers =3;
  
    let mut users_data:HashMap<String, SocketAddr> = HashMap::new();
    
    // Initialize a queue to buffer incoming requests
    let mut request_queue: Vec<(String, String)> = Vec::new();
   
    loop {
   
        //println!("{} ",bind_address);
        
        /*let (num_bytes, sender) = socket.recv_from(&mut buf).await.expect("Failed to receive data");

        let data = &buf[..num_bytes];
        let data_string = String::from_utf8_lossy(data).to_string(); // Convert the data to a String

        let mut flag3=0;*/
        // Store the received data in the HashMap with the sender's address as the key
        
        unsafe{
        // Process the received data based on the receiving port
        match bind_address {
           
            "127.0.0.1:1022" => {
             
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
                //socket.send_to(result.as_bytes(), ip).await.expect("Failed to send response");

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
                             //users_data.insert(data_string, sender);
                             write_file(&data_string, &sender);
                             //let socket2 = UdpSocket::bind("127.0.0.1:1019").await.expect("Failed to bind to address");
                             read_file(&mut users_data);
                             socket.send_to(data_string.as_bytes(), "127.0.0.1:1015".to_string()).await.expect("Failed to send response");
    
                             socket.send_to(sender.to_string().as_bytes(), "127.0.0.1:1015".to_string()).await.expect("Failed to send response");
    
                         
                             socket.send_to(data_string.as_bytes(), "127.0.0.1:1005".to_string()).await.expect("Failed to send response");
    
                             socket.send_to(sender.to_string().as_bytes(), "127.0.0.1:1005".to_string()).await.expect("Failed to send response");
    
                            
                           
                            flagg=1;
                            socket.send_to("Name Registered".to_string().as_bytes(), sender).await.expect("Failed to send response");
    
                  }
        
                    }
        
                }
                else {
                  
                    socket.send_to("Name Registered".to_string().as_bytes(), sender).await.expect("Failed to send response");
                    println!("{} is registered as {} ", sender, data_string);
                    //users_data.insert(data_string, sender);
                    write_file(&data_string, &sender);
                    //let socket2 = UdpSocket::bind("127.0.0.1:1009").await.expect("Failed to bind to address");
                    read_file(&mut users_data);
                    socket.send_to(data_string.as_bytes(), "127.0.0.1:1015".to_string()).await.expect("Failed to send response");
    
                    socket.send_to(sender.to_string().as_bytes(), "127.0.0.1:1015".to_string()).await.expect("Failed to send response");
    
                         
                    socket.send_to(data_string.as_bytes(), "127.0.0.1:1005".to_string()).await.expect("Failed to send response");
    
                    socket.send_to(sender.to_string().as_bytes(), "127.0.0.1:1005".to_string()).await.expect("Failed to send response");
    
                }
        
            }
            flag1=0;
   
        }
            "127.0.0.1:1023" => {
        
                let mut buf = [0; 1024];
                let (num_bytes, sender) = socket.recv_from(&mut buf).await.expect("Failed to receive data");
        
                let data = &buf[..num_bytes];
                let ip = String::from_utf8_lossy(data).to_string(); // Convert the data to a String
                let add = ip.parse::<SocketAddr>().expect("Failed to parse address");
                //let mut buffer = vec![0; 1024]; // Adjust the buffer size to match the maximum fragment size
                
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
              
            
            "127.0.0.1:1024" => {
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
                    socket.send_to("i am down".to_string().as_bytes(), "127.0.0.1:1014").await.expect("Failed to send response");
                    let sleep_duration = Duration::from_secs(15);
                    thread::sleep(sleep_duration);
                    socket.send_to("i am back".to_string().as_bytes(), "127.0.0.1:1004").await.expect("Failed to send response");
                    socket.send_to("i am back".to_string().as_bytes(), "127.0.0.1:1014").await.expect("Failed to send response");
                    unsafe{counter1 =0;}
                    down_flag = 0;
                    let mut file = File::create("users.csv").expect(" ");

                    // Receive data and write it to the file
                    let mut buffer = [0; 1024];
                    let mut total_received = 0;
                
                    loop {
                        match socket.recv_from(&mut buffer).await {
                            Ok((bytes_received, _)) => {
                                if bytes_received == 0 {
                                    // Sender has finished sending
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
                    println!("another server is down");
                    unsafe{ servers=2;}
                    unsafe{counter1 =0;}
                    //unsafe{ servers=2;}
                  
                  unsafe{turn -=1;}
                    
                    println!(" my turn: {} ", turn);
                    //socket.send_to("i am down".to_string().as_bytes(), "127.0.0.1:1014").await.expect("Failed to send response");
                    //socket.send_to("i am down".to_string().as_bytes(), "127.0.0.1:1024").await.expect("Failed to send response");

                }
                if( mess== "i am back")
                {
                    println!("another server is back");
                    unsafe{ servers=3;}
                    unsafe{counter1 =0;}
                    println!(" my turn: {} ", turn);
                    read_file(&mut users_data);
                    //socket.send
                   
                    let mut file = File::open("users.csv").expect(" ");

                
                    // Read the file into a buffer and send it over UDP
                    let mut buffer = [0; 1024];
                    let mut total_sent = 0;
                
                    while let Ok(bytes_read) = file.read(&mut buffer) {
                        if bytes_read == 0 {
                            // Reached the end of the file
                            break;
                        }
                
                        let _ = socket.send_to(&buffer[..bytes_read], sender).await.expect(" ");
                        total_sent += bytes_read;
                    }

                }
                
                
            }
            "127.0.0.1:1025" => {
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
            "127.0.0.1:1026" => {
                let mut buf = [0; 1024];
                let (num_bytes, sender) = socket.recv_from(&mut buf).await.expect("Failed to receive data");
        
                let data = &buf[..num_bytes];
                let ip = String::from_utf8_lossy(data).to_string(); // Convert the data to a String

                socket.send_to("ACK".to_string().as_bytes(), ip).await.expect("Failed to send response");

                let mut received_data = Vec::new(); // Store received fragments
                let mut buf2 = [0; 1024];
                let (num_bytes, sender) = socket.recv_from(&mut buf2).await.expect("Failed to receive data");
                // Receive the expected image size
                //let (bytes_received, _) = socket.recv_from(&mut buffer).await.expect("Failed to receive data");
                let expected_image_size = u64::from_ne_bytes(buf2[..num_bytes].try_into().unwrap());
                
          
            
                println!("Expected image size: {} bytes", expected_image_size);
                let mut buffer = vec![0; 1024];
                while let Ok(bytes) = socket.recv(&mut buffer).await {
                    // Process the received fragment
                    received_data.extend_from_slice(&buffer[..bytes]);
            
                    // Check if the entire image has been received
                    if received_data.len() as u64 == expected_image_size {
                        // Save the received data as an image file
                        fs::write("received_image.jpg", &received_data);
            
                        println!("Received image and saved it as 'received_image.jpg'");
                       /* let imagePath = "received_image.jpg".to_string();
                        let inputImage = steganography::util::file_as_dynamic_image(imagePath);
                        let inputImageBytes = image_to_bytes(&inputImage);
                         // Create a UDP socket
                         
                        let coverPath = "src/low.jpg".to_string();
                        let coverImage = steganography::util::file_as_dynamic_image(coverPath);
                       
                        let encoder = Encoder::new(&inputImageBytes, coverImage);
                     
                        let alpha_encoded_image = encoder.encode_alpha();
                      
                        let outputPath = "test.jpg".to_string();
                        let outputPath2 = outputPath.clone(); 
                        steganography::util::save_image_buffer(alpha_encoded_image,outputPath);
                     
                        // let outputPath2 = "decoded.png".to_string();
                        let encoded_image = steganography::util::file_as_image_buffer(outputPath2);
                     */
                        //let file_path = "received_image.jpg";
                        let imagePath = "received_image.jpg".to_string();
                        let inputImage = steganography::util::file_as_dynamic_image(imagePath);
                        let inputImageBytes = image_to_bytes(&inputImage);
                         // Create a UDP socket
                         
                        let coverPath = "src/low.jpg".to_string();
                        let coverImage = steganography::util::file_as_dynamic_image(coverPath);
                       
                        let encoder = Encoder::new(&inputImageBytes, coverImage);
                     
                        let alpha_encoded_image = encoder.encode_alpha();
                      
                        let outputPath = "output.png".to_string();
                        let outputPath2 = outputPath.clone(); 
                        steganography::util::save_image_buffer(alpha_encoded_image,outputPath);
                     
                        // let outputPath2 = "decoded.png".to_string();
                        let encoded_image = steganography::util::file_as_image_buffer(outputPath2);
                     
                        let file_path = "output.png";
                        send_image_over_udp(file_path,&sender.to_string(),&socket).await.expect("msg");
                        // Read the file asynchronously
                        /*
                        let image_data = fs::read(file_path).await?;
                         let image_data = fs::read("output.png").await.expect("Failed to receive data");
                         let expected_image_size = image_data.len() as u64; // Get the size of the image
                         // Send the expected image size to the server
                         socket.send_to(&expected_image_size.to_ne_bytes(), server_ip)?;
                     
                         // Define the maximum size of each fragment
                         let max_fragment_size = 1024; // Adjust this according to your needs
                     
                         // Split the image data into smaller fragments and send them
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
         
        
            
            
           
        "127.0.0.1:1027" => {
            let mut buf = [0; 1024];

            let (num_bytes, sender) = socket.recv_from(&mut buf).await.expect("Failed to receive data");
            let data = &buf[..num_bytes];
            let data_string = String::from_utf8_lossy(data).to_string(); // Convert the data to a String
            //println!("{}", data_string);
            let count=data_string.parse().unwrap();
            unsafe{ counter1 = count;
                println! ("counter  {} ", counter1);
                }          
            
        }

        "127.0.0.1:1028"=> {
            let mut buf = [0; 1024];
            let (num_bytes, sender) = socket.recv_from(&mut buf).await.expect("Failed to receive data");
            let data = &buf[..num_bytes];
            let data_string = String::from_utf8_lossy(data).to_string(); // Convert the data to a String
            request_num+=1;
            let mut mes = data_string.clone();
            let mut addr_str = sender.to_string();
            unsafe{ counter1 = counter1 % servers;
                println! ("counter  {} ", counter1);}
            if(counter1 == turn && down_flag == 0){
            counter1+=1;
            send_counter(counter1, &socket).await.expect("could not update counter");            //If the server is not busy processing another request, send the data immediately
            if flag1 == 0 && data_string == "Register"{ 
                socket.send_to("ACK".as_bytes(), sender).await.expect("Failed to send response");
                socket.send_to(addr_str.as_bytes(), "127.0.0.1:1022".to_string()).await.expect("Failed to send response");;
                flag1 = 1;
                println!("sent to register {} ", addr_str);
            }
            else if flag2 == 0 && data_string == "Ask"{ 
                socket.send_to("ACK".as_bytes(), sender).await.expect("Failed to send response");
                socket.send_to(addr_str.as_bytes(), "127.0.0.1:1023".to_string()).await.expect("Failed to send response");;
                flag2 = 1;
            }
            else if flag3 == 0 && data_string == "Encrypt"{ 
                socket.send_to("ACK".as_bytes(), sender).await.expect("Failed to send response");
                socket.send_to(addr_str.as_bytes(), "127.0.0.1:1026".to_string()).await.expect("Failed to send response");;
                flag3 = 1; //unsetting where??
            }
            else{ // Otherwise, push the data to the queue
                request_queue.push((data_string, sender.to_string()));
                //buffer_Size+=1;
            }
          
            if mes == "Available"{ //server sent it is available
                let addr: &str = addr_str.as_str();
                match addr {
                    "127.0.0.1:1022" => { //Register
                        let mut i = 0;
                        let mut handled_once = 0;
                        while i != request_queue.len() {
                            if request_queue[i].0.as_str() == "Register" {
                                socket.send_to(request_queue[i].1.as_bytes(), "127.0.0.1:1022".to_string()).await.expect("Failed to send response");;
                                request_queue.remove(i);
                                handled_once = 1;
                                break;
                            } else {
                                i += 1;
                            }
                        }
                        //If we go through the entire array and no match, set the busy flag to 0
                        if handled_once == 0 {
                            flag1 = 0;
                        }
                    }

                    "127.0.0.1:1023" => { //Ask
                        let mut i = 0;
                        let mut handled_once_buf2 = 0;
                        while i != request_queue.len() {
                            if request_queue[i].0.as_str() == "Ask" {
                                socket.send_to(request_queue[i].1.as_bytes(), "127.0.0.1:1023".to_string()).await.expect("Failed to send response");;
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
                    "127.0.0.1:1026" => { //Encrypt
                        let mut i = 0;
                        let mut handled_once_buf3 = 0;
                        while i != request_queue.len() {
                            if request_queue[i].0.as_str() == "Encrypt" {
                                socket.send_to(request_queue[i].1.as_bytes(), "127.0.0.1:1026".to_string()).await.expect("Failed to send response");;
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
                        // Handle data for unknown ports
                    }
            }
           
            
            

            // Then, when you're ready to process a request, pop it from the queue

        //     if let Some((num_bytes, sender, data)) = request_queue.pop_front() {
        //         let data_string = String::from_utf8_lossy(&data).to_string();
        // }
    }
}
        }
            _ => {
                println!("Received data on an unknown port");
                // Handle data for unknown ports
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
   let mut counter =0;
  // let mut turn =0;

    
    let ports_to_bind = vec!["127.0.0.1:1022","127.0.0.1:1023","127.0.0.1:1024","127.0.0.1:1025","127.0.0.1:1026","127.0.0.1:1027","127.0.0.1:1028" ];
    //let mut Namedirectory: Vec<String> = vec![];
    //let mut Add_directory: Vec<SocketAddr> = vec![];
  
    let mut handles = vec![];

    for &bind_address in &ports_to_bind {
  

        counter = counter+1;
    let counter = counter %3;

        //println!(" I handled This as the counter is: {}", counter);
        let handle = tokio::spawn(handle_udp_socket(bind_address));
        handles.push(handle);
    
       
    }

    for handle in handles {
        println!("here");
        handle.await.expect("Task panicked");
    }
}
    // At this point, you can access the received data stored in the 'data_store' HashMap and use it as needed.





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

        // Search values that are greater than val - to right of current mid_index
        if val < target_value {
            low = mid + 1;
        }

        // Search values that are less than val - to the left of current mid_index
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
    //name.replace("\r\n", "");
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
