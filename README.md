# AI File Finder

This system aims to make file search easier in your system enabling Natural Language based file search, Software launching and content context based file search.

## High level Architecture diagram

```mermaid
graph TD

  %% UI Layer
  subgraph UI/UX Layer
    UI1[1a. User Interface - Search Input, Result Viewer]
  end

  %% Backend Layer
  subgraph Backend Layer

    %% 2.1 Indexing
    IDX1[2a. File Indexer]
    IDX2[2 Change Watcher]

    %% 2.2 Preprocessing
    PRE1[3a. Audio Transcriber - Automatic Speech Recognition Model]
    PRE2[3b. Image Describer - Image to Text Model]
    PRE3[3c. Text Extractor - OCR Engine + Document Parser]

    %% 2.3 Embedding Pipeline
    EMB1[4a. Embedding Model]

    %% 2.4 AI Agent Layer
    AG1[5 Orchestrator - LangGraph with local LLM]
    AG2[5b. RAG]
    AG3[5a. Query Router - Search vs Launch]
    AG4[5c. App Launcher]
  end

  %% Data Layer
  subgraph Data Access Layer
    DB1[6a. SQLite DB - File Paths, Extensions]
    DB2[6b. Vector Index - Vector Embeddings]
  end

  %% Data Flow Connections
  UI1 --> AG1

  IDX2 --> IDX1
  IDX1 --> DB1

  IDX1 --> PRE1
  IDX1 --> PRE2
  IDX1 --> PRE3

  PRE1 --> EMB1
  PRE2 --> EMB1
  PRE3 --> EMB1

  EMB1 --> DB2

  AG1 --> AG3
  AG3 --> AG4
  AG3 --> AG2
  AG2 --> DB2

  AG4 --> SystemApp[App/System Launcher]
```

## Currently implemented features:
- Real time file tracking as a background service (Monitors create, rename and delete new files)
- Automatic rule based exclusion of system directories and temp files to reduce noise
- File events storage in an indexed db
