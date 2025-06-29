# IC-Vibe_Coding

🚀 **Internet Computer Protocol (ICP) project with AI-powered natural language database queries**

Transform plain English questions into structured database queries using Groq's ultra-fast LLM API running on the Internet Computer blockchain.

## ✨ Features

- 🤖 **Natural Language Processing** - Ask questions in plain English like "show me todos where due date is not null"
- 🚀 **Ultra-Fast AI** - Powered by Groq's lightning-fast LLM API (llama-3.1-8b-instant)
- 🔍 **Real-time Query Parsing** - Watch your natural language get converted to Supabase REST API queries
- 🌐 **Internet Computer Protocol** - Fully decentralized backend running on ICP canisters
- 🔒 **Secure Architecture** - API credentials securely stored in IC canisters
- ⚡ **Instant Results** - Sub-second query execution with intelligent fallback parsing
- 🎨 **Modern UI** - Clean, responsive interface built with React and Tailwind CSS
- 🧠 **Distributed AI** - LLM processing handled by dedicated IC canister with Groq integration

## 🛠️ Tech Stack

### Backend

- **Rust** - IC canister development
- **Internet Computer Protocol (ICP)** - Decentralized hosting
- **Groq API** - Ultra-fast LLM inference (llama-3.1-8b-instant)
- **PocketIC + Vitest** - Testing framework

### Frontend

- **Vite** - Build tool and development server
- **React + TypeScript** - Component framework
- **Tailwind CSS v4** - Styling with utility classes

### Database & AI

- **Supabase** - PostgreSQL database with REST API
- **Groq Cloud** - Lightning-fast LLM API for natural language understanding
- **Smart Fallback** - Local parsing when AI is unavailable

## 🏗️ Architecture

The project consists of three main canisters:

### 1. **Backend Canister** (`backend`)

- Main application logic and HTTP outcalls
- Secure Supabase credential management
- Database query execution and result formatting
- Coordinates with LLM service for query parsing

### 2. **LLM Service Canister** (`llm_service`)

