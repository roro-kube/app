# Layered Modular Architecture Template

## Overview

This document describes a proven architecture pattern for building complex, modular applications with clear separation of concerns. This pattern is particularly well-suited for applications that need to execute complex processes, manage state, and provide multiple interfaces (CLI, GUI, API).

---

## Architecture Principles

### 1. **Separation of Concerns**
Each layer has a single, well-defined responsibility. This makes the codebase easier to understand, test, and maintain.

### 2. **Dependency Direction**
Dependencies flow downward: Application layers depend on Core, Core depends on Engine and Persistence, but never the reverse. This prevents circular dependencies and keeps the architecture clean.

### 3. **Abstraction Layers**
Bridge/adapter layers translate between different representations of the same data, allowing each layer to work with its optimal data structures.

### 4. **Modularity**
Each major component is a separate module/crate/package, allowing for independent development, testing, and reuse.

### 5. **Testability**
Each layer can be tested independently through well-defined interfaces.

---

## Architecture Layers

```
┌─────────────────────────────────────────────────────────────┐
│                  Application Layer                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   CLI App    │  │   GUI App    │  │   Web App    │      │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘      │
└─────────┼──────────────────┼──────────────────┼─────────────┘
          │                  │                  │
┌─────────┼──────────────────┼──────────────────┼─────────────┐
│         │                  │                  │              │
│  ┌──────▼──────┐  ┌───────▼──────┐  ┌───────▼──────┐      │
│  │   Core API  │  │   Bridge     │  │  Validation  │      │
│  │             │  │   Layer      │  │   Layer      │      │
│  └──────┬──────┘  └───────┬───────┘  └───────┬───────┘      │
│         │                  │                  │              │
│         └──────────────────┼──────────────────┘              │
│                            │                                  │
│  ┌─────────────────────────▼──────────────────────────┐      │
│  │          Business Logic / Domain Layer              │      │
│  │  ┌──────────────┐  ┌──────────────┐              │      │
│  │  │   Parser     │  │   Processor   │              │      │
│  │  │              │  │              │              │      │
│  │  │  ┌─────────┐ │  │  ┌─────────┐ │              │      │
│  │  │  │Handlers │ │  │  │Context  │ │              │      │
│  │  │  └─────────┘ │  │  └─────────┘ │              │      │
│  │  └──────────────┘  └──────────────┘              │      │
│  └───────────────────────────────────────────────────┘      │
│                            │                                  │
│  ┌─────────────────────────▼──────────────────────────┐      │
│  │          Persistence Layer                          │      │
│  │  ┌──────────────┐  ┌──────────────┐              │      │
│  │  │    Store     │  │   Models     │              │      │
│  │  │  (Database)  │  │  (Entities)  │              │      │
│  │  └──────────────┘  └──────────────┘              │      │
│  └───────────────────────────────────────────────────┘      │
└──────────────────────────────────────────────────────────────┘
```

---

## Layer 1: Application Layer

### Purpose
Provide user-facing interfaces for interacting with the system.

### Structure
```
application_layer/
├── cli/              # Command-line interface
├── gui/              # Desktop GUI application
└── web/              # Web application (optional)
```

### Responsibilities
- Parse user input
- Display results to users
- Handle user interactions
- Format output for human consumption

### Why This Structure?
- **Multiple Interfaces**: Different users prefer different interfaces (CLI for automation, GUI for visual work, Web for remote access)
- **Independent Development**: Each interface can be developed and deployed independently
- **Reusability**: All interfaces share the same Core layer, ensuring consistent behavior

### Key Principles
1. **Thin Controllers**: Application layers should be thin - they delegate business logic to Core
2. **No Business Logic**: Application layers should not contain business logic
3. **Input Validation**: Validate user input format, but delegate semantic validation to Core
4. **Error Presentation**: Transform Core errors into user-friendly messages

### Example Structure
```rust
// CLI example
cli/
├── src/
│   ├── main.rs          # Entry point, argument parsing
│   └── commands/        # Command handlers
│       ├── execute.rs
│       ├── list.rs
│       └── show.rs
└── Cargo.toml

// GUI example
gui/
├── src/
│   ├── main.rs          # Application entry point
│   ├── pages/           # Page components
│   ├── components/      # Reusable UI components
│   ├── services/        # Business logic services (thin wrappers)
│   └── queries/         # Data fetching hooks
└── Cargo.toml
```

---

## Layer 2: Core Layer

### Purpose
Orchestrate between business logic layer and persistence, provide unified APIs, and handle cross-cutting concerns.

### Structure
```
core/
├── src/
│   ├── api/             # High-level public APIs
│   ├── bridge/          # Data transformation layer
│   ├── validation/      # Input validation
│   ├── errors.rs        # Error types
│   └── lib.rs           # Public exports
└── Cargo.toml
```

### Responsibilities
1. **API Module**: Provide high-level, application-agnostic APIs
2. **Bridge Module**: Transform data between engine and persistence representations
3. **Validation Module**: Validate inputs against schemas
4. **Error Handling**: Define and transform errors across layers
5. **Initialization**: Set up global state (database, logging, etc.)

### Why This Structure?

#### API Module
- **Unified Interface**: Provides a single, consistent API for all application layers
- **Abstraction**: Hides complexity of coordinating between engine and persistence
- **Testability**: Easy to mock and test application logic

#### Bridge Module
- **Data Transformation**: Business logic and Persistence may use different data structures
- **Decoupling**: Allows business logic and persistence to evolve independently
- **Type Safety**: Provides compile-time guarantees about transformations

#### Validation Module
- **Early Validation**: Catch errors before execution
- **Schema Enforcement**: Ensure data conforms to expected structure
- **Reusability**: Same validation logic used by all application layers

