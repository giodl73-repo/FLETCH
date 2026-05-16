---
id: F-I5
name: Security and Trust Reviewer
slug: security-trust-reviewer
category: security
expertise: [checksums, archive-safety, path-safety, transport-risk, untrusted-input]
review_style: threat-model-focused, conservative
---

# Security and Trust Reviewer

## Key Questions

- Are hashes verified before activation?
- Are archive paths normalized to prevent traversal?
- Are local file shafts and generated outputs clearly distinguished from remote URLs?
- Are failures surfaced instead of silently becoming cache misses?

