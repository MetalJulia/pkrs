use chrono::{DateTime, NaiveDate, Utc};
use sqlx;

#[derive(sqlx::Type)]
#[sqlx(rename = "proxy_tag")]
struct ProxyTag {
    pub prefix: Option<String>,
    pub suffix: Option<String>,
}

#[derive(sqlx::FromRow)]
struct System {
    pub id: i32,
    pub hid: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub tag: Option<String>,
    pub avatar_url: Option<String>,
    pub token: Option<String>,
    pub created: DateTime<Utc>,
    pub ui_tz: String,
    pub description_privacy: i32,
    pub member_list_privacy: i32,
    pub front_privacy: i32,
    pub front_history_privacy: i32,
    pub group_list_privacy: i32,
    pub pings_enabled: bool,
}

#[derive(sqlx::FromRow)]
struct SystemGuild {
    pub system: i32,
    pub guild: i64,
    pub proxy_enabled: bool,
    pub autoproxy_mode: i32,
    pub autoproxy_member: Option<i32>,
}

#[derive(sqlx::FromRow)]
struct Member {
    pub id: i32,
    pub hid: String,
    pub system: i32,
    pub color: Option<String>,
    pub avatar_url: Option<String>,
    pub name: String,
    pub display_name: Option<String>,
    pub birthday: Option<NaiveDate>,
    pub pronouns: Option<String>,
    pub description: Option<String>,
    pub proxy_tags: Vec<ProxyTag>,
    pub keep_proxy: bool,
    pub created: DateTime<Utc>,
    pub message_count: i32,
    pub description_privacy: i32,
    pub name_privacy: i32,
    pub avatar_privacy: i32,
    pub birthday_privacy: i32,
    pub pronoun_privacy: i32,
    pub metadata_privacy: i32,
}

#[derive(sqlx::FromRow)]
struct MemberGuild {
    pub member: i32,
    pub guild: i64,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(sqlx::FromRow)]
struct Message {
    pub mid: i64,
    pub channel: i64,
    pub member: i32,
    pub sender: i64,
    pub original_mid: Option<i64>,
    pub guild: Option<i64>,
}

#[derive(sqlx::FromRow)]
struct Switch {
    pub id: i32,
    pub system: i32,
    pub timestamp: DateTime<Utc>,
}

#[derive(sqlx::FromRow)]
struct SwitchMember {
    pub id: i32,
    pub switch: i32,
    pub member: i32,
}

#[derive(sqlx::FromRow)]
struct Webhook {
    pub channel: i64,
    pub webhook: i64,
    pub token: String,
}

#[derive(sqlx::FromRow)]
struct Server {
    pub id: i64,
    pub log_channel: Option<i64>,
    pub log_blacklist: Vec<i64>,
    pub blacklist: Vec<i64>,
    pub log_cleanup_enabled: bool,
}

#[derive(sqlx::FromRow)]
struct Group {
    pub id: i32,
    pub hid: String,
    pub system: i32,
    pub name: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub description_privacy: i32,
    pub icon_privacy: i32,
    pub list_privacy: i32,
    pub visibility: i32,
    pub created: DateTime<Utc>,
}
