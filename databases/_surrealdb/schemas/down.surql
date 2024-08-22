-- ADMIN
// # =============================== ADMIN TABLE =============================== #
REMOVE INDEX phone_number ON admin;
REMOVE FIELD phone_number ON admin;
REMOVE TABLE admin;


-- MANAGEMENT
// # =============================== USER TABLE =============================== #
REMOVE INDEX phone_number ON user;
REMOVE INDEX email ON user;
REMOVE FIELD phone_number ON user;
REMOVE FIELD email ON user;
REMOVE TABLE user;


-- ORGANIZATION
// # =============================== ORGANIZATION TABLE =============================== #
REMOVE INDEX name ON organization;
REMOVE FIELD name ON organization;
REMOVE TABLE organization;

// # =============================== BRANCH TABLE =============================== #
REMOVE INDEX name ON branch;
REMOVE FIELD name ON branch;
REMOVE TABLE branch;


-- MESSAGE
// # =============================== TelegramGroup TABLE =============================== #
REMOVE INDEX group_id ON telegram_group;
REMOVE FIELD group_id ON telegram_group;
REMOVE TABLE telegram_group;
