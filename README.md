# alibabacloud-imm — Alibaba Cloud IMM SDK for Rust

[![Rust](https://img.shields.io/badge/rust-1.88%2B-brightgreen.svg)](https://www.rust-lang.org)
[![Crates.io](https://img.shields.io/crates/v/alibabacloud-imm.svg)](https://crates.io/crates/alibabacloud-imm)
[![Documentation](https://docs.rs/alibabacloud-imm/badge.svg)](https://docs.rs/alibabacloud-imm)
[![MIT/Apache-2 licensed](https://img.shields.io/crates/l/alibabacloud-imm.svg)](LICENSE-APACHE)

A **modern**, **complete**, and **reqwest-powered** Rust SDK for Alibaba
Cloud Intelligent Media Management (IMM) service. Covers the full
`2020-09-30` API version with type-safe operations and ergonomic
builder-style parameters.

## Highlights

- **Full API surface** — **104 operations** across every category:
  project management, dataset/binding CRUD, image detection (faces,
  labels, bodies, cars, codes, cropping, text OCR), media transcoding,
  office document conversion, file compression, content moderation,
  figure/location/similar clustering, story management, triggers, batch
  processing, and more.
- **reqwest-powered** — built on `reqwest` 0.13 with full async/await
  and `default-tls` enabled by default.
- **ACS3 signing + flexible credentials** — `ACS3-HMAC-SHA256` signature
  with a default credentials chain covering explicit AK/SK, environment
  variables, and STS tokens.
- **Type-safe** — every API is one `struct` + one sub-trait; responses
  are strongly typed JSON deserialization.
- **Modular** — one file per API category under `src/ops/`, with
  `Operations` aggregate supertraits for convenient blanket imports.
- **RPC style** — faithfully implements Alibaba Cloud's RPC calling
  convention (`POST /` with `Action` header, query string + form body
  parameters).

## Quick start

```toml
[dependencies]
alibabacloud-imm = "0.1"
tokio = { version = "1", features = ["full"] }
```

### TLS features

The crate mirrors `reqwest`'s TLS feature names so downstream users can choose
their TLS backend directly:

```toml
# Default: enables reqwest's default-tls backend.
alibabacloud-imm = "0.1"

# Use reqwest with rustls and an included crypto provider.
alibabacloud-imm = { version = "0.1", default-features = false, features = ["rustls"] }

# Use rustls without selecting a crypto provider in this crate.
alibabacloud-imm = { version = "0.1", default-features = false, features = ["rustls-no-provider"] }

# Use the platform-native TLS backend.
alibabacloud-imm = { version = "0.1", default-features = false, features = ["native-tls"] }
```

Available TLS features are `default-tls`, `rustls`, `rustls-no-provider`,
`native-tls`, `native-tls-vendored`, `native-tls-no-alpn`, and
`native-tls-vendored-no-alpn`.

### Hello, IMM

```rust,no_run
use alibabacloud_imm::{Client, Region};
use alibabacloud_imm::ops::project::{CreateProjectOps, CreateProjectQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .region(Region::CnHangzhou)
        .access_key_id("your-access-key-id")
        .access_key_secret("your-access-key-secret")
        .build()?;

    let resp = client
        .create_project(CreateProjectQuery {
            project_name: "my-project".into(),
            description: Some("My first IMM project".into()),
            ..Default::default()
        })
        .await?;

    println!("RequestId: {}", resp.request_id);
    Ok(())
}
```

### Image face detection

```rust,no_run
use alibabacloud_imm::Client;
use alibabacloud_imm::ops::detection::{DetectImageFacesOps, DetectImageFacesQuery};

# async fn demo(client: &Client) -> Result<(), alibabacloud_imm::Error> {
let resp = client
    .detect_image_faces(DetectImageFacesQuery {
        project_name: "my-project".into(),
        source_uri: Some("oss://my-bucket/photo.jpg".into()),
        ..Default::default()
    })
    .await?;

println!("Detected {} faces", resp.faces.len());
# Ok(())
# }
```

### Media transcoding

```rust,no_run
use alibabacloud_imm::Client;
use alibabacloud_imm::ops::media::{CreateMediaConvertTaskOps, CreateMediaConvertTaskQuery};

# async fn demo(client: &Client) -> Result<(), alibabacloud_imm::Error> {
let resp = client
    .create_media_convert_task(CreateMediaConvertTaskQuery {
        project_name: "my-project".into(),
        sources: serde_json::json!([{"URI": "oss://bucket/input.mp4"}]),
        targets: Some(serde_json::json!([{"URI": "oss://bucket/output.mp4", "Video": {"Codec": "h264"}}])),
        ..Default::default()
    })
    .await?;

println!("TaskId: {:?}", resp.task_id);
# Ok(())
# }
```

## API coverage at a glance

| Category                    | Operations                                                                                                                                              |
| --------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Project** (5)             | CreateProject, GetProject, UpdateProject, DeleteProject, ListProjects                                                                                   |
| **Dataset** (5)             | CreateDataset, GetDataset, UpdateDataset, DeleteDataset, ListDatasets                                                                                   |
| **Binding** (4)             | CreateBinding, GetBinding, DeleteBinding, ListBindings                                                                                                  |
| **OSS Bucket** (4)          | AttachOSSBucket, DetachOSSBucket, GetOSSBucketAttachment, ListAttachedOSSBuckets                                                                        |
| **File Meta** (8)           | Index/Get/Update/Delete FileMeta + Batch variants                                                                                                       |
| **Query** (3)               | SimpleQuery, SemanticQuery, FuzzyQuery                                                                                                                  |
| **Detection** (10)          | DetectImage{Faces,Bodies,Cars,Codes,Cropping,Labels,Score,Texts}, DetectMediaMeta, DetectTextAnomaly                                                    |
| **Image** (5)               | AddImageMosaic, CompareImageFaces, EncodeBlindWatermark, CreateImageSplicingTask, CreateImageToPDFTask                                                  |
| **Media** (5)               | CreateMediaConvertTask, CreateVideoLabelClassificationTask, GetVideoLabelClassificationResult, CreateHighlightTask, GenerateVideoPlaylist               |
| **Document** (4)            | CreateOfficeConversionTask, GenerateWebofficeToken, RefreshWebofficeToken, ExtractDocumentText                                                          |
| **File Processing** (3)     | CreateFileCompressionTask, CreateFileUncompressionTask, CreateArchiveFileInspectionTask                                                                 |
| **Moderation** (6)          | CreateImage/VideoModerationTask, GetImage/VideoModerationResult, CreateDecodeBlindWatermarkTask, GetDecodeBlindWatermarkResult                          |
| **Figure Clustering** (8)   | CreateFigureClusteringTask, CreateFigureClustersMergingTask, Get/BatchGet/Query/UpdateFigureCluster, SearchImageFigureCluster, CreateFacesSearchingTask |
| **Location Clustering** (4) | CreateLocationDateClusteringTask, Query/Update/DeleteLocationDateCluster                                                                                |
| **Similar Clustering** (2)  | CreateSimilarImageClusteringTask, QuerySimilarImageClusters                                                                                             |
| **Story** (8)               | Create/CreateCustomized/Get/Query/Update/Delete Story, Add/RemoveStoryFiles                                                                             |
| **Task** (2)                | GetTask, ListTasks                                                                                                                                      |
| **Trigger** (7)             | Create/Get/List/Update/Delete/Suspend/Resume Trigger                                                                                                    |
| **Batch** (7)               | Create/Get/List/Update/Delete/Suspend/Resume Batch                                                                                                      |
| **Region** (1)              | ListRegions                                                                                                                                             |
| **Contextual** (2)          | ContextualAnswer, ContextualRetrieval                                                                                                                   |
| **Point Cloud** (1)         | CreateCompressPointCloudTask                                                                                                                            |

## Configuration

### Client builder

```rust,no_run
use std::time::Duration;
use alibabacloud_imm::{Client, Region};

# fn demo() -> Result<(), alibabacloud_imm::Error> {
let client = Client::builder()
    .region(Region::CnShanghai)
    .access_key_id("your-access-key-id")
    .access_key_secret("your-access-key-secret")
    .security_token("your-sts-token")           // optional
    .http_timeout(Duration::from_secs(60))
    .build()?;

// Use VPC internal endpoint
let vpc_client = Client::builder()
    .region(Region::CnShanghai)
    .vpc()
    .access_key_id("your-access-key-id")
    .access_key_secret("your-access-key-secret")
    .build()?;

// Custom region ID (not in the predefined list)
let custom_client = Client::builder()
    .region("cn-fuzhou")
    .access_key_id("your-access-key-id")
    .access_key_secret("your-access-key-secret")
    .build()?;

// Explicit endpoint (overrides region)
let explicit_client = Client::builder()
    .endpoint("https://imm.cn-shanghai.aliyuncs.com")
    .access_key_id("your-access-key-id")
    .access_key_secret("your-access-key-secret")
    .build()?;
# Ok(())
# }
```

The `.region()` method accepts:
- `Region` enum variants (e.g. `Region::CnHangzhou`) — type-safe with IDE auto-completion
- String literals (e.g. `"cn-hangzhou"`) — known IDs map to enum variants, unknown ones become `Region::Custom`

### Credentials

If neither `access_key_id` / `access_key_secret` nor an explicit
`credentials_provider` is supplied, the builder falls back to
`DefaultCredentialsChain`, which walks:

1. `EnvironmentCredentialsProvider` — reads
   `ALIBABA_CLOUD_ACCESS_KEY_ID` / `ALIBABA_CLOUD_ACCESS_KEY_SECRET`
   (+ optional `ALIBABA_CLOUD_SECURITY_TOKEN`) or the `ALICLOUD_*`
   equivalents.
2. `RrsaCredentialsProvider::from_env` — RRSA/OIDC if its env vars exist.

### RRSA (ACK Pod Identity)

For Kubernetes workloads running in ACK (Alibaba Cloud Container Service)
with [RAM Roles for Service Accounts](https://help.aliyun.com/zh/ack/ack-managed-and-ack-dedicated/user-guide/use-rrsa-to-authorize-pods-to-access-different-cloud-services),
the SDK automatically detects RRSA environment variables and exchanges
the pod's OIDC token for temporary STS credentials:

```rust,no_run
use alibabacloud_imm::{Client, Region};
use alibabacloud_imm::credentials::RrsaCredentialsProvider;

# fn demo() -> Result<(), alibabacloud_imm::Error> {
// Option 1: Auto-detect from ACK-injected environment variables
// (ALIBABA_CLOUD_ROLE_ARN, ALIBABA_CLOUD_OIDC_PROVIDER_ARN,
//  ALIBABA_CLOUD_OIDC_TOKEN_FILE)
// This happens automatically with DefaultCredentialsChain:
let client = Client::builder()
    .region(Region::CnHangzhou)
    .build()?;

// Option 2: Explicit RRSA provider with custom configuration
let http = reqwest::Client::new();
let rrsa = RrsaCredentialsProvider::builder()
    .role_arn("acs:ram::123456789012:role/my-role")
    .oidc_provider_arn("acs:ram::123456789012:oidc-provider/my-provider")
    .oidc_token_file_path("/var/run/secrets/tokens/oidc-token")
    .http_client(http)
    .build()?;

let client = Client::builder()
    .region(Region::CnHangzhou)
    .credentials_provider(rrsa)
    .build()?;
# Ok(())
# }
```

The RRSA provider reads the following environment variables (injected by
ACK):

| Variable                                 | Required | Description                                                  |
| ---------------------------------------- | -------- | ------------------------------------------------------------ |
| `ALIBABA_CLOUD_ROLE_ARN`                 | Yes      | ARN of the RAM role to assume                                |
| `ALIBABA_CLOUD_OIDC_PROVIDER_ARN`        | Yes      | ARN of the OIDC identity provider                            |
| `ALIBABA_CLOUD_OIDC_TOKEN_FILE`          | Yes      | Path to the OIDC JWT token file                              |
| `ALIBABA_CLOUD_ROLE_SESSION_NAME`        | No       | Session name (auto-generated if absent)                      |
| `ALIBABA_CLOUD_STS_ENDPOINT`             | No       | Custom STS endpoint (defaults to `https://sts.aliyuncs.com`) |
| `ALIBABA_CLOUD_SESSION_DURATION_SECONDS` | No       | Session duration (defaults to 3600)                          |

Credentials are cached internally and refreshed automatically before
expiration (with a configurable 5-minute safety margin).

## Architecture

- **RPC-style API**: all requests use `POST /` with the operation
  identified via `x-acs-action` header. Parameters are passed as URL
  query string (`@query`) or form-encoded body (`@formData`).
- **One category = one file**: operations are grouped by domain under
  `src/ops/<category>.rs`, with per-operation `XxxOps` sub-traits and a
  category-level `XxxOperations` supertrait.
- **Uniform Ops trait**: every operation implements `Ops` with associated
  `Query`, `Body`, and `Response` types; `Client` blanket-implements
  all traits via the `Request<P>` internal dispatch.
- **Signing**: `ACS3-HMAC-SHA256` (Alibaba Cloud Signature V3).

### Core dependencies

- [`reqwest`](https://docs.rs/reqwest) — HTTP client with selectable TLS backends.
- [`serde`](https://docs.rs/serde) + [`serde_json`](https://docs.rs/serde_json) — parameter serialization and response parsing.
- [`hmac`](https://docs.rs/hmac) + [`sha2`](https://docs.rs/sha2) — ACS3 signing.
- [`jiff`](https://docs.rs/jiff) — timestamps.
- [`url`](https://docs.rs/url) — URL construction.

## Documentation

- [API docs on docs.rs](https://docs.rs/alibabacloud-imm)
- [IMM API reference](https://help.aliyun.com/zh/imm/developer-reference/api-imm-2020-09-30-overview)

## Contributing

PRs are welcome — please open an issue first for large changes. `cargo
check` + `cargo clippy` must both be clean.

## License

Licensed under either of

- Apache License 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

---

<p align="center"><strong>Happy coding with IMM and Rust! 🦀✨</strong></p>
