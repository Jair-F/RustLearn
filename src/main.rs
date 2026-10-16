mod rust_doc;
mod start;
mod tasks;

fn main() {
    start::test_start();
    tasks::run_tasks();
    rust_doc::run_tasks();
}
