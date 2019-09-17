use std::fmt;
use std::str::FromStr;

use getset::{Getters, MutGetters, Setters};

use crate::attribute::AttributePairs;
use crate::types::{DecimalResolution, HdcpLevel, ProtocolVersion};
use crate::utils::parse_u64;
use crate::utils::{quote, tag, unquote};
use crate::Error;

/// [4.3.4.3. EXT-X-I-FRAME-STREAM-INF]
///
/// [4.3.4.3. EXT-X-I-FRAME-STREAM-INF]: https://tools.ietf.org/html/rfc8216#section-4.3.4.3
#[derive(Getters, Setters, MutGetters, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtXIFrameStreamInf {
    #[get = "pub"]
    #[set = "pub"]
    #[get_mut = "pub"]
    /// The URI, that identifies the associated media playlist.
    uri: String,
    #[get = "pub"]
    #[set = "pub"]
    #[get_mut = "pub"]
    /// The peak segment bit rate of the variant stream.
    bandwidth: u64,
    #[get = "pub"]
    #[set = "pub"]
    #[get_mut = "pub"]
    /// The average segment bit rate of the variant stream.
    average_bandwidth: Option<u64>,
    #[get = "pub"]
    #[set = "pub"]
    #[get_mut = "pub"]
    /// A string that represents the list of codec types contained the variant stream.
    codecs: Option<String>,
    /// The optimal pixel resolution at which to display all the video in the variant stream.
    resolution: Option<DecimalResolution>,
    #[get = "pub"]
    #[set = "pub"]
    #[get_mut = "pub"]
    /// The HDCP level of the variant stream.
    hdcp_level: Option<HdcpLevel>,
    #[get = "pub"]
    #[set = "pub"]
    #[get_mut = "pub"]
    /// The group identifier for the video in the variant stream.
    video: Option<String>,
}

impl ExtXIFrameStreamInf {
    pub(crate) const PREFIX: &'static str = "#EXT-X-I-FRAME-STREAM-INF:";

    /// Makes a new `ExtXIFrameStreamInf` tag.
    pub fn new<T: ToString>(uri: T, bandwidth: u64) -> Self {
        ExtXIFrameStreamInf {
            uri: uri.to_string(),
            bandwidth,
            average_bandwidth: None,
            codecs: None,
            resolution: None,
            hdcp_level: None,
            video: None,
        }
    }

    /// The optimal pixel resolution at which to display all the video in the variant stream.
    pub fn resolution(&self) -> Option<(usize, usize)> {
        if let Some(res) = &self.resolution {
            Some((res.width(), res.height()))
        } else {
            None
        }
    }

    /// Sets the optimal pixel resolution at which to display all the video in the variant stream.
    pub fn set_resolution(&mut self, width: usize, height: usize) -> &mut Self {
        if let Some(res) = &mut self.resolution {
            res.set_width(width);
            res.set_height(height);
        } else {
            self.resolution = Some(DecimalResolution::new(width, height));
        }
        self
    }
    /// Returns the protocol compatibility version that this tag requires.
    pub const fn requires_version(&self) -> ProtocolVersion {
        ProtocolVersion::V1
    }
}

impl fmt::Display for ExtXIFrameStreamInf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::PREFIX)?;
        write!(f, "URI={}", quote(&self.uri))?;
        write!(f, ",BANDWIDTH={}", self.bandwidth)?;

        if let Some(value) = &self.average_bandwidth {
            write!(f, ",AVERAGE-BANDWIDTH={}", value)?;
        }
        if let Some(value) = &self.codecs {
            write!(f, ",CODECS={}", quote(value))?;
        }
        if let Some(value) = &self.resolution {
            write!(f, ",RESOLUTION={}", value)?;
        }
        if let Some(value) = &self.hdcp_level {
            write!(f, ",HDCP-LEVEL={}", value)?;
        }
        if let Some(value) = &self.video {
            write!(f, ",VIDEO={}", quote(value))?;
        }
        Ok(())
    }
}

impl FromStr for ExtXIFrameStreamInf {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = tag(input, Self::PREFIX)?;

        let mut uri = None;
        let mut bandwidth = None;
        let mut average_bandwidth = None;
        let mut codecs = None;
        let mut resolution = None;
        let mut hdcp_level = None;
        let mut video = None;

        for (key, value) in input.parse::<AttributePairs>()? {
            match key.as_str() {
                "URI" => uri = Some(unquote(value)),
                "BANDWIDTH" => bandwidth = Some(parse_u64(value)?),
                "AVERAGE-BANDWIDTH" => average_bandwidth = Some(parse_u64(value)?),
                "CODECS" => codecs = Some(unquote(value)),
                "RESOLUTION" => resolution = Some(value.parse()?),
                "HDCP-LEVEL" => hdcp_level = Some(value.parse()?),
                "VIDEO" => video = Some(unquote(value)),
                _ => {
                    // [6.3.1. General Client Responsibilities]
                    // > ignore any attribute/value pair with an unrecognized AttributeName.
                }
            }
        }

        let uri = uri.ok_or(Error::missing_value("URI"))?;
        let bandwidth = bandwidth.ok_or(Error::missing_value("BANDWIDTH"))?;

        Ok(ExtXIFrameStreamInf {
            uri,
            bandwidth,
            average_bandwidth,
            codecs,
            resolution,
            hdcp_level,
            video,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display() {
        let text = r#"#EXT-X-I-FRAME-STREAM-INF:URI="foo",BANDWIDTH=1000"#;
        assert_eq!(ExtXIFrameStreamInf::new("foo", 1000).to_string(), text);
    }

    #[test]
    fn test_parser() {
        let text = r#"#EXT-X-I-FRAME-STREAM-INF:URI="foo",BANDWIDTH=1000"#;
        let i_frame_stream_inf = ExtXIFrameStreamInf::new("foo", 1000);
        assert_eq!(
            text.parse::<ExtXIFrameStreamInf>().unwrap(),
            i_frame_stream_inf.clone()
        );

        assert_eq!(i_frame_stream_inf.uri(), "foo");
        assert_eq!(*i_frame_stream_inf.bandwidth(), 1000);
        // TODO: test all the optional fields
    }

    #[test]
    fn test_requires_version() {
        assert_eq!(
            ExtXIFrameStreamInf::new("foo", 1000).requires_version(),
            ProtocolVersion::V1
        );
    }
}
