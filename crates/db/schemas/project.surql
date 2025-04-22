DEFINE TABLE project SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD id ON project TYPE string ASSERT $value != NONE;
DEFINE FIELD name ON project TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD owner_id ON project TYPE option<string>;
DEFINE FIELD description ON project TYPE option<string>;
DEFINE FIELD tags ON project TYPE option<array>;
DEFINE FIELD start_date ON project TYPE option<datetime>;
DEFINE FIELD due_date ON project TYPE option<datetime>;
DEFINE FIELD children ON project TYPE option<array>;
DEFINE FIELD dependencies ON project TYPE option<array>;
DEFINE FIELD status ON project TYPE string ASSERT $value INSIDE ['NotStarted', 'Planned', 'InProgress', 'InReview', 'Completed', 'Archived', 'Canceled'];