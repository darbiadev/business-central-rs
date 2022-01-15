use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SalesOrder {
    #[serde(rename = "@odata.context")]
    odata_context: String,
    #[serde(rename = "@odata.etag")]
    odata_etag: String,
    #[serde(rename = "Area")]
    area: String,
    #[serde(rename = "Assigned_User_ID")]
    assigned_user_id: String,
    #[serde(rename = "BillToContactEmail")]
    bill_to_contact_email: String,
    #[serde(rename = "BillToContactMobilePhoneNo")]
    bill_to_contact_mobile_phone_no: String,
    #[serde(rename = "BillToContactPhoneNo")]
    bill_to_contact_phone_no: String,
    #[serde(rename = "BillToOptions")]
    bill_to_options: String,
    #[serde(rename = "Bill_to_Address")]
    bill_to_address: String,
    #[serde(rename = "Bill_to_Address_2")]
    bill_to_address_2: String,
    #[serde(rename = "Bill_to_City")]
    bill_to_city: String,
    #[serde(rename = "Bill_to_Contact")]
    bill_to_contact: String,
    #[serde(rename = "Bill_to_Contact_No")]
    bill_to_contact_no: String,
    #[serde(rename = "Bill_to_Country_Region_Code")]
    bill_to_country_region_code: String,
    #[serde(rename = "Bill_to_County")]
    bill_to_county: String,
    #[serde(rename = "Bill_to_Name")]
    bill_to_name: String,
    #[serde(rename = "Bill_to_Post_Code")]
    bill_to_post_code: String,
    #[serde(rename = "CFDI_Purpose")]
    cfdi_purpose: String,
    #[serde(rename = "CFDI_Relation")]
    cfdi_relation: String,
    #[serde(rename = "Campaign_No")]
    campaign_no: String,
    #[serde(rename = "Combine_Shipments")]
    combine_shipments: bool,
    #[serde(rename = "Compress_Prepayment")]
    compress_prepayment: bool,
    #[serde(rename = "Control1310005")]
    control1310005: bool,
    #[serde(rename = "Currency_Code")]
    currency_code: String,
    #[serde(rename = "Date_Filter")]
    date_filter: String,
    #[serde(rename = "Direct_Debit_Mandate_ID")]
    direct_debit_mandate_id: String,
    #[serde(rename = "Document_Date")]
    document_date: String,
    #[serde(rename = "Document_Type")]
    document_type: String,
    #[serde(rename = "Due_Date")]
    due_date: String,
    #[serde(rename = "EU_3_Party_Trade")]
    eu_3_party_trade: bool,
    #[serde(rename = "Exit_Point")]
    exit_point: String,
    #[serde(rename = "External_Document_No")]
    external_document_no: String,
    #[serde(rename = "Insurer_Name")]
    insurer_name: String,
    #[serde(rename = "Insurer_Policy_Number")]
    insurer_policy_number: String,
    #[serde(rename = "Job_Queue_Status")]
    job_queue_status: String,
    #[serde(rename = "Language_Code")]
    language_code: String,
    #[serde(rename = "Late_Order_Shipping")]
    late_order_shipping: bool,
    #[serde(rename = "Location_Code")]
    location_code: String,
    #[serde(rename = "No")]
    no: String,
    #[serde(rename = "No_of_Archived_Versions")]
    no_of_archived_versions: i16,
    #[serde(rename = "Opportunity_No")]
    opportunity_no: String,
    #[serde(rename = "Order_Date")]
    order_date: String,
    #[serde(rename = "Outbound_Whse_Handling_Time")]
    outbound_whse_handling_time: String,
    #[serde(rename = "Package_Tracking_No")]
    package_tracking_no: String,
    #[serde(rename = "Payment_Discount_Percent")]
    payment_discount_percent: i16,
    #[serde(rename = "Payment_Method_Code")]
    payment_method_code: String,
    #[serde(rename = "Payment_Terms_Code")]
    payment_terms_code: String,
    #[serde(rename = "Pmt_Discount_Date")]
    pmt_discount_date: String,
    #[serde(rename = "Posting_Date")]
    posting_date: String,
    #[serde(rename = "Posting_Description")]
    posting_description: String,
    #[serde(rename = "Prepayment_Due_Date")]
    prepayment_due_date: String,
    #[serde(rename = "Prepayment_Percent")]
    prepayment_percent: i16,
    #[serde(rename = "Prepmt_Include_Tax")]
    prepmt_include_tax: bool,
    #[serde(rename = "Prepmt_Payment_Discount_Percent")]
    prepmt_payment_discount_percent: i16,
    #[serde(rename = "Prepmt_Payment_Terms_Code")]
    prepmt_payment_terms_code: String,
    #[serde(rename = "Prepmt_Pmt_Discount_Date")]
    prepmt_pmt_discount_date: String,
    #[serde(rename = "Prices_Including_VAT")]
    prices_including_vat: bool,
    #[serde(rename = "Promised_Delivery_Date")]
    promised_delivery_date: String,
    #[serde(rename = "Quote_No")]
    quote_no: String,
    #[serde(rename = "Requested_Delivery_Date")]
    requested_delivery_date: String,
    #[serde(rename = "Responsibility_Center")]
    responsibility_center: String,
    #[serde(rename = "Salesperson_Code")]
    salesperson_code: String,
    #[serde(rename = "SelectedPayments")]
    selected_payments: String,
    #[serde(rename = "SellToMobilePhoneNo")]
    sell_to_mobile_phone_no: String,
    #[serde(rename = "Sell_to_Address")]
    sell_to_address: String,
    #[serde(rename = "Sell_to_Address_2")]
    sell_to_address_2: String,
    #[serde(rename = "Sell_to_City")]
    sell_to_city: String,
    #[serde(rename = "Sell_to_Contact")]
    sell_to_contact: String,
    #[serde(rename = "Sell_to_Contact_No")]
    sell_to_contact_no: String,
    #[serde(rename = "Sell_to_Country_Region_Code")]
    sell_to_country_region_code: String,
    #[serde(rename = "Sell_to_County")]
    sell_to_county: String,
    #[serde(rename = "Sell_to_Customer_Name")]
    sell_to_customer_name: String,
    #[serde(rename = "Sell_to_Customer_No")]
    sell_to_customer_no: String,
    #[serde(rename = "Sell_to_E_Mail")]
    sell_to_e_mail: String,
    #[serde(rename = "Sell_to_Phone_No")]
    sell_to_phone_no: String,
    #[serde(rename = "Sell_to_Post_Code")]
    sell_to_post_code: String,
    #[serde(rename = "Ship_to_Address")]
    ship_to_address: String,
    #[serde(rename = "Ship_to_Address_2")]
    ship_to_address_2: String,
    #[serde(rename = "Ship_to_City")]
    ship_to_city: String,
    #[serde(rename = "Ship_to_Code")]
    ship_to_code: String,
    #[serde(rename = "Ship_to_Contact")]
    ship_to_contact: String,
    #[serde(rename = "Ship_to_Country_Region_Code")]
    ship_to_country_region_code: String,
    #[serde(rename = "Ship_to_County")]
    ship_to_county: String,
    #[serde(rename = "Ship_to_Name")]
    ship_to_name: String,
    #[serde(rename = "Ship_to_Post_Code")]
    ship_to_post_code: String,
    #[serde(rename = "Ship_to_UPS_Zone")]
    ship_to_ups_zone: String,
    #[serde(rename = "Shipment_Date")]
    shipment_date: String,
    #[serde(rename = "Shipment_Method_Code")]
    shipment_method_code: String,
    #[serde(rename = "ShippingOptions")]
    shipping_options: String,
    #[serde(rename = "Shipping_Advice")]
    shipping_advice: String,
    #[serde(rename = "Shipping_Agent_Code")]
    shipping_agent_code: String,
    #[serde(rename = "Shipping_Agent_Service_Code")]
    shipping_agent_service_code: String,
    #[serde(rename = "Shipping_Time")]
    shipping_time: String,
    #[serde(rename = "Shortcut_Dimension_1_Code")]
    shortcut_dimension_1_code: String,
    #[serde(rename = "Shortcut_Dimension_2_Code")]
    shortcut_dimension_2_code: String,
    #[serde(rename = "Status")]
    status: String,
    #[serde(rename = "Tax_Area_Code")]
    tax_area_code: String,
    #[serde(rename = "Tax_Liable")]
    tax_liable: bool,
    #[serde(rename = "Trailer_1")]
    trailer_1: String,
    #[serde(rename = "Trailer_2")]
    trailer_2: String,
    #[serde(rename = "Transaction_Specification")]
    transaction_specification: String,
    #[serde(rename = "Transaction_Type")]
    transaction_type: String,
    #[serde(rename = "Transit_Distance")]
    transit_distance: i16,
    #[serde(rename = "Transit_Hours")]
    transit_hours: i16,
    #[serde(rename = "Transit_from_Date_Time")]
    transit_from_date_time: String,
    #[serde(rename = "Transit_to_Location")]
    transit_to_location: String,
    #[serde(rename = "Transport_Method")]
    transport_method: String,
    #[serde(rename = "Transport_Operators")]
    transport_operators: i16,
    #[serde(rename = "VAT_Bus_Posting_Group")]
    vat_bus_posting_group: String,
    #[serde(rename = "Vehicle_Code")]
    vehicle_code: String,
    #[serde(rename = "WorkDescription")]
    work_description: String,
    #[serde(rename = "Your_Reference")]
    your_reference: String,
}
