# Graph Report - backbone-party  (2026-07-22)

## Corpus Check
- 200 files · ~91,588 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 2409 nodes · 4228 edges · 170 communities (167 shown, 3 thin omitted)
- Extraction: 99% EXTRACTED · 1% INFERRED · 0% AMBIGUOUS · INFERRED: 45 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `b4f6ad4c`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- PartyAddress
- Uuid
- Party
- PartyStatus
- PartyContact
- guarded_routes.rs
- Example
- MetadataBuilder
- PartyEmail
- PartyPhone
- ApiVersion
- PartyKind
- PostgresEventStore
- Schema Error Codes Reference
- Recent Generator Changes (Phases 1–10)
- AuditMetadata
- presentation/dto/mod.rs
- ExampleSagaFlowInstance
- OpenAPI Schema Generation
- auth/mod.rs
- ExampleResponseDto
- PostgresSnapshotStore
- PartyContactResponseDto
- TestResult
- ApiTest
- AppState
- Module Integration Schema
- Backbone Schema System
- Schema Standards
- config/generated.rs
- CommonUtils
- Glossary — Ubiquitous Language
- create_guarded_party_routes
- ExampleError
- PartyModule
- create_party_email_routes
- Validation Attributes Quick Reference
- handlers.rs
- create_party_address_routes
- create_party_contact_routes
- create_party_routes
- create_party_phone_routes
- repositories/party_contact_repository.rs
- repositories/party_email_repository.rs
- repositories/party_phone_repository.rs
- Model Schema YAML Rules & Format
- validator/mod.rs
- routes/generated.rs
- PartyAddressApiTest
- PartyContactApiTest
- PartyPhoneApiTest
- Metaphor Domain Module
- party_golden_cases.rs
- Workflow Schema YAML Rules & Format
- services.rs
- SeedExampleSeeder
- SeedPartyAddressSeeder
- SeedPartyContactSeeder
- SeedPartyEmailSeeder
- SeedPartyPhoneSeeder
- SeedPartySeeder
- Schema Architecture
- Developer Guide
- Type System Reference
- ExampleRepository
- PartyAddressRepository
- PartyContactRepository
- PartyEmailRepository
- PartyPhoneRepository
- PartyRepository
- PartyModuleBuilder
- Expression Syntax
- Hook Schema YAML Rules & Format
- Field Attributes
- PartyEmailApiTest
- Maintainer Guide
- Schema Examples
- Common Mistakes
- Common Mistakes
- Custom Types
- ExampleFilter
- Architecture
- Business Flow — Party & Address Book
- Philosophy & Motivation
- Common Mistakes
- Quick Reference Checklist
- PartyApiTest
- backbone-party — FSD
- Action Steps
- Primitive Types
- Backbone Module Skeleton
- backbone-party — PRD
- Quick Reference Checklist
- Special Types
- backbone-party — Extension Guide
- backbone-party — Handbook
- Transitions
- Permissions (RBAC)
- Triggers
- Triggers
- Shared Type Composition
- ADR-001: The party bounded context (identity, roles-as-projections)
- ADR-002: Party data-integrity invariants (one-primary + kind coherence)
- Party — Golden Cases (the oracle)
- Attribute-Based Access Control (ABAC)
- State Actions
- State Machines
- Validation Rules
- Per-Model Generator Filtering
- Field Definitions
- Field Types
- Relations
- Condition Steps
- Loop Steps
- Step Transitions
- Sub-Workflow Composition (Recommended Pattern)
- Terminal Steps
- Value Objects & Typed IDs
- Nullability
- Type Mappings
- Soft Delete & Audit Metadata
- Enums
- Indexes
- Parallel Steps
- Compensation (Rollback)
- Configuration
- Expression Syntax
- Context Variables
- Wait Steps
- seeder.rs
- Seeder
- workflows/README.md

## God Nodes (most connected - your core abstractions)
1. `PartyAddress` - 45 edges
2. `Party` - 39 edges
3. `AuditMetadata` - 37 edges
4. `PartyContact` - 35 edges
5. `PartyStatus` - 35 edges
6. `Example` - 32 edges
7. `PartyEmail` - 30 edges
8. `PartyPhone` - 30 edges
9. `PartyAddressBuilder` - 27 edges
10. `Workflow Schema YAML Rules & Format` - 25 edges

## Surprising Connections (you probably didn't know these)
- `party()` --references--> `NewParty`  [EXTRACTED]
  tests/party_golden_cases.rs → src/application/service/party_write_service.rs
- `module()` --references--> `PartyModule`  [EXTRACTED]
  tests/integrity_probes.rs → src/lib.rs
