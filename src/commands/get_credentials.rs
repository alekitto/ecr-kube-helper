use aws_config::BehaviorVersion;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use clap::Args;
use k8s_openapi::api::core::v1::Secret;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use k8s_openapi::ByteString;
use kube::api::{DeleteParams, PostParams};
use kube::{Api, Client};
use serde::Serialize;
use serde_json::{Map, Value};
use std::collections::BTreeMap;

#[derive(Debug, Args)]
pub struct GetCredentialsArgs {
    /// The ECR registry endpoint to authenticate to. Use ecr-public or public.ecr.aws for public ECR gallery.
    target_ecr: String,
    /// The namespaces where to store the secret into.
    #[arg(long, short = 'n')]
    namespace: Option<Vec<String>>,
    /// The secret name.
    #[arg(long, short = 's')]
    secret: Option<String>,
    /// The email stored in the docker config secret. Not really used.
    #[arg(long, short = 'e')]
    email: Option<String>,
    /// Annotations to apply to the secret in JSON format.
    #[arg(long, short = 'a')]
    annotations: Option<String>,
}

pub async fn get_credentials_command(args: GetCredentialsArgs) {
    let client = Client::try_default().await;

    let mut namespace = args.namespace.unwrap_or_default();
    if namespace.is_empty() {
        namespace.push(if let Ok(client) = client.as_ref() {
            client.default_namespace().to_string()
        } else {
            "default".to_string()
        });
    }

    let secret = args
        .secret
        .unwrap_or_else(|| format!("{}-token", args.target_ecr.replace('.', "-")));
    let email = args
        .email
        .unwrap_or_else(|| "docker@example.com".to_string());
    let annotations = args.annotations.unwrap_or_else(|| "{}".to_string());
    let annotations = serde_json::from_str(&annotations).unwrap_or_else(|_| Default::default());

    let result =
        get_credentials_command_impl(args.target_ecr, namespace, secret, email, annotations).await;

    if let Err(e) = result {
        eprintln!("command failed: {}", e);
        eprintln!("{:#?}", e);
        panic!();
    }
}

async fn get_credentials_command_impl(
    target_ecr: String,
    namespaces: Vec<String>,
    secret: String,
    email: String,
    annotations: BTreeMap<String, String>,
) -> anyhow::Result<()> {
    let client = Client::try_default().await?;
    let auth_data = get_auth_data(target_ecr, email).await?;

    let mut secret_data = BTreeMap::new();
    secret_data.insert(
        ".dockerconfigjson".to_string(),
        ByteString(auth_data.into_bytes()),
    );

    for namespace in namespaces {
        let api = Api::<Secret>::namespaced(client.clone(), &namespace);
        let _ = api
            .delete(
                &secret,
                &DeleteParams {
                    dry_run: false,
                    ..Default::default()
                },
            )
            .await;

        api.create(
            &PostParams {
                dry_run: false,
                ..Default::default()
            },
            &Secret {
                type_: Some("kubernetes.io/dockerconfigjson".to_string()),
                metadata: ObjectMeta {
                    name: Some(secret.clone()),
                    namespace: Some(namespace),
                    annotations: Some(annotations.clone()),
                    ..Default::default()
                },
                data: Some(secret_data.clone()),
                ..Default::default()
            },
        )
        .await?;
    }

    Ok(())
}

async fn get_auth_data(target_ecr: String, email: String) -> anyhow::Result<String> {
    let (auth_token, endpoint) = if target_ecr == "ecr-public" || target_ecr == "public.ecr.aws" {
        let shared_config = aws_config::defaults(BehaviorVersion::v2023_11_09())
            .region("us-east-1")
            .load()
            .await;
        let aws_client = aws_sdk_ecrpublic::Client::new(&shared_config);
        let token = aws_client.get_authorization_token().send().await?;

        (
            token
                .authorization_data
                .unwrap()
                .authorization_token
                .unwrap(),
            "public.ecr.aws".to_string(),
        )
    } else {
        let shared_config = aws_config::load_defaults(BehaviorVersion::v2023_11_09()).await;
        let aws_client = aws_sdk_ecr::Client::new(&shared_config);
        let token = aws_client.get_authorization_token().send().await?;

        let auth_data = token.authorization_data.unwrap();
        let auth_data = auth_data.first().unwrap();

        (
            auth_data.authorization_token.clone().unwrap(),
            auth_data.proxy_endpoint.clone().unwrap(),
        )
    };

    let message = String::from_utf8(STANDARD.decode(&auth_token)?)?;
    let parts = message.splitn(2, ':').collect::<Vec<_>>();
    let username = parts.first().unwrap().to_string();
    let password = parts.get(1).unwrap().to_string();

    #[derive(Serialize)]
    struct EndpointData {
        username: String,
        password: String,
        email: String,
        auth: String,
    }

    #[derive(Serialize)]
    struct AuthData {
        auths: Map<String, Value>,
    }

    let mut auths = Map::new();
    auths.insert(
        endpoint,
        serde_json::to_value(EndpointData {
            username,
            password,
            email,
            auth: auth_token,
        })?,
    );

    Ok(serde_json::to_string(&AuthData { auths })?)
}
