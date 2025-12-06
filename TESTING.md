# Testing Guide

This guide covers testing strategies and workflows for VDownloader.

## Quick Testing

### Automated Integration Test

Run the automated test suite:

```bash
# Start the server
npm start

# In another terminal, run tests
./test-integration.sh
```

This tests:
- ✓ Server health
- ✓ Platform endpoints
- ✓ URL validation
- ✓ Download workflow
- ✓ Frontend accessibility

## Manual Testing

### Test 1: Basic Download Flow

1. **Start the application**:
```bash
npm start
```

2. **Open browser**: Navigate to `http://localhost:4000`

3. **Test YouTube download**:
   - Platform: YouTube
   - URL: `https://www.youtube.com/watch?v=dQw4w9WgXcQ`
   - Format: Video
   - Quality: 720p
   - Click "Download"

4. **Verify**:
   - ✓ Toast notification appears
   - ✓ Progress card displays
   - ✓ Real-time progress updates
   - ✓ Download completes (~3-7 seconds)
   - ✓ Appears in history

### Test 2: Multiple Downloads

1. **Initiate first download** (YouTube video)
2. **Immediately initiate second download** (Different URL)
3. **Verify**:
   - ✓ Both progress cards display
   - ✓ Progress updates independently
   - ✓ Both complete successfully
   - ✓ Both appear in history

### Test 3: WebSocket Connection

1. **Open browser developer tools** (F12)
2. **Navigate to Console tab**
3. **Start a download**
4. **Verify console logs**:
   - ✓ "WebSocket connected"
   - ✓ No WebSocket errors
   - ✓ Progress events logged

### Test 4: Polling Fallback

1. **Block WebSocket connection** (in browser dev tools):
   - Network tab → Block request pattern: `/socket.io/`
2. **Start a download**
3. **Verify**:
   - ✓ Download still works
   - ✓ Progress updates (via polling)
   - ✓ Console shows "Socket.IO not loaded" warning

### Test 5: Error Handling

1. **Test invalid URL**:
   - Enter: `https://invalid-url.com/video`
   - ✓ Validation error appears

2. **Test unsupported platform**:
   - Enter URL from unsupported site
   - ✓ Error message displays

3. **Test missing fields**:
   - Leave quality unselected
   - ✓ Form validation prevents submission

### Test 6: Download History

1. **Complete 3 downloads**
2. **Verify**:
   - ✓ All 3 appear in history
   - ✓ Metadata displays correctly
   - ✓ Timestamps are accurate

3. **Click "Download Again"**:
   - ✓ Form populates with previous settings
   - ✓ Can re-download successfully

4. **Refresh page**:
   - ✓ History persists (localStorage)

## API Testing

### Using cURL

**Test health endpoint**:
```bash
curl http://localhost:4000/api/health
```

**Test validation**:
```bash
curl -X POST http://localhost:4000/api/validate \
  -H "Content-Type: application/json" \
  -d '{"url":"https://www.youtube.com/watch?v=dQw4w9WgXcQ"}'
```

**Test download**:
```bash
curl -X POST http://localhost:4000/api/download \
  -H "Content-Type: application/json" \
  -d '{"url":"https://www.youtube.com/watch?v=dQw4w9WgXcQ","format":"video","quality":"720p"}'
```

**Test status** (replace ID):
```bash
curl http://localhost:4000/api/status/YOUR_DOWNLOAD_ID
```

### Using Postman

1. **Import endpoints**:
   - See API.md for all endpoints
   - Create collection with test requests

2. **Test sequence**:
   - GET /api/health
   - GET /api/platforms
   - POST /api/validate (save download ID)
   - POST /api/download (save download ID)
   - GET /api/status/:id (check progress)

## WebSocket Testing

### Using Socket.IO Client (Browser Console)

```javascript
// Connect
const socket = io('http://localhost:4000');

socket.on('connect', () => {
  console.log('Connected:', socket.id);
});

// Subscribe to a download
socket.emit('subscribe', 'YOUR_DOWNLOAD_ID');

// Listen for events
socket.on('download:progress', (data) => {
  console.log('Progress:', data);
});

socket.on('download:complete', (data) => {
  console.log('Complete:', data);
});

socket.on('download:error', (data) => {
  console.log('Error:', data);
});
```

## Docker Testing

### Test Docker Build

```bash
# Build image
docker build -t vdownloader-test .

# Should complete without errors
```

### Test Docker Compose

```bash
# Start services
docker-compose up -d

# Check status
docker-compose ps

# View logs
docker-compose logs -f

# Test health
curl http://localhost:4000/api/health

# Cleanup
docker-compose down
```

