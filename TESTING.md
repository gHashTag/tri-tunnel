# tri-tunnel — Manual Testing Plan

## Test Environment

- **OS**: macOS
- **Tailscale**: App Store version
- **Rust**: stable

## Test Cases

### TC-001: Start Funnel

**Steps**:
1. Ensure Funnel is not running (`tri-tunnel stop`)
2. Run: `tri-tunnel start`

**Expected**:
- Shows header
- Spinner appears
- Success message "✅ Funnel started successfully!"
- Status box shows ACTIVE
- Shows 3 URLs (main, health, api/status)

**Actual**: ✅ PASS

---

### TC-002: Status When Active

**Steps**:
1. Start Funnel (`tri-tunnel start`)
2. Run: `tri-tunnel status`

**Expected**:
- Shows header
- Status box shows Device name
- Funnel: ACTIVE ✅
- Shows URL

**Actual**: ✅ PASS

---

### TC-003: Status When Inactive

**Steps**:
1. Stop Funnel (`tri-tunnel stop`)
2. Run: `tri-tunnel status`

**Expected**:
- Shows header
- Status box shows Device name
- Funnel: INACTIVE ❌
- No URL shown

**Actual**: ✅ PASS

---

### TC-004: Stop Funnel

**Steps**:
1. Start Funnel
2. Run: `tri-tunnel stop`

**Expected**:
- Shows header
- Spinner appears
- Success message "✅ Funnel stopped"

**Actual**: ✅ PASS

---

### TC-005: Open Dashboard

**Steps**:
1. Start Funnel
2. Run: `tri-tunnel open`

**Expected**:
- Info message with URL
- Success "✅ Dashboard opened in browser"
- Browser opens with funnel URL

**Actual**: ✅ PASS

---

### TC-006: Start When Already Running

**Steps**:
1. Start Funnel
2. Run: `tri-tunnel start` again

**Expected**:
- Info message "Funnel is already running!"
- Shows status box
- Does not create duplicate funnel

**Actual**: ✅ PASS

---

### TC-007: Stop When Not Running

**Steps**:
1. Ensure Funnel is stopped
2. Run: `tri-tunnel stop`

**Expected**:
- May show error or handle gracefully
- Does not crash

**Actual**: ✅ PASS (shows success even if already stopped)

---

### TC-008: Open When Not Running

**Steps**:
1. Stop Funnel
2. Run: `tri-tunnel open`

**Expected**:
- Error message "Funnel is not running"
- Does not crash

**Actual**: ✅ PASS

---

## Integration Test with trios-server

### TC-101: Full Stack Test

**Prerequisites**:
- trios-server running on port 9005

**Steps**:
1. Start trios-server: `cargo run -p trios-server`
2. Start Funnel: `tri-tunnel start`
3. Test local: `curl http://localhost:9005/health`
4. Test via Funnel: `curl https://playras-macbook-pro-1.tail01804b.ts.net:443/health`

**Expected**:
- Both return "ok"
- Funnel proxies to localhost:9005

**Actual**: ✅ PASS

---

## Edge Cases

### EC-001: Tailscale Not Installed

**Steps**:
1. Uninstall Tailscale (or move CLI)
2. Run: `tri-tunnel start`

**Expected**:
- Error message about missing Tailscale CLI
- Link to App Store

**Actual**: ⏸️ NOT TESTED (would require uninstalling Tailscale)

---

### EC-002: Tailscale Not Logged In

**Steps**:
1. Log out of Tailscale
2. Run: `tri-tunnel start`

**Expected**:
- Error message about authentication

**Actual**: ⏸️ NOT TESTED

---

## Color Output Test

Run all commands and verify:
- ✅ Green for success
- ❌ Red for errors
- ℹ️ Blue for info
- 🌐 Yellow for URLs
- Cyan for headers and boxes

**Actual**: ✅ PASS

---

## Summary

| Test | Result |
|------|--------|
| TC-001: Start Funnel | ✅ PASS |
| TC-002: Status (Active) | ✅ PASS |
| TC-003: Status (Inactive) | ✅ PASS |
| TC-004: Stop Funnel | ✅ PASS |
| TC-005: Open Dashboard | ✅ PASS |
| TC-006: Start When Running | ✅ PASS |
| TC-007: Stop When Stopped | ✅ PASS |
| TC-008: Open When Stopped | ✅ PASS |
| TC-101: Full Stack | ✅ PASS |
| Color Output | ✅ PASS |

**Total**: 9/9 tests passed
