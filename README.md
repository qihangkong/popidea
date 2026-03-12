# PopIdea AI Video Studio

An AI-powered video creation tool that transforms novels into animated shorts. Automatically generates storyboards, characters, scenes, and assembles them into complete videos with AI voiceover.

## Features

- 🎬 **Novel Analysis** - Parse novels and extract characters, scenes, and plot
- 🎨 **Character & Scene Generation** - Consistent AI-generated images for characters and locations
- 📽️ **Storyboard to Video** - Auto-generate shots and compose them into videos
- 🎙️ **AI Voiceover** - Multi-character voice synthesis
- 💾 **Local-First Architecture** - All data stored locally on your machine
- 🌐 **Multi-Access** - Use locally as desktop app, or access via web from other devices

## Architecture

PopIdea combines client and server in a single executable:

```
┌─────────────────────────────────────────────┐
│          PopIdea Application                  │
├─────────────────────────────────────────────┤
│  Frontend: React + TypeScript               │
│  Backend:  Rust (Tauri framework)            │
├─────────────────────────────────────────────┤
│  Data Layer:                                 │
│  ├─ SQLite Database                         │
│  └─ LiteS3 Service (local file storage)     │
├─────────────────────────────────────────────┤
│  Network:                                    │
│  ├─ Local IPC (desktop app)                 │
│  └─ HTTP Server (web access)                 │
└─────────────────────────────────────────────┘
```

## Tech Stack

### Frontend
- **React** + **TypeScript**
- **Tailwind CSS**
- **React Query** for data fetching

### Backend
- **Rust** - Core application logic
- **Tauri** - Desktop application framework
- **SQLite** - Local database (via sqlx crate)
- **LiteS3** - Local S3-compatible storage service

### External APIs
- AI providers for text, image, and video generation
- Configurable via settings panel

## Installation

### Prerequisites
- Rust toolchain (1.70+)
- Node.js (18+)
- Tauri CLI

### Build from Source

```bash
# Clone the repository
git clone https://github.com/your-username/popidea.git
cd popidea

# Install dependencies
npm install

# Build the application
npm run tauri build

# Or run in development mode
npm run tauri dev
```

### Release Binaries

Download the latest release from [Releases](https://github.com/your-username/popidea/releases) page.

## Usage

### Desktop Mode
1. Launch the PopIdea executable
2. Create a new project or open an existing one
3. Import your novel text
4. Configure AI provider settings
5. Generate storyboards and videos

### Web Access Mode
1. Start the PopIdea application
2. Enable web server in settings
3. Access via browser at `http://localhost:PORT`
4. Share the URL with other devices on the same network

## Development

### Project Structure

```
popidea/
├── src-tauri/           # Rust backend
│   ├── src/
│   │   ├── main.rs      # Application entry point
│   │   ├── api/         # HTTP API endpoints
│   │   ├── db/          # Database layer (SQLite)
│   │   ├── storage/     # LiteS3 storage service
│   │   ├── tasks/       # Async task management
│   │   └── ai/          # AI provider integrations
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                 # React frontend
│   ├── components/       # React components
│   ├── pages/           # Application pages
│   ├── api/             # API client
│   └── hooks/           # React hooks
├── package.json
└── README.md
```

### Running in Development

```bash
# Install dependencies
npm install

# Start development server (Rust + React)
npm run tauri dev

# Run tests
npm test

# Run Rust tests
cd src-tauri && cargo test
```

### Building

```bash
# Build for development
npm run tauri build --debug

# Build for release
npm run tauri build

# The output will be in src-tauri/target/release/bundle/
```

## Configuration

### Database
- Location: `<app-data>/popidea/database.sqlite`
- Automatic migrations on startup
- Backup before schema changes

### Storage
- Location: `<app-data>/popidea/storage/`
- S3-compatible API
- Automatic cleanup of unused assets

### Network
- Default HTTP port: 3000
- Configurable in settings
- Supports CORS for web access

## API Endpoints

### Projects
- `GET /api/projects` - List all projects
- `POST /api/projects` - Create a new project
- `GET /api/projects/:id` - Get project details
- `PUT /api/projects/:id` - Update project
- `DELETE /api/projects/:id` - Delete a project

### Tasks
- `GET /api/tasks` - List tasks
- `POST /api/tasks` - Create a task
- `GET /api/tasks/:id` - Get task status
- `DELETE /api/tasks/:id` - Cancel a task

### Storage
- `GET /api/storage/*` - Retrieve stored files
- `PUT /api/storage/*` - Upload files
- `DELETE /api/storage/*` - Delete files

## Roadmap

- [ ] Novel text import and parsing
- [ ] Character profile generation
- [ ] Scene visualization
- [ ] Storyboard creation
- [ ] AI image generation
- [ ] AI video generation
- [ ] Voice synthesis
- [ ] Video export
- [ ] Project sharing
- [ ] Multi-user collaboration

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

MIT License - see [LICENSE](LICENSE) file for details

## Support

For issues, questions, or suggestions, please open an issue on [GitHub Issues](https://github.com/your-username/popidea/issues).

## Acknowledgments

Built with ❤️ using [Tauri](https://tauri.app/), [React](https://reactjs.org/), and [Rust](https://www.rust-lang.org/).
