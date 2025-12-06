# VDownloader Deployment Guide

This guide covers various deployment strategies for the VDownloader application.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Local Development](#local-development)
- [Docker Deployment](#docker-deployment)
- [Production Deployment](#production-deployment)
- [Cloud Deployment](#cloud-deployment)
- [Environment Configuration](#environment-configuration)
- [Monitoring & Logging](#monitoring--logging)
- [Troubleshooting](#troubleshooting)

## Prerequisites

### Required
- Node.js 18 or higher
- npm or yarn package manager
- Git (for cloning the repository)

### Optional
- Docker and Docker Compose (for containerized deployment)
- FFmpeg (for advanced video processing features)
- Reverse proxy (nginx, Apache, Caddy) for SSL/TLS
- Process manager (PM2, systemd) for production

## Local Development

### Quick Start

1. Clone the repository:
```bash
git clone https://github.com/st93642/VDownloader.git
cd VDownloader
```

2. Install dependencies:
```bash
npm install
```

3. Configure environment:
```bash
cp .env.example .env
# Edit .env with your preferred settings
```

4. Start development server:
```bash
npm run dev
```

The application will be available at `http://localhost:4000`

### Development Features

- **Hot Reload**: Uses nodemon for automatic server restart on code changes
- **Debug Logging**: Enhanced logging in development mode
- **API Testing**: Built-in health check at `/api/health`

## Docker Deployment

### Using Docker Compose (Recommended)

1. **Clone and navigate to project**:
```bash
git clone https://github.com/st93642/VDownloader.git
cd VDownloader
```

2. **Configure environment** (optional):
```bash
cp .env.docker .env
# Edit .env if needed
```

3. **Build and start**:
```bash
docker-compose up -d
```

4. **Verify deployment**:
```bash
# Check container status
docker-compose ps

# View logs
docker-compose logs -f

# Test health endpoint
curl http://localhost:4000/api/health
```

5. **Manage deployment**:
```bash
# Stop application
docker-compose down

# Rebuild and restart
docker-compose down
docker-compose build --no-cache
docker-compose up -d

# View real-time logs
docker-compose logs -f vdownloader
```

### Using Docker Directly

1. **Build image**:
```bash
docker build -t vdownloader:latest .
```

2. **Run container**:
```bash
docker run -d \
  --name vdownloader-app \
  -p 4000:4000 \
  -e NODE_ENV=production \
  -e PORT=4000 \
  -v vdownloader-data:/app/.tmp/downloads \
  vdownloader:latest
```

3. **Manage container**:
```bash
# Stop container
docker stop vdownloader-app

# Start container
docker start vdownloader-app

# Remove container
docker rm vdownloader-app

# View logs
docker logs -f vdownloader-app
```

### Docker Configuration

#### Environment Variables

Set environment variables in `.env` file or via Docker command:

```env
NODE_ENV=production
PORT=4000
APP_BASE_URL=http://localhost:4000
DOWNLOAD_TEMP_DIR=.tmp/downloads
MAX_CONCURRENT_JOBS=2
```

#### Volumes

For persistent download storage:

```yaml
volumes:
  - ./downloads:/app/.tmp/downloads
  - ./logs:/app/logs
```

#### Health Checks

Docker Compose includes health checks by default. For manual setup:

```bash
docker run \
  --health-cmd="node -e \"require('http').get('http://localhost:4000/api/health', (r) => {process.exit(r.statusCode === 200 ? 0 : 1)})\"" \
  --health-interval=30s \
  --health-timeout=10s \
  --health-retries=3 \
  ...
```

## Production Deployment

### Using PM2 (Process Manager)

1. **Install PM2 globally**:
```bash
npm install -g pm2
```

2. **Create PM2 ecosystem file** (`ecosystem.config.js`):
```javascript
module.exports = {
  apps: [{
    name: 'vdownloader',
    script: './backend/server.js',
    instances: 'max',
    exec_mode: 'cluster',
    env: {
      NODE_ENV: 'production',
      PORT: 4000
    },
    error_file: './logs/err.log',
    out_file: './logs/out.log',
    log_date_format: 'YYYY-MM-DD HH:mm:ss Z'
  }]
};
```

3. **Start application**:
```bash
pm2 start ecosystem.config.js
pm2 save
pm2 startup
```

4. **Manage application**:
```bash
# Monitor
pm2 monit

# View logs
pm2 logs vdownloader

# Restart
pm2 restart vdownloader

# Stop
pm2 stop vdownloader

# Delete
pm2 delete vdownloader
```

### Using systemd

1. **Create systemd service file** (`/etc/systemd/system/vdownloader.service`):
```ini
[Unit]
Description=VDownloader Service
After=network.target

[Service]
Type=simple
User=www-data
WorkingDirectory=/var/www/vdownloader
ExecStart=/usr/bin/node backend/server.js
Restart=on-failure
RestartSec=10
StandardOutput=syslog
StandardError=syslog
SyslogIdentifier=vdownloader
Environment=NODE_ENV=production
Environment=PORT=4000

[Install]
WantedBy=multi-user.target
```

2. **Enable and start service**:
```bash
sudo systemctl daemon-reload
sudo systemctl enable vdownloader
sudo systemctl start vdownloader
```

3. **Manage service**:
```bash
# Check status
sudo systemctl status vdownloader

# View logs
sudo journalctl -u vdownloader -f

# Restart
sudo systemctl restart vdownloader

# Stop
sudo systemctl stop vdownloader
```

### Reverse Proxy with Nginx

1. **Install Nginx**:
```bash
sudo apt update
sudo apt install nginx
```

2. **Create Nginx configuration** (`/etc/nginx/sites-available/vdownloader`):
```nginx
server {
    listen 80;
    server_name yourdomain.com;

    location / {
        proxy_pass http://localhost:4000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # WebSocket support
    location /socket.io/ {
        proxy_pass http://localhost:4000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }
}
```

3. **Enable configuration and restart Nginx**:
```bash
sudo ln -s /etc/nginx/sites-available/vdownloader /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl restart nginx
```

4. **Add SSL with Let's Encrypt**:
```bash
sudo apt install certbot python3-certbot-nginx
sudo certbot --nginx -d yourdomain.com
```

## Cloud Deployment

### Heroku

1. **Create Heroku app**:
```bash
heroku create vdownloader-app
```

2. **Set environment variables**:
```bash
heroku config:set NODE_ENV=production
heroku config:set PORT=4000
```

3. **Create `Procfile`**:
```
web: npm start
```

4. **Deploy**:
```bash
git push heroku main
```

### DigitalOcean App Platform

1. **Connect your repository**
2. **Configure build settings**:
   - Build Command: `npm install`
   - Run Command: `npm start`
3. **Set environment variables** in the UI
4. **Deploy**

### AWS EC2

1. **Launch EC2 instance** (Ubuntu 20.04 or later)
2. **Install Node.js**:
```bash
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt install -y nodejs
```

3. **Clone and setup application**:
```bash
cd /var/www
sudo git clone https://github.com/st93642/VDownloader.git
cd VDownloader
sudo npm install --production
```

4. **Configure with PM2 or systemd** (see above)

5. **Setup security group** to allow HTTP (80), HTTPS (443), and application port

### Google Cloud Run

1. **Build container image**:
```bash
gcloud builds submit --tag gcr.io/PROJECT_ID/vdownloader
```

2. **Deploy to Cloud Run**:
```bash
gcloud run deploy vdownloader \
  --image gcr.io/PROJECT_ID/vdownloader \
  --platform managed \
  --region us-central1 \
  --allow-unauthenticated
```

## Environment Configuration

### Development
```env
NODE_ENV=development
PORT=4000
APP_BASE_URL=http://localhost:4000
DOWNLOAD_TEMP_DIR=.tmp/downloads
MAX_CONCURRENT_JOBS=2
```

### Production
```env
NODE_ENV=production
PORT=4000
APP_BASE_URL=https://yourdomain.com
DOWNLOAD_TEMP_DIR=/var/www/vdownloader/downloads
MAX_CONCURRENT_JOBS=5
```

### Docker
```env
NODE_ENV=production
PORT=4000
APP_BASE_URL=http://localhost:4000
DOWNLOAD_TEMP_DIR=.tmp/downloads
MAX_CONCURRENT_JOBS=2
```

## Monitoring & Logging

### Application Logs

**With PM2**:
```bash
pm2 logs vdownloader
pm2 logs vdownloader --lines 100
```

**With systemd**:
```bash
sudo journalctl -u vdownloader -f
sudo journalctl -u vdownloader --since "1 hour ago"
```

**With Docker**:
```bash
docker-compose logs -f
docker logs -f vdownloader-app
```

### Health Monitoring

Monitor application health:
```bash
# Check health endpoint
curl http://localhost:4000/api/health

# Monitor with watch
watch -n 5 'curl -s http://localhost:4000/api/health | jq'
```

### Performance Monitoring

Consider using:
- **New Relic** for APM
- **DataDog** for infrastructure monitoring
- **Sentry** for error tracking
- **Prometheus + Grafana** for metrics

## Troubleshooting

### Port Already in Use

The application automatically tries the next available port. To specify a different port:

```bash
PORT=4001 npm start
```

### WebSocket Connection Issues

1. Check firewall allows WebSocket connections
2. Ensure reverse proxy is configured for WebSocket (see Nginx example)
3. Verify CORS settings in production

### Memory Issues

Monitor memory usage:
```bash
# With PM2
pm2 monit

# With Docker
docker stats vdownloader-app
```

Adjust `MAX_CONCURRENT_JOBS` if memory is limited.

### Download Failures

1. Check internet connectivity from server
2. Verify video URL is accessible
3. Check logs for specific errors
4. Some videos may be region-restricted

### Docker Issues

**Container won't start**:
```bash
docker-compose logs vdownloader
docker-compose down
docker-compose build --no-cache
docker-compose up -d
```

**Volume permissions**:
```bash
docker exec -it vdownloader-app sh
ls -la /app/.tmp/downloads
```

## Security Best Practices

1. **Use HTTPS in production** with SSL/TLS certificates
2. **Set up firewall rules** to restrict access
3. **Keep dependencies updated**: `npm audit fix`
4. **Use environment variables** for sensitive configuration
5. **Implement rate limiting** (already included)
6. **Regular security audits**: `npm audit`
7. **Use non-root user** in Docker containers (already configured)

## Performance Optimization

1. **Use PM2 cluster mode** for multi-core utilization
2. **Enable Nginx caching** for static assets
3. **Use CDN** for frontend assets
4. **Implement Redis** for session/download state management
5. **Monitor and optimize** database queries (if added)
6. **Use compression middleware** for API responses

## Backup & Recovery

### Backup Strategy

1. **Application files**: Version controlled in Git
2. **Environment configuration**: Securely backup `.env` files
3. **Download history**: If using persistent storage, backup volumes
4. **Logs**: Rotate and archive logs regularly

### Disaster Recovery

1. **Document deployment process**
2. **Keep infrastructure as code** (Docker Compose, Terraform, etc.)
3. **Test recovery procedures** regularly
4. **Maintain backup of environment variables**

## Updates & Maintenance

### Updating Application

**Standard deployment**:
```bash
git pull origin main
npm install
pm2 restart vdownloader
```

**Docker deployment**:
```bash
git pull origin main
docker-compose down
docker-compose build --no-cache
docker-compose up -d
```

### Dependency Updates

```bash
# Check for updates
npm outdated

# Update dependencies
npm update

# Audit security
npm audit
npm audit fix
```

## Support

For issues, questions, or deployment assistance:
- GitHub Issues: https://github.com/st93642/VDownloader/issues
- Documentation: See README.md and API.md

---

**Last Updated**: December 2024
