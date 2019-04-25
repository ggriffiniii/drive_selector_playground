use std::path::Path;
use std::error::Error;
  
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

use yup_oauth2::{Authenticator, FlowType, ApplicationSecret, DiskTokenStorage,
                 DefaultAuthenticatorDelegate, read_application_secret};
use google_drive3 as gdrive;
use serde::{Deserialize, de::DeserializeOwned};
use drive_selector::DriveSelector;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug,Deserialize,DriveSelector)]
#[serde(rename_all = "camelCase")]
struct FileAttrs {
    id: String,
    mime_type: String,
    sharing_user: Option<UserInfo>,
    viewed_by_me_time: Option<DateTime<Utc>>,
    #[serde(default)]
    export_links: HashMap<String, String>,
}

#[derive(Debug,Deserialize, DriveSelector)]
#[serde(rename_all = "camelCase")]
struct UserInfo {
    me: bool,
    email_address: String,
}

fn main() {
    let drive = initialize_gdrive().unwrap();
    let response: Vec<FileAttrs> = list_files(&drive, 10).unwrap();
    for file in response {
        println!("{:?}", file);
    }
}

type Drive = gdrive::Drive<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, DiskTokenStorage, hyper::Client>>;

fn initialize_gdrive() -> Result<Drive, Box<Error>> {
    // Get an ApplicationSecret instance by some means. It contains the `client_id` and
    // `client_secret`, among other things.
    let secret: ApplicationSecret = read_application_secret(Path::new("credentials.json"))?;
    let client = hyper::Client::with_connector(
        HttpsConnector::new(NativeTlsClient::new()?));
    let authenticator = Authenticator::new(&secret,
                                           DefaultAuthenticatorDelegate,
                                           client,
                                           DiskTokenStorage::new(&"token_store.json".to_string())?,
                                           Some(FlowType::InstalledInteractive));
    let client = hyper::Client::with_connector(
        HttpsConnector::new(NativeTlsClient::new()?));
    Ok(gdrive::Drive::new(client, authenticator))
}

fn list_files<T>(drive: &Drive, page_size: i32) -> Result<Vec<T>, Box<Error>>
where
    T: DriveSelector + DeserializeOwned,
{
    #[derive(Debug,Deserialize,DriveSelector)]
    #[serde(rename_all = "camelCase")]
    struct ListResponse<T> where T: DriveSelector {
        files: Vec<T>,
    }
    let response: ListResponse<T> = drive.files().list().page_size(page_size).add_scope("https://www.googleapis.com/auth/drive").q("sharedWithMe=true").doit()?;
    Ok(response.files)
}

#[cfg(test)]
mod tests {
    use drive_selector::DriveSelector;
    use serde::Deserialize;

    #[derive(Deserialize, DriveSelector)]
    #[serde(rename_all = "camelCase")]
    struct File {
        id: String,
        mime_type: String,
        sharing_user: Option<UserInfo>,
    }

    #[derive(Deserialize, DriveSelector)]
    #[serde(rename_all = "camelCase")]
    struct UserInfo {
        me: bool,
        email_address: String,
    }

    #[test]
    fn basic() {
        #[derive(Deserialize, DriveSelector)]
        #[serde(rename_all = "camelCase")]
        struct Response {
            next_page_token: String,
            files: Vec<File>,
        }
        assert_eq!(
            Response::selector(),
            "nextPageToken,files(id,mimeType,sharingUser/me,sharingUser/emailAddress)"
        );
    }

    #[test]
    fn generic_with_flatten() {
        #[derive(Deserialize, DriveSelector)]
        #[serde(rename_all = "camelCase")]
        struct Response<T>
        where
            T: DriveSelector,
        {
            next_page_token: String,
            #[serde(flatten)]
            payload: T,
        }

        #[derive(Deserialize, DriveSelector)]
        #[serde(rename_all = "camelCase")]
        struct ListFiles {
            files: Vec<File>,
        }
        assert_eq!(
            Response::<ListFiles>::selector(),
            "nextPageToken,files(id,mimeType,sharingUser/me,sharingUser/emailAddress)"
        );
    }
}
