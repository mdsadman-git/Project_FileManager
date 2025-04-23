pub const LOGGER_FORMAT_DATE:     &str        = "[year padding:zero]-[month padding:zero]-[day padding:zero]";
pub const LOGGER_FORMAT_TIME:     &str        = "[hour padding:zero]:[minute padding:zero]:[second padding:zero].[subsecond digits:6]";

pub const HOST_IP_ADDRESS:        &str        = "127.0.0.1";
pub const HOST_DEFAULT_PORT:      &str        = "7000";
pub const TOTAL_ACTIVE_THREADS:   usize       = 10;

// TODO: TEMPORARY
pub const THREADING_ROUTING_TABLE: &'static [&str] = &[
  "/",
  "/sleep",
  "/folder/create",
  "/folder/delete",
  "/file/create",
  "/file/delete",
];

pub const ASYNC_ROUTING_TABLE: &'static [&str] = &[
  "/notification",
];

