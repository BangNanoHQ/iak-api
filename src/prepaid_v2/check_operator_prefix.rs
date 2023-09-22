// Operator Prefix List
// Below is the prefix list for mobile recharge top up. If the customer ID prefix not match with below table, IAK will give response NUMBER NOT MATCH WITH OPERATOR.
// INDOSAT ( hindosat, isatdata )	0814,0815,0816,0855,0856,0857,0858
// XL ( xld, xldata )	0817,0818,0819,0859,0878,0877
// AXIS ( haxis, axisdata )	0838,0837,0831,0832
// TELKOMSEL ( htelkomsel, tseldata)	0812,0813,0852,0853,0821,0823,0822,0851
// SMARTFREN ( hsmart )	0881,0882,0883,0884, 0885,0886,0887,0888
// THREE ( hthree, threedata)	0896,0897,0898,0899,0895
// by.U (hbyu)	085154, 085155, 085156, 085157,085158

use serde::{Deserialize, Serialize};

// create enum that represents the operator and the phone numbers that are valid for that operator
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Operator {
    #[serde(rename = "INDOSAT")]
    Indosat(Vec<IndosatCodes>),
    #[serde(rename = "XL")]
    XL(Vec<XLCodes>),
    #[serde(rename = "AXIS")]
    Axis(Vec<AxisCodes>),
    #[serde(rename = "TELKOMSEL")]
    Telkomsel(Vec<TelkomselCodes>),
    #[serde(rename = "SMARTFREN")]
    Smartfren(Vec<SmartfrenCodes>),
    #[serde(rename = "THREE")]
    Three(Vec<ThreeCodes>),
    #[serde(rename = "by.U")]
    Byu(Vec<ByuCodes>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum OperatorType {
    #[serde(rename = "INDOSAT")]
    Indosat,
    #[serde(rename = "XL")]
    XL,
    #[serde(rename = "AXIS")]
    Axis,
    #[serde(rename = "TELKOMSEL")]
    Telkomsel,
    #[serde(rename = "SMARTFREN")]
    Smartfren,
    #[serde(rename = "THREE")]
    Three,
    #[serde(rename = "by.U")]
    Byu,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OperatorResponse {
    pub operator: OperatorType,
    pub codes: Operator,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IndosatCodes {
    Hindosat,
    Isatdata,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum XLCodes {
    Xld,
    Xldata,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AxisCodes {
    Haxis,
    Axisdata,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TelkomselCodes {
    Htelkomsel,
    Tseldata,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SmartfrenCodes {
    Hsmart,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThreeCodes {
    Hthree,
    Threedata,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ByuCodes {
    Hbyu,
}

const INDOSAT_PREFIXES: [&str; 7] = ["0814", "0815", "0816", "0855", "0856", "0857", "0858"];
const XL_PREFIXES: [&str; 6] = ["0817", "0818", "0819", "0859", "0878", "0877"];
const AXIS_PREFIXES: [&str; 4] = ["0838", "0837", "0831", "0832"];
const TELKOMSEL_PREFIXES: [&str; 8] = [
    "0812", "0813", "0852", "0853", "0821", "0823", "0822", "0851",
];
const SMARTFREN_PREFIXES: [&str; 8] = [
    "0881", "0882", "0883", "0884", "0885", "0886", "0887", "0888",
];
const THREE_PREFIXES: [&str; 5] = ["0896", "0897", "0898", "0899", "0895"];
const BYU_PREFIXES: [&str; 5] = ["085154", "085155", "085156", "085157", "085158"];

// create a function that takes a phone number and returns the operator also to use variart of +62
pub fn check_operator_prefix(phone_number: String) -> Option<OperatorResponse> {
    let phone_number = phone_number.replace("+62", "0");

    // pattern match the phone number to the prefix list
    match phone_number.get(0..6) {
        Some(prefix) => {
            if BYU_PREFIXES.contains(&prefix) {
                Some(OperatorResponse {
                    operator: OperatorType::Byu,
                    codes: Operator::Byu(vec![ByuCodes::Hbyu]),
                })
            } else {
                let prefix = phone_number.get(0..4).unwrap_or("");
                if INDOSAT_PREFIXES.contains(&prefix) {
                    Some(OperatorResponse {
                        operator: OperatorType::Indosat,
                        codes: Operator::Indosat(vec![
                            IndosatCodes::Hindosat,
                            IndosatCodes::Isatdata,
                        ]),
                    })
                } else if XL_PREFIXES.contains(&prefix) {
                    Some(OperatorResponse {
                        operator: OperatorType::XL,
                        codes: Operator::XL(vec![XLCodes::Xld, XLCodes::Xldata]),
                    })
                } else if AXIS_PREFIXES.contains(&prefix) {
                    Some(OperatorResponse {
                        operator: OperatorType::Axis,
                        codes: Operator::Axis(vec![AxisCodes::Haxis, AxisCodes::Axisdata]),
                    })
                } else if TELKOMSEL_PREFIXES.contains(&prefix) {
                    Some(OperatorResponse {
                        operator: OperatorType::Telkomsel,
                        codes: Operator::Telkomsel(vec![
                            TelkomselCodes::Htelkomsel,
                            TelkomselCodes::Tseldata,
                        ]),
                    })
                } else if SMARTFREN_PREFIXES.contains(&prefix) {
                    Some(OperatorResponse {
                        operator: OperatorType::Smartfren,
                        codes: Operator::Smartfren(vec![SmartfrenCodes::Hsmart]),
                    })
                } else if THREE_PREFIXES.contains(&prefix) {
                    Some(OperatorResponse {
                        operator: OperatorType::Three,
                        codes: Operator::Three(vec![ThreeCodes::Hthree, ThreeCodes::Threedata]),
                    })
                } else {
                    None
                }
            }
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_operator_prefix() {
        // Test valid phone numbers
        assert_eq!(
            check_operator_prefix(String::from("081400000000")),
            Some(OperatorResponse {
                operator: OperatorType::Indosat,
                codes: Operator::Indosat(vec![IndosatCodes::Hindosat, IndosatCodes::Isatdata,]),
            })
        );
        assert_eq!(
            check_operator_prefix(String::from("+6281850000000")),
            Some(OperatorResponse {
                operator: OperatorType::XL,
                codes: Operator::XL(vec![XLCodes::Xld, XLCodes::Xldata]),
            })
        );
        assert_eq!(
            check_operator_prefix(String::from("083800000000")),
            Some(OperatorResponse {
                operator: OperatorType::Axis,
                codes: Operator::Axis(vec![AxisCodes::Haxis, AxisCodes::Axisdata]),
            })
        );
        assert_eq!(
            check_operator_prefix(String::from("081200000000")),
            Some(OperatorResponse {
                operator: OperatorType::Telkomsel,
                codes: Operator::Telkomsel(vec![
                    TelkomselCodes::Htelkomsel,
                    TelkomselCodes::Tseldata,
                ]),
            })
        );
        assert_eq!(
            check_operator_prefix(String::from("088100000000")),
            Some(OperatorResponse {
                operator: OperatorType::Smartfren,
                codes: Operator::Smartfren(vec![SmartfrenCodes::Hsmart]),
            })
        );
        assert_eq!(
            check_operator_prefix(String::from("089600000000")),
            Some(OperatorResponse {
                operator: OperatorType::Three,
                codes: Operator::Three(vec![ThreeCodes::Hthree, ThreeCodes::Threedata]),
            })
        );
        assert_eq!(
            check_operator_prefix(String::from("085154000000")),
            Some(OperatorResponse {
                operator: OperatorType::Byu,
                codes: Operator::Byu(vec![ByuCodes::Hbyu]),
            })
        );

        // Test invalid phone numbers
        assert_eq!(check_operator_prefix(String::from("123")), None);
        assert_eq!(check_operator_prefix(String::from("0811")), None);
        // assert_eq!(check_operator_prefix(String::from("0814000000000")), None);
    }
}
