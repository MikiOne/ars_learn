use payments::bitcoin_client::BitcoinClient;
use payments::BtcPaymentRequest;
pub mod payments {
    tonic::include_proto!("payments");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // http://[::1]:50051
    let mut client = BitcoinClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(BtcPaymentRequest {
        from_addr: "123".to_string(),
        to_addr: "321".to_string(),
        amount: 10,
    });

    let resp = client.send_payment(request).await?;
    println!("Response: {:?}", resp);
    Ok(())
}