DEFINE TABLE task SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD id ON task TYPE string ASSERT $value != NONE;
DEFINE FIELD name ON task TYPE string ASSERT string::len($value) > 0;
DEFINE FIELD owner_id ON task TYPE option<string>;
DEFINE FIELD description ON task TYPE option<string>;
DEFINE FIELD tags ON task TYPE option<array>;
DEFINE FIELD start_date ON task TYPE option<datetime>;
DEFINE FIELD due_date ON task TYPE option<datetime>;
DEFINE FIELD children ON task TYPE option<array>;
DEFINE FIELD dependencies ON task TYPE option<array>;
DEFINE FIELD status ON task TYPE string ASSERT $value INSIDE ['NotStarted', 'Planned', 'InProgress', 'InReview', 'Completed', 'Archived', 'Canceled'];