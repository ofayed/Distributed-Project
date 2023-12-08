use std::io;
use std::process::exit;
use tokio::time::{sleep, Duration};
use tokio::net::UdpSocket;
use std::net::{SocketAddr, ToSocketAddrs};
use std::process::Output;
use minifb::{Key, Window, WindowOptions, Scale, ScaleMode};
use image::{ImageBuffer, DynamicImage, Rgba, GenericImageView};
use steganography::encoder::Encoder;
use steganography::decoder::Decoder;
use std::fs;
use std::convert::TryInto;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::env;
use std::collections::HashMap;
use std::sync::Arc;
extern crate csv;
use csv::Writer;
use csv::ReaderBuilder;
use std::error::Error;

use std::io::prelude::*;

struct Picture {
    access_count: usize,
    image_data: Vec<u8>,
    encrypted_data: Vec<u8>
}
impl Picture {
    fn new(image_data: Vec<u8>, access_count: usize, encrypted_data: Vec<u8>) -> Self {
        Picture { access_count, image_data,encrypted_data }
    }
}


async fn send_message_with_retransmission(
    message: &str,
    destination: SocketAddr,
    max_retransmissions: usize,
    timeout: Duration,
    socket: &UdpSocket
) -> Result<(), Box<dyn std::error::Error>> {


    let mut retransmissions = 0;
    let mut flag =0;
    let mut images: Vec<(String, i32)> = Vec::new();
    loop {
       
    

        socket.send_to(message.as_bytes(), destination).await?;
        println!("Message sent to {}", destination);
      

        let mut buf = [0; 1024];
        let result = tokio::select! {
            res = socket.recv_from(&mut buf) => res,
            _ = sleep(timeout) => Ok((0, SocketAddr::from(([0, 0, 0, 0], 0)))),
        };

        match result {
            Ok((num_bytes, sender)) if num_bytes > 0 => {
                if String::from_utf8_lossy(&buf[..num_bytes]) != "Ack"
                {
                    let path =  String::from_utf8_lossy(&buf[..num_bytes]).to_string();
                    let mut buf1 = [0; 1024];
                    let (num_bytes2, sender) = socket.recv_from(&mut buf1).await.expect("Failed to receive data");
                    let acc = i32::from_ne_bytes(buf1[..num_bytes2].try_into().unwrap());
                    read_file(&mut images);

                    for i in 0..images.len()
                    {
                        if images[i].0 == path
                        {
                            images[i].1 = acc;
                        }
                    }
                    overwrite_file(&mut images);

                  
                }
                else{
                    let mut buf1 = [0; 1024];
                    let (num_bytes2, sender) = socket.recv_from(&mut buf1).await.expect("Failed to receive data");
                }
                println!("Received response from {}: {:?}", sender, String::from_utf8_lossy(&buf[..num_bytes]));
                println!(" entered");
               
                let mut buf2 = [0; 1024];
                let (num_bytes2, sender) = socket.recv_from(&mut buf2).await.expect("Failed to receive data");
                let data = &buf2[..num_bytes2];

                println!("Received response from {}: {:?}", sender, String::from_utf8_lossy(&buf2[..num_bytes2]));
                if String::from_utf8_lossy(&buf2[..num_bytes2]).trim().to_string() == "Name Registered".to_string() {
                    flag = 1;

                } else {
                    let destination =sender;
                    println!("Enter Your Name ");
                    let mut name = String::new();
                    io::stdin().read_line(&mut name)?;
                    socket.send_to(name.trim().as_bytes(), destination).await?;
                }
                break Ok(());
            }
            _ => {
                println!("No response, retransmitting...");
                retransmissions += 1;
                if retransmissions >= max_retransmissions {
                    break Err("Max retransmissions reached".into());


                }
            }
        }
    }
}

