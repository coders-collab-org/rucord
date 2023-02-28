use rucord_api_types::GatewayBotObject;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, Semaphore};

pub struct IdentifyQueue {
    send_rate_limit: Semaphore,
    identify_state: Mutex<IdentifyState>,
    gateway_info: Arc<Mutex<GatewayBotObject>>,
}

struct IdentifyState {
    remaining: u64,
    reset_time: Instant,
}

impl IdentifyQueue {
    const FIVE_SECOND: Duration = Duration::from_secs(5);

    pub fn new(gateway_info: Arc<Mutex<GatewayBotObject>>) -> Self {
        IdentifyQueue {
            send_rate_limit: Semaphore::new(1),
            identify_state: Mutex::new(IdentifyState {
                remaining: 0,
                reset_time: Instant::now() - Self::FIVE_SECOND,
            }),
            gateway_info,
        }
    }

    pub async fn wait_for_identify(&self) {
        let permit = self.send_rate_limit.acquire().await.unwrap();

        {
            let mut lock = self.identify_state.lock().await;

            if lock.remaining == 0 {
                let elapsed_since_reset = lock.reset_time.elapsed();

                if elapsed_since_reset < Self::FIVE_SECOND {
                    tokio::time::sleep(Self::FIVE_SECOND - elapsed_since_reset).await;
                }

                lock.remaining = self
                    .gateway_info
                    .lock()
                    .await
                    .session_start_limit
                    .max_concurrency;

                lock.reset_time = Instant::now();
            }

            lock.remaining -= 1;
        }

        permit.forget();
    }
}
