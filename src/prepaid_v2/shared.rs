use thiserror::Error;
use std::fmt;
use serde::{Deserialize, Deserializer, Serialize};


// DEV prepaid V2
pub const DEV_PREPAID_V2: &str = "https://prepaid.iak.dev/api";
// PROD prepaid V2
pub const PROD_PREPAID_V2: &str = "https://prepaid.iak.id/api";


#[derive(Error, Debug)]
pub enum Error {
    #[error("Response error: `{0}`")]
    ResponseError(String),

    #[error("Decryption error: `{0}`")]
    DecryptionError(String),

    #[error("Deserialization error: `{0}`")]
    DeserializationError(String),

    #[error("unknown model error")]
    Unknown,
}

#[derive(Serialize, PartialEq, Debug)]
pub enum ProductType {
    #[serde(rename = "pulsa")]
    Pulsa,
    #[serde(rename = "data")]
    Data,
    #[serde(rename = "etoll")]
    Etoll,
    #[serde(rename = "voucher")]
    Voucher,
    #[serde(rename = "game")]
    Game,
    #[serde(rename = "pln")]
    Pln,
    #[serde(rename = "international")]
    International,
    Other(String)
}

impl fmt::Display for ProductType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProductType::Pulsa => write!(f, "pulsa"),
            ProductType::Data => write!(f, "data"),
            ProductType::Etoll => write!(f, "etoll"),
            ProductType::Voucher => write!(f, "voucher"),
            ProductType::Game => write!(f, "game"),
            ProductType::Pln => write!(f, "pln"),
            ProductType::International => write!(f, "international"),
            ProductType::Other(s) => write!(f, "{}", s),
        }
    }
}

impl<'de> Deserialize<'de> for ProductType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "pulsa" => Ok(ProductType::Pulsa),
            "data" => Ok(ProductType::Data),
            "etoll" => Ok(ProductType::Etoll),
            "voucher" => Ok(ProductType::Voucher),
            "game" => Ok(ProductType::Game),
            "pln" => Ok(ProductType::Pln),
            "international" => Ok(ProductType::International),
            _ => Ok(ProductType::Other(s))
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ProductTypeOperator {
    #[serde(rename = "pulsa")]
    Pulsa(PulsaOperator),
    #[serde(rename = "data")]
    Data(DataOperator),
    #[serde(rename = "etoll")]
    Etoll(EtollOperator),
    #[serde(rename = "voucher")]
    Voucher(VoucherOperator),
    #[serde(rename = "game")]
    Game,
    #[serde(rename = "pln")]
    Pln(PlnOperator),
    #[serde(rename = "international")]
    International,
}



#[derive(Serialize, PartialEq, Debug)]
pub enum PulsaOperator{
    #[serde(rename = "axis")]
    Axis,
    #[serde(rename = "indosat")]
    Indosat,
    #[serde(rename = "smart")]
    Smart,
    #[serde(rename = "telkomsel")]
    Telkomsel,
    #[serde(rename = "three")]
    Three,
    #[serde(rename = "xixi_games")]
    XixiGames,
    #[serde(rename = "xl")]
    Xl,
    Other(String),
}
impl<'de> Deserialize<'de> for PulsaOperator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {

        let s: String = Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "axis" => Ok(PulsaOperator::Axis),
            "indosat" => Ok(PulsaOperator::Indosat),
            "smart" => Ok(PulsaOperator::Smart),
            "telkomsel" => Ok(PulsaOperator::Telkomsel),
            "three" => Ok(PulsaOperator::Three),
            "xixi_games" => Ok(PulsaOperator::XixiGames),
            "xl" => Ok(PulsaOperator::Xl),
            _ => Ok(PulsaOperator::Other(s))
        }
    }
}