static mut imagcount:i32 = 0;
async fn send_message_with_retransmission2(
    message: &str,
    destination: SocketAddr,
    max_retransmissions: usize,
    timeout: Duration,
    socket: &UdpSocket
) -> Result<(), Box<dyn std::error::Error>> {

    
    let mut retransmissions = 0;
    let mut flag =0;
    loop {

        socket.send_to(message.as_bytes(), destination).await?;
        println!("Message sent to {}", destination);


        let mut buf = [0; 1024];
        let result = tokio::select! {
            res = socket.recv_from(&mut buf) => res,
            _ = sleep(timeout) => Ok((0, SocketAddr::from(([0, 0, 0, 0], 0)))),
        };

        match result {
            Ok((num_bytes, sender)) if num_bytes > 0 => {
                let mut flag = 0;
                while flag == 0 {
                
                    let mut buf2 = [0; 1024];
                    let (num_bytes, sender) = socket.recv_from(&mut buf2).await.expect("Failed to receive data");



                    
                    println!("Received Ack from {}", sender);
                    println!("Enter Receiver's Name ");
                    let mut message = String::new();
                    io::stdin().read_line(&mut message)?;
                    let destination = sender;
                    socket.send_to(message.trim().as_bytes(), destination).await?;


                    let mut buffer = [0; 1024];
                    let (num_bytes, src_addr) = socket.recv_from(&mut buffer).await?;
                    let received_message: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[0..num_bytes]);
                    println!("You received a message '{}'", received_message);

                    if received_message.to_string() == "Name not found".to_string() {
                        println!("Name not found, Try another name");
                    } else {
                        let mut add = received_message.to_string();
                        let dest_addr = add.as_str();
                        println!("Name found at address '{}' ", dest_addr);
                        socket.send_to("request".trim().as_bytes(), dest_addr).await?;
                  
                        let mut buf = [0; 1024];
                        let (num_bytes2, sender) = socket.recv_from(&mut buf).await.expect("Failed to receive data");
                        let imagnum = u64::from_ne_bytes(buf[..num_bytes2].try_into().unwrap());
            

                        for i in 0..imagnum
                        {
                            let mut received_data = Vec::new(); // Store received fragments
                            let mut buf2 = [0; 1024];
                            let (num_bytes, sender) = socket.recv_from(&mut buf2).await.expect("Failed to receive data");

                            let expected_image_size = u64::from_ne_bytes(buf2[..num_bytes].try_into().unwrap());
                            
                      
                        
                            println!("Expected image size: {} bytes", expected_image_size);
                            let mut buffer = vec![0; 1024];
                            while let Ok(bytes) = socket.recv(&mut buffer).await {

                                received_data.extend_from_slice(&buffer[..bytes]);
                        

                                if received_data.len() as u64 == expected_image_size {
                                            let mut buf2 = [0; 1024];
                                            let (num_bytes2, sender) = socket.recv_from(&mut buf2).await.expect("Failed to receive data");
                                            let data2 = &buf2[..num_bytes2];
                                            let path = String::from_utf8_lossy(data2).to_string(); // Convert the data to a String
                                            println!("Received encrypted image {} ",path);
                                            unsafe{ 
                                                

                                                let newpath = path.replace("encryption/", "");
                                                let newpath = newpath.replace(".jpeg", "");
                                                let mut combined_string = "received/".to_string() + newpath.as_str() +  ".png" ;
                                            fs::write(&combined_string, &received_data)?;
                    
                                            
                                            let mut buf = [0; 1024];
                                            let (num_bytes2, sender) = socket.recv_from(&mut buf).await.expect("Failed to receive data");
                                           
                                            let access = u64::from_ne_bytes(buf[..num_bytes2].try_into().unwrap());
                                
                                            println!("Received  image and saved it as '{}' with number of access {} ", combined_string,access);
                                            write_file(combined_string.as_str(), &(access as i32));
                                        
                                            
                                        }
                                            break; // Exit the loop once the entire image is received
                                        }
                                 
                
                                    }
                        }
            
                        flag = 1;
                    }
                }
                break Ok(());
            }
            _ => {
                println!("No response, retransmitting...");
                retransmissions += 1;
                if retransmissions >= max_retransmissions {
                    break Err("Max retransmissions reached".into());

                }
            }
        }
    }
}




