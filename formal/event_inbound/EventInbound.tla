---- MODULE EventInbound ----
\* Model of HttpEventInbound::handle in
\* crates/openlark-client/src/event_inbound/handler.rs
\*
\* One HTTP request at a time (no concurrency). TLC explores every combination
\* of config × body shape × req type × signature × decrypt/token outcomes.
\* Steps match the Rust control flow one-for-one:
\*   Extract     <-> extract_payload
\*   Decrypt     <-> decrypt_event (only if encrypt_key set AND envelope)
\*   Parse       <-> serde_json::from_slice FuzzyEvent + nested-encrypt guard
\*   AuthGate    <-> verify_sign (events only; skipped for url_verification /
\*                  empty encrypt_key / skip_sign_verify)
\*   Finish      <-> auth_challenge OR dispatcher.dispatch_with_response
\*
\* Crypto bits (AES / SHA256) are abstracted as CipherOk / Sig ∈ {Good,Bad,Missing}.
\* Signature is over the raw HTTP body (body_str), as in verify_sign(req, body_str).

EXTENDS Naturals, FiniteSets, TLC

VARIABLES
  encrypt_key_set,     \* encrypt_key.is_empty() == FALSE
  skip_sign,           \* skip_sign_verify
  body_kind,           \* "Envelope" | "Plain" | "BlankEncrypt"
  req_type,            \* "Challenge" | "Event"
  sig,                 \* "Missing" | "Bad" | "Good"  (headers / verify result)
  cipher_ok,           \* decrypt_event Ok vs Err
  challenge_token_ok,  \* verification_token match in auth_challenge
  nested_encrypt,      \* FuzzyEvent.encrypt nonempty (empty-key path)
  pc,                  \* pipeline program counter
  did_decrypt,         \* TRUE iff decrypt_event was invoked
  outcome              \* "Pending" | "Dispatched" | "ChallengeEcho" | "Rejected"

vars == <<
  encrypt_key_set, skip_sign, body_kind, req_type, sig,
  cipher_ok, challenge_token_ok, nested_encrypt,
  pc, did_decrypt, outcome
>>

TypeOK ==
  /\ encrypt_key_set \in BOOLEAN
  /\ skip_sign \in BOOLEAN
  /\ body_kind \in {"Envelope", "Plain", "BlankEncrypt"}
  /\ req_type \in {"Challenge", "Event"}
  /\ sig \in {"Missing", "Bad", "Good"}
  /\ cipher_ok \in BOOLEAN
  /\ challenge_token_ok \in BOOLEAN
  /\ nested_encrypt \in BOOLEAN
  /\ pc \in {"Extract", "Decrypt", "Parse", "AuthGate", "Finish", "Done"}
  /\ did_decrypt \in BOOLEAN
  /\ outcome \in {"Pending", "Dispatched", "ChallengeEcho", "Rejected"}

\* Request parameters chosen once; pipeline then runs to Done.
Init ==
  /\ encrypt_key_set \in BOOLEAN
  /\ skip_sign \in BOOLEAN
  /\ body_kind \in {"Envelope", "Plain", "BlankEncrypt"}
  /\ req_type \in {"Challenge", "Event"}
  /\ sig \in {"Missing", "Bad", "Good"}
  /\ cipher_ok \in BOOLEAN
  /\ challenge_token_ok \in BOOLEAN
  /\ nested_encrypt \in BOOLEAN
  /\ pc = "Extract"
  /\ did_decrypt = FALSE
  /\ outcome = "Pending"

\* ---- extract_payload ----
\* empty encrypt_key → always plaintext (used_encrypt_envelope = false)
\* Envelope with blank encrypt → validation_error("encrypt", ...)
\* Envelope with ciphertext → used_encrypt_envelope = true → Decrypt
\* Non-envelope JSON → plaintext path → Parse
Extract ==
  /\ pc = "Extract"
  /\ outcome = "Pending"
  /\ IF ~encrypt_key_set THEN
       /\ pc' = "Parse"
       /\ UNCHANGED <<
            encrypt_key_set, skip_sign, body_kind, req_type, sig,
            cipher_ok, challenge_token_ok, nested_encrypt,
            did_decrypt, outcome
          >>
     ELSE IF body_kind = "BlankEncrypt" THEN
       /\ outcome' = "Rejected"
       /\ pc' = "Done"
       /\ UNCHANGED <<
            encrypt_key_set, skip_sign, body_kind, req_type, sig,
            cipher_ok, challenge_token_ok, nested_encrypt, did_decrypt
          >>
     ELSE IF body_kind = "Envelope" THEN
       /\ pc' = "Decrypt"
       /\ UNCHANGED <<
            encrypt_key_set, skip_sign, body_kind, req_type, sig,
            cipher_ok, challenge_token_ok, nested_encrypt,
            did_decrypt, outcome
          >>
     ELSE
       /\ pc' = "Parse"
       /\ UNCHANGED <<
            encrypt_key_set, skip_sign, body_kind, req_type, sig,
            cipher_ok, challenge_token_ok, nested_encrypt,
            did_decrypt, outcome
          >>

