# habit-tracker
Habit tracker app

## Setup

### Backend

1. Make sure that you have set up your Rust toolchain correctly. Check out
   the [instructions](https://rust-lang.github.io/rustup/installation/index.html)
   in the official Rust documentation.

2. Ensure that you have a [PostgreSQL](https://www.postgresql.org/) database
   running.
   
3. Create a `backend/refinery.toml` file using `backend/refinery.toml.sample`
   as a reference, with your DB access data.

4. Install `refinery_cli`: `cargo install refinery_cli`.

5. Set up DB by running migrations: `cd backend` and
   `refinery migrate -c refinery.toml -p src/db/migrations`.