use dotenv::dotenv;
use reqwest::Response;
use simple_aws_s3::S3;
use std::env;

#[derive(Debug)]
struct S3Data {
    access_key: String,
    bucket: String,
    endpoint: String,
    region: String,
    secret_key: String,
}

impl Default for S3Data {
    fn default () -> S3Data {
        S3Data { 
            access_key: "".to_string(), 
            bucket: "".to_string(), 
            endpoint: "".to_string(), 
            region: "".to_string(), 
            secret_key: "".to_string() 
        }
    }
}

fn print_details(file_name: String, response: Response) {
    println!("Successfully retrieved the details for {}:", file_name);
    println!("  Status code: {status_code}.", status_code=response.status());
    println!("  Content type: {content_type:?}.", content_type=response.headers().get("Content-Type").unwrap());
    println!("  File size: {file_size:?}", file_size=response.headers().get("Content-Length").unwrap());
}

// Get Information of Object such as content type and content length (bytes)
async fn get_file_details(s3: &S3, file_name: String) {
    let head_result = s3.head_object(&file_name).await;
    match head_result {
        Ok(response) => print_details(file_name, response),
        Err(error) => panic!("Error retrieving the file's head information: {:?}", error),
    };
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let s3_data = S3Data{
        access_key: env::var("ACCESS_KEY".to_string()).unwrap_or("".to_string()),
        bucket: env::var("BUCKET".to_string()).unwrap_or("".to_string()),
        endpoint: env::var("ENDPOINT".to_string()).unwrap_or("".to_string()),
        region: env::var("REGION".to_string()).unwrap_or("".to_string()),
        secret_key: env::var("SECRET_KEY".to_string()).unwrap_or("".to_string()),
    };

    /*match env::var("SECRET_KEY".to_string()) {
        Ok(val) => s3_data.secret_key = val,
        Err(error) => println!("Could not retrieve key for secret key: {:?}", error)
    }*/

    let s3 = S3::new(
        s3_data.bucket, 
        s3_data.region, 
        s3_data.endpoint, 
        s3_data.access_key, 
        s3_data.secret_key
    );

    get_file_details(&s3, "upload-file.txt".to_string()).await
}