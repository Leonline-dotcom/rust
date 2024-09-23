use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;


/*
Reviews have
    user who made review
    contents of review
*/

#[derive(Serialize, Deserialize, Debug)]
pub struct Review {
    pub user: String,
    pub review: String,
}

/*
 Movies should have
    title of movie
    Possible list of user reviews
 */

#[derive(Serialize, Deserialize, Debug)]
pub struct Movie {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub reviews: Vec<Review>,
}