- `guarded_address_rejects_missing_party()` --calls--> `create_guarded_party_routes()`  [INFERRED]
  tests/integrity_probes.rs → src/presentation/http/guarded_routes.rs
- `guarded_enforces_kind_field_consistency()` --calls--> `create_guarded_party_routes()`  [INFERRED]
  tests/integrity_probes.rs → src/presentation/http/guarded_routes.rs
- `guarded_party_rejects_bad_npwp()` --calls--> `create_guarded_party_routes()`  [INFERRED]
  tests/integrity_probes.rs → src/presentation/http/guarded_routes.rs

## Import Cycles
- 2-file cycle: `src/domain/entity/mod.rs -> src/domain/entity/party_email.rs -> src/domain/entity/mod.rs`
- 2-file cycle: `src/domain/entity/mod.rs -> src/domain/entity/party_address.rs -> src/domain/entity/mod.rs`
- 2-file cycle: `src/domain/entity/mod.rs -> src/domain/entity/party_contact.rs -> src/domain/entity/mod.rs`
- 2-file cycle: `src/domain/entity/mod.rs -> src/domain/entity/party.rs -> src/domain/entity/mod.rs`
- 2-file cycle: `src/domain/entity/mod.rs -> src/domain/entity/party_phone.rs -> src/domain/entity/mod.rs`

## Communities (170 total, 3 thin omitted)

### Community 0 - "PartyAddress"
Cohesion: 0.05
Nodes (25): PartyAddress, PartyAddressBuilder, PartyAddressId, AsRef, DateTime, Decimal, Deref, Display (+17 more)

### Community 1 - "Uuid"
Cohesion: 0.07
Nodes (59): Clone, Debug, PartyAddressId, PartyContactId, PartyEmailId, PartyId, PartyPhoneId, Entity (+51 more)

### Community 2 - "Party"
Cohesion: 0.06
Nodes (24): Party, PartyBuilder, PartyId, AsRef, DateTime, Deref, Display, EntityRepoMeta (+16 more)

### Community 3 - "PartyStatus"
Cohesion: 0.05
Nodes (48): AddressType, Default, Display, Err, Formatter, FromStr, Result, Self (+40 more)

### Community 4 - "PartyContact"
Cohesion: 0.06
Nodes (24): PartyContact, PartyContactBuilder, PartyContactId, AsRef, DateTime, Deref, Display, EntityRepoMeta (+16 more)

### Community 5 - "guarded_routes.rs"
Cohesion: 0.09
Nodes (44): Json, PgQueryResult, NewAddress, NewContact, NewEmail, NewParty, NewPhone, PartyWriteError (+36 more)

### Community 6 - "Example"
Cohesion: 0.07
Nodes (25): Example, ExampleBuilder, ExampleId, AsRef, DateTime, Deref, Display, EntityRepoMeta (+17 more)

### Community 7 - "MetadataBuilder"
Cohesion: 0.07
Nodes (27): Actors, ActorsBuilder, Default, Option, Result, Self, String, Uuid (+19 more)

### Community 8 - "PartyEmail"
Cohesion: 0.07
Nodes (24): PartyEmail, PartyEmailBuilder, PartyEmailId, AsRef, DateTime, Deref, Display, EntityRepoMeta (+16 more)

### Community 9 - "PartyPhone"
Cohesion: 0.07
Nodes (24): PartyPhone, PartyPhoneBuilder, PartyPhoneId, AsRef, DateTime, Deref, Display, EntityRepoMeta (+16 more)

### Community 10 - "ApiVersion"
Cohesion: 0.06
Nodes (40): HeaderMap, Next, Request, S, ApiVersion, RenamedField, RenamedField<T>, Option (+32 more)

### Community 11 - "PartyKind"
Cohesion: 0.06
Nodes (38): PartyKind, Default, Display, Err, Formatter, FromStr, Result, Self (+30 more)

### Community 12 - "PostgresEventStore"
Cohesion: 0.09
Nodes (30): Box, Item, Pin, EventEnvelope, EventEnvelope<T>, EventMetadata, DateTime, HashMap (+22 more)

### Community 13 - "Schema Error Codes Reference"
Cohesion: 0.04
Nodes (49): 1. "Unknown type" errors, 2. YAML indentation errors, 3. Circular reference detection, Common Issues, E001: Lexer Error, E002: Syntax Error, E003: Unexpected Token, E004: Unexpected End of File (+41 more)