- **Groq API integration** - Ultra-fast LLM processing
- **Natural language parsing** - Converts English to Supabase REST API format
- **Intelligent fallback** - Local parsing when external API is unavailable
- **Query validation** - Ensures generated queries are safe and valid
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
- [Groq API Key](https://console.groq.com/) (for AI functionality)

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

3. **Configure Groq API (Optional but recommended)**

The LLM service includes a Groq API key for demo purposes, but for production:

- Get your API key from [Groq Console](https://console.groq.com/)
- Update the API key in `src/llm_service/src/lib.rs` (line ~94)

4. **Start the local IC replica**

```bash
dfx start --background --clean
```

5. **Deploy all canisters**

```bash
# Deploy LLM service first (provides AI functionality)
dfx deploy llm_service

# Deploy backend canister (depends on LLM service)
dfx deploy backend

# Deploy frontend canister
dfx deploy frontend

# Or deploy all at once
dfx deploy
```

6. **Start the development server**

```bash
npm start
```

7. **Open the application**
   - Visit `http://localhost:5173`
   - Try the natural language queries in the "Natural Query" tab

## 🎯 Usage

### Natural Language Queries

Navigate to the **"Natural Query"** tab and try these example queries:

```
"get all todos"
"show me todos where due date is not null"
"find todos that are completed" 
"show incomplete tasks"
"list todos with due dates"
"find todos without due dates"
"get todos by id 1"
```

### How It Works

1. **User Input** → Frontend captures natural language query
2. **Backend Canister** → Receives query and forwards to LLM service canister
3. **LLM Service** → 
   - **Primary**: Calls Groq API (llama-3.1-8b-instant) for ultra-fast parsing
   - **Fallback**: Uses local intelligent parsing if API unavailable
4. **Query Conversion** → Natural language → Supabase REST API format
5. **Backend Canister** → Executes database query using parsed results
6. **Frontend** → Displays formatted results to user

### Example Query Transformation

```
Input:  "show me todos where due date is not null"
Output: "select=*&due_date=not.is.null"

Input:  "find completed todos"  
Output: "select=*&is_done=eq.true"

Input:  "get incomplete tasks with due dates"
Output: "select=*&is_done=eq.false&due_date=not.is.null"
```

### AI-Powered Features

- **Groq Lightning Speed** - Sub-second response times
- **Smart Fallback** - Works even when external AI is unavailable  
- **Query Validation** - Ensures safe and valid database queries
- **Natural Understanding** - Handles various phrasings of the same intent

### Demo Features

- **Counter Demo** - Basic canister interaction with state management
- **Greeting Demo** - Simple text processing and response  
- **Natural Query** - Database querying with natural language powered by Groq AI
- **LLM Chat** - Direct conversation interface (if implemented)

## 🏗️ Project Structure

```
IC-Vibe_Coding/
├── src/
│   ├── backend/                 # Main Rust IC canister
│   │   ├── src/lib.rs          # Backend logic, database integration, HTTP outcalls
│   │   └── Cargo.toml          # Backend dependencies
│   ├── llm_service/            # AI processing canister  
│   │   ├── src/lib.rs          # Groq API integration, smart fallback parsing
│   │   └── Cargo.toml          # LLM service dependencies
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
dfx deploy llm_service
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

### Canister Communication

The canisters communicate as follows:

```rust
// Backend canister calls LLM service canister
#[ic_cdk::update]
async fn query_supabase_with_natural_language(user_query: String) -> Result<SupabaseResponse, String> {
    let llm_canister_id = Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai")?;
    
    // Call LLM service for query parsing
    let parse_result: (Result<QueryParseResult, String>,) = 
        ic_cdk::call(llm_canister_id, "parse_natural_language_to_sql", (user_query,)).await?;
    
    match parse_result.0 {
        Ok(query_result) => {
            // Execute the parsed query against Supabase
            fetch_from_supabase(&query_result.table, &query_result.query).await
        }
        Err(e) => Err(format!("LLM parsing failed: {}", e))
    }
}
```

### LLM Service API

The LLM service exposes these key functions:

```rust
// Main parsing function using Groq API + fallback
parse_natural_language_to_sql(user_query: String) -> Result<QueryParseResult, String>

// Groq API integration (internal)
call_groq_api(messages: Vec<ChatMessage>) -> Result<String, String>

// Smart fallback parser (internal)  
parse_query_smart_fallback(user_query: String) -> Result<QueryParseResult, String>
```

### Testing

```bash
# Run backend tests
cargo test

# Test specific canister
cd src/backend && cargo test
cd src/llm_service && cargo test

# Test LLM service directly
dfx canister call llm_service parse_natural_language_to_sql '("show me completed todos")'

# Test backend integration
dfx canister call backend query_supabase_with_natural_language '("get all todos")'

# Frontend tests (if configured)
npm test
```

## 🔒 Security & Configuration

### Canister Security

- **LLM Service** - Processes only text input, API keys stored securely in canister
- **Backend Canister** - Secure credential storage, controlled database access  
- **Frontend Canister** - Public hosting, no sensitive operations

### API Configuration

#### Groq API Setup

1. Get your API key from [Groq Console](https://console.groq.com/)
2. Update in `src/llm_service/src/lib.rs`:

```rust
let groq_api_key = "your_groq_api_key_here"; // Line ~94
```

3. Redeploy the LLM service:

```bash
dfx deploy llm_service
```

#### Supabase Setup (Optional)

The project works with mock data by default. For real database integration:

1. Create a [Supabase](https://supabase.com) account
2. Create a new project with the following schema:

```sql
-- todos table
CREATE TABLE todos (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    is_done BOOLEAN DEFAULT FALSE,
    due_date TIMESTAMP,
    status TEXT DEFAULT 'active',
    created_at TIMESTAMP DEFAULT NOW()
);
```

3. Update credentials in backend canister code

## 🚀 Deployment

### Local Development

```bash
dfx start --background --clean

# Deploy all canisters (order matters!)
dfx deploy llm_service  # Deploy AI service first
dfx deploy backend      # Deploy backend (depends on LLM service)
dfx deploy frontend     # Deploy frontend

# Or deploy all at once
dfx deploy
```

### IC Mainnet

```bash
# Deploy to IC mainnet (update API keys first!)
dfx deploy --network ic

# Check canister status
dfx canister status --network ic backend
dfx canister status --network ic llm_service  
dfx canister status --network ic frontend
```

### Canister URLs

After deployment, your canisters will be available at:

- **Frontend**: `https://{frontend-canister-id}.ic0.app`
- **Backend**: Accessible via Candid interface at `https://{backend-canister-id}.ic0.app/_/candid`
- **LLM Service**: Accessible via Candid interface at `https://{llm-service-canister-id}.ic0.app/_/candid`

## 📋 Available Scripts

```bash
npm start              # Start frontend development server
npm run build          # Build frontend for production
npm run format         # Format TypeScript and Rust code
npm run generate-candid # Generate Candid interface declarations for all canisters
dfx start              # Start local IC replica
dfx deploy             # Deploy all canisters
dfx deploy llm_service # Deploy only LLM service canister
dfx deploy backend     # Deploy only backend canister  
dfx deploy frontend    # Deploy only frontend canister
dfx stop               # Stop local IC replica
```

## 🧠 LLM Service Details

### Core Functionality

- **Groq API Integration** - Ultra-fast LLM inference using llama-3.1-8b-instant
- **Natural Language Parsing** - Converts English to Supabase REST API format
- **Smart Fallback System** - Local parsing when external API unavailable
- **Query Validation** - Ensures generated queries are safe and valid

### Supported Query Patterns

```rust
// Status filtering
"completed todos" → "is_done=eq.true"
"incomplete tasks" → "is_done=eq.false"

// Date filtering  
"todos with due dates" → "due_date=not.is.null"
"todos without due dates" → "due_date=is.null"

// Text searching
"todos containing 'groceries'" → "title=ilike.*groceries*"

// ID filtering
"todo with id 5" → "id=eq.5"

// Sorting & limiting
"latest todos" → "order=created_at.desc"
"first 5 todos" → "limit=5"
```

### API Methods

```rust
// Main parsing function (uses Groq + fallback)
parse_natural_language_to_sql(user_query: String) -> Result<QueryParseResult, String>

// Direct Groq API call (internal)
call_groq_api(messages: Vec<ChatMessage>) -> Result<String, String>

// Intelligent fallback parser (internal)
parse_query_smart_fallback(user_query: String) -> Result<QueryParseResult, String>

// HTTP response transformer (for IC HTTP outcalls)
transform(raw: TransformArgs) -> HttpResponse
```

### Configuration

The LLM service uses:
- **Model**: `llama-3.1-8b-instant` (ultra-fast inference)
- **Provider**: Groq Cloud API
- **Fallback**: Local pattern matching and parsing
- **Temperature**: 0.1 (deterministic responses)
- **Max Tokens**: 300 (optimized for query parsing)

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
- Test both Groq API and fallback parsing scenarios
- Ensure API keys are secure and not exposed in logs

### LLM Service Development

```bash
# Test LLM service directly
dfx canister call llm_service parse_natural_language_to_sql '("show completed todos")'

# Test fallback when API unavailable  
# (temporarily disable API or use invalid key to test fallback)

# Check service logs
dfx canister logs llm_service
```

## 🔗 Links & Resources

- [Internet Computer Documentation](https://internetcomputer.org/docs/)
- [DFX SDK Reference](https://internetcomputer.org/docs/current/references/cli-reference/dfx-parent)
- [Candid Interface Guide](https://internetcomputer.org/docs/current/references/candid-ref/)
- [Groq Cloud API](https://console.groq.com/) - Ultra-fast LLM inference
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

**LLM Service Communication Issues**

```bash
# Check canister status
dfx canister status llm_service
dfx canister status backend

# Redeploy with dependencies
dfx deploy llm_service
dfx deploy backend
```

**Groq API Issues**

```bash
# Test LLM service directly
dfx canister call llm_service parse_natural_language_to_sql '("test query")'

# Check logs for API errors
dfx canister logs llm_service

# Verify API key is valid (update in code and redeploy)
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

**Built with ❤️ on the Internet Computer • Powered by ⚡ Groq AI**
