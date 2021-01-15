use chrono::{DateTime, Duration, Local, TimeZone};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt, str::FromStr};

const AUTH_URI: &str = "https://oauth.vk.com/authorize";
const AUTH_PARAMS: [(&str, &str); 6] = [
    ("client_id", "7720259"),
    ("display", "page"),
    ("redirect_uri", "https://oauth.vk.com/blank.html"),
    (
        "scope",
        "offline,friends,groups,photos,audio,video,stories,status,notes,wall",
    ), // "friends" is possible too
    ("response_type", "token"),
    ("v", rvk::API_VERSION),
];
const IDX_REDIRECT_URI: usize = 2;

fn datetime_deserializer<'de, D>(de: D) -> Result<Option<DateTime<Local>>, D::Error>
where
    D: Deserializer<'de>,
{
    i64::deserialize(de).map(|secs| {
        if secs > 0 {
            Some(Local.timestamp(secs, 0))
        } else {
            None
        }
    })
}

fn datetime_serializer<S>(x: &Option<DateTime<Local>>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(dt) = x {
        s.serialize_i64(dt.timestamp())
    } else {
        s.serialize_i64(0)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AuthResponse {
    #[serde(default)]
    access_token: String,
    #[serde(
        default = "AuthResponse::default_expires_on",
        deserialize_with = "datetime_deserializer",
        serialize_with = "datetime_serializer"
    )]
    expires_on: Option<DateTime<Local>>,
    #[serde(default)]
    user_id: String,
}

impl AuthResponse {
    pub fn get_access_token(&self) -> &str {
        &self.access_token
    }

    // not decide yet how to use it
    #[allow(dead_code)]
    pub fn get_expires_on(&self) -> Option<DateTime<Local>> {
        self.expires_on
    }

    pub fn get_user_id(&self) -> &str {
        &self.user_id
    }

    fn default_expires_on() -> Option<DateTime<Local>> {
        None
    }
}

impl Default for AuthResponse {
    fn default() -> Self {
        AuthResponse {
            access_token: String::default(),
            expires_on: AuthResponse::default_expires_on(),
            user_id: String::default(),
        }
    }
}

impl fmt::Display for AuthResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let token: &str = if self.access_token.len() > 6 {
            &self.access_token[..6]
        } else {
            &self.access_token
        };
        let expires_on = if let Some(val) = self.expires_on {
            format!("until {}", val)
        } else {
            format!("forever")
        };
        write!(
            f,
            "user {} with token {}... valid {}",
            self.user_id.as_str(),
            token,
            expires_on
        )
    }
}

impl FromStr for AuthResponse {
    type Err = AuthResponseParseError;