### Community 14 - "Recent Generator Changes (Phases 1–10)"
Cohesion: 0.04
Nodes (45): After Generation, API Layer (5), Business Logic (12), CLI Reference, Code Generation Reference, Common Issues, CQRS / Projection, Custom Block Markers (in `.rs` files) (+37 more)

### Community 15 - "AuditMetadata"
Cohesion: 0.09
Nodes (25): AuditMetadata, DateTime, Option, Self, Utc, Uuid, CreatePartyPhoneDto, PartyPhone (+17 more)

### Community 16 - "presentation/dto/mod.rs"
Cohesion: 0.09
Nodes (29): ApiError, ApiResponse, ApiResponse<T>, PaginationParams, Into, Option, Self, String (+21 more)

### Community 17 - "ExampleSagaFlowInstance"
Cohesion: 0.10
Nodes (24): H, ExampleSagaFlowExecutor, ExampleSagaFlowExecutor<H>, ExampleSagaFlowInstance, ExampleSagaFlowStatus, ExampleSagaFlowStep, ExampleSagaStepHandler, FlowError (+16 more)

### Community 18 - "OpenAPI Schema Generation"
Cohesion: 0.06
Nodes (36): 1. Keep Custom Endpoints Separate, 2. Use Descriptive Operation IDs, 3. Document Business Rules, 4. Version Your API, API Gateway (Kong), Attribute to OpenAPI Constraint, Best Practices, CLI Commands (+28 more)

### Community 19 - "auth/mod.rs"
Cohesion: 0.07
Nodes (25): PartyAddressPolicy, AuthContext, PartyAddress, ResourceAction, ResourcePolicy, PartyPolicy, AuthContext, Party (+17 more)

### Community 20 - "ExampleResponseDto"
Cohesion: 0.15
Nodes (20): CreateExampleDto, Example, ExampleListResponseDto, ExampleResponseDto, ExampleSummaryDto, PatchExampleDto, ApplyUpdateDto, DateTime (+12 more)

### Community 21 - "PostgresSnapshotStore"
Cohesion: 0.13
Nodes (17): PostgresSnapshotStore, DateTime, Default, Into, Option, PgPool, Result, Self (+9 more)

### Community 22 - "PartyContactResponseDto"
Cohesion: 0.17
Nodes (19): CreatePartyContactDto, PartyContact, PartyContactListResponseDto, PartyContactResponseDto, PartyContactSummaryDto, PatchPartyContactDto, ApplyUpdateDto, DateTime (+11 more)

### Community 23 - "TestResult"
Cohesion: 0.21
Nodes (10): G, TestResult, create_and_get_id(), CrudTestConfig, GenericCrudTest, GenericCrudTest<G>, Option, Self (+2 more)

### Community 24 - "ApiTest"
Cohesion: 0.24
Nodes (12): Client, RequestBuilder, ApiResponse, ApiTest, Error, HashMap, Into, Option (+4 more)

### Community 25 - "AppState"
Cohesion: 0.22
Nodes (13): health_check(), IntoResponse, State, AppState, AppStateBuilder, Arc, Option, PartyAddressService (+5 more)

### Community 26 - "Module Integration Schema"
Cohesion: 0.09
Nodes (22): Cross-Module Event Subscriptions, Cross-Module Foreign Keys, Current `external_imports` Syntax, Dependency Rules, Event Flow Diagram, Event Subscriptions, Exports, File Locations (+14 more)

### Community 27 - "Backbone Schema System"
Cohesion: 0.09
Nodes (22): 1. Define Your Model, 2. Define Entity Hook, 3. Generate Everything, Architecture, Backbone Schema System, Design Principles, Documentation Map, Example: Creating a New Entity (+14 more)

### Community 28 - "Schema Standards"
Cohesion: 0.09
Nodes (21): Audit Fields, Common Field Patterns, Common Patterns, Enum Definition, Field Standards, Foreign Key Attributes, Model Naming Conventions, Model Standards (+13 more)

### Community 29 - "config/generated.rs"
Cohesion: 0.19
Nodes (17): Path, DatabaseConfig, expand_env_vars(), FeaturesConfig, LoggingConfig, merge_yaml(), MetricsConfig, ModuleConfig (+9 more)

### Community 30 - "CommonUtils"
Cohesion: 0.18
Nodes (8): CommonUtils, Send, Sync, TestDataGenerator, PartyTestData, Value, PartyEmailTestData, Value

### Community 31 - "Glossary — Ubiquitous Language"
Cohesion: 0.10
Nodes (21): Address book (multi-channel), BackboneCrudHandler, CUSTOM marker, Full / unguarded surface, GenericCrudService / GenericCrudRepository, Glossary — Ubiquitous Language, Guarded routes, Logical FK (+13 more)