### Example Structure
```rust
// core/src/api/mod.rs
pub mod execution;
pub mod queries;
pub mod mutations;

pub use execution::*;
pub use queries::*;
pub use mutations::*;

// core/src/api/execution.rs
pub async fn execute_process(process_id: &str) -> Result<ExecutionResult, CoreError> {
    // 1. Load from persistence
    // 2. Validate
    // 3. Transform to business logic format
    // 4. Execute via business logic layer
    // 5. Transform results
    // 6. Save to persistence
    // 7. Return result
}

// core/src/bridge/mod.rs
pub mod execution;
pub mod data;

// core/src/bridge/execution.rs
pub fn persistence_to_domain(
    definition: &PersistenceDefinition
) -> Result<DomainDefinition, BridgeError> {
    // Transform persistence model to domain model
}

pub fn domain_to_persistence(
    result: &DomainResult
) -> PersistenceResult {
    // Transform domain result to persistence model
}

// core/src/validation/mod.rs
pub mod schema;
pub mod validator;

pub fn validate_input(input: &str) -> Result<(), ValidationError> {
    // Validate against JSON schema or similar
}
```

### Key Design Patterns

#### 1. Facade Pattern (API Module)
The API module acts as a facade, providing a simple interface to complex subsystems.

#### 2. Adapter Pattern (Bridge Module)
The bridge module adapts between incompatible interfaces (business logic ↔ persistence).

#### 3. Strategy Pattern (Validation)
Different validation strategies can be plugged in (schema validation, custom validators, etc.).

---

## Layer 3: Business Logic / Domain Layer

### Purpose
Contains the core business logic and domain models. This layer is independent of infrastructure concerns like databases or UI frameworks.

### Structure
```
domain/
├── src/
│   ├── processor.rs     # Main business logic processor
│   ├── parser.rs        # Parse input format (if needed)
│   ├── types.rs         # Domain models and types
│   ├── handlers/        # Business logic handlers
│   │   ├── mod.rs       # Handler registry
│   │   ├── handler_a.rs
│   │   ├── handler_b.rs
│   │   └── handler_c.rs
│   ├── errors.rs        # Domain-specific errors
│   └── lib.rs           # Public API
└── Cargo.toml
```

### Responsibilities
1. **Business Rules**: Implement core business logic
2. **Domain Models**: Define domain entities and value objects
3. **Processing**: Process domain operations
4. **Validation**: Domain-level validation rules
5. **Error Handling**: Domain-specific error types

### Why This Structure?

#### Separate from Infrastructure
- **Independence**: Business logic doesn't depend on databases, UIs, or external services
- **Testability**: Can test business logic without infrastructure
- **Reusability**: Same business logic can be used by different interfaces

#### Handler Pattern
- **Extensibility**: Easy to add new business operations
- **Separation**: Each operation type is isolated
- **Testability**: Handlers can be tested independently

#### Domain Models
- **Rich Models**: Domain models contain business logic, not just data
- **Type Safety**: Strong typing prevents invalid states
- **Ubiquitous Language**: Models reflect domain terminology

### Example Structure
```rust
// domain/src/types.rs
pub struct DomainEntity {
    pub id: String,
    pub name: String,
    pub state: EntityState,
}

pub enum EntityState {
    Pending,
    Processing,
    Complete,
    Failed,
}

pub struct ProcessingContext {
    pub context_id: String,
    pub entity_name: String,
    pub state: HashMap<String, Value>,
    pub logs: Vec<String>,
}

// domain/src/processor.rs
pub struct DomainProcessor {
    handlers: Arc<HandlerRegistry>,
}

impl DomainProcessor {
    pub async fn process(&self, entity: DomainEntity) -> Result<ProcessingResult, DomainError> {
        // Business logic processing
    }
}

// domain/src/handlers/mod.rs
pub trait OperationHandler: Send + Sync {
    async fn handle(
        &self,
        context: &mut ProcessingContext,
        entity: &DomainEntity
    ) -> Result<HandlerResult, HandlerError>;
}

pub struct HandlerRegistry {
    handlers: HashMap<String, Box<dyn OperationHandler>>,
}
```

### Key Design Patterns

#### 1. Strategy Pattern (Handlers)
Different operation types use different processing strategies.

#### 2. Registry Pattern (Handler Registry)
Central registry for looking up handlers by operation type.

#### 3. Domain-Driven Design
Focus on domain models and business logic, separate from infrastructure.

---

## Layer 4: Persistence Layer

### Purpose
Store and retrieve data (definitions, executions, state, etc.).

### Structure
```
persistence/
├── src/
│   ├── store/           # Database operations
│   │   ├── mod.rs       # Store trait
│   │   ├── definitions.rs
│   │   ├── executions.rs
│   │   └── state.rs
│   ├── models/          # Data models
│   │   ├── mod.rs
│   │   ├── definition.rs
│   │   ├── execution.rs
│   │   └── state.rs
│   ├── errors.rs        # Persistence errors
│   └── lib.rs           # Public API
└── Cargo.toml
```

### Responsibilities
1. **Data Models**: Define database entities
2. **Store Operations**: CRUD operations for all entities
3. **Migrations**: Database schema management
4. **Transactions**: Ensure data consistency
5. **Querying**: Efficient data retrieval

### Why This Structure?

#### Separate Models
- **Type Safety**: Strongly-typed models prevent errors
- **Serialization**: Models handle serialization/deserialization
- **Validation**: Models can validate their own data

#### Store Trait
- **Abstraction**: Allows swapping database implementations
- **Testability**: Easy to mock for testing
- **Flexibility**: Can support multiple databases

