extern crate base64;
extern crate uuid;

use uuid::Uuid;

pub fn encode_base64_basic_token(user: String, pass: Option<String>) -> String {
    let token = format!("{}:{}", user,
                        if let Some(p) = pass { p } else { "".to_string() });
    base64::encode_config(&token, base64::URL_SAFE)
}

pub fn decode_base64_basic_token(token: String)
                                 -> Result<(String, Option<String>), String> {
    base64::decode_config(token.trim(), base64::URL_SAFE)
        .map_err(|e| format!("Base64 Decode error: {:?}", e))
        .and_then(|bytes| String::from_utf8(bytes)
            .map_err(|e| format!("UTF-8 Parse error: {:?}", e)))
        .map(|st| {
            let v: Vec<&str> = st.splitn(2, ":").collect();
            (v[0].to_string(),
             v.get(1).and_then(|s| if s.len() > 0 {
                 Some(s.to_string())
             } else {
                 None
            }))
        })
}

pub fn generate_base64_token() -> String {
    let rand_id = Uuid::new_v4();
    let rand_bytes = rand_id.as_bytes();
    base64::encode_config(rand_bytes, base64::URL_SAFE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_no_pw() {
        let username = "test".to_string();
        let password: Option<String> = None;

        let token = encode_base64_basic_token(username.clone(), password);
        let res = decode_base64_basic_token(token).unwrap();
        assert_eq!(username, res.0);
        assert!(res.1.is_none());
    }

    #[test]
    fn test_decode_with_pw() {
        let username = "test".to_string();
        let password: Option<String> = Some("cat".to_string());

        let token = encode_base64_basic_token(username.clone(), password);
        let res = decode_base64_basic_token(token).unwrap();
        assert_eq!(username, res.0);
        assert_eq!(res.1.unwrap(), "cat");
    }

    #[test]
    fn test_decode_with_pw_spaces() {
        let username = "test".to_string();
        let password: Option<String> = Some("   ".to_string());

        let token = encode_base64_basic_token(username.clone(), password);
        let res = decode_base64_basic_token(token).unwrap();
        assert_eq!(username, res.0);
        assert_eq!(res.1.unwrap(), "   ");
    }

}