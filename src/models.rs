use jose_jwk::jose_b64::serde::Bytes;
use rsa::RsaPublicKey;
use rsa::pkcs1::LineEnding;
use rsa::pkcs8::{DecodePublicKey, EncodePublicKey};
use serde::Serialize;
use uuid::Uuid;

const SUBJECT_TYPES_SUPPORTED: &[SubjectType] = &[SubjectType::Public];

const RESPONSE_TYPES_SUPPORTED: &[ResponseType] = &[ResponseType::IdToken];

const SIGNING_ALGORITHMS_SUPPORTED: &[SigningAlg] = &[SigningAlg::RS256];

const CLAIMS_SUPPORTED: &[Claim] = &[
    Claim::Sub,
    Claim::Aud,
    Claim::Exp,
    Claim::Iat,
    Claim::Iss,
    Claim::BranchName,
    Claim::Compliant,
    Claim::DeploymentEnvironmentUuid,
    Claim::PipelineUuid,
    Claim::RepositoryUuid,
    Claim::StepUuid,
    Claim::WorkspaceUuid,
];

const SCOPES_SUPPORTED: &[Scope] = &[Scope::OpenID];

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
enum SubjectType {
    Public,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
enum ResponseType {
    IdToken,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
enum Claim {
    Sub,
    Aud,
    Exp,
    Iat,
    Iss,
    BranchName,
    Compliant,
    DeploymentEnvironmentUuid,
    PipelineUuid,
    RepositoryUuid,
    StepUuid,
    WorkspaceUuid,
}

#[derive(Debug, Clone, Serialize)]
enum SigningAlg {
    RS256,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "UPPERCASE")]
enum KeyType {
    Rsa,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
enum Scope {
    OpenID,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct OpenIDConfigurationSchema {
    issuer: String,
    jwks_uri: String,
    subject_types_supported: &'static [SubjectType],
    response_types_supported: &'static [ResponseType],
    claims_supported: &'static [Claim],
    id_token_signing_alg_values_supported: &'static [SigningAlg],
    scopes_supported: &'static [Scope],
}
impl OpenIDConfigurationSchema {
    pub(super) fn new(issuer: String, jwks_uri: String) -> Self {
        Self {
            issuer,
            jwks_uri,
            subject_types_supported: SUBJECT_TYPES_SUPPORTED,
            response_types_supported: RESPONSE_TYPES_SUPPORTED,
            claims_supported: CLAIMS_SUPPORTED,
            id_token_signing_alg_values_supported: SIGNING_ALGORITHMS_SUPPORTED,
            scopes_supported: SCOPES_SUPPORTED,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct JWKSetSchema {
    keys: Vec<JWKSchema>,
}

impl JWKSetSchema {
    pub(super) fn new(keys: Vec<JWKSchema>) -> Self {
        Self { keys }
    }
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct JWKSchema {
    kid: String,
    kty: KeyType,
    alg: SigningAlg,
    e: Bytes,
    n: Bytes,
}

// impl From<RsaPublicKey> for JWK {
//     fn from(val: RsaPublicKey) -> Self {
//         Self {
//             issuer: val.issuer,
//             jwks_uri: val.jwks_uri,
//             subject_types_supported: SUBJECT_TYPES_SUPPORTED,
//             response_types_supported: RESPONSE_TYPES_SUPPORTED,
//             claims_supported: CLAIMS_SUPPORTED,
//             id_token_signing_alg_values_supported: SIGNING_ALGORITHMS_SUPPORTED,
//             scopes_supported: SCOPES_SUPPORTED,
//         }
//     }
// }

impl From<&str> for JWKSchema {
    fn from(val: &str) -> Self {
        let pubkey = RsaPublicKey::from_public_key_pem(val).expect("invalid public key");
        let key = jose_jwk::Rsa::from(&pubkey);

        let kid = Uuid::new_v5(
            &Uuid::NAMESPACE_OID,
            pubkey.to_public_key_pem(LineEnding::LF).unwrap().as_bytes(),
        );

        Self {
            kid: kid.to_string(),
            kty: KeyType::Rsa,
            alg: SigningAlg::RS256,
            e: key.e,
            n: key.n,
        }
    }
}