### Test Docker Volume Persistence

```bash
# Start with compose
docker-compose up -d

# Create some downloads
# ...

# Stop and remove
docker-compose down

# Start again
docker-compose up -d

# Verify data persists
```

## Performance Testing

### Load Testing with Apache Bench

```bash
# Test health endpoint
ab -n 1000 -c 10 http://localhost:4000/api/health

# Test platform listing
ab -n 1000 -c 10 http://localhost:4000/api/platforms
```

### Concurrent Downloads

```bash
# Start multiple downloads simultaneously
for i in {1..5}; do
  curl -X POST http://localhost:4000/api/download \
    -H "Content-Type: application/json" \
    -d '{"url":"https://www.youtube.com/watch?v=dQw4w9WgXcQ","format":"video","quality":"720p"}' &
done

# Wait and verify all complete
wait
```

## Rate Limiting Testing

### Test Download Rate Limit (10/hour)

```bash
# Send 15 requests rapidly
for i in {1..15}; do
  echo "Request $i:"
  curl -X POST http://localhost:4000/api/download \
    -H "Content-Type: application/json" \
    -d '{"url":"https://www.youtube.com/watch?v=dQw4w9WgXcQ","format":"video","quality":"720p"}'
  echo ""
done

# After 10, should get 429 rate limit error
```

### Test Validation Rate Limit (30/minute)

```bash
# Send 35 requests
for i in {1..35}; do
  curl -X POST http://localhost:4000/api/validate \
    -H "Content-Type: application/json" \
    -d '{"url":"https://www.youtube.com/watch?v=dQw4w9WgXcQ"}' &
done

# Should see rate limit errors
```

## Browser Compatibility Testing

Test in multiple browsers:
- ✓ Chrome/Chromium (latest)
- ✓ Firefox (latest)
- ✓ Safari (latest)
- ✓ Edge (latest)
- ✓ Mobile browsers (iOS Safari, Chrome Android)

### Verify:
- WebSocket connections work
- UI displays correctly
- Downloads function properly
- Responsive design works
- History persists

## Mobile Testing

### Responsive Design
1. Open in mobile device or browser dev tools
2. Test portrait and landscape
3. Verify:
   - ✓ Layout adjusts properly
   - ✓ Touch targets are adequate
   - ✓ Forms are usable
   - ✓ Progress displays correctly

### Mobile Networks
1. Test on slower connections (3G simulation)
2. Verify:
   - ✓ Page loads
   - ✓ WebSocket connects or falls back
   - ✓ Downloads work
   - ✓ UI remains responsive

## Security Testing

### Input Validation

```bash
# Test XSS attempt
curl -X POST http://localhost:4000/api/validate \
  -H "Content-Type: application/json" \
  -d '{"url":"<script>alert(1)</script>"}'

# Should be rejected or sanitized
```

### CORS Testing

```javascript
// From different origin (browser console)
fetch('http://localhost:4000/api/health')
  .then(r => r.json())
  .then(console.log)

// Should work (CORS enabled)
```

## Troubleshooting Tests

### Server Won't Start

```bash
# Check if port is in use
lsof -i :4000

# Server will auto-increment port
# Check logs for actual port used
```

### WebSocket Connection Fails

```bash
# Check browser console for errors
# Verify Socket.IO client loads
# Test with polling-only mode
```

### Downloads Hang

```bash
# Check server logs
docker-compose logs -f

# Verify internet connection
# Check platform availability
```

## Continuous Integration

### Example GitHub Actions Workflow

```yaml
name: Test
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions/setup-node@v2
        with:
          node-version: '18'
      - run: npm install
      - run: npm start &
      - run: sleep 5
      - run: ./test-integration.sh
```

## Test Coverage

Current automated test coverage:
- ✓ API endpoints (health, platforms, validate, download, status)
- ✓ Frontend files (HTML, CSS, JS)
- ✓ Server startup
- ✓ Basic download flow

Areas for future test coverage:
- WebSocket connection tests
- Rate limiting verification
- Error handling scenarios
- Platform-specific adapters
- UI interaction tests (E2E)

## Reporting Issues

When reporting bugs, include:
1. Steps to reproduce
2. Expected vs actual behavior
3. Browser/environment details
4. Console errors (if any)
5. Server logs (if applicable)
6. Screenshots (if UI issue)

## Next Steps

For production deployment:
1. Set up monitoring (New Relic, DataDog)
2. Implement error tracking (Sentry)
3. Add analytics
4. Set up automated testing in CI/CD
5. Performance monitoring
6. Regular security audits

---

**Happy Testing! 🧪**
