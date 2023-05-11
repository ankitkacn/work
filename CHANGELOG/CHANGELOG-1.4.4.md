
<!--This means we can use HTML elements in Markdown, such as the comment 
element, and they won't be affected by a markdown parser. However, if you 
create an HTML element in your markdown file, you cannot use markdown syntax 
within that element's contents.-->

# ChangeLog 1.4.4 - Release Notes
-----
# Two Security Fixes
-----
## 1. First Security Fix
---
## [move-prover] re-enable the verification of diem-framework [#9049](https://github.com/diem/diem/pull/9049)<h1>
## **Overview** <h3>
---
### This commit re-enables the verification of diem-framework spec, which is disabled in [#9081](https://github.com/diem/diem/pull/9081)

-----------------------------

Major Modules Impacted

```
language/diem-framework/tests/DiemTimestampTests.move
language/move-prover/tests/sources/functional
language/move-prover/bytecode/src/mono_analysis.rs
language/move-prover/bytecode/src/spec_instrumentation.rs
language/testing-infra/e2e-tests/goldens/language_e2e_testsuite__tests__multi_agent__multi_agent_mint.exp
language/testing-infra/e2e-tests/goldens/language_e2e_testsuite__tests__verify_txn__publish_and_register_new_currency.exp

```
------------------------------------

Observed changes are as following :

-----------------------
- [diem-framework][df-cli] add/use a testing version of set_time_has_started
- This commit added a new public function set_time_has_started_for_testing because set_time_has_started is now public(friend) in order to delegate invariant checking to its caller (which is Genesis::initialize()).
Unfortunately, the customized genesis module in df-cli relies on that and same for the unit-test
DiemTimestampTests.move. Therefore, we created a public version as a temporary solution. We need
a better way to solve this dependency issue in a separate PR.
- fixup! [diem-framework][df-cli] add/use a testing version of set_time_has_started
- This commit updates the golden files in the language-e2e-tests, and the reason for this change is the
addition of DiemTimestamp::assert_operating(); in create_designated_dealer and create_child_vasp_account
changes the gas consumption.
------
# 2. Second Security Fix
----
## [reconfiguration] another attempt to fix the bug and better test coverage [#9119](https://github.com/diem/diem/pull/9119) <h1>
## **Overview** <h3>
---
### This PR fixes the problem as well as making sure test covers the edge case.

-----

Major Modules Impacted

```
consensus/consensus-types/src/block_data.rs
consensus/consensus-types/src/executed_block.rs
consensus/src/network.rs
consensus/src/round_manager.rs

```


------------------------------------

Observed changes are as following :

-----------------------
The fix is in executed_block.rs.

For test, specifically we forces a timeout (by sending proposals to half of the validators) on the direct child block of reconfiguration when failpoints is enabled, so it'll deterministically reproduce the error.

Also we enabled failpoints in forge local build, so all tests involving multi nodes and epoch change would cover the missing case. 

It covers CI/CD tests and cargo x test -p smoke-test -- genesis --nocapture.

-----------------------------