### Example Structure
```rust
// persistence/src/models/mod.rs
pub mod definition;
pub mod execution;
pub mod state;

pub use definition::*;
pub use execution::*;
pub use state::*;

// persistence/src/models/definition.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Definition {
    pub id: String,
    pub name: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// persistence/src/store/mod.rs
#[async_trait]
pub trait Store: Send + Sync {
    async fn save_definition(&self, definition: &Definition) -> Result<(), PersistenceError>;
    async fn get_definition(&self, id: &str) -> Result<Option<Definition>, PersistenceError>;
    async fn list_definitions(&self) -> Result<Vec<Definition>, PersistenceError>;
    // ... more operations
}

// persistence/src/store/lib.rs
pub struct SqliteStore {
    pool: SqlitePool,
}

impl Store for SqliteStore {
    // Implementation using SQLx or similar
}
```

### Key Design Patterns

#### 1. Repository Pattern (Store)
Store acts as a repository, abstracting database operations.

#### 2. Active Record Pattern (Models)
Models contain both data and behavior.

---

## Frontend Asset Management

### Overview

For GUI applications, you'll need to manage various frontend assets:
- **Icons**: SVG icons for UI elements
- **Styling**: CSS framework (Tailwind CSS, etc.)
- **Branding**: Logos and app icons for different platforms

This section explains how to structure and integrate these assets into your application.

### Icons System

#### Purpose
Manage SVG icons used throughout the GUI application. Icons should be:
- Easy to add/remove
- Consistent in style
- Optimized for size
- Accessible from Rust code

#### Structure
```
icons/
├── package.json          # Node.js dependencies (heroicons, etc.)
├── copy-icons.js         # Script to copy icons to GUI assets
├── scripts/
│   ├── build.mjs         # Build branding icons
│   └── verify.mjs        # Verify icon integrity
├── branding/             # Custom branding icons
│   └── logo.svg
└── dist/                 # Generated platform-specific icons
    ├── macos/
    ├── windows/
    └── png/
```

#### Icon Source Management
1. **Use Icon Library**: Install an icon library (e.g., Heroicons) via npm
2. **Icon Mapping**: Create a mapping file that maps your internal icon names to library icon names
3. **Copy Script**: Write a script that copies icons from the library to your GUI assets directory

#### Example Icon Copy Script
```javascript
// icons/copy-icons.js
import fs from "fs";
import path from "path";

const iconMap = {
  home: "home",
  settings: "cog-6-tooth",
  plus: "plus",
  // ... more mappings
};

const heroiconsDir = path.join(__dirname, "node_modules", "heroicons");
const iconsOutputDir = path.join(__dirname, "..", "gui", "assets", "icons");

Object.entries(iconMap).forEach(([ourName, heroiconName]) => {
  const src = path.join(heroiconsDir, "24", "outline", `${heroiconName}.svg`);
  const dest = path.join(iconsOutputDir, `${ourName}-outline.svg`);
  fs.copyFileSync(src, dest);
});
```

#### Icon Component in GUI
```rust
// gui/src/icons.rs
use dioxus::prelude::*;

#[component]
pub fn Icon(
    name: String,
    class: Option<String>,
    size: Option<String>,
) -> Element {
    let icon_svg = get_icon_svg(&name);
    let size = size.unwrap_or_else(|| "w-4 h-4".to_string());
    
    rsx! {
        div {
            class: format!("{} {} inline-block", size, class.unwrap_or_default()),
            dangerous_inner_html: icon_svg
        }
    }
}

fn get_icon_svg(name: &str) -> String {
    match name {
        "home" => include_str!("../assets/icons/home-outline.svg").to_string(),
        "settings" => include_str!("../assets/icons/settings-outline.svg").to_string(),
        // ... more icons
        _ => default_icon(),
    }
}
```

#### Platform-Specific Icons
For app bundle icons (macOS .icns, Windows .ico, Linux PNGs):
1. Start with a high-resolution source SVG
2. Generate platform-specific formats using tools like `@ctjs/png2icons` or `sharp`
3. Copy to GUI assets directory
4. Reference in app bundle configuration

#### Why This Structure?
- **Separation**: Icons are managed separately from GUI code
- **Reusability**: Same icons can be used across different platforms
- **Maintainability**: Easy to update icons without touching Rust code
- **Build Process**: Icons are built as part of the build pipeline

---

### CSS Framework Integration (Tailwind CSS)

#### Purpose
Use Tailwind CSS (or similar utility-first CSS framework) for styling the GUI application.

#### Structure
```
tailwind/
├── package.json          # Tailwind, PostCSS, Vite dependencies
├── tailwind.config.js    # Tailwind configuration
├── postcss.config.js     # PostCSS configuration
├── vite.config.ts        # Vite build configuration
└── src/
    └── tailwind.css      # Tailwind source file
```

#### Tailwind Configuration
```javascript
// tailwind/tailwind.config.js
export default {
  content: [
    "../gui/src/**/*.rs",        // Scan Rust files for class names
    "../gui/src/**/*.html",
    "../react-app/src/**/*.{ts,tsx}",  // If you have a React app too
  ],
  theme: {
    extend: {
      colors: {
        primary: {
          // Your custom color palette
        },
      },
    },
  },
  plugins: [],
  darkMode: "class",  // Enable dark mode
};
```

#### Build Configuration
```typescript
// tailwind/vite.config.ts
import { defineConfig } from 'vite';
import tailwindcss from 'tailwindcss';
import autoprefixer from 'autoprefixer';

export default defineConfig({
  build: {
    outDir: '../gui/assets',  // Output to GUI assets
    emptyOutDir: false,
    rollupOptions: {
      input: {
        'tailwind': resolve(__dirname, 'src/tailwind.css'),
      },
      output: {
        assetFileNames: (assetInfo) => {
          if (assetInfo.name === 'tailwind.css') {
            return 'tailwind.css';  // Keep consistent name
          }
          return '[name].[ext]';
        }
      }
    }
  },
  css: {
    postcss: {
      plugins: [
        tailwindcss,
        autoprefixer,
      ],
    },
  },
});
```

