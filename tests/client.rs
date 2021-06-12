// use chrono::{Datelike, Local};
// use talenta::api;

use talenta::client::Client;

#[test]
#[should_panic]
fn login_invalid() {
    Client::default()
        .login("email", "password")
        .expect("Invalid login");
}

#[test]
fn login_valid() {
    let response = Client::default()
        .login(
            &std::env::var("EMAIL").expect("EMAIL"),
            &std::env::var("PASSWORD").expect("PASSWORD"),
        )
        .unwrap();
    assert!(response.data.is_some());
}

//     pub fn calendar(
//         self,
//         year: i32,
//         month: u32,
//         date: Option<u32>,
//     ) -> anyhow::Result<Root<Calendar>> {
//         let (start_date, end_date) = match date {
//             None => {
//                 let start_date = NaiveDate::from_ymd(year, month, 1);
//                 let next_month = match month {
//                     12 => NaiveDate::from_ymd(year + 1, month, 1),
//                     _ => NaiveDate::from_ymd(year, month + 1, 1),
//                 };
//                 let end_date = next_month - Duration::days(1);
//                 (start_date, end_date)
//             }
//             Some(date) => {
//                 let naive_date = NaiveDate::from_ymd(year, month, date);
//                 (naive_date, naive_date)
//             }
//         };
//
//         let mut url = Api::build_url("calendar")?;
//         url.query_pairs_mut().extend_pairs(&[
//             ("startDate", start_date.to_string()),
//             ("endDate", end_date.to_string()),
//             ("month", month.to_string()),
//         ]);
//
//         let calendar: Root<Calendar> = self.client.get(url).send()?.json()?;
//         calendar.result()
//     }

// #[test]
// fn new() {
//     let api = Api::new(std::env::var("TOKEN").expect("TOKEN"));
//     assert!(api.is_ok());
// }

// #[test]
// fn calendar() {
//     let client = talenta::build_client(std::env::var("TOKEN").expect("TOKEN"));
//     let date = Local::now().naive_local().date();
//     // let calendar = api::calendar(client, date.year(), date.month(), Some(date.day())).unwrap();
//     let calendar = api::calendar(client, date.year(), date.month(), None).unwrap();
//     assert!(calendar.data.is_some());
// }
