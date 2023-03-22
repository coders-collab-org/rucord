use rucord_api_types::GatewayBotObject;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

pub struct IdentifyQueue {
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
            identify_state: Mutex::new(IdentifyState {
                remaining: 0,
                reset_time: Instant::now().checked_sub(Self::FIVE_SECOND).unwrap(),
            }),
            gateway_info,
        }
    }

    pub async fn wait_for_identify(&self) {
        let mut identify_state = self.identify_state.lock().await;

        if identify_state.remaining == 0 {
            let elapsed_since_reset = identify_state.reset_time.elapsed();

            if elapsed_since_reset < Self::FIVE_SECOND {
                tokio::time::sleep(Self::FIVE_SECOND - elapsed_since_reset).await;
            }

            identify_state.remaining = self
                .gateway_info
                .lock()
                .await
                .session_start_limit
                .max_concurrency;

            identify_state.reset_time = Instant::now();
        }

        identify_state.remaining -= 1;
    }
}
