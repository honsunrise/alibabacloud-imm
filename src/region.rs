//! IMM service regions.
//!
//! Official documentation: <https://www.alibabacloud.com/help/en/imm/developer-reference/api-imm-2020-09-30-endpoint>

/// Alibaba Cloud regions that support the IMM service.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Region {
    /// China (Qingdao)
    CnQingdao,
    /// China (Beijing)
    CnBeijing,
    /// China (Zhangjiakou)
    CnZhangjiakou,
    /// China (Ulanqab)
    CnWulanchabu,
    /// China (Hangzhou)
    CnHangzhou,
    /// China (Shanghai)
    CnShanghai,
    /// China (Shenzhen)
    CnShenzhen,
    /// China (Guangzhou)
    CnGuangzhou,
    /// China (Chengdu)
    CnChengdu,
    /// China (Hong Kong)
    CnHongkong,
    /// Singapore
    ApSoutheast1,
    /// Australia (Sydney)
    ApSoutheast2,
    /// Indonesia (Jakarta)
    ApSoutheast5,
    /// India (Mumbai)
    ApSouth1,
    /// Germany (Frankfurt)
    EuCentral1,
    /// UK (London)
    EuWest1,
    /// US (Virginia)
    UsEast1,
    /// US (Silicon Valley)
    UsWest1,
    /// China North 2 (Government Cloud)
    CnNorth2Gov1,
    /// Custom region ID not in the predefined list.
    Custom(String),
}

impl Region {
    /// Returns the region ID string (e.g. `"cn-hangzhou"`).
    pub fn id(&self) -> &str {
        match self {
            Self::CnQingdao => "cn-qingdao",
            Self::CnBeijing => "cn-beijing",
            Self::CnZhangjiakou => "cn-zhangjiakou",
            Self::CnWulanchabu => "cn-wulanchabu",
            Self::CnHangzhou => "cn-hangzhou",
            Self::CnShanghai => "cn-shanghai",
            Self::CnShenzhen => "cn-shenzhen",
            Self::CnGuangzhou => "cn-guangzhou",
            Self::CnChengdu => "cn-chengdu",
            Self::CnHongkong => "cn-hongkong",
            Self::ApSoutheast1 => "ap-southeast-1",
            Self::ApSoutheast2 => "ap-southeast-2",
            Self::ApSoutheast5 => "ap-southeast-5",
            Self::ApSouth1 => "ap-south-1",
            Self::EuCentral1 => "eu-central-1",
            Self::EuWest1 => "eu-west-1",
            Self::UsEast1 => "us-east-1",
            Self::UsWest1 => "us-west-1",
            Self::CnNorth2Gov1 => "cn-north-2-gov-1",
            Self::Custom(id) => id,
        }
    }

    /// Returns the public endpoint URL.
    pub fn public_endpoint(&self) -> String {
        format!("https://imm.{}.aliyuncs.com", self.id())
    }

    /// Returns the VPC endpoint URL.
    pub fn vpc_endpoint(&self) -> String {
        format!("https://imm-vpc.{}.aliyuncs.com", self.id())
    }
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.id())
    }
}

impl AsRef<str> for Region {
    fn as_ref(&self) -> &str {
        self.id()
    }
}

impl<T: AsRef<str> + ?Sized> From<&T> for Region {
    fn from(s: &T) -> Self {
        match s.as_ref() {
            "cn-qingdao" => Self::CnQingdao,
            "cn-beijing" => Self::CnBeijing,
            "cn-zhangjiakou" => Self::CnZhangjiakou,
            "cn-wulanchabu" => Self::CnWulanchabu,
            "cn-hangzhou" => Self::CnHangzhou,
            "cn-shanghai" => Self::CnShanghai,
            "cn-shenzhen" => Self::CnShenzhen,
            "cn-guangzhou" => Self::CnGuangzhou,
            "cn-chengdu" => Self::CnChengdu,
            "cn-hongkong" => Self::CnHongkong,
            "ap-southeast-1" => Self::ApSoutheast1,
            "ap-southeast-2" => Self::ApSoutheast2,
            "ap-southeast-5" => Self::ApSoutheast5,
            "ap-south-1" => Self::ApSouth1,
            "eu-central-1" => Self::EuCentral1,
            "eu-west-1" => Self::EuWest1,
            "us-east-1" => Self::UsEast1,
            "us-west-1" => Self::UsWest1,
            "cn-north-2-gov-1" => Self::CnNorth2Gov1,
            other => Self::Custom(other.to_string()),
        }
    }
}

impl From<String> for Region {
    fn from(s: String) -> Self {
        let region = Self::from(s.as_str());
        if matches!(region, Self::Custom(_)) {
            Self::Custom(s)
        } else {
            region
        }
    }
}
