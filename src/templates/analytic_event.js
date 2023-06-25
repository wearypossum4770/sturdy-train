const analyticsConfig = (id, payload) => {};

const methods = ["cookie_update"];
class LogEvent {
  constructor({ eventName, ...eventParams }) {
    Object.assign(this, { eventName, payload: eventParams });
  }
}

const config = [
  "allow_ad_personalization_signals",
  "allow_google_signals",
  "cookie_domain",
  "cookie_expires",
  "cookie_flags",
  "cookie_prefix",
  "cookie_update",
  "page_location",
  "allow_ad_features",
  "restricted_data_processing",
  "google_analytics_opt_out",
  "disable_google_analytics",
  "page_title",
  "send_page_view",
];

const consent = [
  "ad_storage",
  "analytics_storage",
  "ads_data_redaction",
  "functionality_storage",
  "personalization_storage",
  "security_storage",
];
