Bug Tracker Web Application

This project implements a simple web-based bug tracking system using Actix-Web, SQLx (SQLite), and Tera templates.

Features
    1. Bug CRUD API
        - Create (POST /bugs/new):
            - Accepts JSON payload with title, description, reported_by, severity, and optional project.
            - Validates required fields, generates a UUID bug ID, and returns the created record (HTTP 201).

        - Read
            - List (GET /bugs): Returns all bugs as JSON, with optional query parameters status, severity, project for server-side filtering.
            - Retrieve (GET /bugs/{id}):Fetches a single bug by its UUID. Returns HTTP 404 if not found.

        - Update (PATCH /bugs/{id}): 
            - Accepts JSON with any subset of fields (title, description, severity, status, assigned_to, project), dynamically builds 
              an UPDATE statement, and returns the updated record or HTTP 404.

        - Delete (DELETE /bugs/{id}): 
            - Removes a bug by UUID. Returns confirmation (HTTP 200) or HTTP 404 if missing.

    2. HTML Front‑End
        - Login (GET /login & POST /login): 
            - Renders a Tera-powered login form.
            - Uses a mock in-memory user with bcrypt-hashed password and configurable SALT.
            - On success, stores username and role in a cookie-based session; redirects to /home.
            - On failure, redirects back to /login?error=1 with an error banner.
            - Note: Authentication is backed by a mock in-memory user store (no persistent users table), so user credentials are not stored in the database as 
                    user management and registration fall outside the core bug-tracking functionality. By using a mock store, we keep the scope focused on 
                    demonstrating session handling and template integration without adding the complexity of a full user schema and migrations.

        - Home Dashboard (GET /home):
            - Reads username and role from the session.
            - Renders home.html via Tera, displaying personalized content and role-based sections (e.g. admin panel).
            - Redirects to /login if session data is missing.

        - Bug Assignment Form (GET /bugs/assign & POST /bugs/assign):
            - Renders an HTML form to assign an existing bug to a developer.
            - Validates input against a list of valid developer IDs; on success, updates the bug and spawns an async email notification to the admin.

    3. Templating & Static Assets
        - Uses Tera to manage HTML templates in templates/.
        - Templates include placeholders and conditional blocks (e.g. {% if role == "admin" %}) for dynamic role-based views.

    4. Session & Security
        - Uses actix-session with CookieSessionStore and a randomly generated secret_key per launch.
        - Passwords are checked using bcrypt with a project-wide SALT.
        - Session stores username and role for authentication/authorization.

    5. Database & Migrations
        - SQLite (in-memory by default) with a one-step migration at startup (loads migrations/001_create_tables.sql).
        - Schema defines bugs table with bug_id TEXT PRIMARY KEY, title, description, severity, status, assigned_to, and project.
        - SQLx dynamic queries (sqlx::query(...)) guarantee runtime flexibility

    6. Logging & Debugging
        - Actix’s Logger middleware records incoming requests.
        - Enable RUST_LOG=actix_web=debug for detailed extractor and template errors.

    7. Additional Feautres: Email Notifications
        - On bug assignment, spawns a Tokio task to send an email via a stubbed send_email() function.
        - Uses EMAIL_FROM environment variable to set the sender address for outgoing emails.

Setup & Usage
1. Run:
    
    cargo run

    - The server binds to 127.0.0.1:8080 by default.

2. Access
    - Browse http://127.0.0.1:8080/login to log in.
    - Visit http://127.0.0.1:8080/bugs for JSON API endpoints.
    - sample curl commands can be found in commands.txt

Project Structure

g22_practical
├── Cargo.toml
├── migrations/
│   └── 001_create_tables.sql
├── src/
│   ├── main.rs
│   ├── routes/
│   │   ├── auth.rs
│   │   ├── bugs.rs
│   │   ├── assign.rs
│   │   └── projects.rs
│   ├── state.rs
│   └── models.rs
└── templates/
    ├── login.html
    ├── home.html
    └── assign.html

Contributors
    - Koh Shi Min, 2401793@sit.singaporetech.edu.sg
    - Natalie Narayanan, 2403264@sit.singaporetech.edu.sg
    - Kaam Yan Hye, 2402955@sit.singaporetech.edu.sg
    - Lim Mei Yuen, 2401984@sit.singaporetech.edu.sg