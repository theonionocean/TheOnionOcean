DEFINE TABLE project SCHEMAFULL;

DEFINE FIELD name ON project TYPE string;
DEFINE FIELD slug ON project TYPE string;
DEFINE FIELD description ON project TYPE string;
DEFINE FIELD created_by ON project TYPE record<user>;
DEFINE FIELD modified_by ON project TYPE record<user>;
DEFINE FIELD created_at ON project TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON project TYPE datetime DEFAULT time::now();

DEFINE INDEX idx_name ON project FIELDS name UNIQUE;
DEFINE INDEX idx_slug ON project FIELDS slug UNIQUE;