    fn from_str(uri: &str) -> Result<Self, Self::Err> {
        // https://oauth.vk.com/blank.html#
        // access_token=7de238693ce753e240abd1e1845480667bed442daf94e86df12d6c376322137fd2649661fbfd1352741fa&
        // expires_in=0&
        // user_id=184946538
        match uri.find("#") {
            None => Err(AuthResponseParseError::Malformed),
            Some(pos) => {
                let (_, fragment) = uri.split_at(pos);
                if fragment.len() < 2 {
                    // "" and "="
                    return Err(AuthResponseParseError::Malformed);
                }
                let fragment = fragment.get(1..).unwrap(); // skip "#"
                let mut access_token = None;
                let mut expires_on = None;
                let mut user_id = None;
                for part in fragment.split('&') {
                    match part.find("=") {
                        None => continue,
                        Some(pos) => {
                            let (name, value) = part.split_at(pos);
                            if value.len() < 2 {
                                // "" and "="
                                continue;
                            }
                            let value = value.get(1..).unwrap(); // skip '='
                            match name {
                                "access_token" => access_token = Some(value),
                                "expires_in" => {
                                    if let Ok(num) = value.parse::<i64>() {
                                        if num > 0 {
                                            expires_on =
                                                Some(Local::now() + Duration::seconds(num));
                                        }
                                    }
                                }
                                "user_id" => user_id = Some(value),
                                &_ => continue,
                            }
                        }
                    }
                }
                if access_token.is_none() {
                    Err(AuthResponseParseError::NoAccessToken)
                } else if user_id.is_none() {
                    Err(AuthResponseParseError::NoUserId)
                } else {
                    Ok(AuthResponse {
                        access_token: access_token.unwrap().into(),
                        expires_on,
                        user_id: user_id.unwrap().into(),
                    })
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum AuthResponseParseError {
    Malformed,
    NoAccessToken,
    NoUserId,
}

impl fmt::Display for AuthResponseParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Malformed => write!(f, "No fragment part (#) in the source URI"),
            Self::NoAccessToken => write!(f, "access_token is not set in URI fragment"),
            Self::NoUserId => write!(f, "user_id is not set in URI fragment"),
        }
    }
}

pub struct AccessTokenProvider;

impl AccessTokenProvider {
    pub fn get_auth_uri() -> String {
        let mut uri = AUTH_URI.to_string();
        uri.push('?');
        for (n, (name, value)) in AUTH_PARAMS.iter().enumerate() {
            if n > 0 {
                uri.push('&');
            }
            uri.push_str(name);
            uri.push('=');
            uri.push_str(value);
        }
        uri
    }

    pub fn is_auth_succeeded_uri(uri: &str) -> bool {
        if let Some(pos) = uri.find('#') {
            let (url, _) = uri.split_at(pos);
            url.starts_with(AUTH_PARAMS[IDX_REDIRECT_URI].1)
        } else {
            false
        }
    }
}

#[test]
fn auth_response_from_str() {
    // correct
    let atr = "https://oauth.vk.com/blank.html#access_token=7de238693ce753e240abd1e1845480667bed442daf94e86df12d6c376322137fd2649661fbfd1352741fa&expires_in=86400&user_id=184946538"
        .parse::<AuthResponse>();
    assert!(atr.is_ok());
    let atr = atr.unwrap();
    assert_eq!(
        &atr.access_token,
        "7de238693ce753e240abd1e1845480667bed442daf94e86df12d6c376322137fd2649661fbfd1352741fa"
    );
    assert!(atr.expires_on.is_some());
    assert_eq!(&atr.user_id, "184946538");

    // another correct variant
    let atr2 = "abc#access_token=1234&expires_in=1&user_id=1".parse::<AuthResponse>();
    assert!(atr2.is_ok());
    let atr2 = atr2.unwrap();
    assert_eq!(&atr2.access_token, "1234");
    assert!(atr2.expires_on.is_some());
    assert_eq!(&atr2.user_id, "1");

    // incorrect variants
    assert!("abc#access_token=1234&expires_in=&user_id=1"
        .parse::<AuthResponse>()
        .unwrap()
        .expires_on
        .is_none());
    assert!("abc#access_token=1234&expires_in_=1&user_id=1"
        .parse::<AuthResponse>()
        .unwrap()
        .expires_on
        .is_none());
    assert!("abc#access_token=1234&expires_in=_1&user_id=1"
        .parse::<AuthResponse>()
        .unwrap()
        .expires_on
        .is_none());

    assert!("abc#access_token=&expires_in=1&user_id=1"
        .parse::<AuthResponse>()
        .is_err());
    assert!("abc#access_token_=1234&expires_in=1&user_id=1"
        .parse::<AuthResponse>()
        .is_err());

    assert!("abc#access_token=1234&expires_in=1&user_id="
        .parse::<AuthResponse>()
        .is_err());
    assert!("abc#access_token=1234&expires_in=1&user_id_=1"
        .parse::<AuthResponse>()
        .is_err());
}

#[test]
fn timestamp_conversion() {
    let ts_from: i64 = 12345678;
    let dt = Local.timestamp(ts_from, 0);
    let ts_to = dt.timestamp();
    assert_eq!(ts_from - ts_to, 0);
}