#### Tailwind Source File
```css
/* tailwind/src/tailwind.css */
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {
  * {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }
  
  html {
    font-family: system-ui, sans-serif;
  }
}

@layer components {
  .btn-primary {
    @apply bg-primary-600 hover:bg-primary-700 text-white font-semibold py-2 px-4 rounded-lg;
  }
}
```

#### Integration in GUI
```rust
// gui/src/layout/app.rs
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    rsx! {
        // Include the compiled Tailwind CSS
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        
        div {
            class: "min-h-screen bg-white dark:bg-zinc-900",
            // Your app content
        }
    }
}
```

#### Why This Structure?
- **Separation**: CSS is built separately from Rust code
- **Performance**: Tailwind purges unused CSS, keeping bundle small
- **Developer Experience**: Utility classes are fast to write
- **Dark Mode**: Easy to implement with class-based dark mode
- **Build Process**: CSS is compiled as part of build pipeline

---

### GUI Asset Integration

#### Asset Directory Structure
```
gui/
├── assets/
│   ├── icons/            # SVG icons (copied from icons/)
│   ├── branding/         # Logos and app icons
│   ├── tailwind.css      # Compiled Tailwind CSS
│   └── workflow-visualizer/  # Other frontend assets
└── src/
    ├── icons.rs          # Icon component
    └── layout/
        └── app.rs        # Main app layout
```

#### Build Process Integration

Use a task runner (like Taskfile.yml) to coordinate asset building:

```yaml
# Taskfile.yml
tasks:
  run-gui:
    desc: Run GUI application
    cmds:
      - task: tailwind      # Build Tailwind CSS
      - task: icons         # Copy icons
      - task: visualizer    # Build other assets
      - dx run --package gui  # Run the GUI

  tailwind:
    desc: Build Tailwind CSS
    dir: tailwind
    cmds:
      - npm install
      - npm run build

  icons:
    desc: Copy icons to GUI assets
    dir: icons
    cmds:
      - npm install
      - npm run build
```

#### Asset Loading in Dioxus

Dioxus provides the `asset!()` macro for loading assets:

```rust
// Load CSS
document::Stylesheet { href: asset!("/assets/tailwind.css") }

// Load icons (via include_str!)
let icon = include_str!("../assets/icons/home-outline.svg");

// Load images
img { src: asset!("/assets/branding/logo.svg") }
```

#### Why This Integration Pattern?
- **Build-Time**: Assets are built before Rust compilation
- **Runtime**: Assets are embedded or served efficiently
- **Development**: Hot reload for assets during development
- **Production**: Optimized assets in production builds

---

### Complete Build Workflow

1. **Icons Build**:
   ```bash
   cd icons
   npm install
   npm run build  # Copies icons to gui/assets/icons/
   ```

2. **Tailwind Build**:
   ```bash
   cd tailwind
   npm install
   npm run build  # Compiles CSS to gui/assets/tailwind.css
   ```

3. **GUI Build**:
   ```bash
   cd gui
   cargo build  # Rust compilation (assets already in place)
   ```

4. **Run Application**:
   ```bash
   dx run --package gui  # Or cargo run -p gui
   ```

#### Development Workflow

For development, use watch mode:

```yaml
# Taskfile.yml
tasks:
  dev:
    desc: Run GUI in development mode with asset watching
    cmds:
      - task: tailwind:watch &
      - task: icons:watch &
      - dx run --package gui

  tailwind:watch:
    dir: tailwind
    cmds:
      - npm run dev  # Watch mode

  icons:watch:
    dir: icons
    cmds:
      - npm run build:watch  # Watch and rebuild icons
```

---

## Reusable Library Patterns

### Query/Cache System Library

A reusable query/cache system library provides data fetching, caching, and state management for UI frameworks. This pattern is inspired by React Query but can be adapted for any UI framework.

### Purpose
Provide a generic, framework-agnostic (or framework-specific) query and mutation system that handles:
- Data fetching with caching
- Automatic cache invalidation
- Loading and error states
- Optimistic updates
- Background refetching

### Structure
```
query_library/
├── src/
│   ├── query.rs         # Query hook/trait
│   ├── mutation.rs      # Mutation hook/trait
│   ├── cache/           # Cache management
│   │   ├── storage.rs   # Cache storage implementation
│   │   ├── entry.rs     # Cache entry structure
│   │   ├── cleanup.rs   # Cache cleanup/eviction
│   │   └── synchronization.rs  # Cache sync logic
│   ├── invalidate.rs    # Cache invalidation
│   ├── query_key.rs     # Query key management
│   ├── state.rs         # Query/mutation state types
│   └── lib.rs           # Public API
└── Cargo.toml
```

### Key Components

#### Query System
- **Query Hook/Trait**: Provides `use_query`-like functionality
- **Query State**: Manages loading, error, and data states
- **Query Options**: Configurable options (stale time, cache time, etc.)

#### Mutation System
- **Mutation Hook/Trait**: Provides `use_mutation`-like functionality
- **Mutation State**: Manages pending, error, and success states
- **Callbacks**: onSuccess, onError, onSettled callbacks

#### Cache Management
- **Storage**: In-memory or persistent cache storage
- **Entry**: Individual cache entries with metadata
- **Cleanup**: Automatic eviction of stale entries
- **Synchronization**: Keep cache in sync across components

