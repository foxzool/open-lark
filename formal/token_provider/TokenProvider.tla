---- MODULE TokenProvider ----
\* Model of AuthTokenProvider::get_or_fetch in
\* crates/openlark-auth/src/token_provider.rs
\*
\* Each process is one concurrent caller of get_or_fetch for a single cache key.
\* Steps match the Rust control flow one-for-one:
\*   CheckCache  <-> get_cached (read lock, is_expired filter, drop lock)
\*   Fetch       <-> fetch().await  (HTTP; lock NOT held — the race window)
\*   WriteCache  <-> set_cached on Ok only
\*   Done        <-> return Ok(token) or Err
\*
\* CachedToken::new skew: expires_at = now + saturating_sub(expires_in, 60).
\* When expires_in < 60, EffectiveTTL = 0 so the entry is immediately expired.

EXTENDS Naturals, FiniteSets, TLC

CONSTANTS
  Procs,           \* concurrent get_or_fetch callers
  CacheKey,        \* one key (same role as Rust cache_key(...)); unused in
                   \* transitions — kept so the cfg documents the scope
  ExpiresIn,       \* fetch response expires_in_seconds (Feishu typically 7200)
  SkewSecs,        \* code constant 60 in CachedToken::new
  MaxTime          \* discrete clock bound for model checking

ASSUME
  /\ Procs # {}
  /\ ExpiresIn \in Nat
  /\ SkewSecs \in Nat
  /\ MaxTime \in Nat
  /\ MaxTime > 0

NoToken == "NoToken"
FetchFailToken == "FetchFail"

VARIABLES
  pc,              \* [Procs -> {"CheckCache","Fetch","WriteCache","Done"}]
  cache_token,     \* token in the HashMap entry, or NoToken if absent
  cache_expires,   \* expires_at for that entry (ignored when cache_token = NoToken)
  now,             \* wall-clock seconds (CachedToken::now_epoch_secs)
  pending_token,   \* token from a successful fetch, awaiting set_cached
  served,          \* result returned to each proc
  fetch_outcome    \* nondet Ok/Fail chosen when entering Fetch

vars == <<pc, cache_token, cache_expires, now, pending_token, served, fetch_outcome>>

\* saturating_sub(ExpiresIn, SkewSecs)
EffectiveTTL ==
  IF ExpiresIn >= SkewSecs THEN ExpiresIn - SkewSecs ELSE 0

TypeOK ==
  /\ pc \in [Procs -> {"CheckCache", "Fetch", "WriteCache", "Done"}]
  /\ cache_token \in {NoToken} \cup Procs
  /\ cache_expires \in Nat
  /\ now \in 0..MaxTime
  /\ pending_token \in [Procs -> {NoToken} \cup Procs]
  /\ served \in [Procs -> {NoToken, FetchFailToken} \cup Procs]
  /\ fetch_outcome \in [Procs -> {"Unset", "Ok", "Fail"}]

Init ==
  /\ pc = [p \in Procs |-> "CheckCache"]
  /\ cache_token = NoToken
  /\ cache_expires = 0
  /\ now = 0
  /\ pending_token = [p \in Procs |-> NoToken]
  /\ served = [p \in Procs |-> NoToken]
  /\ fetch_outcome = [p \in Procs |-> "Unset"]

\* ---- get_cached ----
\* Read the map under the read lock, filter with !is_expired(), drop the lock.
\* is_expired <=> now_epoch_secs() >= expires_at
CacheHit(p) ==
  /\ pc[p] = "CheckCache"
  /\ cache_token # NoToken
  /\ now < cache_expires
  /\ served' = [served EXCEPT ![p] = cache_token]
  /\ pc' = [pc EXCEPT ![p] = "Done"]
  /\ UNCHANGED <<cache_token, cache_expires, now, pending_token, fetch_outcome>>

CacheMiss(p) ==
  /\ pc[p] = "CheckCache"
  /\ \/ cache_token = NoToken
     \/ now >= cache_expires
  /\ pc' = [pc EXCEPT ![p] = "Fetch"]
  /\ UNCHANGED <<cache_token, cache_expires, now, pending_token, served, fetch_outcome>>

\* ---- fetch().await ----
\* Lock is not held. Outcome is nondeterministic (network / Feishu).
\* On Ok the pending token is the proc id (stands in for a unique token string)
\* so last-writer-wins among stampedes is observable.
BeginFetch(p) ==
  /\ pc[p] = "Fetch"
  /\ fetch_outcome[p] = "Unset"
  /\ \/ /\ fetch_outcome' = [fetch_outcome EXCEPT ![p] = "Ok"]
        /\ pending_token' = [pending_token EXCEPT ![p] = p]
        /\ UNCHANGED <<pc, cache_token, cache_expires, now, served>>
     \/ /\ fetch_outcome' = [fetch_outcome EXCEPT ![p] = "Fail"]
        /\ UNCHANGED <<pc, cache_token, cache_expires, now, pending_token, served>>

FetchSucceed(p) ==
  /\ pc[p] = "Fetch"
  /\ fetch_outcome[p] = "Ok"
  /\ pc' = [pc EXCEPT ![p] = "WriteCache"]
  /\ UNCHANGED <<cache_token, cache_expires, now, pending_token, served, fetch_outcome>>

\* Failed fetch does not call set_cached (the `?` on fetch().await).
FetchFail(p) ==
  /\ pc[p] = "Fetch"
  /\ fetch_outcome[p] = "Fail"
  /\ served' = [served EXCEPT ![p] = FetchFailToken]
  /\ pc' = [pc EXCEPT ![p] = "Done"]
  /\ UNCHANGED <<cache_token, cache_expires, now, pending_token, fetch_outcome>>

\* ---- set_cached ----
\* CachedToken::new(token, expires_in_seconds) then write-lock insert.
WriteCache(p) ==
  /\ pc[p] = "WriteCache"
  /\ cache_token' = pending_token[p]
  /\ cache_expires' = now + EffectiveTTL
  /\ served' = [served EXCEPT ![p] = pending_token[p]]
  /\ pc' = [pc EXCEPT ![p] = "Done"]
  /\ UNCHANGED <<now, pending_token, fetch_outcome>>

\* Clock can advance between get_cached and set_cached.
Tick ==
  /\ now < MaxTime
  /\ now' = now + 1
  /\ UNCHANGED <<pc, cache_token, cache_expires, pending_token, served, fetch_outcome>>

Terminating ==
  /\ \A p \in Procs : pc[p] = "Done"
  /\ UNCHANGED vars

Next ==
  \/ \E p \in Procs :
       \/ CacheHit(p)
       \/ CacheMiss(p)
       \/ BeginFetch(p)
       \/ FetchSucceed(p)
       \/ FetchFail(p)
       \/ WriteCache(p)
  \/ Tick
  \/ Terminating

Spec == Init /\ [][Next]_vars

-----------------------------------------------------------------------------
\* Properties

\* Safety that matches current code and must keep holding:
\* a failed fetch never inserts that call's token (Failed path skips set_cached).
FailedFetchDoesNotWrite ==
  \A p \in Procs :
    (pc[p] = "Done" /\ served[p] = FetchFailToken) =>
      cache_token # p

\* Primary intended concurrency promise for a correct token cache:
\* at most one in-flight fetch/write per cache key.
\*
\* CURRENT CODE VIOLATES THIS (known finding): get_or_fetch drops the read lock
\* before fetch(), so two cold callers both CacheMiss then both enter Fetch.
SingleFlight ==
  Cardinality({p \in Procs : pc[p] \in {"Fetch", "WriteCache"}}) <= 1

====
