use std::sync::Arc;

use crate::db::image::get_profile_image;
use crate::db::user::{get_user_profile_by_username, update_user_by_id};
use crate::helpers::auth::get_session_user_id;
use crate::helpers::response::{build_error, build_response, build_standard_response};
use crate::models::users::UpdateUser;
use crate::TideState;
use tide::log::{error, warn};
use tide::Request;
use tide::Response;
use validator::Validate;

// Profile parameters struct
#[derive(Debug, serde::Deserialize, Validate)]
pub struct GetProfileParams {
    #[validate(length(min = 5, max = 50))]
    username: String,
}

// Profile parameters for getting the profile response body
#[derive(Debug, serde::Serialize)]
struct GetProfileResponseBody {
    display_name: String,
    bio: String,
    is_owner: bool,
    picture: String,
    following: Option<i32>,
    followers: Option<i32>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Validate)]
struct UpdateDisplayProfilePayload {
    display_name: Option<String>,
    bio: Option<String>,
}

// update profile response body
pub async fn update_display_profile(mut req: Request<Arc<TideState>>) -> tide::Result {
    // extract user id from session
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(err) => return Err(err),
    };
    // get json body as UpdateProfilePayload
    let update_body: UpdateDisplayProfilePayload = match req.body_json().await {
        Ok(body) => body,
        Err(_) => return build_error("Bad request body.".to_string(), 400),
    };

    // construct UpdateProfile model
    let update_user = UpdateUser {
        username: None,
        password: None,
        salt: None,
        email: None,
        is_private: None,
        bio: update_body.bio,
        display_name: update_body.display_name,
    };
    // get connection state
    let state = req.state();
    let mut conn = state.tide_pool.get().unwrap();
    // call orm
    return match update_user_by_id(&mut conn, user_id, &update_user).await {
        Ok(result) => build_standard_response(result, "".to_string(), 200),
        Err(err) => build_error(err, 400),
    };
}

pub async fn update_profile_image(mut req: Request<Arc<TideState>>) -> tide::Result {
    // get user_id from session
    build_standard_response(true, "".to_string(), 200)
}

// Get profile route
pub async fn get_profile(req: Request<Arc<TideState>>) -> tide::Result {
    let username = match req.param("username") {
        Ok(name) => name.to_owned(),
        // last match clause should not happen.
        Err(e) => return build_error(e.to_string(), 400),
    };
    // let ses = req.session().clone().validate();
    // println!("{}",ses.is_some());
    // println!("{}", req.session().id());
    // println!("{}", req.session().is_destroyed());
    // println!("{}", req.session().is_expired());
    // get relevant username session field
    let session_username: String = req.session().get("username").unwrap_or("".to_owned());
    println!("session username: {}", &session_username);
    log::info!("Obtained username in get_profile: {}", &username);

    let state = req.state();
    let mut conn = state.tide_pool.get().unwrap();

    // get profile view from database
    let profile_query_result = get_user_profile_by_username(&mut conn, &username).await;
    let is_owner = session_username == username;

    let res_body = match profile_query_result {
        Ok(profile) => {
            // TODO: we need to update is_private to profile.is_private when DB is updated
            let is_private = true;
            if !is_owner && is_private {
                // return object with certain fields defaulted to empty values
                GetProfileResponseBody {
                    display_name: profile.display_name,
                    bio: "".to_owned(),
                    picture: String::from("picture placeholder"),
                    is_owner: false,
                    followers: None,
                    following: None,
                }
            } else {
                // either is_owner or not private account, either ways all fields are accessible.
                // So return all fields.

                // get cdn_href from db
                let picture = get_profile_image(&mut conn, profile.id)
                    .await
                    .map(|img| img.img_src)
                    .unwrap_or_else(|e| {
                        warn!(
                            "Error in retrieving profile picture, using default. (error: {})",
                            e
                        );
                        String::from("")
                    });

                GetProfileResponseBody {
                    display_name: profile.display_name,
                    bio: profile.bio.unwrap_or("".to_owned()),
                    is_owner,
                    picture,
                    followers: Some(0),
                    following: Some(0),
                }
            }
        }
        Err(message) => {
            error!("error in retrieving profile: {}", message);
            return build_error(message, 500);
        }
    };

    build_response(res_body, 200)
}

pub async fn get_links(req: Request<Arc<TideState>>) -> tide::Result {
    Ok(Response::builder(200).build())
}
// delete profile response builder
pub async fn delete_profile(req: Request<Arc<TideState>>) -> tide::Result {
    // TODO: implementation
    Ok(Response::builder(200).build())
}