#### Query Keys
- **Key Structure**: Hierarchical key structure for invalidation
- **Key Matching**: Pattern matching for bulk invalidation

### Example Implementation Pattern
```rust
// query_library/src/query.rs
pub struct QueryOptions<T> {
    pub query_key: QueryKey,
    pub query_fn: Box<dyn Fn() -> Future<Output = Result<T, Error>>>,
    pub stale_time: Option<Duration>,
    pub cache_time: Option<Duration>,
}

pub struct QueryState<T> {
    pub data: Option<T>,
    pub error: Option<Error>,
    pub is_loading: bool,
    pub is_fetching: bool,
    pub is_stale: bool,
}

pub trait QueryHook {
    fn use_query<T>(options: QueryOptions<T>) -> QueryState<T>;
}

// query_library/src/mutation.rs
pub struct MutationOptions<T, V> {
    pub mutation_fn: Box<dyn Fn(V) -> Future<Output = Result<T, Error>>>,
    pub on_success: Option<Box<dyn Fn(&T)>>,
    pub on_error: Option<Box<dyn Fn(&Error)>>,
}

pub struct MutationState<T> {
    pub data: Option<T>,
    pub error: Option<Error>,
    pub is_pending: bool,
}

pub trait MutationHook {
    fn use_mutation<T, V>(options: MutationOptions<T, V>) -> (MutationState<T>, MutationFn<V>);
}

// query_library/src/invalidate.rs
pub fn invalidate_query(key: &QueryKey);
pub fn invalidate_queries_by_prefix(prefix: &str);
pub fn invalidate_all_queries();
```

### Why Create a Separate Library?
- **Reusability**: Can be used across multiple projects
- **Framework Integration**: Can provide framework-specific adapters
- **Testing**: Can be tested independently
- **Evolution**: Can evolve separately from main application
- **Community**: Can be open-sourced and shared

### Framework-Specific Adapters

The library can provide adapters for specific frameworks:

```
query_library/
├── src/
│   └── ... (core implementation)
├── adapters/
│   ├── dioxus/          # Dioxus-specific hooks
│   ├── react/           # React-specific hooks (if porting)
│   └── yew/             # Yew-specific hooks
└── Cargo.toml
```

### Usage in Application
```rust
// In your GUI application
use query_library::prelude::*;

// Use the query hook
let query_state = use_query(QueryOptions {
    query_key: QueryKey::new(&["items", id]),
    query_fn: Box::new(|| async {
        core_api::get_item(id).await
    }),
    stale_time: Some(Duration::from_secs(60)),
    cache_time: Some(Duration::from_secs(300)),
});

// Use the mutation hook
let (mutation_state, mutate) = use_mutation(MutationOptions {
    mutation_fn: Box::new(|input| async {
        core_api::create_item(input).await
    }),
    on_success: Some(Box::new(|_| {
        invalidate_queries_by_prefix("items");
    })),
});
```

### Benefits
- **Performance**: Reduces redundant data fetching
- **UX**: Automatic loading/error states
- **Consistency**: Keeps UI in sync with data
- **Developer Experience**: Simple API for common patterns
- **Flexibility**: Can be extended for specific needs

---

## Global State Management

### Singleton Pattern for Shared Resources

For resources that should be shared across the application (database connection, configuration, etc.), use a singleton pattern.

### Example
```rust
// core/src/singleton.rs
use std::sync::{Arc, OnceLock};

static GLOBAL_STORE: OnceLock<Arc<dyn Store>> = OnceLock::new();

pub async fn init_global_store() -> Result<(), String> {
    let store = create_store().await?;
    GLOBAL_STORE
        .set(Arc::new(store))
        .map_err(|_| "Store already initialized")?;
    Ok(())
}

pub fn get_global_store() -> Result<Arc<dyn Store>, String> {
    GLOBAL_STORE
        .get()
        .cloned()
        .ok_or_else(|| "Store not initialized".to_string())
}
```

### Why?
- **Single Connection**: Database connections should be pooled and shared
- **Configuration**: Global configuration should be accessible everywhere
- **Thread Safety**: Arc ensures safe sharing across threads

---

## Error Handling Strategy

### Layered Error Types

Each layer should define its own error types, with Core providing transformation.

### Structure
```rust
// domain/src/errors.rs
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Processing error: {0}")]
    Processing(#[from] ProcessingError),
    #[error("Validation error: {0}")]
    Validation(String),
}

// persistence/src/errors.rs
#[derive(Debug, thiserror::Error)]
pub enum PersistenceError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Not found: {0}")]
    NotFound(String),
}

// core/src/errors.rs
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),
    #[error("Persistence error: {0}")]
    Persistence(#[from] PersistenceError),
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),
}
```

### Why?
- **Type Safety**: Each layer's errors are distinct
- **Context**: Errors include layer-specific context
- **Transformation**: Core can transform errors for application layers

---

## Validation Strategy

### Schema-Based Validation

Use JSON Schema or similar for validating structured input.

### Structure
```
core/
├── schema/
│   ├── definition.schema.json
│   └── README.md
└── src/
    └── validation/
        ├── validator.rs
        ├── schema.rs
        └── errors.rs
```

### Why?
- **Declarative**: Validation rules defined in schema, not code
- **Reusable**: Same schema used by all application layers
- **Maintainable**: Schema changes don't require code changes
- **Standard**: JSON Schema is a well-known standard

---

## Testing Strategy

### Layer-by-Layer Testing

Each layer should have comprehensive tests.

### Structure
```
domain/
├── src/
└── tests/
    ├── processor_tests.rs
    ├── handler_tests.rs
    └── model_tests.rs

core/
├── src/
└── tests/
    ├── api_tests.rs
    ├── bridge_tests.rs
    └── validation_tests.rs

persistence/
├── src/
└── tests/
    ├── store_tests.rs
    └── model_tests.rs
```

