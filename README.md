# IC-Vibe_Coding

🚀 **Internet Computer Protocol (ICP) project with AI-powered natural language database queries**

Transform plain English questions into structured database queries using advanced AI models running on the Internet Computer blockchain.

## ✨ Features

- 🤖 **Natural Language Processing** - Ask questions in plain English like "show me completed todos"
- 🔍 **Real-time Query Parsing** - Watch your natural language get converted to structured queries
- 🌐 **Internet Computer Protocol** - Fully decentralized backend running on ICP canisters
- 🔒 **Secure Architecture** - Database credentials never leave the IC canister
- ⚡ **Real-time Results** - Instant query execution and formatted results
- 🎨 **Modern UI** - Clean, responsive interface built with React and Tailwind CSS
- 🧠 **Distributed AI** - LLM processing handled by dedicated IC canister

## 🛠️ Tech Stack

### Backend

- **Rust** - IC canister development
- **Internet Computer Protocol (ICP)** - Decentralized hosting
- **PocketIC + Vitest** - Testing framework

### Frontend

- **Vite** - Build tool and development server
- **React + TypeScript** - Component framework
- **Tailwind CSS v4** - Styling with utility classes

### Database & AI

- **Supabase** - PostgreSQL database with REST API
- **LLM Canister** - Dedicated AI processing canister for natural language understanding

## 🏗️ Architecture

The project consists of three main canisters:

### 1. **Backend Canister** (`backend`)

- Main application logic
- Secure Supabase credential management
- Database query execution
- Natural language query coordination

### 2. **LLM Canister** (`llm`)

- AI model processing
- Natural language to SQL conversion
- Query parsing and validation
- Text analysis and understanding

### 3. **Frontend Canister** (`frontend`)

- React application hosting
- User interface delivery
- Static asset management

## 🚀 Quick Start

### Prerequisites

