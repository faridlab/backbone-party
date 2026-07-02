# Party acceptance oracle — backbone-party
# Flow map:    docs/business-flows/party.md
# Golden cases: docs/business-flows/golden-cases.md
# Executable truth: tests/party_golden_cases.rs + tests/integrity_probes.rs

Feature: Canonical party identity and address book
  In order to give every ERP module one stable party to reference
  As a master-data admin
  I want to register parties and their multi-channel address book under validated rules

  Background:
    Given the tenant schema "party" is migrated

  @happy-path @module:party @pgc-1
  Scenario: Register an organization party
    When I register party "ACME" as an organization with a valid NPWP
    Then it is created with kind "organization" and status "active"

  @validation @module:party @pgc-2
  Scenario: A malformed NPWP is rejected
    When I register a party with NPWP "123"
    Then the request is rejected with "invalid_npwp"

  @validation @module:party @pgc-3
  Scenario: A duplicate party code is rejected
    Given a party "DUP" already exists
    When I register another party with code "DUP"
    Then the request is rejected with "duplicate_party_code"

  @geo @module:party @pgc-5
  Scenario: An address links to geo by logical foreign key
    Given a party exists
    When I add a shipping address with a geo subdistrict id
    Then the address stores that subdistrict id verbatim

  @validation @module:party @igc-3
  Scenario: Adding a child to a missing party is rejected
    When I add an address for a non-existent party
    Then the request is rejected with status 422