### Testing Principles
1. **Unit Tests**: Test each component in isolation
2. **Integration Tests**: Test layer interactions
3. **Mocking**: Mock dependencies for isolated testing
4. **Fixtures**: Use fixtures for consistent test data

---

## Dependency Management

### Dependency Direction Rules

1. **Application → Core**: Applications depend on Core
2. **Application → Query Library**: Applications can depend on query library
3. **Core → Domain**: Core depends on Domain (business logic)
4. **Core → Persistence**: Core depends on Persistence
5. **Never Reverse**: Domain and Persistence never depend on Core or Application
6. **Query Library → Core**: Query library can depend on Core for data fetching

### Why?
- **No Circular Dependencies**: Prevents dependency cycles
- **Clear Ownership**: Each layer owns its dependencies
- **Testability**: Easy to mock and test layers independently

---

## File Organization Principles

### 1. **Group by Feature, Not Type**
Organize by what the code does, not what it is.

```
❌ Bad:
src/
├── structs.rs
├── functions.rs
└── traits.rs

✅ Good:
src/
├── execution/
│   ├── mod.rs
│   ├── engine.rs
│   └── context.rs
└── parsing/
    ├── mod.rs
    └── parser.rs
```

### 2. **Public API in lib.rs**
Only export what's needed by other crates.

```rust
// lib.rs
pub mod api;
pub mod bridge;
pub mod errors;

pub use api::*;
pub use errors::CoreError;
```

### 3. **Tests Close to Code**
Keep tests in the same crate, in a `tests/` directory.

### 4. **Examples Separate**
Keep example code in an `examples/` directory.

---

## Migration Guide

### Step 1: Identify Your Layers
1. What are your application interfaces? (CLI, GUI, Web)
2. What is your business logic? (What domain operations do you need?)
3. What do you need to persist? (Entities, state, configurations)
4. What validation do you need? (Input validation, schema validation)
5. Do you need a query/cache library? (For GUI applications)

### Step 2: Create Module Structure
1. Create separate crates/packages for each layer
2. Set up workspace configuration
3. Define dependencies between crates

### Step 3: Implement Bottom-Up
1. **Persistence**: Start with data models and store
2. **Domain**: Implement business logic layer
3. **Query Library** (if needed): Build reusable query/cache system
4. **Core**: Build bridge and API layers
5. **Applications**: Build application interfaces

### Step 4: Add Cross-Cutting Concerns
1. Error handling
2. Logging
3. Validation
4. Configuration

### Step 5: Testing
1. Write tests for each layer
2. Write integration tests
3. Set up CI/CD

---

## Common Pitfalls and Solutions

### Pitfall 1: Business Logic in Application Layer
**Problem**: Business logic scattered across CLI, GUI, Web
**Solution**: Move all business logic to Core API layer

### Pitfall 2: Direct Domain/Persistence Access from Applications
**Problem**: Applications directly use domain or persistence layers
**Solution**: Always go through Core API layer

### Pitfall 3: Tight Coupling Between Layers
**Problem**: Layers depend on implementation details
**Solution**: Use traits/interfaces, not concrete types

### Pitfall 4: Circular Dependencies
**Problem**: Engine depends on Core, Core depends on Engine
**Solution**: Follow dependency direction rules strictly

### Pitfall 5: God Objects
**Problem**: Single massive struct/class doing everything
**Solution**: Split into focused modules with single responsibilities

---

## Benefits of This Architecture

1. **Maintainability**: Clear separation makes code easy to understand and modify
2. **Testability**: Each layer can be tested independently
3. **Scalability**: Easy to add new features without affecting existing code
4. **Reusability**: Core logic can be reused across different interfaces
5. **Flexibility**: Can swap implementations (different databases, UIs, etc.)
6. **Team Collaboration**: Different teams can work on different layers
7. **Documentation**: Architecture is self-documenting through structure

---

## Embedded Frontend Applications

### Overview

Sometimes you need to embed a separate frontend application (React, Vue, etc.) within your desktop GUI. This pattern allows you to leverage existing web technologies for complex visualizations or editors while keeping the main application in Rust.

### Use Cases

- **Visual Editors**: Complex graph/node editors (React Flow, D3.js, etc.)
- **Rich Text Editors**: WYSIWYG editors (TinyMCE, Quill, etc.)
- **Data Visualization**: Charts and dashboards (Chart.js, D3.js, etc.)
- **Code Editors**: Syntax-highlighted code editors (Monaco, CodeMirror, etc.)

### Architecture Pattern

```
┌─────────────────────────────────────┐
│     Desktop GUI (Rust/Dioxus)      │
│  ┌───────────────────────────────┐ │
│  │  iframe                       │ │
│  │  ┌─────────────────────────┐ │ │
│  │  │  Embedded Web App       │ │ │
│  │  │  (React/Vue/etc.)       │ │ │
│  │  └─────────────────────────┘ │ │
│  └───────────────────────────────┘ │
└─────────────────────────────────────┘
         │                    │
         │ postMessage        │ postMessage
         │                    │
         ▼                    ▼
    ┌─────────┐         ┌─────────┐
    │  Load   │         │  Save   │
    │  Data   │         │  Data   │
    └─────────┘         └─────────┘
```

### Implementation Steps

#### 1. Create Standalone Frontend App

```bash
# Create React/Vue/etc. app
npm create vite@latest embedded-app -- --template react-ts
cd embedded-app
npm install
```

#### 2. Build Configuration

Configure the build to output to your GUI assets directory:

