pub mod dal;

use tokio::io::AsyncWriteExt;
use tonic::{transport::Server, Request, Response, Status};

use filetransfer::file_service_server::{FileService, FileServiceServer};
use filetransfer::{FileChunk, UploadStatus};

pub mod filetransfer {
    tonic::include_proto!("studyvault");
}

#[derive(Default)]
pub struct FileServiceProt;

#[tonic::async_trait]
impl FileService for FileServiceProt {
    async fn upload_file(
        &self,
        request: Request<tonic::Streaming<FileChunk>>,
    ) -> Result<Response<UploadStatus>, Status> {
        let mut stream = request.into_inner();
        let mut file = None;

        let mut file_name: Option<String> = None;
        let mut channel_id: Option<u32> = None;
        let mut title: Option<String> = None;
        let mut description: Option<String> = None;

        print!("Uploading file...");

        while let Some(file_chunk) = stream.message().await? {
            if file_name.is_none() {
                file_name = Some(file_chunk.filename);
                println!("File name: {:?}", file_name.clone().unwrap());
                let file_path = format!("/data/files/{}", file_name.clone().unwrap());

                file = Some(tokio::fs::File::create(file_path).await.map_err(|e| {
                    eprintln!("Failed to create file: {:?}", e);
                    Status::internal("Failed to create file")
                })?);
            }

            channel_id = Some(file_chunk.channel_id);
            title = Some(file_chunk.title);
            description = Some(file_chunk.description);
            print!(
                "channel_id: {:?}, title: {:?}, description: {:?}",
                channel_id, title, description
            );

            if let Some(ref mut f) = file {
                f.write_all(&file_chunk.content).await.map_err(|e| {
                    eprintln!("Failed to write file data: {:?}", e);
                    Status::internal("Failed to write file data")
                })?;
            }
        }

        let result = dal::posts::create_post(
            channel_id.unwrap(),
            file_name.unwrap(),
            title.unwrap(),
            description.unwrap(),
        )
        .await;

        println!("File uploaded successfully");

        Ok(Response::new(UploadStatus {
            success: result,
            message: format!("File uploaded successfully"),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let file_service = FileServiceProt::default();

    println!("gRPC Server listening on {}", addr);

    Server::builder()
        .add_service(FileServiceServer::new(file_service))
        .serve(addr)
        .await?;

    Ok(())
}
