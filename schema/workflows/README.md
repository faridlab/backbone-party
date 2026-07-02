# Party Workflows

Party has no multi-step sagas — creation is a single validated `create_party`, and children
(address/contact/email/phone) are single validated adds. Status transitions (Party
active↔inactive↔blocked) are declared as a state machine in `schema/hooks/party.hook.yaml`.

One cross-module saga lives in **CRM, not here**: Lead → Customer conversion mints a canonical
Party via an ACL step (docs/erp/relationship-crm.md). That saga is owned by the converting context.
