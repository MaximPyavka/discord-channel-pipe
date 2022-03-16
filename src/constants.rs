const BASE_URL: &str = "https://apptoogoodtogo.com/api/";
const API_ITEM_ENDPOINT: &str = "item/v7/";
const AUTH_BY_EMAIL_ENDPOINT: &str = "auth/v3/authByEmail";
const AUTH_POLLING_ENDPOINT: &str = "auth/v3/authByRequestPollingId";
const SIGNUP_BY_EMAIL_ENDPOINT: &str  = "auth/v3/signUpByEmail";
const REFRESH_ENDPOINT: &str = "auth/v3/token/refresh";
const ACTIVE_ORDER_ENDPOINT: &str = "order/v6/active";
const INACTIVE_ORDER_ENDPOINT: &str = "order/v6/inactive";

const USER_AGENTS: [&str; 3] = [
    "TGTG/21.12.1 Dalvik/2.1.0 (Linux; U; Android 6.0.1; Nexus 5 Build/M4B30Z)",
    "TGTG/21.12.1 Dalvik/2.1.0 (Linux; U; Android 7.0; SM-G935F Build/NRD90M)",
    "TGTG/21.12.1 Dalvik/2.1.0 (Linux; Android 6.0.1; SM-G920V Build/MMB29K)",
];

const DEFAULT_ACCESS_TOKEN_LIFETIME: u64 = 3600 * 4;