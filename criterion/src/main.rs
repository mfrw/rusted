mod csv;
mod http;
mod sieve;

fn main() {
    sieve::primes();
    csv::csv_writer();
    csv::csv_reader();
    http::get();
}