```typescript
// embedded-app/vite.config.ts
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
  build: {
    outDir: '../gui/assets/embedded-app',
    emptyOutDir: true,
    rollupOptions: {
      input: {
        main: resolve(__dirname, 'index.html'),
      },
    },
  },
});
```

#### 3. Communication Protocol

Define a message protocol for parent-child communication:

```typescript
// embedded-app/src/types.ts
export interface LoadMessage {
  type: 'LOAD_DATA';
  payload: {
    data: any;  // Your data structure
  };
}

export interface SaveMessage {
  type: 'SAVE_DATA';
  payload: {
    data: any;  // Updated data
  };
}

export type Message = LoadMessage | SaveMessage;
```

#### 4. Embedded App Implementation

```typescript
// embedded-app/src/App.tsx
import { useEffect, useState } from 'react';

function App() {
  const [data, setData] = useState(null);

  useEffect(() => {
    // Listen for messages from parent (Dioxus)
    const handleMessage = (event: MessageEvent<Message>) => {
      if (event.data.type === 'LOAD_DATA') {
        setData(event.data.payload.data);
      }
    };

    window.addEventListener('message', handleMessage);
    return () => window.removeEventListener('message', handleMessage);
  }, []);

  const handleSave = () => {
    // Send data back to parent
    window.parent.postMessage({
      type: 'SAVE_DATA',
      payload: { data },
    }, '*');
  };

  return (
    <div>
      {/* Your embedded app UI */}
    </div>
  );
}
```

#### 5. GUI Integration (Dioxus)

```rust
// gui/src/pages/embedded/mod.rs
use dioxus::prelude::*;

#[component]
pub fn EmbeddedAppPage(data_id: String) -> Element {
    let iframe_ref = use_signal(|| None);
    
    use_effect(move || {
        // Load data from database/API
        let data = load_data(&data_id);
        
        // Send to iframe when ready
        if let Some(iframe) = iframe_ref() {
            iframe.post_message(&LoadMessage {
                type: "LOAD_DATA",
                payload: data,
            });
        }
    });
    
    rsx! {
        div {
            class: "h-full w-full",
            iframe {
                src: asset!("/assets/embedded-app/index.html"),
                class: "w-full h-full border-0",
                onload: move |_| {
                    // Handle iframe load
                }
            }
        }
    }
}
```

#### 6. Message Handling in GUI

```rust
// gui/src/pages/embedded/mod.rs
use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn addEventListener(event: &str, callback: &Closure<dyn FnMut(web_sys::MessageEvent)>);
}

// Listen for messages from iframe
use_effect(move || {
    let closure = Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
        // Handle SAVE_DATA message
        if let Ok(data) = serde_json::from_str(&event.data().as_string().unwrap()) {
            save_data(&data_id, &data);
        }
    }) as Box<dyn FnMut(web_sys::MessageEvent)>);
    
    window().add_event_listener_with_callback("message", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
});
```

### Why This Pattern?

- **Technology Choice**: Use the best tool for complex visualizations
- **Separation**: Embedded app is independent and testable
- **Performance**: Web technologies are optimized for UI
- **Reusability**: Embedded app can be used in web version too
- **Development**: Can develop embedded app independently

### Build Integration

Add to your build process:

```yaml
# Taskfile.yml
tasks:
  embedded-app:
    desc: Build embedded frontend application
    dir: embedded-app
    cmds:
      - npm install
      - npm run build

  run-gui:
    desc: Run GUI with all assets
    cmds:
      - task: tailwind
      - task: icons
      - task: embedded-app
      - dx run --package gui
```

---

## Workspace Configuration

### Cargo Workspace Setup

For multi-crate Rust projects, use a Cargo workspace:

```toml
# Cargo.toml (root)
[workspace]
members = [
  "cli",
  "core",
  "domain",
  "persistence",
  "gui",
  "query_library",
]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]

[workspace.lints.clippy]
pedantic = "warn"
cognitive_complexity = "warn"
type_complexity = "warn"

[workspace.dependencies]
# Shared dependencies
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
```

### Benefits

- **Shared Dependencies**: Define versions once
- **Unified Versioning**: All crates share version
- **Consistent Linting**: Shared clippy configuration
- **Single Build**: `cargo build --workspace` builds all

---

## Development Environment Setup

### Prerequisites

1. **Rust Toolchain**: Install via rustup
2. **Node.js**: For frontend asset building
3. **Database**: SQLite (or your chosen database)
4. **Build Tools**: Task runner (Task, Make, etc.)

### Setup Steps

1. **Clone Repository**
   ```bash
   git clone <repo>
   cd <project>
   ```

2. **Install Rust Dependencies**
   ```bash
   cargo build
   ```

3. **Install Node Dependencies**
   ```bash
   cd icons && npm install
   cd ../tailwind && npm install
   cd ../embedded-app && npm install
   ```

4. **Build Assets**
   ```bash
   task: icons
   task: tailwind
   task: embedded-app
   ```

5. **Initialize Database**
   ```bash
   cargo run -p cli -- init-db
   ```

6. **Run Application**
   ```bash
   task: run-gui
   ```

### Development Scripts

Create helper scripts for common tasks:

```yaml
# Taskfile.yml
tasks:
  setup:
    desc: Initial project setup
    cmds:
      - cargo build
      - task: install-node-deps
      - task: build-assets
      - cargo run -p cli -- init-db

  install-node-deps:
    desc: Install all Node.js dependencies
    cmds:
      - cd icons && npm install
      - cd ../tailwind && npm install
      - cd ../embedded-app && npm install

  build-assets:
    desc: Build all frontend assets
    cmds:
      - task: icons
      - task: tailwind
      - task: embedded-app

  dev:
    desc: Run in development mode with hot reload
    cmds:
      - task: tailwind:watch &
      - task: embedded-app:watch &
      - dx run --package gui
```

