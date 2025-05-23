use domain::element;
use sousarc_content_types::{
  storage::StandardStorage,
  traits::{
    SousARCStorage, SousARCStorageMut, prelude::SousARCData,
  },
};

mod domain;

fn main() {
  let user = domain::user::UserData::new("test_user");
  let user_id = user.id();
  let mut user_storage = StandardStorage::new();
  user_storage.insert(user);
  let work = user_storage
    .get_mut(user_id)
    .unwrap()
    .spawn("test_work");
  let work_id = work.id();
  let mut work_storage = StandardStorage::new();
  work_storage.insert(work);
  let element = work_storage
    .get_mut(work_id)
    .unwrap()
    .spawn("test_element");
  let element_id = element.id();
  let mut element_storage = StandardStorage::new();
  element_storage.insert(element);
  let element_2 = element_storage
    .get_mut(element_id)
    .unwrap()
    .spawn("test_element_2");
  let element_id_2 = element_2.id();
  element_storage.insert(element_2);
  let element_3 = element_storage
    .get_mut(element_id_2)
    .unwrap()
    .spawn("test_element_3");
  let element_id_3 = element_3.id();
  element_storage.insert(element_3);

  println!("{:?}", user_storage.get(user_id).unwrap());
  println!("{:?}", work_storage.get(work_id).unwrap());
  println!("{:?}", element_storage.get(element_id).unwrap());
  println!(
    "{:?}",
    element_storage.get(element_id_2).unwrap()
  );
  println!(
    "{:?}",
    element_storage.get(element_id_3).unwrap()
  );
  let mut buf = String::new();
  work_storage
    .get(work_id)
    .map(|w| w.key().fq_name(&mut buf, &user_storage))
    .unwrap_or(Ok(()))
    .unwrap();
  println!("{buf}");
  buf.clear();
  element_storage
    .get(element_id)
    .map(|e| {
      e.key().fq_name(
        &mut buf,
        &user_storage,
        &work_storage,
        &element_storage,
      )
    })
    .unwrap_or(Ok(()))
    .unwrap();
  println!("{buf}");
  buf.clear();
  element_storage
    .get(element_id_2)
    .map(|e| {
      e.key().fq_name(
        &mut buf,
        &user_storage,
        &work_storage,
        &element_storage,
      )
    })
    .unwrap_or(Ok(()))
    .unwrap();
  println!("{buf}");
  buf.clear();
  element_storage
    .get(element_id_3)
    .map(|e| {
      e.key().fq_name(
        &mut buf,
        &user_storage,
        &work_storage,
        &element_storage,
      )
    })
    .unwrap_or(Ok(()))
    .unwrap();
  println!("{buf}");
  user_storage.remove(user_id);
  buf.clear();
  work_storage
    .get(work_id)
    .map(|w| w.key().fq_name(&mut buf, &user_storage))
    .unwrap_or(Ok(()))
    .unwrap();
  println!("{buf}");
}
