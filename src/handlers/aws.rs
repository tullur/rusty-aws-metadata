use tide::Request;

extern crate ec2_instance_metadata;

pub async fn metadata(_req: Request<()>) -> tide::Result<String> {
    let client = ec2_instance_metadata::InstanceMetadataClient::new();
    let metadata = client.get().unwrap();

    Ok(format!("Region: {}, Zone: {}", metadata.region, metadata.availability_zone))
}