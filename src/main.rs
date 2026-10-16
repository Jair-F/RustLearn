mod start;
mod tasks;
mod rust_doc;

fn main() {
    start::test_start();
    tasks::run_tasks();
    rust_doc::run_tasks();
}
