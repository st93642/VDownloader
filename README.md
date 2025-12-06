# VDownloader

VDownloader is a Node.js/Express based service that will power a cross-platform video downloading web application. This initial scaffolding sets up the core architecture, dependencies, and configuration so future feature work can focus on building user-facing functionality.

## Project Structure

```
├── .env.example
├── .gitignore
├── backend
│   ├── app.js
│   ├── controllers
│   │   ├── healthController.js
│   │   └── platformController.js
│   ├── routes
│   │   ├── healthRoutes.js
│   │   ├── index.js
│   │   └── platformRoutes.js
│   ├── server.js
│   └── services
│       └── platformService.js
├── config
│   ├── index.js
│   └── platforms.js
├── frontend
│   └── README.md
├── LICENSE
├── README.md
├── package.json
└── package-lock.json
```

## Key Dependencies

- **Express** – HTTP server framework used to build the REST API.
- **cors** – Enables controlled Cross-Origin Resource Sharing for the frontend.
- **dotenv** – Loads environment variables from `.env` files for local development.
- **ytdl-core** – Provides YouTube download and metadata capabilities.
- **fluent-ffmpeg** – Adapter around FFmpeg for advanced media processing.
- **nodemon** – Development dependency that reloads the server as files change.

## Getting Started

1. **Install dependencies**
   ```bash
   npm install
   ```
2. **Run the development server**
   ```bash
   npm run dev
   ```
3. **Run the production server**
   ```bash
   npm start
   ```

The API will default to `http://localhost:4000` unless the `PORT` environment variable is set.

## Configuration

Environment variables are loaded with [dotenv](https://github.com/motdotla/dotenv). Duplicate `.env.example` to `.env` and adjust as needed. You may supply any of the following:

| Variable | Default | Description |
| --- | --- | --- |
| `PORT` | `4000` | Port the Express server listens on |
| `NODE_ENV` | `development` | Runtime environment label |
| `APP_BASE_URL` | `http://localhost:4000` | External URL for generated links |
| `DOWNLOAD_TEMP_DIR` | `.tmp/downloads` | Temporary directory for downloaded files |
| `MAX_CONCURRENT_JOBS` | `2` | Limits concurrent download jobs |

Supported platforms and their metadata are centralized in `config/platforms.js`. Toggle the `enabled` flag or extend the list to allow additional sources.

## API Routes

| Method | Route | Description |
| --- | --- | --- |
| `GET` | `/api/health` | Returns service health metadata and enabled platforms |
| `GET` | `/api/platforms` | Lists all configured platforms |
| `GET` | `/api/platforms/supported` | Lists only currently enabled platforms |

## Frontend Placeholder

The `frontend` directory is reserved for a future client application. Until then it contains documentation only so frontend development can begin independently of the backend skeleton.