### Community 32 - "create_guarded_party_routes"
Cohesion: 0.29
Nodes (20): create_guarded_party_routes(), create_party_write_routes(), Router, StatusCode, guarded_address_rejects_missing_party(), guarded_enforces_kind_field_consistency(), guarded_party_rejects_bad_npwp(), guarded_rejects_second_primary_address() (+12 more)

### Community 33 - "ExampleError"
Cohesion: 0.15
Nodes (15): ExampleService, Module, create_example_routes(), ExampleError, Arc, From, IntoResponse, Response (+7 more)

### Community 34 - "PartyModule"
Cohesion: 0.24
Nodes (15): PartyModule, Arc, PartyAddressService, PartyContactService, PartyEmailService, PartyPhoneService, PartyService, Router (+7 more)

### Community 35 - "create_party_email_routes"
Cohesion: 0.19
Nodes (15): create_party_email_read_routes(), create_party_email_routes(), create_party_email_write_routes(), create_protected_party_email_routes(), PartyEmailError, A, Arc, From (+7 more)

### Community 36 - "Validation Attributes Quick Reference"
Cohesion: 0.11
Nodes (18): Array, Conditional, Cross-Field, Database, Date & Time, Documentation, Enum & Choice, File & Image (+10 more)

### Community 37 - "handlers.rs"
Cohesion: 0.16
Nodes (12): PartyAddressEvent, PartyContactEvent, PartyEmailEvent, PartyPhoneEvent, PartyAddressEventHandler, EventHandler, PartyContactEventHandler, EventHandler (+4 more)

### Community 38 - "create_party_address_routes"
Cohesion: 0.20
Nodes (15): create_party_address_read_routes(), create_party_address_routes(), create_party_address_write_routes(), create_protected_party_address_routes(), PartyAddressError, A, Arc, From (+7 more)

### Community 39 - "create_party_contact_routes"
Cohesion: 0.20
Nodes (15): create_party_contact_read_routes(), create_party_contact_routes(), create_party_contact_write_routes(), create_protected_party_contact_routes(), PartyContactError, A, Arc, From (+7 more)

### Community 40 - "create_party_routes"
Cohesion: 0.20
Nodes (15): create_party_read_routes(), create_party_routes(), create_party_write_routes(), create_protected_party_routes(), PartyError, A, Arc, From (+7 more)

### Community 41 - "create_party_phone_routes"
Cohesion: 0.20
Nodes (15): create_party_phone_read_routes(), create_party_phone_routes(), create_party_phone_write_routes(), create_protected_party_phone_routes(), PartyPhoneError, A, Arc, From (+7 more)

### Community 42 - "repositories/party_contact_repository.rs"
Cohesion: 0.13
Nodes (12): PartyContactFilter, PartyContactPaginatedResult, PartyContactPaginationParams, PartyContactRepository, Option, PartyContact, Self, Send (+4 more)

### Community 43 - "repositories/party_email_repository.rs"
Cohesion: 0.13
Nodes (12): PartyEmailFilter, PartyEmailPaginatedResult, PartyEmailPaginationParams, PartyEmailRepository, Option, PartyEmail, Self, Send (+4 more)

### Community 44 - "repositories/party_phone_repository.rs"
Cohesion: 0.13
Nodes (12): PartyPhoneFilter, PartyPhonePaginatedResult, PartyPhonePaginationParams, PartyPhoneRepository, Option, PartyPhone, Self, Send (+4 more)

### Community 45 - "Model Schema YAML Rules & Format"
Cohesion: 0.12
Nodes (16): Complete Structure, Complete Structure, Definition, Definition in index.model.yaml, Domain Entities, Enhanced Entity Definition, Entity Model Files, File Organization (+8 more)

### Community 46 - "validator/mod.rs"
Cohesion: 0.12
Nodes (10): PartyAddressValidator, PartyContactValidator, PartyEmailValidator, PartyPhoneValidator, PartyValidator, party_address_validator(), party_contact_validator(), party_email_validator() (+2 more)

### Community 47 - "routes/generated.rs"
Cohesion: 0.30
Nodes (14): configure_routes(), HttpServices, party_address_routes(), party_contact_routes(), party_email_routes(), party_phone_routes(), party_routes(), Arc (+6 more)

### Community 48 - "PartyAddressApiTest"
Cohesion: 0.20
Nodes (8): PartyAddressApiTest, PartyAddressTestData, Default, Self, String, Value, Vec, test_party_address_crud()

### Community 49 - "PartyContactApiTest"
Cohesion: 0.20
Nodes (8): PartyContactApiTest, PartyContactTestData, Default, Self, String, Value, Vec, test_party_contact_crud()

