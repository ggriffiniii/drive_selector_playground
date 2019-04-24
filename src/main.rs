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
