// DEV postpaid
const DEV_POSTPAID: &str = "https://testpostpaid.mobilepulsa.net/api/v1";
// PROD postpaid
const PROD_POSTPAID: &str = "https://mobilepulsa.net/api/v1";


pub enum ApiEndpoint {
    DevPrepaidV2 = DEV_PREPAID_V2,
    ProdPrepaidV2 = PROD_PREPAID_V2,
    DevPostpaid = DEV_POSTPAID,
    ProdPostpaid = PROD_POSTPAID,
}

