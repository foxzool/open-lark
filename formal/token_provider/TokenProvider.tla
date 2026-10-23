---- MODULE TokenProvider ----
\* Model of AuthTokenProvider::get_or_fetch in
\* crates/openlark-auth/src/token_provider.rs
\*
\* Each process is one concurrent caller for a single cache key.
\* Steps match the Rust control flow one-for-one:
\*   CheckCache  <-> get_cached (read lock, is_expired filter, drop lock)
\*   Fetch       <-> fetch().await  (HTTP; lock NOT held — the race window)
\*   WriteCache  <-> set_cached on Ok only
\*   Done        <-> return Ok(token) or Err
\*
\* #693 user-token flow (procs in UserProcs, mirroring
\* get_or_refresh_user_token): on miss, ReadRefresh picks the rotation
\* refresh token from the cache entry (CachedToken.refresh_token) or the
\* Config seed (RefreshSeed); none -> configuration error, no write.
\* A successful refresh fetch writes the access token AND the rotated
\* refresh token as one set_cached_with_refresh insert.
\*
\* CachedToken::new skew: expires_at = now + saturating_sub(expires_in, 60).
\* When expires_in < 60, EffectiveTTL = 0 so the entry is immediately expired.

EXTENDS Naturals, FiniteSets, TLC