### Community 50 - "PartyPhoneApiTest"
Cohesion: 0.20
Nodes (8): PartyPhoneApiTest, PartyPhoneTestData, Default, Self, String, Value, Vec, test_party_phone_crud()

### Community 51 - "Metaphor Domain Module"
Cohesion: 0.14
Nodes (13): Anti-patterns, Common tasks, Deeper knowledge (load on demand), Four-layer folder cheatsheet, Golden path, graphify, Key files to read before editing, Metaphor Domain Module (+5 more)

### Community 52 - "party_golden_cases.rs"
Cohesion: 0.34
Nodes (12): address_links_geo_by_logical_fk(), children_require_existing_party(), create_organization_party(), nik(), npwp(), party(), pool(), rejects_duplicate_code_and_npwp() (+4 more)

### Community 54 - "Workflow Schema YAML Rules & Format"
Cohesion: 0.15
Nodes (13): Complete Template, File Organization, Human Task Steps, Naming Conventions, Overview, Overview, Step Types, Subprocess Steps (+5 more)

### Community 55 - "services.rs"
Cohesion: 0.18
Nodes (9): R, ExportSummary, PartyQueryService, PartyQueryServiceImpl, PartyQueryServiceImpl<R>, Arc, Self, Send (+1 more)

### Community 56 - "SeedExampleSeeder"
Cohesion: 0.26
Nodes (5): Default, PgPool, Result, Self, SeedExampleSeeder

### Community 57 - "SeedPartyAddressSeeder"
Cohesion: 0.26
Nodes (5): Default, PgPool, Result, Self, SeedPartyAddressSeeder

### Community 58 - "SeedPartyContactSeeder"
Cohesion: 0.26
Nodes (5): Default, PgPool, Result, Self, SeedPartyContactSeeder

### Community 59 - "SeedPartyEmailSeeder"
Cohesion: 0.26
Nodes (5): Default, PgPool, Result, Self, SeedPartyEmailSeeder

### Community 60 - "SeedPartyPhoneSeeder"
Cohesion: 0.26
Nodes (5): Default, PgPool, Result, Self, SeedPartyPhoneSeeder

### Community 61 - "SeedPartySeeder"
Cohesion: 0.26
Nodes (5): Default, PgPool, Result, Self, SeedPartySeeder

### Community 62 - "Schema Architecture"
Cohesion: 0.17
Nodes (12): Application Layer, Bounded Context Model, Data Flow, Domain Layer, Generated File Layout, Infrastructure Layer, Layer Overview, Module Boundaries (+4 more)

### Community 63 - "Developer Guide"
Cohesion: 0.18
Nodes (11): Configuration, Developer Guide, How do I keep my projection in sync?, How do I read parties?, How do I register a person (not an organization)?, How do I switch which address is primary?, Install, Key concepts (+3 more)

### Community 65 - "Type System Reference"
Cohesion: 0.18
Nodes (11): Arrays, Automatic Coercion, Automatic PascalCase Conversion, Collection Types, Explicit Casting, Maps, Next Steps, Table of Contents (+3 more)

### Community 66 - "ExampleRepository"
Cohesion: 0.22
Nodes (8): ExampleRepository, Deref, Example, GenericCrudRepository, PgPool, Self, SoftDelete, Target

### Community 67 - "PartyAddressRepository"
Cohesion: 0.22
Nodes (8): PartyAddressRepository, Deref, GenericCrudRepository, PartyAddress, PgPool, Self, SoftDelete, Target

### Community 68 - "PartyContactRepository"
Cohesion: 0.22
Nodes (8): PartyContactRepository, Deref, GenericCrudRepository, PartyContact, PgPool, Self, SoftDelete, Target

### Community 69 - "PartyEmailRepository"
Cohesion: 0.22
Nodes (8): PartyEmailRepository, Deref, GenericCrudRepository, PartyEmail, PgPool, Self, SoftDelete, Target

### Community 70 - "PartyPhoneRepository"
Cohesion: 0.22
Nodes (8): PartyPhoneRepository, Deref, GenericCrudRepository, PartyPhone, PgPool, Self, SoftDelete, Target

### Community 71 - "PartyRepository"
Cohesion: 0.22
Nodes (8): PartyRepository, Deref, GenericCrudRepository, Party, PgPool, Self, SoftDelete, Target

### Community 72 - "PartyModuleBuilder"
Cohesion: 0.29
Nodes (6): PartyModuleBuilder, Default, Option, PgPool, Result, Self