async fn send_message_with_retransmission3(
    message: &str,
    destination: SocketAddr,
    max_retransmissions: usize,
    timeout: Duration,
    socket: &UdpSocket,

) -> Result<(), Box<dyn std::error::Error>> {

  


    let mut retransmissions = 0;
    let mut flag =0;
    loop {
        socket.send_to(message.as_bytes(), destination).await?;
        println!("Message sent to {}", destination);


        let mut buf = [0; 1024];
        let result = tokio::select! {
            res = socket.recv_from(&mut buf) => res,
            _ = sleep(timeout) => Ok((0, SocketAddr::from(([0, 0, 0, 0], 0)))),
        };

        match result {
            Ok((num_bytes, sender)) if num_bytes > 0 => {
             
                let data = &buf[..num_bytes];
                let data_string = String::from_utf8_lossy(data).to_string(); // Convert the data to a String
                println!("received from: {} : {}", sender, data_string);
                let mut buf2 = [0; 1024];
                let (num_bytes, sender) = socket.recv_from(&mut buf2).await.expect("Failed to receive data");
              
                
        println!("received from: {} : {}", sender, data_string);
        let dir_path = "pictures/";
        let mut images: Vec<(String)> = Vec::new();
    
        if let Ok(entries) = fs::read_dir(dir_path) {

            for entry in entries {
                if let Ok(entry) = entry {
             
                    let image_path = entry.path().display().to_string();

                  
                    let access_rights = 5; 

                
                    images.push((image_path));
                }
            }
        }

        println!("Images");
        for (index, (image_path)) in images.iter().enumerate() {
            println!("Index: {}, Image Path: {}", index, image_path);
        }

        println!("Enter the index of the image you want to send:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let index: usize = input.trim().parse().expect("Please type a number!");
        let destination = sender;

        if index < images.len() {
            let (image_path) = &images[index];
            let image_data = fs::read(image_path)?;
        
            let expected_image_size = image_data.len() as u64;

     
            socket.send_to(&expected_image_size.to_ne_bytes(), destination).await?;

            let max_fragment_size = 1024;

  
            for chunk in image_data.chunks(max_fragment_size) {
       
                socket.send_to(chunk, destination).await?;
            }
            
        let mut buffer = vec![0; 1024]; // Adjust the buffer size to match the maximum fragment size
        let mut received_data = Vec::new(); // Store received fragments
        let mut buf2 = [0; 1024];
        let (num_bytes, sender) = socket.recv_from(&mut buf2).await.expect("Failed to receive data");

        let expected_image_size = u64::from_ne_bytes(buf2[..num_bytes].try_into().unwrap());
        
  
    
        println!("Expected image size: {} bytes", expected_image_size);
        let mut buffer = vec![0; 1024];
        while let Ok(bytes) = socket.recv(&mut buffer).await {

            received_data.extend_from_slice(&buffer[..bytes]);
    

            if received_data.len() as u64 == expected_image_size {
                let newpath = image_path.replace("pictures/", "");
           
                let mut combined_string = "encryption/".to_string() + newpath.as_str() ;

                  fs::write(combined_string, &received_data)?;
            
                println!("Received image and saved it");
                break; // Exit the loop once the entire image is received
            }

            
           
        }
        } else {
            println!("Invalid index");
        }

                break Ok(());
    }
    _ => {
        println!("No response, retransmitting...");
        retransmissions += 1;
        if retransmissions >= max_retransmissions {

            break Err("Max retransmissions reached".into());

        }
    }
}

}
}
fn bytes_to_image(bytes: Vec<u8>) -> DynamicImage {
    image::load_from_memory(&bytes).unwrap()
}


async fn update_access(
    message: &str,
    destination: SocketAddr,
    socket: &UdpSocket,
    newacc: i32,
    updated: &(String, SocketAddr)
) {

        socket.send_to(message.as_bytes(), destination).await.expect("msg");
        println!("Message sent to {}", destination);

        socket.send_to(updated.0.as_bytes(), destination).await.expect("msg");
        println!("Message sent to {}", destination);
        println!("new access {}", newacc);
        socket.send_to(&newacc.to_ne_bytes(), destination).await.expect("msg");
        println!("Message sent to {}", destination);

        socket.send_to(updated.1.to_string().as_bytes(), destination).await.expect("msg");
        println!("Message sent to {}", destination);
    
      

}



async fn send_image_over_udp(
    server_ip: &str,
    socket: &UdpSocket,
) -> Result<(), Box<dyn std::error::Error>> {
    let dir_path = "encryption/";
    let mut images: Vec<(String)> = Vec::new();

    if let Ok(entries) = fs::read_dir(dir_path) {

        for entry in entries {
            if let Ok(entry) = entry {

                let image_path = entry.path().display().to_string();





                images.push((image_path));
            }
        }
    }

    println!("Images and their access rights:");
    for (index, (image_path)) in images.iter().enumerate() {
        println!("Index: {}, Image Path: {}", index, image_path);
    }


    println!("Enter the index of the image you want to send:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let index: usize = input.trim().parse().expect("Please type a number!");


    if index < images.len() {
        let (image_path) = &images[index];
        let image_data = fs::read(image_path)?;

      
        let expected_image_size = image_data.len() as u64;


        socket.send_to(&expected_image_size.to_ne_bytes(), server_ip).await?;


        let max_fragment_size = 1024; // Adjust this according to your needs


        for chunk in image_data.chunks(max_fragment_size) {

            socket.send_to(chunk, server_ip).await?;
        }

        println!("Image sent successfully!");
        println!("Enter number of times allowed to access ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let access: usize = input.trim().parse().expect("Please type a number!");
        socket.send_to(&access.to_ne_bytes(), server_ip).await?;
     
    } else {
        println!("Invalid index");
    }



    Ok(())
}

fn read_file(user_data: &mut Vec<(String, i32)> ) -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::open("access.csv")?;
    let mut read = ReaderBuilder::new().from_reader(file);

    for result in read.records() {
        let record = result?;
        if record.len() >= 2 {
            let image = record[0].to_string();
            let access = record[1].parse();




            user_data.push((image, access.unwrap()));
        }
    }

    Ok(())
}


fn write_file( image: &str, access: &i32) -> Result<(), Box<dyn Error>> {

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("access.csv")?;

    let mut write = Writer::from_writer(file);


    write.write_record(&[image, &access.to_string()])?;
    write.flush()?;

    Ok(())
}

fn overwrite_file( user_data: &mut Vec<(String, i32)> ) -> Result<(), Box<dyn Error>> {

    
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open("access.csv")?;
    let mut write = Writer::from_writer(file);
    write.write_record(&["image_path", &"access"])?;
        for mut i in 0..user_data.len() {



    write.write_record([user_data[i].0.as_str(),&((user_data[i].1).to_string())])?;
  
    write.flush()?;
        }

    Ok(())
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let socket = UdpSocket::bind("127.0.0.1:1101").await?;
    let flag2 = 0;
    let mut images: Vec<(String, i32)> = Vec::new();
    let mut imageCounter = 0;
    let mut encryptioncounter=0;
    let mut recieved_red = 0;
    let mut image_data:HashMap<String, i32> = HashMap::new();
    let mut images_sent: Vec<(String, SocketAddr)> = Vec::new();
  
    let mut flag3 = 1;


    

    loop {
        println!("Choose to 1. Go Online \n 2. View Pictures Offline");
        let mut choice: String = String::new();
        io::stdin().read_line(&mut choice)?;
        let ch: i32 = choice.trim().parse()?;
        if ch == 1{
        
    let message = "Register";
    let max_retransmissions = 0;
    let timeout = Duration::from_secs(1); // Set the timeout to 5 seconds
    match send_message_with_retransmission(
        message,
        "127.0.0.1:1008".parse::<SocketAddr>().unwrap(),
        max_retransmissions,
        timeout,
        &socket,
    ).await {
        Ok(()) => {
        
        }
        Err(err) => {
          
            match send_message_with_retransmission(
                message,
                "127.0.0.1:1018".parse::<SocketAddr>().unwrap(),
                max_retransmissions,
                timeout,
                &socket,
            ).await {
                Ok(()) => {
                   
                }
                Err(err) => {
                   
                    match send_message_with_retransmission(
                        message,
                        "127.0.0.1:1028".parse::<SocketAddr>().unwrap(),
                        max_retransmissions,
                        timeout,
                        &socket,
                    ).await {
                        Ok(()) => {
                         
                        }
                        Err(err) => {
                            println!("Did not recieve any reply");
                       
                        }
                    }
                }
            }
        }
    }

        println!("Choose to 1. Request \n 2. Receive \n 3.Encrypt \n 4.View Picture \n 5. Update Access \n 6.Exit");
        let mut choice: String = String::new();
        io::stdin().read_line(&mut choice)?;
        let c: i32 = choice.trim().parse()?;
        if c == 1 {
      
            let message="Ask";
            match send_message_with_retransmission2(
                message,
                "127.0.0.1:1008".parse::<SocketAddr>().unwrap(),
                max_retransmissions,
                timeout,
                &socket,
            ).await {
                Ok(()) => {
                 
     
                }
                Err(err) => {
                  
                    match send_message_with_retransmission2(
                        message,
                        "127.0.0.1:1018".parse::<SocketAddr>().unwrap(),
                        max_retransmissions,
                        timeout,
                        &socket,
                    ).await {
                        Ok(()) => {
                     
                   
                        }
                        Err(err) => {
                          
                            match send_message_with_retransmission2(
                                message,
                                "127.0.0.1:1028".parse::<SocketAddr>().unwrap(),
                                max_retransmissions,
                                timeout,
                                &socket,
                            ).await {
                                Ok(()) => {
                                  
                          
                                }
                                Err(err) => {
                                    println!("Did not recieve any reply");
                                  
                                }
                            }
                        }
                    }
                }
            }

        } else if c == 2 {
            loop {


                
                
                let mut buffer = [0; 1024];
                let (num_bytes, src_addr) = socket.recv_from(&mut buffer).await?;
                let received_message: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[0..num_bytes]);
                println!("You received a message '{}'", received_message);
                if received_message == "request"
                {
                    let dir_path = "encryption/";
                    let mut images: Vec<(String)> = Vec::new();

                    if let Ok(entries) = fs::read_dir(dir_path) {

                        for entry in entries {
                            if let Ok(entry) = entry {
                               
                                let image_path = entry.path().display().to_string();
            
                         
                                images.push((image_path));
                            }
                        }
                    }

                    println!("Images");
                    for (index, (image_path)) in images.iter().enumerate() {
                        println!("Index: {}, Image Path: {}", index, image_path);
                    }
                 
                    println!("how many pictures do you want to send?");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read line");
                    let num: usize = input.trim().parse().expect("Please type a number!");
                    socket.send_to(&num.to_ne_bytes(), src_addr).await?;
                 
                    for j in 0..num{
                    println!("Enter the index of the image you want to send:");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read line");
                    let index: usize = input.trim().parse().expect("Please type a number!");
                    let destination = src_addr;

                    if index < images.len() {
                       
                        println!("Enter number of accesses allowed");
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).expect("Failed to read line");
                        let access: usize = input.trim().parse().expect("Please type a number!");
                        let (image_path) = &images[index];
                        let image_data = fs::read(image_path)?;
                      
                        let expected_image_size = image_data.len() as u64;
            
            
                 
                        socket.send_to(&expected_image_size.to_ne_bytes(), destination).await?;
            
                        let max_fragment_size = 1024;
            
                        for chunk in image_data.chunks(max_fragment_size) {
                          
                            socket.send_to(chunk, destination).await?;
                        }
                        socket.send_to(&image_path.as_bytes(), destination).await?;
                        socket.send_to(&access.to_ne_bytes(), destination).await?;
                        println!("Image sent succesfully");
                        images_sent.push((image_path.to_string(),destination));
                         
                    } 
                    else {
                        println!("Invalid index");
                        
                    }
                 
                }
                break;
                }
                break;
            
            }

        }
        else if c == 3 {

            let image_data = fs::read("src/cover.jpg")?;
            let expected_image_size = image_data.len() as u64;

       

            let message = "Encrypt";
            match send_message_with_retransmission3(
                message,
                "127.0.0.1:1008".parse::<SocketAddr>().unwrap(),
                max_retransmissions,
                timeout,
                &socket,
         
            ).await {
                Ok(()) => {
                 
                }
                Err(err) => {
                
                    match send_message_with_retransmission3(
                        message,
                        "127.0.0.1:1018".parse::<SocketAddr>().unwrap(),
                        max_retransmissions,
                        timeout,
                        &socket,
                  
                    ).await {
                        Ok(()) => {
                           
                        }
                        Err(err) => {
                    
                            match send_message_with_retransmission3(
                                message,
                                "127.0.0.1:1028".parse::<SocketAddr>().unwrap(),
                                max_retransmissions,
                                timeout,
                                &socket,
                        
                            ).await {
                                Ok(()) => {
                                   
                                }
                                Err(err) => {
                                    println!("Did not recieve any reply");
                               
                                }
                            }
                        }
                    }
                }
            }

           
        }
        else if c==4
        {


       
            let dir_path = "received/";

         
            read_file(&mut images);

            println!("Images and their access rights:");
            for (index, (image_path, access_rights)) in images.iter().enumerate() {
                println!("Index: {}, Image Path: {}, Access Rights: {}", index, image_path, access_rights);
            }

            println!("Enter the index of the image you want to display:");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let index: usize = input.trim().parse().expect("Please type a number!");

            if index < images.len() {
                let mut new_rights:i32 = 0;
                let (image_path, access_rights) =  &mut images[index];
                println!("path {} access {}", image_path,access_rights);
          
                let path= image_path;
                if  *access_rights > 0 {
                    let encrypted = image::open(path.clone()).expect("Failed to open image");
                    let encoded_image = steganography::util::file_as_image_buffer(path.as_str().to_string());
                    println!("passed");
                    let decoder = Decoder::new(encoded_image);

                    let decodedImageBytes = decoder.decode_alpha();

                    let decodedImage = bytes_to_image(decodedImageBytes);

                    let (width, height) = decodedImage.dimensions();
                    let u32_image_datanew: Vec<u32> = decodedImage
                        .to_rgba8()  // Convert to RGBA format
                        .into_raw()
                        .chunks_exact(4)
                        .map(|chunk| {
                            let r = chunk[0] as u32;
                            let g = chunk[1] as u32;
                            let b = chunk[2] as u32;
                            let a = chunk[3] as u32;
                            (a << 24) | (r << 16) | (g << 8) | b
                        })
                        .collect();
               
                    println!("access {}", access_rights);
                    new_rights = access_rights.clone();
                    new_rights -=1;
                    images[index].1 = new_rights;
                    let mut window = Window::new(
                        "Image Viewer",
                        width as usize,
                        height as usize,
                        WindowOptions {
                            resize: true,
                            scale: Scale::X2,
                            scale_mode: ScaleMode::AspectRatioStretch,
                            ..WindowOptions::default()
                        },
                    )
                        .unwrap_or_else(|e| {
                            panic!("{}", e);
                        });

                    while window.is_open() && !window.is_key_down(Key::Escape) {
                        window
                            .update_with_buffer(&u32_image_datanew, width as usize, height as usize)
                            .unwrap_or_else(|e| {
                                println!("Error updating window: {}", e);
                            });
                    }
                }else{

                    let image = image::open(path).expect("Failed to open image");
                    let (width, height) = image.dimensions();
                    let u32_image_datanew: Vec<u32> = image
                        .to_rgba8()  // Convert to RGBA format
                        .into_raw()
                        .chunks_exact(4)
                        .map(|chunk| {
                            let r = chunk[0] as u32;
                            let g = chunk[1] as u32;
                            let b = chunk[2] as u32;
                            let a = chunk[3] as u32;
                            (a << 24) | (r << 16) | (g << 8) | b
                        })
                        .collect();
                    let mut window = Window::new(
                        "Image Viewer",
                        width as usize,
                        height as usize,
                        WindowOptions {
                            resize: true,
                            scale: Scale::X2,
                            scale_mode: ScaleMode::AspectRatioStretch,
                            ..WindowOptions::default()
                        },
                    )
                        .unwrap_or_else(|e| {
                            panic!("{}", e);
                        });

                    while window.is_open() && !window.is_key_down(Key::Escape) {
                        window
                            .update_with_buffer(&u32_image_datanew, width as usize, height as usize)
                            .unwrap_or_else(|e| {
                                println!("Error updating window: {}", e);
                            });

                    }}

              
                for mut i in 0..images.len() {
                    println!("image path : {} access: {}",images[i].0,images[i].1 );
                
                    overwrite_file(&mut images)?;
                    i+=1;
                  
                }
                images.clear();
                images.shrink_to_fit();

                

            } else {
                println!("Invalid index");
            }







        }
        else if c == 5 {

            for (index, (sent)) in images_sent.iter().enumerate() {
                println!("Index: {}, Image Sent: {} , Receiver: {}", index, sent.0 , sent.1);
            }
            println!("Enter the index of the image you want to update access for:");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let chosenind: usize = input.trim().parse().expect("Please type a number!");


            if chosenind < images_sent.len() {
                
            println!("Enter the new access count of the image :");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let newac: usize = input.trim().parse().expect("Please type a number!");

            update_access("update", "127.0.0.1:1008".parse::<SocketAddr>().unwrap(), &socket,newac as i32, &images_sent[chosenind]).await;
            update_access("update", "127.0.0.1:1018".parse::<SocketAddr>().unwrap(), &socket,newac as i32, &images_sent[chosenind]).await;
            update_access("update", "127.0.0.1:1028".parse::<SocketAddr>().unwrap(), &socket,newac as i32, &images_sent[chosenind]).await;
            }
           

        }

        else if c == 6 {
            exit(1);
        }
    }
    if ch == 2
    {
        

            let dir_path = "received/";

        
           
            read_file(&mut images);
    
            println!("Images and their access rights:");
            for (index, (image_path, access_rights)) in images.iter().enumerate() {
                println!("Index: {}, Image Path: {}, Access Rights: {}", index, image_path, access_rights);
            }

     
            println!("Enter the index of the image you want to display:");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let index: usize = input.trim().parse().expect("Please type a number!");


            if index < images.len() {
                let mut new_rights:i32 = 0;
                let (image_path, access_rights) =  &mut images[index];
                println!("path {} access {}", image_path,access_rights);
             
                let path= image_path;
                if  *access_rights > 0 {
                    let encrypted = image::open(path.clone()).expect("Failed to open image");
                    let encoded_image = steganography::util::file_as_image_buffer(path.as_str().to_string());
                    println!("passed");
                    let decoder = Decoder::new(encoded_image);

                    let decodedImageBytes = decoder.decode_alpha();

                    let decodedImage = bytes_to_image(decodedImageBytes);

                    let (width, height) = decodedImage.dimensions();
                    let u32_image_datanew: Vec<u32> = decodedImage
                        .to_rgba8()  // Convert to RGBA format
                        .into_raw()
                        .chunks_exact(4)
                        .map(|chunk| {
                            let r = chunk[0] as u32;
                            let g = chunk[1] as u32;
                            let b = chunk[2] as u32;
                            let a = chunk[3] as u32;
                            (a << 24) | (r << 16) | (g << 8) | b
                        })
                        .collect();
                
                    println!("access {}", access_rights);
                    new_rights = access_rights.clone();
                    new_rights -=1;
                    images[index].1 = new_rights;
                    let mut window = Window::new(
                        "Image Viewer",
                        width as usize,
                        height as usize,
                        WindowOptions {
                            resize: true,
                            scale: Scale::X2,
                            scale_mode: ScaleMode::AspectRatioStretch,
                            ..WindowOptions::default()
                        },
                    )
                        .unwrap_or_else(|e| {
                            panic!("{}", e);
                        });

                    while window.is_open() && !window.is_key_down(Key::Escape) {
                        window
                            .update_with_buffer(&u32_image_datanew, width as usize, height as usize)
                            .unwrap_or_else(|e| {
                                println!("Error updating window: {}", e);
                            });
                    }
                }else{

                    let image = image::open(path).expect("Failed to open image");
                    let (width, height) = image.dimensions();
                    let u32_image_datanew: Vec<u32> = image
                        .to_rgba8()  // Convert to RGBA format
                        .into_raw()
                        .chunks_exact(4)
                        .map(|chunk| {
                            let r = chunk[0] as u32;
                            let g = chunk[1] as u32;
                            let b = chunk[2] as u32;
                            let a = chunk[3] as u32;
                            (a << 24) | (r << 16) | (g << 8) | b
                        })
                        .collect();
                    let mut window = Window::new(
                        "Image Viewer",
                        width as usize,
                        height as usize,
                        WindowOptions {
                            resize: true,
                            scale: Scale::X2,
                            scale_mode: ScaleMode::AspectRatioStretch,
                            ..WindowOptions::default()
                        },
                    )
                        .unwrap_or_else(|e| {
                            panic!("{}", e);
                        });

                    while window.is_open() && !window.is_key_down(Key::Escape) {
                        window
                            .update_with_buffer(&u32_image_datanew, width as usize, height as usize)
                            .unwrap_or_else(|e| {
                                println!("Error updating window: {}", e);
                            });

                    }}

              
                for mut i in 0..images.len() {
                    println!("image path : {} access: {}",images[i].0,images[i].1 );
                
                    overwrite_file(&mut images)?;
                    i+=1;
                  
                }
                images.clear();
                images.shrink_to_fit();

                

            } else {
                println!("Invalid index");
            }

    }
}
}