- [DFX](https://internetcomputer.org/docs/current/developer-docs/setup/install/) (Internet Computer SDK)
- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://rustup.rs/)

### Installation

1. **Clone the repository**

```bash
git clone https://github.com/YOUR_USERNAME/IC-Vibe_Coding.git
cd IC-Vibe_Coding
```

2. **Install dependencies**

```bash
npm install
```

3. **Start the local IC replica**

```bash
dfx start --background --clean
```

4. **Deploy all canisters**

```bash
# Deploy LLM canister first (required for natural language processing)
dfx deploy llm

# Deploy backend canister (depends on LLM canister)
dfx deploy backend

# Deploy frontend canister
dfx deploy frontend

# Or deploy all at once
dfx deploy
```

5. **Start the development server**

```bash
npm start
```

6. **Open the application**
   - Visit `http://localhost:5173`
   - The app will automatically use mock data for immediate testing

## 🎯 Usage

### Natural Language Queries

Navigate to the **"Natural Query"** tab and try these example queries:

```
"get all todos"
"show completed todos"
"find incomplete tasks"
"list all users"
"show me todos that are done"
"find todos with id 1"
```

### How It Works

1. **User Input** → Frontend captures natural language query
2. **Backend Canister** → Receives query and forwards to LLM canister
3. **LLM Canister** → Processes natural language and returns structured query
4. **Backend Canister** → Executes database query using parsed results
5. **Frontend** → Displays formatted results to user

### Demo Features

- **Counter Demo** - Basic canister interaction with state management
- **Greeting Demo** - Simple text processing and response
- **LLM Chat** - Direct conversation with the LLM canister
- **Natural Query** - Database querying with natural language (uses both backend and LLM canisters)

## 🏗️ Project Structure

```
IC-Vibe_Coding/
├── src/
│   ├── backend/                 # Main Rust IC canister
│   │   ├── src/lib.rs          # Backend logic, database integration
│   │   └── Cargo.toml          # Backend dependencies
│   ├── llm/                    # LLM processing canister
│   │   ├── src/lib.rs          # AI model integration
│   │   └── Cargo.toml          # LLM dependencies
│   └── frontend/               # React TypeScript frontend
│       ├── src/
│       │   ├── components/     # Reusable UI components
│       │   ├── views/          # Page-level components
│       │   ├── services/       # IC canister interaction
│       │   └── App.tsx         # Main application
│       └── package.json        # Frontend dependencies
├── dfx.json                    # IC project configuration (defines all canisters)
└── package.json               # Workspace configuration
```

## 🔧 Development

### Backend Development

```bash
# Check Rust code for errors
cargo check

# Generate Candid interfaces after changes
npm run generate-candid

# Deploy specific canisters
dfx deploy backend
dfx deploy llm
```

### Frontend Development

```bash
# Check TypeScript for errors
npx tsc -p src/frontend/tsconfig.json

# Format code (TypeScript + Rust)
npm run format

# Deploy frontend canister
dfx deploy frontend
```

### Canister Interaction

The canisters communicate as follows:

```rust
// Backend canister calls LLM canister
use ic_cdk::api::call::call;

#[ic_cdk::update]
async fn process_natural_language(query: String) -> String {
    let llm_canister_id = /* LLM canister ID */;
    let result: (String,) = call(llm_canister_id, "process_query", (query,))
        .await
        .expect("Failed to call LLM canister");
    result.0
}
```

### Testing

```bash
# Run backend tests
cargo test

# Test specific canister
cd src/backend && cargo test
cd src/llm && cargo test

# Frontend tests (if configured)
npm test
```

## 🔒 Security & Configuration

### Canister Security

- **LLM Canister** - Processes only text input, no sensitive data access
- **Backend Canister** - Secure credential storage, controlled database access
- **Frontend Canister** - Public hosting, no sensitive operations

### Database Setup (Optional)

The project works with mock data by default. For real database integration:

1. Create a [Supabase](https://supabase.com) account
2. Create a new project
3. Copy the template:

```bash
cp .env.example .env
```

4. Add your credentials to `.env`:

```bash
VITE_SUPABASE_URL=your_supabase_url_here
VITE_SUPABASE_ANON_KEY=your_supabase_anon_key_here
```

### LLM Configuration

The LLM canister can be configured for different AI models or providers. Check the canister documentation for specific setup requirements.

## 🚀 Deployment

### Local Development

```bash
dfx start --background --clean

# Deploy all canisters
dfx deploy

# Or deploy individually
dfx deploy llm      # Deploy LLM canister first
dfx deploy backend  # Deploy backend (depends on LLM)
dfx deploy frontend # Deploy frontend
```

### IC Mainnet

```bash
# Deploy to IC mainnet
dfx deploy --network ic

# Check canister status
dfx canister status --network ic backend
dfx canister status --network ic llm
dfx canister status --network ic frontend
```

### Canister URLs

After deployment, your canisters will be available at:

- **Frontend**: `https://{frontend-canister-id}.ic0.app`
- **Backend**: Accessible via Candid interface
- **LLM**: Accessible via Candid interface

## 📋 Available Scripts

```bash
npm start              # Start frontend development server
npm run build          # Build frontend for production
npm run format         # Format TypeScript and Rust code
npm run generate-candid # Generate Candid interface declarations for all canisters
dfx start              # Start local IC replica
dfx deploy             # Deploy all canisters
dfx deploy llm         # Deploy only LLM canister
dfx deploy backend     # Deploy only backend canister
dfx deploy frontend    # Deploy only frontend canister
dfx stop               # Stop local IC replica
```

## 🧠 LLM Canister Details

### Functionality

- Natural language processing and understanding
- Query parsing and SQL generation
- Text analysis and intent recognition
- Response formatting and validation

### API Methods

- `process_query(text: String) -> String` - Main NLP processing
- `chat(messages: Vec<Message>) -> String` - Conversational interface
- `parse_sql_intent(query: String) -> ParseResult` - Specific SQL parsing

### Dependencies

The LLM canister may require specific dependencies or external service integration. Check the canister's `Cargo.toml` for requirements.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes across relevant canisters
4. Test all canister interactions: `npm run format && dfx deploy`
5. Commit your changes: `git commit -m 'Add amazing feature'`
6. Push to the branch: `git push origin feature/amazing-feature`
7. Open a Pull Request

### Development Guidelines

- Test canister-to-canister communication thoroughly
- Update Candid interfaces when changing canister APIs
- Use proper error handling for inter-canister calls
- Document any new LLM capabilities or configuration options

## 🔗 Links & Resources

- [Internet Computer Documentation](https://internetcomputer.org/docs/)
- [DFX SDK Reference](https://internetcomputer.org/docs/current/references/cli-reference/dfx-parent)
- [Candid Interface Guide](https://internetcomputer.org/docs/current/references/candid-ref/)
- [React Documentation](https://react.dev/)
- [Tailwind CSS v4](https://tailwindcss.com/)
- [Supabase Documentation](https://supabase.com/docs)

## 📝 License

This project is open source and available under the [MIT License](LICENSE).

## 🙋‍♂️ Support

Having issues? Check out:

1. **Common Issues** - Review the troubleshooting section below
2. **IC Community** - [Internet Computer Developer Forum](https://forum.dfinity.org/)
3. **GitHub Issues** - Create an issue for bugs or feature requests

### Troubleshooting

**DFX Port Already in Use**

```bash
dfx stop
dfx start --clean
```

**LLM Canister Communication Issues**

```bash
# Check canister status
dfx canister status llm
dfx canister status backend

# Redeploy with dependencies
dfx deploy llm
dfx deploy backend
```

**TypeScript Errors**

```bash
npx tsc -p src/frontend/tsconfig.json
npm run format
```

**Canister Deploy Issues**

```bash
dfx stop
dfx start --clean
dfx deploy
```

---

**Built with ❤️ on the Internet Computer**
