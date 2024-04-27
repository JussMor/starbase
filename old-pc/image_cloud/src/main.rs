use aws_sdk_s3::types::{BucketLocationConstraint, CreateBucketConfiguration};
use aws_sdk_s3::{config::Region, Client};
use uuid::Uuid;


#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("Error: {:?}", err);
    }
}

async fn run() -> Result<(), aws_sdk_s3::Error> {
    let (client, bucket_name) = initialize_variables().await;
    let constrain = BucketLocationConstraint::from("us-east-1");
    let bucket_config = CreateBucketConfiguration::builder().location_constraint(constrain).build();

    client
        .create_bucket()
        .create_bucket_configuration(bucket_config)
        .bucket(bucket_name)
        .send()
        .await?;



    Ok(())
}

async fn initialize_variables() -> (Client, String) {
    let sdk_config = aws_config::load_from_env().await;
    let config = aws_sdk_s3::config::Builder::from(&sdk_config)
        .region(Region::new("us-east-1"))
        .endpoint_url("http://127.0.0.1:9000/api")
        .build();

    let client = Client::from_conf(config);

    let bucket_name = format!("test-{}", Uuid::new_v4());

    (client, bucket_name)
}