\* ---- decrypt_event ----
\* CURRENT CODE: decrypt runs BEFORE verify_sign when envelope + key set.
Decrypt ==
  /\ pc = "Decrypt"
  /\ outcome = "Pending"
  /\ did_decrypt' = TRUE
  /\ IF cipher_ok THEN
       /\ pc' = "Parse"
       /\ UNCHANGED outcome
     ELSE
       /\ outcome' = "Rejected"
       /\ pc' = "Done"
  /\ UNCHANGED <<
       encrypt_key_set, skip_sign, body_kind, req_type, sig,
       cipher_ok, challenge_token_ok, nested_encrypt
     >>

\* ---- FuzzyEvent parse + nested encrypt guard ----
\* if !fuzzy.encrypt.is_empty() && encrypt_key.is_empty() → Err
Parse ==
  /\ pc = "Parse"
  /\ outcome = "Pending"
  /\ IF ~encrypt_key_set /\ nested_encrypt THEN
       /\ outcome' = "Rejected"
       /\ pc' = "Done"
     ELSE
       /\ pc' = "AuthGate"
       /\ UNCHANGED outcome
  /\ UNCHANGED <<
       encrypt_key_set, skip_sign, body_kind, req_type, sig,
       cipher_ok, challenge_token_ok, nested_encrypt, did_decrypt
     >>

\* ---- verify_sign gate ----
\* Rust: if req_type != url_verification && !encrypt_key.is_empty() && !skip_sign_verify
NeedsSign ==
  /\ req_type # "Challenge"
  /\ encrypt_key_set
  /\ ~skip_sign

AuthGate ==
  /\ pc = "AuthGate"
  /\ outcome = "Pending"
  /\ IF NeedsSign THEN
       IF sig = "Good" THEN
         /\ pc' = "Finish"
         /\ UNCHANGED outcome
       ELSE
         \* Missing or Bad → validation_error("signature", ...)
         /\ outcome' = "Rejected"
         /\ pc' = "Done"
     ELSE
       /\ pc' = "Finish"
       /\ UNCHANGED outcome
  /\ UNCHANGED <<
       encrypt_key_set, skip_sign, body_kind, req_type, sig,
       cipher_ok, challenge_token_ok, nested_encrypt, did_decrypt
     >>

\* ---- auth_challenge OR dispatch ----
Finish ==
  /\ pc = "Finish"
  /\ outcome = "Pending"
  /\ IF req_type = "Challenge" THEN
       IF challenge_token_ok THEN
         /\ outcome' = "ChallengeEcho"
       ELSE
         /\ outcome' = "Rejected"
     ELSE
       /\ outcome' = "Dispatched"
  /\ pc' = "Done"
  /\ UNCHANGED <<
       encrypt_key_set, skip_sign, body_kind, req_type, sig,
       cipher_ok, challenge_token_ok, nested_encrypt, did_decrypt
     >>

Terminating ==
  /\ pc = "Done"
  /\ UNCHANGED vars

Next ==
  \/ Extract
  \/ Decrypt
  \/ Parse
  \/ AuthGate
  \/ Finish
  \/ Terminating

Spec == Init /\ [][Next]_vars

-----------------------------------------------------------------------------
\* Properties

\* Safety the current design must keep: forged / unsigned events must not
\* reach dispatcher.dispatch_with_response when encrypt_key is configured
\* and skip_sign_verify is false.
NoUnauthenticatedDispatch ==
  (outcome = "Dispatched") =>
    (~encrypt_key_set \/ skip_sign \/ sig = "Good")

\* url_verification must not call the event dispatcher.
ChallengeNeverDispatches ==
  (req_type = "Challenge") => (outcome # "Dispatched")

\* Intended hardening: do not run decrypt_event on traffic that still needs a
\* signature check and whose signature is missing/bad (CPU / oracle cost).
\*
\* CURRENT CODE VIOLATES THIS (known finding): handle decrypts the envelope
\* then parses JSON, and only afterwards calls verify_sign.
VerifyBeforeDecrypt ==
  ~(did_decrypt /\ NeedsSign /\ sig \in {"Missing", "Bad"})

====
