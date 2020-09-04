use ::url::Url;

#[derive(thiserror::Error, Debug)]
pub enum UrlError {
    #[error(transparent)]
    ParseFailed(#[from] ::url::ParseError),
    #[error("URL is not a valid fully-qualified HTTP URL")]
    InvalidURLFormat,
    #[error("URL is not a known Buildkite URL format")]
    UnknownFormat,
}

pub fn build_number(input: &str) -> Result<(String, String, String), UrlError> {
    let parsed = Url::parse(input)?;

    if parsed.cannot_be_a_base() {
        return Err(UrlError::InvalidURLFormat);
    }

    let segments = if let Some(segments) = parsed.path_segments() {
        segments.collect::<Vec<_>>()
    } else {
        return Err(UrlError::InvalidURLFormat);
    };

    match parsed.host_str() {
        Some("buildkite.com") => match segments.as_slice() {
            [organization, pipeline, "builds", raw_number, ..] => Ok((
                organization.to_string(),
                pipeline.to_string(),
                raw_number.to_string(),
            )),
            _ => Err(UrlError::UnknownFormat),
        },
        Some("api.buildkite.com") => match segments.as_slice() {
            ["v2", "organizations", organization, "pipelines", pipeline, "builds", raw_number] => Ok((
                organization.to_string(),
                pipeline.to_string(),
                raw_number.to_string(),
            )),
            _ => Err(UrlError::UnknownFormat),
        },
        _ => Err(UrlError::UnknownFormat),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_a_web_url() {
        let input = "https://buildkite.com/my-org/my-project/builds/87";
        let expected = (
            "my-org".to_string(),
            "my-project".to_string(),
            "87".to_string(),
        );

        let subject = build_number(input).expect("Unable to parse valid URL");

        assert_eq!(subject, expected);
    }

    #[test]
    fn it_parses_an_api_url() {
        let input = "https://api.buildkite.com/v2/organizations/my-great-org/pipelines/my-pipeline/builds/1";
        let expected = (
            "my-great-org".to_string(),
            "my-pipeline".to_string(),
            "1".to_string(),
        );

        let subject = build_number(input).expect("Unable to parse valid URL");

        assert_eq!(subject, expected);
    }
}
