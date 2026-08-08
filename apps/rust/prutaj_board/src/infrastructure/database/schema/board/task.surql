DEFINE TABLE task SCHEMAFULL;

DEFINE FIELD title ON task TYPE string;
DEFINE FIELD description ON task TYPE string;
DEFINE FIELD status ON task TYPE string | null; // TODO: Add enum
DEFINE FIELD assigned_to ON task TYPE record<user> | null;
DEFINE FIELD created_by ON task TYPE string;
DEFINE FIELD modified_by ON task TYPE string;
DEFINE FIELD created_at ON task TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON task TYPE datetime DEFAULT time::now();
DEFINE FIELD project ON task TYPE record<project> REFERENCE ON DELETE CASCADE;

DEFINE INDEX idx_title ON task FIELDS title UNIQUE;