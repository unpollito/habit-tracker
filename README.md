# habit-tracker
Habit tracker app

## Setup

### Backend

1. Make sure that you have set up your Rust toolchain correctly. Check out
   the [instructions](https://rust-lang.github.io/rustup/installation/index.html)
   in the official Rust documentation.

2. Ensure that you have a [PostgreSQL](https://www.postgresql.org/) database
   running.
   
3. Ensure that you have your `libpq-dev` package installed in Linux, or that
   [it is added to the PATH in Windows](https://stackoverflow.com/questions/62708607/how-to-fix-rust-diesel-cli-link-libpq-lib-error-on-install).
   
4. Install `diesel_cli` with PostgreSQL support:
   `cargo install diesel_cli --no-default-features --features postgres`.

    1. On Windows, you might need to add the PostgreSQL `bin` file (e.g.,
	   `C:\Program Files\PostgreSQL\bin\14` to your PATH for Diesel to work.
   
5. Set your DB data in the `.env` file. Use `.env.sample` as a reference.

6. Set the up DB by running migrations: `diesel migration run`.