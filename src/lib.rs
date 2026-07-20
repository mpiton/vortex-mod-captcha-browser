//! Human-assisted CAPTCHA fallback marker for Vortex.

#[cfg(target_family = "wasm")]
mod plugin_api;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CaptchaRequest {
    pub challenge_id: String,
    pub challenge_type: String,
    pub challenge_url: String,
    pub image_data: Option<String>,
}

pub fn handle_can_solve(input: &str) -> Result<String, String> {
    let request = parse_request(input)?;
    let supported = match request.challenge_type.as_str() {
        "image" => request.image_data.is_some(),
        "text_input" => true,
        _ => false,
    };
    Ok(supported.to_string())
}

pub fn interaction_required_response(input: &str) -> Result<String, String> {
    if handle_can_solve(input)? != "true" {
        return Err("unsupported CAPTCHA type".into());
    }
    Ok(r#"{"status":"interaction_required"}"#.into())
}

fn parse_request(input: &str) -> Result<CaptchaRequest, String> {
    serde_json::from_str(input).map_err(|_| "invalid CAPTCHA request".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn browser_supports_only_non_javascript_manual_challenges() {
        let image = r#"{"challenge_id":"1","challenge_type":"image","challenge_url":"https://example.test","image_data":"aW1hZ2U="}"#;
        let text = r#"{"challenge_id":"1","challenge_type":"text_input","challenge_url":"https://example.test","image_data":null}"#;
        let recaptcha = r#"{"challenge_id":"1","challenge_type":"recaptcha_v2","challenge_url":"https://example.test","image_data":null}"#;
        assert_eq!(handle_can_solve(image).unwrap(), "true");
        assert_eq!(handle_can_solve(text).unwrap(), "true");
        assert_eq!(handle_can_solve(recaptcha).unwrap(), "false");
    }

    #[test]
    fn solve_requests_host_managed_interaction() {
        let input = r#"{"challenge_id":"1","challenge_type":"text_input","challenge_url":"https://example.test","image_data":null}"#;
        assert_eq!(
            interaction_required_response(input).unwrap(),
            r#"{"status":"interaction_required"}"#
        );
    }
}
