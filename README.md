# 🐾 Pet Health Tracker

A simple open-source application for tracking your pet's health, built with **Rust**, **Leptos**, **SeaORM**, and **PostgreSQL**.

This app allows you to manage your pets' profiles, track vaccination records, schedule appointments, and more. 

---

## 🌟 Upcoming Features

- Add and manage pet profiles (name, breed, age, etc.).
- Track vaccination and medical history.
- Schedule appointment reminders.
- Extensible and open-source: contribute your own features!

---

## 🚀 Getting Started

### 1. Clone the Repository
```bash
git clone https://github.com/yourusername/pet-health-tracker.git
cd pet-health-tracker
```
### 2. Prerequisites

    Rust: Install Rust (version 1.70+ recommended).
    PostgreSQL: Make sure PostgreSQL is installed and running on your system.

### 3. Set Up the Database

Create a PostgreSQL database:

    CREATE DATABASE pet_health;

Update the .env file with your PostgreSQL credentials:

    cp .env.example .env

Edit the DATABASE_URL inside .env:

    DATABASE_URL=postgres://username:password@localhost:5432/pet_health

### 4. Run Migrations

Use the included migration files to set up your database schema:

    cargo run -- migrate up

### 5. Run the Application

Start the application using:

    cargo run

The app will be available at http://localhost:8080 (or as configured).

### 🛠️ Project Structure

    .
    ├── src/
    │   ├── entities/         # Auto-generated ORM models for database tables
    │   ├── migrations/       # Database migrations
    │   ├── api/              # REST API handlers
    │   └── main.rs           # Main entry point
    ├── Cargo.toml            # Rust dependencies and configuration
    ├── .env.example          # Example environment variables
    ├── README.md             # Project documentation

### 🤝 Contributing

We welcome contributions of all kinds! Here's how you can get involved:

- Fork the repository and create a new branch.
- Make your changes (e.g., fix a bug, add a feature).
- Submit a pull request with a description of your changes.


### 📜 License

This project is licensed under the GPL-3.0 License (or specify the version you’re using).

You are free to:

- Use the software for any purpose.
- Modify the software and create derivative works.
- Distribute the software and your modifications under the same terms.

However, if you distribute copies or modified versions of this software, you must:

- Include the source code or provide a way for recipients to obtain it.
- Distribute the software under the same GPL-3.0 License (or specify version).
- Include a copy of this license with the software.

For more details, refer to the full text of the GPL-3.0 License (or specify version).

### 💬 Feedback and Support

If you have any questions or suggestions, feel free to open an issue or contact us at your-email@example.com.

### 🐾 Let's Build Together!

This project is beginner-friendly, so don't hesitate to ask questions or propose ideas. We look forward to your contributions!