### Community 73 - "Expression Syntax"
Cohesion: 0.20
Nodes (10): Aggregate Functions, Collection Operations, Comparison Operators, Date/Time, Expression Syntax, Field References, Logical Operators, Null Handling (+2 more)

### Community 74 - "Hook Schema YAML Rules & Format"
Cohesion: 0.20
Nodes (10): Basic Computed Fields, Complete Template, Computed Field Rules, Computed Fields, File Organization, Hook File Structure, Hook Schema YAML Rules & Format, Naming Conventions (+2 more)

### Community 75 - "Field Attributes"
Cohesion: 0.20
Nodes (10): Array Validation, Choice Validation, Cross-Field Validation, Date/Time Validation, Field Attributes, Identity & Keys, Numeric Validation, Required & Defaults (+2 more)

### Community 76 - "PartyEmailApiTest"
Cohesion: 0.29
Nodes (6): PartyEmailApiTest, Default, Self, String, Vec, test_party_email_crud()

### Community 77 - "Maintainer Guide"
Cohesion: 0.22
Nodes (9): Before you touch anything, Maintainer Guide, Migrations, The regeneration contract, Versioning & release, Walkthrough A — add a field to Party (regen-safe), Walkthrough B — add a validated write rule, What will break things (+1 more)

### Community 78 - "Schema Examples"
Cohesion: 0.22
Nodes (9): Example 1: Module Index File, Example 2: Simple Entity Model, Example 3: Entity Hook with Complex State Machine, Example 4: Sub-Workflow (Phase 10 Pattern), Example 5: Cross-Module Foreign Key, Example 6: Per-Entity Generator Filtering, Schema Examples, Table of Contents (+1 more)

### Community 79 - "Common Mistakes"
Cohesion: 0.22
Nodes (9): 1. Missing or Multiple Initial States, 2. Transition to Non-Existent State, 3. Invalid From Array Syntax, 4. Rule Without Condition, 5. Permission Without Role, 6. Trigger with Wrong Name Pattern, 7. Action String Syntax Errors, 8. Computed Field with Side Effects (+1 more)

### Community 80 - "Common Mistakes"
Cohesion: 0.22
Nodes (9): 1. Missing @id on Primary Key, 2. Self-Reference Without @foreign_key, 3. Using @required on Optional Fields, 4. Wrong Enum Default Syntax, 5. Missing Quotes for Array Types, 6. Wrong Collection Name Format, 7. Duplicate Index Names, 8. External Reference Without Import (+1 more)

### Community 81 - "Custom Types"
Cohesion: 0.22
Nodes (9): 1. Type Inheritance with `extends`, 2. Shared Type as JSONB Field, Custom Types, Defining Custom Types, File-Level Types, JSONB Validation, Type Composition, Type with Validation (+1 more)

### Community 82 - "ExampleFilter"
Cohesion: 0.22
Nodes (7): ExampleFilter, ExampleRepository, ExampleStatus, Option, Send, String, Sync

### Community 83 - "Architecture"
Cohesion: 0.25
Nodes (8): 1. Context, 2. Containers, 3. Components — the DDD 4-layer shape, 4. Data & control flow — `POST /parties`, traced, 5. The stack, and why, Architecture, Key decisions, Two mounting surfaces — pick deliberately

### Community 84 - "Business Flow — Party & Address Book"
Cohesion: 0.25
Nodes (8): Actors, Add a contact person / email / phone, Add an address (structured against geo), Business Flow — Party & Address Book, Flows, Not here (projections / other modules), Register a party, The accounting seam (now fulfilled)

### Community 85 - "Philosophy & Motivation"
Cohesion: 0.25
Nodes (8): Indonesia-first, Non-goals (what this module deliberately will not do), Philosophy & Motivation, Roles are projections — the picture, The problem, The worldview (four commitments), What we borrowed, and what we rejected, Why trust it today

### Community 86 - "Common Mistakes"
Cohesion: 0.25
Nodes (8): 1. Unreachable Steps (CRITICAL), 2. Missing Terminal Step, 3. Condition Without Else, 4. Loop Without Terminal for Iterations, 5. Missing on_failure Handler, 6. Referencing Wrong Context, 7. Step Name Conflicts, Common Mistakes

### Community 87 - "Quick Reference Checklist"
Cohesion: 0.25
Nodes (8): Conditions, Context, Loops, Parallel, Quick Reference Checklist, Steps, Transitions, Workflow Structure

### Community 88 - "PartyApiTest"
Cohesion: 0.36
Nodes (5): PartyApiTest, Default, Self, Vec, test_party_crud()

