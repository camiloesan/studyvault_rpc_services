use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tonic::transport::Channel;
use tonic::Request;

use filetransfer::file_service_client::FileServiceClient;
use filetransfer::FileChunk;

pub mod filetransfer {
    tonic::include_proto!("studyvault");
}

async fn upload_file(
    client: &mut FileServiceClient<Channel>,
    file_path: &str,
    channel_id: u32,
    title: String,
    description: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = file_path.split('/').last().unwrap().to_string();
    let mut file = File::open(file_path).await?;

    // Chunk generator
    let stream = async_stream::stream! {
        let mut buffer = vec![0; 64 * 1024]; // 64KB buffer
        loop {
            let bytes_read = file.read(&mut buffer).await.unwrap();
            if bytes_read == 0 {
                break;
            }
            yield FileChunk {
                content: buffer[..bytes_read].to_vec(),
                filename: file_name.clone(),
                channel_id,
                title: title.clone(),
                description: description.clone(),
            };
        }
    };

    let request = Request::new(stream);

    let response = client.upload_file(request).await?;

    println!("Upload Response: {:?}", response.into_inner());

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = FileServiceClient::connect("http://0.0.0.0:50051").await?;
    upload_file(
        &mut client,
        "/Users/camiloespejo/Downloads/thefirefromwithin.pdf",
        1,
        "testfromclient".to_string(),
        "testfromclient".to_string(),
    )
    .await?;
    Ok(())
}
