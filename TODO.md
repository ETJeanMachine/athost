# TODO for AThost implementation

this is my wonderful little sheet of things that i need to do for this project :p

[bryan's github on this issue](https://github.com/bluesky-social/atproto/discussions/2961)

---

## early stage stuff for creating an appview:

### accounts and identities
- resolve and validate identities (handle and DID), include a caching layer
- keeping track of account state (activate, deleted, deactivated, etc.)
- consume account and identity events from the firehose and update status as needed

some notes:
- need to think more about account deletion/deactivation and how to handle it in a way that makes sense - do users want to delete their entire atproto account (likely not). so restricting this sort of record creation to *within* athost makes more sense, but it needs to make sense.

TODO EVEN MORE BUT THIS IS GOOD FOR NOW IG LOL