#[derive(Serialize, PartialEq, Debug)]
pub enum DataOperator {
    #[serde(rename = "axis_paket_internet")]
    AxisPaketInternet,
    #[serde(rename = "telkomsel")]
    Telkomsel,
    #[serde(rename = "indosat_paket_internet")]
    IndosatPaketInternet,
    #[serde(rename = "smartfren_paket_internet")]
    SmartfrenPaketInternet,
    #[serde(rename = "tri_paket_internet")]
    TriPaketInternet,
    #[serde(rename = "telkomsel_paket_internet")]
    TelkomselPaketInternet,
    #[serde(rename = "xl_paket_internet")]
    XlPaketInternet,
    Other(String),
}

impl<'de> Deserialize<'de> for DataOperator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {

        let s: String = Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "axis_paket_internet" => Ok(DataOperator::AxisPaketInternet),
            "telkomsel" => Ok(DataOperator::Telkomsel),
            "indosat_paket_internet" => Ok(DataOperator::IndosatPaketInternet),
            "smartfren_paket_internet" => Ok(DataOperator::SmartfrenPaketInternet),
            "tri_paket_internet" => Ok(DataOperator::TriPaketInternet),
            "telkomsel_paket_internet" => Ok(DataOperator::TelkomselPaketInternet),
            "xl_paket_internet" => Ok(DataOperator::XlPaketInternet),
            _ => Ok(DataOperator::Other(s))
        }
    }
}



#[derive(Serialize, PartialEq, Debug)]
pub enum EtollOperator{
    #[serde(rename = "dana")]
    Dana,
    #[serde(rename = "mandiri_e-toll")]
    MandiriEToll,
    #[serde(rename = "indomaret_card_e-money")]
    IndomaretCardEMoney,
    #[serde(rename = "gopay_e-money")]
    GopayEMoney,
    #[serde(rename = "linkaja")]
    Linkaja,
    #[serde(rename = "ovo")]
    Ovo,
    #[serde(rename = "shopee_pay")]
    ShopeePay,
    #[serde(rename = "tix_id")]
    TixId,
    Other(String),
}

impl<'de> Deserialize<'de> for EtollOperator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {

        let s: String = Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "dana" => Ok(EtollOperator::Dana),
            "mandiri_e-toll" => Ok(EtollOperator::MandiriEToll),
            "indomaret_card_e-money" => Ok(EtollOperator::IndomaretCardEMoney),
            "gopay_e-money" => Ok(EtollOperator::GopayEMoney),
            "linkaja" => Ok(EtollOperator::Linkaja),
            "ovo" => Ok(EtollOperator::Ovo),
            "shopee_pay" => Ok(EtollOperator::ShopeePay),
            "tix_id" => Ok(EtollOperator::TixId),
            _ => Ok(EtollOperator::Other(s))
        }
    }
}


#[derive(Serialize, PartialEq, Debug)]
pub enum VoucherOperator{
    #[serde(rename = "alfamart")]
    Alfamart,
    #[serde(rename = "carrefour")]
    Carrefour,
    #[serde(rename = "indomaret")]
    Indomaret,
    #[serde(rename = "map")]
    Map,
    #[serde(rename = "tokopedia")]
    Tokopedia,
    #[serde(rename = "traveloka")]
    Traveloka,
    #[serde(rename = "udemy")]
    Udemy,
    Other(String),
}

impl<'de> Deserialize<'de> for VoucherOperator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {

        let s: String = Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "alfamart" => Ok(VoucherOperator::Alfamart),
            "carrefour" => Ok(VoucherOperator::Carrefour),
            "indomaret" => Ok(VoucherOperator::Indomaret),
            "map" => Ok(VoucherOperator::Map),
            "tokopedia" => Ok(VoucherOperator::Tokopedia),
            "traveloka" => Ok(VoucherOperator::Traveloka),
            "udemy" => Ok(VoucherOperator::Udemy),
            _ => Ok(VoucherOperator::Other(s))
        }
    }
}



