use serde::{Serialize, Deserialize, Deserializer};
use serde_repr::*;
use std::fmt;

// DEV postpaid V1
pub const DEV_POSTPAID_V1: &str = "https://testpostpaid.mobilepulsa.net/api/v1";
// PROD postpaid V1
pub const PROD_POSTPAID_V1: &str = "https://mobilepulsa.net/api/v1";





// RESPONSE CODE	DESCRIPTION	STATUS	SOLUTION
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ResponseCode {
  // 00	PAYMENT / INQUIRY SUCCESS	Success	Your inquiry or payment is successfully processed.
  #[serde(rename = "00")]
  Success,
  // 01	INVOICE HAS BEEN PAID	Failed	The invoice with your inputted data has already been paid. You don’t need to pay it again, or you can check for your inputted customer_id.
  #[serde(rename = "01")]
  InvoiceHasBeenPaid,
  // 02	BILL UNPAID	Failed	Your bill is unpaid, only reaching inquiry status. Please finish your payment first.
  #[serde(rename = "02")]
  BillUnpaid,
  // 03	INVALID REF ID	Failed	Your inputted ref_id isn’t valid. Please follow the correct format for ref_id (alpha_num only without space). Try again with a valid ref_id.
  #[serde(rename = "03")]
  InvalidRefId,
  // 04	BILLING ID EXPIRED	Failed	Your reference ID (ref_id) is expired. Please retry your inquiry request with a different reference ID (ref_id).
  #[serde(rename = "04")]
  BillingIdExpired,
  // 05	UNDEFINED ERROR	Pending	Your transaction failed because of an undefined error. Please try again.
  #[serde(rename = "05")]
  UndefinedError,
  // 06	INQUIRY ID NOT FOUND	Failed	The inquiry ID (tr_id) that you’ve inputted is not found, there is no inquiry with that ID. You can check the inquiry ID (tr_id) field for any typos, or try using another inquiry ID.
  #[serde(rename = "06")]
  InquiryIdNotFound,
  // 07	TRANSACTION FAILED	Failed	Your transaction has failed. Please try again.
  #[serde(rename = "07")]
  TransactionFailed,
  // 08	BILLING ID BLOCKED	Failed	The customer ID for your inputted product code is blocked by IAK. Please contact our Customer Service.
  #[serde(rename = "08")]
  BillingIdBlocked,
  // 09	INQUIRY FAILED	Failed	Your inquiry process failed. Please try to do the inquiry again.
  #[serde(rename = "09")]
  InquiryFailed,
  // 10	BILL IS NOT AVAILABLE	Failed	The bill isn’t available yet. Please try again when the bill is already available.
  #[serde(rename = "10")]
  BillIsNotAvailable,
  // 11	DUPLICATE REF ID	Failed	The ref_id that you’ve inputted is already been inputted, try again with another ref_id.
  #[serde(rename = "11")]
  DuplicateRefId,
  // 13	CUSTOMER NUMBER BLOCKED	Failed	Your customer number (hp) has been blocked. You can change your customer number (hp) or contact our Customer Service.
  #[serde(rename = "13")]
  CustomerNumberBlocked,
  // 14	INCORRECT DESTINATION NUMBER	Failed	The destination number (hp) that you’ve inputted is incorrect. Try checking your destination number (hp) for typos or try with another destination number (hp).
  #[serde(rename = "14")]
  IncorrectDestinationNumber,
  // 15	NUMBER THAT YOU ENTERED IS NOT SUPPORTED	Failed	Customer number that you’ve inputted isn’t supported by your inputted product_code. Try again with another customer number or product_code.
  #[serde(rename = "15")]
  NumberThatYouEnteredIsNotSupported,
  // 16	NUMBER DOESN'T MATCH THE OPERATOR	Failed	Your customer number (hp) that you’ve inputted doesn’t match with your desired operator (code). Please check again your customer number or change your operator.
  #[serde(rename = "16")]
  NumberDoesntMatchTheOperator,
  // 17	BALANCE NOT ENOUGH	Failed	Your current balance is lower than the product price you want to buy. You can add more money into your deposit by doing top up on iak.id deposit menu (https://iak.id/webapp/top-up), or if you are in development mode, you can add your development deposit by clicking the + (plus) sign on development deposit menu (https://developer.iak.id/sandbox-report).
  #[serde(rename = "17")]
  BalanceNotEnough,
  // 20	PRODUCT UNREGISTERED	Failed	Your inputted code isn’t in the database. Check again your code, you can check code list by using pricelist API (https://api.iak.id/docs/reference/ad7437ff00ecc-price-list).
  #[serde(rename = "20")]
  ProductUnregistered,
  // 30	PAYMENT HAVE TO BE DONE VIA COUNTER / PDAM	Failed	Your inputted payment has to be done at the counter. Please do your transaction at the counter.
  #[serde(rename = "30")]
  PaymentHaveToBeDoneViaCounter,
  // 31	TRANSACTION REJECTED DUE TO EXCEEDING MAXIMAL TOTAL BILL ALLOWED	Failed	Your current transaction is exceeding the maximum total bill allowed. Please try again later.
  #[serde(rename = "31")]
  TransactionRejectedDueToExceedingMaximalTotalBillAllowed,
  // 32	TRANSACTION FAILED, PLEASE PAY BILL OF ALL PERIOD	Failed	Your current transaction isn’t covering all periods. Please try again to pay with all periods.
  #[serde(rename = "32")]
  TransactionFailedPleasePayBillOfAllPeriod,
  // 33	TRANSACTION CAN'T BE PROCESS, PLEASE TRY AGAIN LATER	Failed	Transaction can't be process, please try again later.
  #[serde(rename = "33")]
  TransactionCantBeProcessPleaseTryAgainLater,
  // 34	BILL HAS BEEN PAID	Failed	Your bill for your current transaction has been paid. Please try again for another transaction.
  #[serde(rename = "34")]
  BillHasBeenPaid,
  // 35	TRANSACTION REJECTED DUE TO ANOTHER UNPAID ARREAR	Failed	Your transaction id failed. Please pay for your other arrear first, then try again your transaction.
  #[serde(rename = "35")]
  TransactionRejectedDueToAnotherUnpaidArrear,
  // 36	EXCEEDING DUE DATE, PLEASE PAY IN THE COUNTER / PDAM	Failed	Your current transaction is exceeding the due date. Please pay the transaction at the counter.
  #[serde(rename = "36")]
  ExceedingDueDatePleasePayInTheCounter,
  // 37	PAYMENT FAILED	Failed	Your payment failed, please try again later.
  #[serde(rename = "37")]
  PaymentFailed,
  // 38	PAYMENT FAILED, PLEASE DO ANOTHER REQUEST	Failed	Your payment failed, please try again with a new request.
  #[serde(rename = "38")]
  PaymentFailedPleaseDoAnotherRequest,
  // 39	PENDING / TRANSACTION IN PROCESS	Pending	Your transaction is still being processed. Please wait for further status updates.
  #[serde(rename = "39")]
  PendingTransactionInProcess,
  // 40	TRANSACTION REJECTED DUE TO ALL OR ONE OF THE ARREAR/INVOICE HAS BEEN PAID	Failed	Your transaction has already been paid. You can try again with another transaction.
  #[serde(rename = "40")]
  TransactionRejectedDueToAllOrOneOfTheArrearInvoiceHasBeenPaid,
  // 41	CAN'T BE PAID IN COUNTER, PLEASE PAY TO THE CORRESPONDING COMPANY	Failed	Your current transaction cannot be paid in the counter. You can pay your transaction to the corresponding company.
  #[serde(rename = "41")]
  CantBePaidInCounterPleasePayToTheCorrespondingCompany,
  // 42	PAYMENT REQUEST HAVEN'T BEEN RECEIEVED	Failed	Your current transaction is still in the inquiry process. Please continue your payment process first.
  #[serde(rename = "42")]
  PaymentRequestHaventBeenReceieved,
  // 91	DATABASE CONNECTION ERROR	Failed	There is an error on the database connection. Please try again later.
  #[serde(rename = "91")]
  DatabaseConnectionError,
  // 92	GENERAL ERROR	Failed	The received response code is undefined yet. Please contact our Customer Service.
  #[serde(rename = "92")]
  GeneralError,
  // 93	INVALID AMOUNT	Failed	The amount inputted isn’t valid. Please check again your inputted amount.
  #[serde(rename = "93")]
  InvalidAmount,
  // 94	SERVICE HAS EXPIRED	Failed	-
  #[serde(rename = "94")]
  ServiceHasExpired,
  // 100	INVALID SIGNATURE	Failed	Your sign field doesn’t contain the right key for your current request. Please check again your sign value.
  #[serde(rename = "100")]
  InvalidSignature,
  // 101	INVALID COMMAND	Failed	The command that you’ve inputted is not a valid command, try checking your commands field for typos or try another command.
  #[serde(rename = "101")]
  InvalidCommand,
  // 102	INVALID IP ADDRESS	Failed	Your IP address isn’t allowed to make a transaction. You can add your IP address to your allowed IP address list in https://developer.iak.id/prod-setting.
  #[serde(rename = "102")]
  InvalidIpAddress,
  // 103	TIMEOUT	Failed	Your current request exceeds the timeout limit. You can try to request it again.
  #[serde(rename = "103")]
  Timeout,
  // 105	MISC ERROR / BILLER SYSTEM ERROR	Failed	There is an error from the supplier. Please try again later.
  #[serde(rename = "105")]
  MiscErrorBillerSystemError,
  // 106	PRODUCT IS TEMPORARILY OUT OF SERVICE	Failed	The code is in non-active status. You can retry your transaction with another code that has active status. (https://api.iak.id/docs/reference/ZG9jOjEyNjIwNjQy-price-list)
  #[serde(rename = "106")]
  ProductIsTemporarilyOutOfService,
  // 107	XML FORMAT ERROR	Failed	The body format of your request isn’t correct or there is an error in your body (required, ajax error, etc). Please use the correct JSON or XML format corresponding to your request to API. You can see the required body request for each request in the API Documentation (https://api.iak.id/docs/reference).
  #[serde(rename = "107")]
  XmlFormatError,
  // 108	SORRY, YOUR ID CAN'T BE USED FOR THIS PRODUCT TRANSACTION	Failed	The customer ID that you’ve inputted can’t be used for this product transaction. Please try again with another customer ID or another product.
  #[serde(rename = "108")]
  SorryYourIdCantBeUsedForThisProductTransaction,
  // 109	SYSTEM CUT OFF	Failed	PLN product code cannot receive a request at 11PM until 1AM (GMT +7). Please try again when the service is available.
  #[serde(rename = "109")]
  SystemCutOff,
  // 110	SYSTEM UNDER MAINTENANCE	Failed	The system is currently under maintenance, you can try again later.
  #[serde(rename = "110")]
  SystemUnderMaintenance,
  // 117	PAGE NOT FOUND	Failed	The API URL that you want to hit is not found. Try checking your request URL for any typos or try other API URLs.
  #[serde(rename = "117")]
  PageNotFound,
  // 201	UNDEFINED RESPONSE CODE	Pending	The received response code is undefined yet. Please contact our Customer Service.
  #[serde(rename = "201")]
  UndefinedResponseCode,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ProductType{
  #[serde(rename = "pdam")]
  Pdam,
  #[serde(rename = "bpjs")]
  Bpjs,
  #[serde(rename = "internet")]
  Internet,
  #[serde(rename = "pajak-kendaraan")]
  PajakKendaraan,
  #[serde(rename = "finance")]
  Finance,
  #[serde(rename = "hp")]
  Hp,
  #[serde(rename = "estate")]
  Estate,
  #[serde(rename = "emoney")]
  Emoney,
  #[serde(rename = "kereta")]
  Kereta,
  #[serde(rename = "tv")]
  Tv,
  #[serde(rename = "airline")]
  Airline,
  #[serde(rename = "o2o")]
  O2o,
  #[serde(rename = "pbb")]
  Pbb,
  #[serde(rename = "gas")]
  Gas,
  #[serde(rename = "pajak-daerah")]
  PajakDaerah,
  #[serde(rename = "pln")]
  Pln,
  #[serde(rename = "pasar")]
  Pasar,
  #[serde(rename = "retribusi")]
  Retribusi,
  #[serde(rename = "pendidikan")]
  Pendidikan,
  #[serde(rename = "asuransi")]
  Asuransi,
  #[serde(rename = "iuran")]
  Iuran,
  Other(String),
}

impl fmt::Display for ProductType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
          ProductType::Pdam => write!(f, "pdam"),
          ProductType::Bpjs => write!(f, "bpjs"),
          ProductType::Internet => write!(f, "internet"),
          ProductType::PajakKendaraan => write!(f, "pajak-kendaraan"),
          ProductType::Finance => write!(f, "finance"),
          ProductType::Hp => write!(f, "hp"),
          ProductType::Estate => write!(f, "estate"),
          ProductType::Emoney => write!(f, "emoney"),
          ProductType::Kereta => write!(f, "kereta"),
          ProductType::Tv => write!(f, "tv"),
          ProductType::Airline => write!(f, "airline"),
          ProductType::O2o => write!(f, "o2o"),
          ProductType::Pbb => write!(f, "pbb"),
          ProductType::Gas => write!(f, "gas"),
          ProductType::PajakDaerah => write!(f, "pajak-daerah"),
          ProductType::Pln => write!(f, "pln"),
          ProductType::Pasar => write!(f, "pasar"),
          ProductType::Retribusi => write!(f, "retribusi"),
          ProductType::Pendidikan => write!(f, "pendidikan"),
          ProductType::Asuransi => write!(f, "asuransi"),
          ProductType::Iuran => write!(f, "iuran"),
          ProductType::Other(s) => write!(f, "{}", s),
      }
  }
}

#[derive(Serialize, PartialEq, Debug, Clone)]
// #[repr(u8)]
pub enum ProductStatus {
    #[serde(rename = "active")]
    Active = 1,
    #[serde(rename = "non active")]
    NonActive = 4,
}

impl<'de> Deserialize<'de> for ProductStatus {
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
                  1 => Ok(ProductStatus::Active),
                  4 => Ok(ProductStatus::NonActive),
                  _ => Err(serde::de::Error::custom(format!(
                      "invalid ProductStatus value: {}",
                      num
                  ))),
              }
          }
          serde_json::Value::String(s) => match s.as_str() {
              "1" => Ok(ProductStatus::Active),
              "4" => Ok(ProductStatus::NonActive),
              _ => Err(serde::de::Error::custom(format!(
                  "invalid ProductStatus value: {}",
                  s
              ))),
          },
          _ => Err(serde::de::Error::custom(format!(
              "invalid ProductStatus value: {:?}",
              value
          ))),
      }
  }
}

