-- ADMIN
// # =============================== ADMIN TABLE =============================== #
DEFINE TABLE admin SCHEMALESS;

DEFINE FIELD phone_number ON admin TYPE option<string>;
DEFINE INDEX phone_number ON admin COLUMNS UNIQUE;


-- MANAGMENT
// # =============================== USER TABLE =============================== #
DEFINE TABLE user SCHEMALESS;

DEFINE FIELD phone_number ON user TYPE option<string>;
DEFINE INDEX phone_number ON user COLUMNS UNIQUE;

DEFINE FIELD email ON user TYPE option<string>;
DEFINE INDEX email ON user COLUMNS email UNIQUE;


-- ORGANIZATION

// # =============================== ORGANIZATION TABLE =============================== #
DEFINE TABLE organization SCHEMALESS;

DEFINE FIELD name ON organization TYPE option<string>;
DEFINE INDEX name ON organization COLUMNS name UNIQUE;

// # =============================== BRANCH TABLE =============================== #
DEFINE TABLE branch SCHEMALESS;

DEFINE FIELD name ON branch TYPE option<string>;
DEFINE INDEX name ON branch COLUMNS name UNIQUE;


-- MESSAGE

// # =============================== TelegramGroup TABLE =============================== #
DEFINE TABLE telegram_group SCHEMALESS;

DEFINE FIELD group_id ON telegram_group TYPE option<string>;
DEFINE INDEX group_id ON telegram_group COLUMNS group_id UNIQUE;