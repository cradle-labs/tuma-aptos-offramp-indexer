// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "ledger_entry_type"))]
    pub struct LedgerEntryType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "offramp_request_status"))]
    pub struct OfframpRequestStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "onramp_request_status"))]
    pub struct OnrampRequestStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "payment_method_type"))]
    pub struct PaymentMethodType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "transaction_type"))]
    pub struct TransactionType;
}

diesel::table! {
    account (address) {
        address -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    kvstore (key) {
        key -> Text,
        value -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::LedgerEntryType;
    use super::sql_types::TransactionType;

    ledger (id) {
        id -> Uuid,
        address -> Text,
        entry_type -> Nullable<LedgerEntryType>,
        on_chain_transaction_version -> Nullable<Numeric>,
        off_chain_transaction_hash -> Nullable<Text>,
        transaction_type -> Nullable<TransactionType>,
        payment_method_id -> Nullable<Uuid>,
        timestamp -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::OfframpRequestStatus;

    off_ramp_requests (id) {
        id -> Uuid,
        requester -> Text,
        from_token -> Text,
        from_token_amount -> Numeric,
        transaction_version -> Text,
        transaction_hash -> Text,
        transaction_code -> Nullable<Text>,
        data -> Nullable<Jsonb>,
        requested_at -> Timestamp,
        finalized_at -> Nullable<Timestamp>,
        status -> OfframpRequestStatus,
        to_amount -> Nullable<Numeric>,
        observer_key -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::OnrampRequestStatus;

    on_ramp_requests (id) {
        id -> Uuid,
        requester -> Text,
        payment_method_id -> Uuid,
        status -> OnrampRequestStatus,
        transaction_ref -> Nullable<Text>,
        data -> Nullable<Jsonb>,
        amount -> Nullable<Numeric>,
        requested_at -> Timestamp,
        finalized_at -> Nullable<Timestamp>,
        target_token -> Text,
        final_token_quote -> Nullable<Numeric>,
        on_chain_transaction_hash -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PaymentMethodType;

    payment_method (id) {
        id -> Uuid,
        owner -> Text,
        payment_method_type -> Nullable<PaymentMethodType>,
        identity -> Text,
        provider_id -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::OfframpRequestStatus;

    payment_sessions (id) {
        id -> Uuid,
        payment_provider_id -> Text,
        payment_identity -> Text,
        account_identity -> Nullable<Text>,
        payer -> Text,
        requested_at -> Nullable<Timestamp>,
        finalized_at -> Nullable<Timestamp>,
        data -> Nullable<Jsonb>,
        transaction_hash -> Nullable<Text>,
        transferred_amount -> Numeric,
        transferred_token -> Nullable<Text>,
        final_fiat_value -> Numeric,
        status -> Nullable<OfframpRequestStatus>,
        transaction_code -> Nullable<Text>,
    }
}

diesel::joinable!(ledger -> account (address));
diesel::joinable!(ledger -> payment_method (payment_method_id));
diesel::joinable!(off_ramp_requests -> account (requester));
diesel::joinable!(on_ramp_requests -> account (requester));
diesel::joinable!(on_ramp_requests -> payment_method (payment_method_id));
diesel::joinable!(payment_method -> account (owner));

diesel::allow_tables_to_appear_in_same_query!(
    account,
    kvstore,
    ledger,
    off_ramp_requests,
    on_ramp_requests,
    payment_method,
    payment_sessions,
);
