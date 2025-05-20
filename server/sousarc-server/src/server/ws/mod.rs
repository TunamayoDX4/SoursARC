use sousarc_content_types::prelude::*;
use std::sync::LazyLock;

static USERS: LazyLock<StandardStorage<UserData, UserData>> =
  LazyLock::new(|| {
    let mut storage =
      StandardStorage::<UserData, UserData>::new();
    let user = UserData {
      works: Vec::new(),
      display_name: "ツナマヨ".to_string(),
      introduction: "冒険や日常、SFが好きなしがない創作者"
        .to_string(),
    };
    let user = SousARCDataWrap {
      data: user,
      ident: SousARCIdentPair {
        id: UserId::new(),
        key: UserKey::new("tunamayo"),
      },
    };
    storage.insert(user);
    storage
  });