### Community 89 - "backbone-party — FSD"
Cohesion: 0.29
Nodes (7): backbone-party — FSD, Behavior specs, Endpoints, Entities, Integration (logical FKs — no DB FK, no Cargo edge), Non-goals, Validated write rules (R1–R8)

### Community 90 - "Action Steps"
Cohesion: 0.29
Nodes (7): Action Steps, Create Action, Custom Action, Delete Action, Emit Event Action, Query Action, Update Action

### Community 91 - "Primitive Types"
Cohesion: 0.29
Nodes (7): Boolean, Date and Time, Decimal Precision, Identifiers, Numeric Types, Primitive Types, String Types

### Community 92 - "Backbone Module Skeleton"
Cohesion: 0.29
Nodes (6): Backbone Module Skeleton, Custom code (regeneration safety), Directory layout, Getting started, Going further, What you get

### Community 94 - "backbone-party — PRD"
Cohesion: 0.33
Nodes (6): backbone-party — PRD, Indonesia-first notes, Personas, Problem, Scope, Success criteria

### Community 95 - "Quick Reference Checklist"
Cohesion: 0.33
Nodes (6): Computed Fields, Permissions, Quick Reference Checklist, Rules, State Machine, Triggers

### Community 96 - "Special Types"
Cohesion: 0.33
Nodes (6): Binary Types, File Types, Password Type, Special Types, Structured Types, Validated String Types

### Community 99 - "backbone-party — Extension Guide"
Cohesion: 0.40
Nodes (5): backbone-party — Extension Guide, Composing into a service, Consuming the multi-master seam, Public / stable surface, Regeneration safety

### Community 100 - "backbone-party — Handbook"
Cohesion: 0.40
Nodes (5): backbone-party — Handbook, Reference material (already in-repo), Start here by who you are, The handbook (canonical sections), The one rule that governs everything

### Community 101 - "Transitions"
Cohesion: 0.40
Nodes (5): Basic Transition, Multiple Source States, Transition Condition Expressions, Transition with Guards, Transitions

### Community 102 - "Permissions (RBAC)"
Cohesion: 0.40
Nodes (5): Complete Example, Permission Actions, Permission Expression Variables, Permissions (RBAC), Structure

### Community 103 - "Triggers"
Cohesion: 0.40
Nodes (5): Scheduled Triggers (in index.hook.yaml), Trigger Examples, Trigger Structure, Trigger Types, Triggers

### Community 104 - "Triggers"
Cohesion: 0.40
Nodes (5): Event Trigger, Extract Variables, HTTP Endpoint Trigger, Schedule Trigger (Cron), Triggers

### Community 105 - "Shared Type Composition"
Cohesion: 0.40
Nodes (5): Defining Shared Types, Shared Type Composition, Using Shared Types as Columns (`extends`), Using Shared Types as JSONB Fields, When to Use Which

### Community 106 - "ADR-001: The party bounded context (identity, roles-as-projections)"
Cohesion: 0.50
Nodes (4): ADR-001: The party bounded context (identity, roles-as-projections), Consequences, Context, Decision

### Community 107 - "ADR-002: Party data-integrity invariants (one-primary + kind coherence)"
Cohesion: 0.50
Nodes (4): ADR-002: Party data-integrity invariants (one-primary + kind coherence), Consequences, Context, Decision

### Community 108 - "Party — Golden Cases (the oracle)"
Cohesion: 0.50
Nodes (4): Conventions, Guarded write path (`tests/integrity_probes.rs`), Party — Golden Cases (the oracle), Validated writes (`tests/party_golden_cases.rs`)

### Community 109 - "Attribute-Based Access Control (ABAC)"
Cohesion: 0.50
Nodes (4): ABAC Attributes, Attribute-Based Access Control (ABAC), Policies, Resource Policies

### Community 110 - "State Actions"
Cohesion: 0.50
Nodes (4): Action Expression Variables, Action Types, Conditional Actions, State Actions

### Community 111 - "State Machines"
Cohesion: 0.50
Nodes (4): Basic Structure, Critical Rules for States, State Definition, State Machines

### Community 112 - "Validation Rules"
Cohesion: 0.50
Nodes (4): Rule Examples, Rule Structure, Validation Rules, When Clauses

### Community 113 - "Per-Model Generator Filtering"
Cohesion: 0.50
Nodes (4): Available Targets, Module-Level Filtering, Per-Entity Override, Per-Model Generator Filtering

### Community 114 - "Field Definitions"
Cohesion: 0.50
Nodes (4): Field Definitions, Full Syntax, Shorthand Syntax, Special Fields

