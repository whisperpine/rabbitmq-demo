// const HOST_NAME: &str = "localhost";
const HOST_NAME: &str = "test.mosquitto.org";

#[tokio::test]
async fn test_async_mqtt() {
    use rumqttc::{AsyncClient, MqttOptions, QoS};
    use std::time::Duration;
    use tokio::{task, time};

    let mut mqttoptions = MqttOptions::new("rumqtt-async", HOST_NAME, 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe("hello/rumqtt", QoS::AtLeastOnce)
        .await
        .unwrap();

    task::spawn(async move {
        for i in 0..3 {
            client
                .publish("hello/rumqtt", QoS::AtLeastOnce, false, vec![i; i as usize])
                .await
                .unwrap();
            time::sleep(Duration::from_millis(100)).await;
        }
    });

    // loop {
    //     let notification = eventloop.poll().await.unwrap();
    //     println!("Received = {notification:?}");
    // }

    while let Ok(notification) = time::timeout(Duration::from_secs(1), eventloop.poll()).await {
        let notification = notification.unwrap();
        println!("Received = {notification:?}");
    }
}

#[test]
fn test_sync_mqtt() {
    use rumqttc::{Client, MqttOptions, QoS};
    use std::thread;
    use std::time::{Duration, Instant};

    let mut mqttoptions = MqttOptions::new("rumqtt-sync", HOST_NAME, 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(1));

    let (client, mut connection) = Client::new(mqttoptions, 10);
    client.subscribe("hello/rumqtt", QoS::AtMostOnce).unwrap();

    thread::spawn(move || {
        for i in 0..3 {
            client
                .publish("hello/rumqtt", QoS::AtLeastOnce, false, vec![i; i as usize])
                .unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    let instant = Instant::now();
    for notification in connection.iter() {
        let notification = notification.unwrap();
        println!("Notification = {notification:?}");
        if instant.elapsed().as_secs_f32() > 2. {
            break;
        }
    }
}
