# Test Results

This directory contains automated test results from the Token ACL testing suite.

## Latest Results

See [REAL_TEST_RESULTS.md](REAL_TEST_RESULTS.md) for the most recent test run.

## How to Generate

Test results are automatically generated when you run:

```bash
./run_tests.sh
```

The tests will:
1. Run all validation tests
2. Generate `REAL_TEST_RESULTS.md` with detailed results
3. Save console output to `test_output.txt`

## Files

- **REAL_TEST_RESULTS.md** - Formatted markdown report with all test details
- **test_output.txt** - Raw console output from test run

## Current Status

Check [REAL_TEST_RESULTS.md](REAL_TEST_RESULTS.md) to see:
- Total tests run
- Pass/fail status
- Number of assertions
- Detailed results per test
- Timestamp of last run

---

**To view online**: This file will display on GitHub at:
https://github.com/BlocksOnAChain/token-acl-testing-suite/tree/main/test-results

