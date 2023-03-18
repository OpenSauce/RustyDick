#[macro_export]
macro_rules! send {
    ($a:expr,$b:expr,$c:expr) => {
        if let Err(why) = $b.channel_id.say($a, $c).await {
            println!("Error sending message: {:?}", why);
        }
    };
}