CONSTANTS
  Procs,           \* concurrent callers
  UserProcs,       \* subset modeled as AccessTokenType::User callers (#693);
                   \* others mirror plain get_or_fetch (app/tenant/assertion)
  RefreshSeed,     \* Config.user_refresh_token seed value; NoToken = unset
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
ConfigErrToken == "ConfigErr"

VARIABLES
  pc,              \* [Procs -> {"CheckCache","ReadRefresh","Fetch","WriteCache","Done"}]
  cache_token,     \* token in the HashMap entry, or NoToken if absent
  cache_expires,   \* expires_at for that entry (ignored when cache_token = NoToken)
  cache_refresh,   \* CachedToken.refresh_token for a user entry; NoToken = none
  now,             \* wall-clock seconds (CachedToken::now_epoch_secs)
  pending_token,   \* token from a successful fetch, awaiting set_cached
  pending_refresh, \* refresh token chosen by ReadRefresh / rotated by fetch Ok
  served,          \* result returned to each proc
  fetch_outcome,   \* nondet Ok/Fail chosen when entering Fetch
  last_writer      \* proc of the most recent cache write; NoToken = never

vars == <<pc, cache_token, cache_expires, cache_refresh, now, pending_token,
          pending_refresh, served, fetch_outcome, last_writer>>

\* saturating_sub(ExpiresIn, SkewSecs)
EffectiveTTL ==
  IF ExpiresIn >= SkewSecs THEN ExpiresIn - SkewSecs ELSE 0

TypeOK ==
  /\ pc \in [Procs -> {"CheckCache", "ReadRefresh", "Fetch", "WriteCache", "Done"}]
  /\ cache_token \in {NoToken} \cup Procs
  /\ cache_expires \in Nat
  /\ cache_refresh \in {NoToken, RefreshSeed} \cup Procs
  /\ now \in 0..MaxTime
  /\ pending_token \in [Procs -> {NoToken} \cup Procs]
  /\ pending_refresh \in [Procs -> {NoToken, RefreshSeed} \cup Procs]
  /\ served \in [Procs -> {NoToken, FetchFailToken, ConfigErrToken} \cup Procs]
  /\ fetch_outcome \in [Procs -> {"Unset", "Ok", "Fail"}]
  /\ last_writer \in {NoToken} \cup Procs

Init ==
  /\ pc = [p \in Procs |-> "CheckCache"]
  /\ cache_token = NoToken
  /\ cache_expires = 0
  /\ cache_refresh = NoToken
  /\ now = 0
  /\ pending_token = [p \in Procs |-> NoToken]
  /\ pending_refresh = [p \in Procs |-> NoToken]
  /\ served = [p \in Procs |-> NoToken]
  /\ fetch_outcome = [p \in Procs |-> "Unset"]
  /\ last_writer = NoToken

\* ---- get_cached ----
\* Read the map under the read lock, filter with !is_expired(), drop the lock.
\* is_expired <=> now_epoch_secs() >= expires_at
CacheHit(p) ==
  /\ pc[p] = "CheckCache"
  /\ cache_token # NoToken
  /\ now < cache_expires
  /\ served' = [served EXCEPT ![p] = cache_token]
  /\ pc' = [pc EXCEPT ![p] = "Done"]
  /\ UNCHANGED <<cache_token, cache_expires, cache_refresh, now,
                  pending_token, pending_refresh, fetch_outcome, last_writer>>

CacheMiss(p) ==
  /\ pc[p] = "CheckCache"
  /\ \/ cache_token = NoToken
     \/ now >= cache_expires
  /\ pc' = [pc EXCEPT ![p] = IF p \in UserProcs THEN "ReadRefresh" ELSE "Fetch"]
  /\ UNCHANGED <<cache_token, cache_expires, cache_refresh, now,
                  pending_token, pending_refresh, served, fetch_outcome, last_writer>>

\* ---- cached_refresh_token / config seed ----
\* Read the rotation refresh token under the read lock (entry may hold one
\* even when its access token is expired); fall back to the Config seed.
\* Neither present -> configuration error, no write (#693 AC4).
ReadRefresh(p) ==
  /\ pc[p] = "ReadRefresh"
  /\ \/ /\ cache_refresh # NoToken
        /\ pending_refresh' = [pending_refresh EXCEPT ![p] = cache_refresh]
     \/ /\ cache_refresh = NoToken /\ RefreshSeed # NoToken
        /\ pending_refresh' = [pending_refresh EXCEPT ![p] = RefreshSeed]
  /\ pc' = [pc EXCEPT ![p] = "Fetch"]
  /\ UNCHANGED <<cache_token, cache_expires, cache_refresh, now,
                  pending_token, served, fetch_outcome, last_writer>>

RefreshMissing(p) ==
  /\ pc[p] = "ReadRefresh"
  /\ cache_refresh = NoToken
  /\ RefreshSeed = NoToken
  /\ served' = [served EXCEPT ![p] = ConfigErrToken]
  /\ pc' = [pc EXCEPT ![p] = "Done"]
  /\ UNCHANGED <<cache_token, cache_expires, cache_refresh, now,
                  pending_token, pending_refresh, fetch_outcome, last_writer>>

\* ---- fetch().await ----
\* Lock is not held. Outcome is nondeterministic (network / Feishu).
\* On Ok the pending token is the proc id (stands in for a unique token string)
\* so last-writer-wins among stampedes is observable.
BeginFetch(p) ==
  /\ pc[p] = "Fetch"
  /\ fetch_outcome[p] = "Unset"
  /\ \/ /\ fetch_outcome' = [fetch_outcome EXCEPT ![p] = "Ok"]
        /\ pending_token' = [pending_token EXCEPT ![p] = p]
        /\ pending_refresh' = [pending_refresh EXCEPT ![p] =
             IF p \in UserProcs THEN p ELSE pending_refresh[p]]
        /\ UNCHANGED <<pc, cache_token, cache_expires, cache_refresh, now, served, last_writer>>
     \/ /\ fetch_outcome' = [fetch_outcome EXCEPT ![p] = "Fail"]
        /\ UNCHANGED <<pc, cache_token, cache_expires, cache_refresh, now,
                        pending_token, pending_refresh, served, last_writer>>

FetchSucceed(p) ==
  /\ pc[p] = "Fetch"
  /\ fetch_outcome[p] = "Ok"
  /\ pc' = [pc EXCEPT ![p] = "WriteCache"]
  /\ UNCHANGED <<cache_token, cache_expires, cache_refresh, now,
                  pending_token, pending_refresh, served, fetch_outcome, last_writer>>

\* Failed fetch does not call set_cached (the `?` on fetch().await).
FetchFail(p) ==
  /\ pc[p] = "Fetch"
  /\ fetch_outcome[p] = "Fail"
  /\ served' = [served EXCEPT ![p] = FetchFailToken]
  /\ pc' = [pc EXCEPT ![p] = "Done"]
  /\ UNCHANGED <<cache_token, cache_expires, cache_refresh, now,
                  pending_token, pending_refresh, fetch_outcome, last_writer>>

\* ---- set_cached ----
\* CachedToken::new(token, expires_in_seconds) then write-lock insert.
WriteCache(p) ==
  /\ pc[p] = "WriteCache"
  /\ cache_token' = pending_token[p]
  /\ cache_expires' = now + EffectiveTTL
  /\ served' = [served EXCEPT ![p] = pending_token[p]]
  /\ pc' = [pc EXCEPT ![p] = "Done"]
  /\ last_writer' = p
  /\ \* user 路径：set_cached_with_refresh —— 访问令牌与轮转 refresh token 成对写入
     \/ /\ p \in UserProcs
        /\ cache_refresh' = pending_refresh[p]
     \/ /\ p \notin UserProcs
        /\ UNCHANGED cache_refresh
  /\ UNCHANGED <<now, pending_token, pending_refresh, fetch_outcome>>

\* 简化说明 1：真实 Rust 中 plain 与 user 是不同 cache key 的不同条目；
\* 本模型只跟踪单个条目，plain 写不触碰 cache_refresh 即可保持
\* UserEntryPairsRefresh 的语义。
\* 简化说明 2：BeginFetch 对 user 进程无条件轮转 pending_refresh = p；
\* Rust 侧在响应缺 refresh_token 时沿用本次使用的值（.or(Some(...))），
\* 该回退仅影响写回哪个既有值，不影响成对写回不变量。

\* Clock can advance between get_cached and set_cached.
Tick ==
  /\ now < MaxTime
  /\ now' = now + 1
  /\ UNCHANGED <<pc, cache_token, cache_expires, cache_refresh,
                  pending_token, pending_refresh, served, fetch_outcome, last_writer>>

Terminating ==
  /\ \A p \in Procs : pc[p] = "Done"
  /\ UNCHANGED vars

Next ==
  \/ \E p \in Procs :
       \/ CacheHit(p)
       \/ CacheMiss(p)
       \/ ReadRefresh(p)
       \/ RefreshMissing(p)
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

\* #693：user 路径的写回是成对原子语义——最后写入者是 user 进程时，
\* cache_refresh 必须等于该进程写下的轮转 refresh token（要么一起写，
\* 要么都不写；失败路径已由 FailedFetchDoesNotWrite 覆盖）。
UserEntryPairsRefresh ==
  (last_writer # NoToken /\ last_writer \in UserProcs) =>
    cache_refresh = pending_refresh[last_writer]

\* Primary intended concurrency promise for a correct token cache:
\* at most one in-flight fetch/write per cache key.
\*
\* CURRENT CODE VIOLATES THIS (known finding): get_or_fetch drops the read lock
\* before fetch(), so two cold callers both CacheMiss then both enter Fetch.
SingleFlight ==
  Cardinality({p \in Procs : pc[p] \in {"Fetch", "WriteCache"}}) <= 1

====
