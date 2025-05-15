use std::{
  collections::{BTreeMap, BTreeSet, HashMap, VecDeque},
  net::SocketAddr,
};

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimiterConfig {
  pub expire_duration: Duration,
  pub limit_per_dur: u64,
  pub cleanup_interval: u64,
}
impl Default for RateLimiterConfig {
  fn default() -> Self {
    Self {
      expire_duration: Duration::seconds(60 * 5),
      limit_per_dur: 100,
      cleanup_interval: 60,
    }
  }
}

/// # RateLimiter
/// レートリミッター
pub struct RateLimiter(Mutex<RateLimiterInner>);
impl RateLimiter {
  /// 新しいレートリミッターを作成する
  /// Create a new rate limiter
  pub fn new(config: &RateLimiterConfig) -> Self {
    Self(Mutex::new(RateLimiterInner::new(config)))
  }

  /// IPアドレスのレートをチェックする
  /// Check the rate of the IP address
  pub async fn check(&self, ip: IpAddr) -> bool {
    self.0.lock().await.check(ip)
  }
}

//// # RateLimiterInner
/// レートリミッターの内部構造
#[derive(Debug, Clone)]
struct RateLimiterInner {
  records: HashMap<IpAddr, VecDeque<DateTime<Utc>>>,
  limit_per_dur: u64,
  expire_duration: Duration,
  cleanup_ctr: u64,
  cleanup_interval: u64,
}
impl RateLimiterInner {
  /// 新しいレートリミッターを作成する
  /// Create a new rate limiter
  pub fn new(config: &RateLimiterConfig) -> Self {
    Self {
      records: HashMap::new(),
      limit_per_dur: config.limit_per_dur,
      expire_duration: config.expire_duration,
      cleanup_ctr: 0,
      cleanup_interval: config.cleanup_interval,
    }
  }

  fn update(&mut self, now: DateTime<Utc>) {
    let expire_time = now - self.expire_duration;
    self.records.values_mut().for_each(|queue| {
      // 期限切れのレコードを削除
      while queue.front().map_or(false, |t| t < &expire_time)
      {
        queue.pop_front();
      }
    });
    self.records.retain(|_, queue| !queue.is_empty());
  }

  /// IPアドレスのレートをチェックする
  /// Check the rate of the IP address
  pub fn check(&mut self, ip: IpAddr) -> bool {
    let now = Utc::now();
    let expire_time = now - self.expire_duration;

    self.cleanup_ctr += 1;
    if self.cleanup_interval < self.cleanup_ctr {
      self.update(now);
      self.cleanup_ctr = 0;
    }

    match self.records.entry(ip) {
      std::collections::hash_map::Entry::Occupied(
        mut entry,
      ) => {
        tracing::trace!(
          "RateLimiter: {}: {}",
          ip,
          entry.get().len()
        );

        let queue = entry.get_mut();
        queue.push_back(now);
        // 期限切れのレコードを削除
        while queue
          .front()
          .map_or(false, |t| t < &expire_time)
        {
          queue.pop_front();
        }
        // 制限を超えた場合はfalseを返す
        queue.len() < (self.limit_per_dur as usize)
      }
      std::collections::hash_map::Entry::Vacant(entry) => {
        entry.insert(VecDeque::from([now]));
        true
      }
    }
  }
}

/// # extract_client_ip
/// クライアントのIPアドレスを抽出する
pub fn extract_client_ip(
  headers: &axum::http::HeaderMap,
  addr: &SocketAddr,
) -> Option<IpAddr> {
  if let Some(forwarded) = headers.get("X-Forwarded-For") {
    if let Some(forwarded) = forwarded
      .to_str()
      .map(|s| s.split(',').next())
      .ok()
      .flatten()
    {
      return forwarded.parse().ok();
    } else {
      return None;
    }
  } else if let Some(real_ip) = headers.get("X-Real-IP") {
    if let Ok(real_ip) = real_ip.to_str() {
      return real_ip.parse().ok();
    } else {
      return None;
    }
  } else {
    return Some(addr.ip());
  }
}