---

## Configuration Management

### Environment Variables

Use environment variables for configuration:

```rust
// core/src/config.rs
pub struct Config {
    pub database_path: String,
    pub log_level: String,
    pub api_url: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Config {
            database_path: std::env::var("DATABASE_PATH")
                .unwrap_or_else(|_| "~/.app/data.db".to_string()),
            log_level: std::env::var("LOG_LEVEL")
                .unwrap_or_else(|_| "info".to_string()),
            api_url: std::env::var("API_URL").ok(),
        })
    }
}
```

### Configuration Files

For more complex configuration, use TOML/JSON:

```toml
# config.toml
[database]
path = "~/.app/data.db"
pool_size = 10

[logging]
level = "info"
file = "~/.app/app.log"

[api]
url = "https://api.example.com"
timeout_secs = 30
```

```rust
// core/src/config.rs
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
    pub api: Option<ApiConfig>,
}

pub fn load_config() -> Result<Config, ConfigError> {
    let config_path = std::env::var("CONFIG_PATH")
        .unwrap_or_else(|_| "config.toml".to_string());
    
    let content = std::fs::read_to_string(&config_path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
```

### Why This Approach?

- **Flexibility**: Different configs for dev/staging/prod
- **Security**: Sensitive data via environment variables
- **Version Control**: Config files can be versioned (with secrets excluded)
- **Documentation**: Config files serve as documentation

---

## Database Migrations

### Migration Strategy

Handle database schema changes over time:

```rust
// persistence/src/migrations/mod.rs
pub struct Migration {
    pub version: u32,
    pub name: String,
    pub up: &'static str,  // SQL for upgrade
    pub down: Option<&'static str>,  // SQL for downgrade
}

pub const MIGRATIONS: &[Migration] = &[
    Migration {
        version: 1,
        name: "initial_schema",
        up: include_str!("migrations/001_initial_schema.sql"),
        down: None,
    },
    Migration {
        version: 2,
        name: "add_metadata",
        up: include_str!("migrations/002_add_metadata.sql"),
        down: Some(include_str!("migrations/002_add_metadata_down.sql")),
    },
];

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), MigrationError> {
    // Create migrations table if it doesn't exist
    sqlx::query("CREATE TABLE IF NOT EXISTS schema_migrations (version INTEGER PRIMARY KEY)")
        .execute(pool)
        .await?;
    
    // Get current version
    let current_version: Option<i64> = sqlx::query_scalar(
        "SELECT MAX(version) FROM schema_migrations"
    )
    .fetch_optional(pool)
    .await?;
    
    let current_version = current_version.unwrap_or(0) as u32;
    
    // Run pending migrations
    for migration in MIGRATIONS {
        if migration.version > current_version {
            sqlx::query(migration.up)
                .execute(pool)
                .await?;
            
            sqlx::query("INSERT INTO schema_migrations (version) VALUES (?)")
                .bind(migration.version as i64)
                .execute(pool)
                .await?;
        }
    }
    
    Ok(())
}
```

### Migration Files

```
persistence/
├── src/
│   └── migrations/
│       ├── mod.rs
│       └── migrations/
│           ├── 001_initial_schema.sql
│           ├── 002_add_metadata.sql
│           └── 002_add_metadata_down.sql
```

### Why This Approach?

- **Version Control**: Track schema changes
- **Reproducibility**: Same schema across environments
- **Rollback**: Can downgrade if needed
- **Team Collaboration**: Everyone gets same schema

---

## Logging and Observability

### Structured Logging Setup

```rust
// core/src/logging.rs
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logging(config: &LoggingConfig) -> Result<(), LoggingError> {
    let filter = tracing_subscriber::EnvFilter::new(&config.level);
    
    let subscriber = tracing_subscriber::registry()
        .with(filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_thread_ids(true)
                .json(),  // JSON format for production
        );
    
    if let Some(file_path) = &config.file {
        let file = std::fs::File::create(file_path)?;
        subscriber
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(file)
                    .json(),
            );
    }
    
    subscriber.init();
    Ok(())
}
```

### Logging Best Practices

1. **Use Structured Fields**: Include context in logs
   ```rust
   tracing::info!(
       user_id = %user_id,
       action = "create_item",
       item_id = %item_id,
       "Item created successfully"
   );
   ```

2. **Appropriate Log Levels**:
   - `ERROR`: System errors, failures
   - `WARN`: Recoverable issues, deprecations
   - `INFO`: Important business events
   - `DEBUG`: Detailed execution flow
   - `TRACE`: Very detailed, verbose

3. **Performance Logging**: Log slow operations
   ```rust
   let start = std::time::Instant::now();
   // ... operation ...
   tracing::debug!(
       duration_ms = start.elapsed().as_millis(),
       "Operation completed"
   );
   ```

### Why Structured Logging?

- **Searchability**: Easy to filter and search logs
- **Analysis**: Can analyze patterns and issues
- **Debugging**: Rich context for debugging
- **Monitoring**: Can feed into monitoring systems

---

## Conclusion

This architecture template provides a solid foundation for building complex, modular applications. The key is maintaining clear boundaries between layers and following the dependency direction rules. Start simple, add complexity only when needed, and always keep the principles of separation of concerns and single responsibility in mind.

### Key Takeaways

1. **Layer Separation**: Keep layers independent and testable
2. **Dependency Direction**: Always flow downward
3. **Asset Management**: Build assets separately, integrate cleanly
4. **Communication**: Use well-defined protocols for embedded apps
5. **Configuration**: Make configuration flexible and environment-aware
6. **Observability**: Log everything, structure it well
7. **Migrations**: Version your database schema
8. **Development**: Make setup and development easy

