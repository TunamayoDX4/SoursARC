use parking_lot::Mutex;
use sousarc_content_types::prelude::*;
use wasm_bindgen::{JsValue, prelude::*};

static USERDATA: Mutex<
  Option<Vec<SousARCDataWrap<UserData, UserData>>>,
> = Mutex::new(None);

#[wasm_bindgen]
pub fn userdata_from_bytes(
  input: &[u8],
  idx: Option<usize>,
) -> Result<(), JsValue> {
  let data: SousARCDataWrap<UserData, UserData> =
    rmp_serde::from_slice(input).map_err(|e| {
      JsValue::from_str(&format!(
        "Failed to deserialize: {}",
        e
      ))
    })?;
  let mut v = USERDATA.lock();
  let v = v.get_or_insert(Vec::new());
  if let Some(idx) = idx {
    if idx < v.len() {
      v[idx] = data;
    } else {
      return Err(JsValue::from_str("Index out of bounds"));
    }
  } else {
    v.push(data);
  }

  Ok(())
}

#[wasm_bindgen]
pub fn userdata_into_bytes(
  idx: usize,
) -> Result<Vec<u8>, JsValue> {
  USERDATA
    .lock()
    .as_ref()
    .ok_or(JsValue::from_str("No data available"))?
    .get(idx)
    .ok_or(JsValue::from_str("Index out of bounds"))
    .and_then(|data| {
      rmp_serde::to_vec(data).map_err(|e| {
        JsValue::from_str(&format!(
          "Failed to serialize: {}",
          e
        ))
      })
    })
}

#[wasm_bindgen]
pub fn userdata_from_json(
  input: &str,
  idx: Option<usize>,
) -> Result<(), JsValue> {
  let data: SousARCDataWrap<UserData, UserData> =
    serde_json::from_str(input).map_err(|e| {
      JsValue::from_str(&format!(
        "Failed to deserialize: {}",
        e
      ))
    })?;
  let mut v = USERDATA.lock();
  let v = v.get_or_insert(Vec::new());
  if let Some(idx) = idx {
    if idx < v.len() {
      v[idx] = data;
    } else {
      return Err(JsValue::from_str("Index out of bounds"));
    }
  } else {
    v.push(data);
  }

  Ok(())
}

#[wasm_bindgen]
pub fn userdata_into_json(
  idx: usize,
) -> Result<String, JsValue> {
  USERDATA
    .lock()
    .as_ref()
    .ok_or(JsValue::from_str("No data available"))?
    .get(idx)
    .ok_or(JsValue::from_str("Index out of bounds"))
    .and_then(|data| {
      serde_json::to_string(data).map_err(|e| {
        JsValue::from_str(&format!(
          "Failed to serialize: {}",
          e
        ))
      })
    })
}

#[wasm_bindgen]
pub fn userdata_len() -> usize {
  USERDATA
    .lock()
    .as_ref()
    .map(|data| data.len())
    .unwrap_or_default()
}

#[wasm_bindgen]
pub fn user_ids() -> String {
  USERDATA
    .lock()
    .as_ref()
    .map(|data| {
      data
        .iter()
        .map(|d| d.id().to_string())
        .collect::<Vec<_>>()
        .join(", ")
    })
    .unwrap_or_default()
}

#[wasm_bindgen]
pub fn user_keys() -> Vec<String> {
  USERDATA
    .lock()
    .as_ref()
    .map(|data| {
      data
        .iter()
        .map(|d| d.key().to_string())
        .collect::<Vec<_>>()
    })
    .unwrap_or_default()
}
