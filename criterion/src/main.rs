mod csv;
mod sieve;

fn main() {
    sieve::primes();
    csv::csv_writer();
    csv::csv_reader();
}