### Community 115 - "Field Types"
Cohesion: 0.50
Nodes (4): Field Types, Primitive Types, String Format Types, Type Modifiers

### Community 116 - "Relations"
Cohesion: 0.50
Nodes (4): Relation Attributes, Relation Types, Relations, Self-Referencing Relations

### Community 117 - "Condition Steps"
Cohesion: 0.50
Nodes (4): Basic Condition, Complex Conditions, Condition Steps, Multiple Conditions

### Community 118 - "Loop Steps"
Cohesion: 0.50
Nodes (4): Basic Loop Structure, Loop Steps, Loop with Index, Nested Loops

### Community 119 - "Step Transitions"
Cohesion: 0.50
Nodes (4): Basic Transition, Retry Configuration, Setting Context on Transition, Step Transitions

### Community 120 - "Sub-Workflow Composition (Recommended Pattern)"
Cohesion: 0.50
Nodes (4): Example: Order Processing Chain, Rules for Sub-Workflow Chains, Sub-Workflow Composition (Recommended Pattern), Why Decompose

### Community 121 - "Terminal Steps"
Cohesion: 0.50
Nodes (4): Failed Terminal, Success Terminal, Terminal Steps, Terminal with Event Emission

### Community 122 - "Value Objects & Typed IDs"
Cohesion: 0.50
Nodes (4): Composite Value Objects, Typed IDs, Value Objects & Typed IDs, Wrapper Value Objects

### Community 123 - "Nullability"
Cohesion: 0.50
Nodes (4): Default Values, Nullability, Nullability Rules, Required vs Optional

### Community 124 - "Type Mappings"
Cohesion: 0.50
Nodes (4): Schema to PostgreSQL, Schema to Proto, Schema to Rust, Type Mappings

### Community 125 - "Soft Delete & Audit Metadata"
Cohesion: 0.67
Nodes (3): Audit Metadata Pattern, Soft Delete, Soft Delete & Audit Metadata

### Community 126 - "Enums"
Cohesion: 0.67
Nodes (3): Enums, Full Enum Definition, Rules for Enums

### Community 127 - "Indexes"
Cohesion: 0.67
Nodes (3): Index Options, Index Types, Indexes

### Community 128 - "Parallel Steps"
Cohesion: 0.67
Nodes (3): Basic Parallel Structure, Join Strategies, Parallel Steps

### Community 129 - "Compensation (Rollback)"
Cohesion: 0.67
Nodes (3): Compensation (Rollback), Compensation Structure, Complete Example

### Community 130 - "Configuration"
Cohesion: 0.67
Nodes (3): Complete Config Options, Configuration, Timeout Formats

### Community 131 - "Expression Syntax"
Cohesion: 0.67
Nodes (3): Condition Expressions, Expression Syntax, Template Syntax

### Community 132 - "Context Variables"
Cohesion: 0.67
Nodes (3): Context Variables, Declaring Context, Using Context in Steps

### Community 133 - "Wait Steps"
Cohesion: 0.67
Nodes (3): Wait for Duration, Wait for Event, Wait Steps

## Knowledge Gaps
- **485 isolated node(s):** `ExportSummary`, `PartyQueryServiceImpl<R>`, `VersionTransform`, `VersionedResponse<T>`, `RenamedField<T>` (+480 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **3 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `AuditMetadata` connect `AuditMetadata` to `PartyAddress`, `Uuid`, `Party`, `PartyStatus`, `PartyContact`, `Example`, `PartyEmail`, `PartyPhone`, `PartyKind`, `presentation/dto/mod.rs`, `ExampleResponseDto`, `PartyContactResponseDto`?**
  _High betweenness centrality (0.130) - this node is a cross-community bridge._
- **Why does `PartyStatus` connect `PartyStatus` to `PartyAddress`, `Uuid`, `Party`, `PartyKind`?**
  _High betweenness centrality (0.046) - this node is a cross-community bridge._
- **What connects `ExportSummary`, `PartyQueryServiceImpl<R>`, `VersionTransform` to the rest of the system?**
  _485 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `PartyAddress` be split into smaller, more focused modules?**
  _Cohesion score 0.05280437756497948 - nodes in this community are weakly interconnected._
- **Should `Uuid` be split into smaller, more focused modules?**
  _Cohesion score 0.07056962025316456 - nodes in this community are weakly interconnected._
- **Should `Party` be split into smaller, more focused modules?**
  _Cohesion score 0.05974124809741248 - nodes in this community are weakly interconnected._
- **Should `PartyStatus` be split into smaller, more focused modules?**
  _Cohesion score 0.05285592497868713 - nodes in this community are weakly interconnected._