#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[non_exhaustive]
pub enum PlnOperator {
    #[serde(rename = "pln")]
    Pln,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ResponseCode {
    // 00	SUCCESS	Success	Transaction success.
    #[serde(rename = "00")]
    Success,
    // 06	TRANSACTION NOT FOUND	Failed	There is no transaction with your inputted ref_id. Please check again your inputted ref_id to find your transaction.
    #[serde(rename = "06")]
    TransactionNotFound,
    // 07	FAILED	Failed	Your current transaction has failed. Please try again.
    #[serde(rename = "07")]
    Failed,
    // 10	REACH TOP UP LIMIT USING SAME DESTINATION NUMBER IN 1 DAY	Failed	Your current destination number top up request is reaching the limit on that day. Please try again tomorrow.
    #[serde(rename = "10")]
    ReachTopUpLimitUsingSameDestinationNumberIn1Day,
    // 12	BALANCE MAXIMUM LIMIT EXCEEDED	Failed	-
    #[serde(rename = "12")]
    BalanceMaximumLimitExceeded,
    // 13	CUSTOMER NUMBER BLOCKED	Failed	Your customer number (customer_id) has been blocked. You can change your customer number (customer_id) or contact our Customer Service.
    #[serde(rename = "13")]
    CustomerNumberBlocked,
    // 14	INCORRECT DESTINATION NUMBER	Failed	Your customer_id that you’ve inputted isn’t a valid phone number. Please check again your customer_id.
    #[serde(rename = "14")]
    IncorrectDestinationNumber,
    // 16	NUMBER NOT MATCH WITH OPERATOR	Failed	Your phone number (customer_id) that you’ve inputted doesn’t match with your desired operator (product_code). Please check again your phone number or change your operator.
    #[serde(rename = "16")]
    NumberNotMatchWithOperator,
    // 17	INSUFFICIENT DEPOSIT	Failed	Your current deposit is lower than the product_price you want to buy. You can add more money into your deposit by doing top up on iak.id deposit menu, or if you are in development mode, you can add your development deposit by clicking the + (plus) sign on development deposit menu (https://developer.iak.id/sandbox-report).
    #[serde(rename = "17")]
    InsufficientDeposit,
    // 20	CODE NOT FOUND	Failed	Your inputted product_code isn’t in the database. Check again your product_code, you can check product_code list by using pricelist API (https://api.iak.id/docs/reference/ZG9jOjEyNjIwNjQy-price-list).
    #[serde(rename = "20")]
    CodeNotFound,
    // 39	PROCESS	Pending	Your current transaction is being processed, please wait until your transaction is fully processed. You can check the status by using check-status API or by receiving a callback (if you use callback).
    #[serde(rename = "39")]
    Process,
    // 102	INVALID IP ADDRESS	Failed	Your IP address isn’t allowed to make a transaction. You can add your IP address to your allowed IP address list in https://developer.iak.id/prod-setting.
    #[serde(rename = "102")]
    InvalidIpAddress,
    // 106	PRODUCT IS TEMPORARILY OUT OF SERVICE	Failed	The product_code that you pick is in non-active status. You can retry your transaction with another product_code that has active status.
    #[serde(rename = "106")]
    ProductIsTemporarilyOutOfService,
    // 107	ERROR IN XML FORMAT	Failed	The body format of your request isn’t correct or there is an error in your body (required, ajax error, etc). Please use the correct JSON or XML format corresponding to your request to API. You can see the required body request for each request in the API Documentation (https://api.iak.id/docs/reference).
    #[serde(rename = "107")]
    ErrorInXmlFormat,
    // 110	SYSTEM UNDER MAINTENANCE	Failed	The system is currently under maintenance, you can try again later.
    #[serde(rename = "110")]
    SystemUnderMaintenance,
    // 117	PAGE NOT FOUND	Failed	The API URL that you want to hit is not found. Try checking your request URL for any typos or try other API URLs.
    #[serde(rename = "117")]
    PageNotFound,
    // 121	MONTHLY TOP UP LIMIT EXCEEDED	Failed	-
    #[serde(rename = "121")]
    MonthlyTopUpLimitExceeded,
    // 131	TOP UP REGION BLOCKED FOR PLAYER	Failed	Your current destination number top up request is blocked in that region. Please try again with a different destination number.
    #[serde(rename = "131")]
    TopUpRegionBlockedForPlayer,
    // 141	INVALID USER ID / ZONE ID / SERVER ID / ROLENAME	Failed	Your inputted user ID / Zone ID / Server ID / Role name isn’t valid. Please try again with another user ID / Zone ID / Server ID / Role name. (https://api.iak.id/docs/reference/ec804704ff306-inquiry-game-server)
    #[serde(rename = "141")]
    InvalidUserIdZoneIdServerIdRoleName,
    // 142	INVALID USER ID	Failed	Your current destination number (user id) top up request is invalid. Please try again with a different destination number or try checking for typos in your field.
    #[serde(rename = "142")]
    InvalidUserId,
    // 201	UNDEFINED RESPONSE CODE	Pending	The received response code is undefined yet. Please contact our Customer Service.
    #[serde(rename = "201")]
    UndefinedResponseCode,
    // 202	MAXIMUM 1 NUMBER 1 TIME IN 1 DAY	Failed	You can only top up to a phone number once in a day (based on your developer setting). If you want to allow more than one top up to a phone number, you can go to https://developer.iak.id/ then choose API Setting menu, you can turn on “Allow multiple transactions for the same number” in development or production settings.
    #[serde(rename = "202")]
    Maximum1Number1TimeIn1Day,
    // 203	NUMBER IS TOO LONG	Failed	Your inputted customer ID is too long. Please check again your customer ID.
    #[serde(rename = "203")]
    NumberIsTooLong,
    // 204	WRONG AUTHENTICATION	Failed	Your sign (signature) field doesn’t contain the right key for your current request. Please check again your sign value.
    #[serde(rename = "204")]
    WrongAuthentication,
    // 205	WRONG COMMAND	Failed	The command that you’ve inputted is not a valid command, try check your commands field for typos or try another command.
    #[serde(rename = "205")]
    WrongCommand,
    // 206	THIS DESTINATION NUMBER HAS BEEN BLOCKED	Failed	The customer_id that you inputted is blocked. You can unblock it by remove customer number blacklist in API Security menu on developer.iak.id (https://developer.iak.id/end-user-blacklist).
    #[serde(rename = "206")]
    ThisDestinationNumberHasBeenBlocked,
    // 207	MAXIMUM 1 NUMBER WITH ANY CODE 1 TIME IN 1 DAY	Failed	You’ve already done a transaction today. Please do another transaction tomorrow, or disable the high restriction setting in https://developer.iak.id/prod-setting.
    #[serde(rename = "207")]
    Maximum1NumberWithAnyCode1TimeIn1Day,
    // GAME ONLY:
    // 143	INQUIRY NOT NEEDED	Failed	The inputted operator is a voucher type therefore it doesn't need the player id validation. You don’t need to hit inquiry on voucher purchase.
    #[serde(rename = "143")]
    InquiryNotNeeded,
}

#[derive(Serialize, PartialEq, Debug)]
pub enum ResponseStatus {
    #[serde(rename = "0")]
    Process,
    #[serde(rename = "1")]
    Success,
    #[serde(rename = "2")]
    Failed,
}

impl<'de> Deserialize<'de> for ResponseStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        match value {
            serde_json::Value::Number(n) => {
                let num = n.as_u64().ok_or_else(|| {
                    serde::de::Error::custom(format!("invalid number: {:?}", n))
                })?;
                match num {
                    0 => Ok(ResponseStatus::Process),
                    1 => Ok(ResponseStatus::Success),
                    2 => Ok(ResponseStatus::Failed),
                    _ => Err(serde::de::Error::custom(format!(
                        "invalid ResponseStatus value: {}",
                        num
                    ))),
                }
            }
            serde_json::Value::String(s) => match s.as_str() {
                "0" => Ok(ResponseStatus::Process),
                "1" => Ok(ResponseStatus::Success),
                "2" => Ok(ResponseStatus::Failed),
                _ => Err(serde::de::Error::custom(format!(
                    "invalid ResponseStatus value: {}",
                    s
                ))),
            },
            _ => Err(serde::de::Error::custom(format!(
                "invalid ResponseStatus value: {:?}",
                value
            ))),
        }
    }
}

