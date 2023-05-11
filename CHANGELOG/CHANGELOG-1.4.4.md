# ChangeLog 1.4.4 - Release Notes
-----
# Two Security Fixes
-----
## 1. First Security Fix - BCS Vulnerability 
---
### Overview
---
Introducing Serialization and deserialization of nested enums and creating test case for the same [#2](https://github.com/zefchain/bcs/pull/2)


----

Major modules impacted :

```
* 1 Client
* 2.Config
* 3.Consensus
* 4.common
* 5.Crypto
* 6.Execution
* 7.json-rpc
* 8.language
* 9.mempool
* 10.network
* 11.sdk
* 12.secure
* 13.state-sync
* 14.storage
* 15.testsuite
* 16.types

```

---

### Observed Changes

---

1. For doing bcs serialization and deserialization anywhere in the diem code, external package is used and these external packages are nested down in Cargo.toml file .
   Address of Cargo.toml file where bcs = "0.1.2" is used:
2. First step is to apply the fixes in diem bcs version 0.1.2 regarding the matching of serialization and de-serialization of Nested enums 
and updating it to version 0.1.4.
           

3. Then, building diem bcs version 0.14 and performing unit testing.

4. After successful build and testing, referring diem bcs version 0.1.4 to diem code base.

5. To refer, first making changes in bcs of cargo file of a single module (Applying path od diem bcs- ) then,
        
     1. cargo build
     2. cargo test


6. After succesfull build of a single module applying the changes in all the modules by giving address of diem bcs in cargo.toml file.

7. Then, 
* 1. cargo build diem [ cargo build].
* 2. Smoke test [cargo x test --package smoke-test -- --test-threads 1]
* 3. integration test [cargo xtest -p jsonrpc-integration-tests]


8. Thus, all test cases are working fine and diem code is build. Fixed PR [#5](https://github.com/diem/bcs/pull/5)

Hash - 4b6129db12b0e2ad72e02c58a1dfee3294e4bc3b

-----
## 2. Second Security Fix 
---

### Overview 

---

This Security fix contains bug fixes, version updates, compatibility fixes, unit test cases and module additions.

---

Major Modules Impacted 

```
language/move-stdlib/src/natives/vector.rs
language/move-vm/transactional-tests/tests/builtins
language/move-vm/runtime/src/interpreter.rs
language/move-prover/bytecode/src/lib.rs
language/move-prover/doc/report/injection.tex
consensus/consensus-types/src/block_data.rs
consensus/consensus-types/src/executed_block.rs
consensus/src/network.rs
consensus/src/round_manager.rs

```
---
### Notable Changes
---
[Consensus]
1. Remove Move IR type inference [#9040](https://github.com/diem/diem/pull/9040) [#10707](https://github.com/diem/diem/pull/10717)
2. connect everything and add tests [#9111](https://github.com/diem/diem/pull/9111) [#10695](https://github.com/diem/diem/pull/10695)

[Diem Framework]
1. Port ValidatorConfig and ValidatorOperatorConfig to unit tests [9131](https://github.com/diem/diem/pull/9131) [#10717](https://github.com/diem/diem/pull/10717)

[Language]
1. make inconsistency check instrumentation a separate pass [#10475](https://github.com/diem/diem/pull/10475)
2. refactor the verification analysis [#9034](https://github.com/diem/diem/pull/10667) [#10667](https://github.com/diem/diem/pull/10667)
3. [move-vm] support bytecode for vector ops in the Move VM [#8821](https://github.com/diem/diem/pull/10669/files) [#10669](https://github.com/diem/diem/pull/10669)
4. Remove unused diagnostics [#9023](https://github.com/diem/diem/pull/10663/files) [#10663](https://github.com/diem/diem/pull/10663)
5. Move : [transactional-tests] More migrated tests [#10675](https://github.com/diem/diem/pull/10675)

[State-Sync]
1. Add a new EventSubscriptionService and tests[#9117](https://github.com/diem/diem/pull/9117) [#10717](https://github.com/diem/diem/pull/10717)
2. [State Sync] Add MempoolNotifier interface and implementation [#10683](https://github.com/diem/diem/pull/10683)

[Testsuite]
1. smoke-test: convert a number of smoke-tests to use LocalSwarm [#9027](https://github.com/diem/diem/pull/9027) [#10680](https://github.com/diem/diem/pull/10680)
2. Add forge LBT in ci-test [#9039](https://github.com/diem/diem/pull/9039) [#10701](https://github.com/diem/diem/pull/10701)