use criterion::{criterion_group, Criterion};
use num_cpus;
use std::sync::{Arc, Barrier, Mutex, RwLock};
use std::thread;

fn benchmark_mutex_single_thread(c: &mut Criterion) {
    let mutex = Arc::new(Mutex::new(0));
    c.bench_function("std_mutex_single_thread", |b| {
        b.iter(|| {
            let m = Arc::clone(&mutex);
            let _guard = m.lock().unwrap();
        })
    });
}

fn benchmark_mutex_multi_thread(c: &mut Criterion) {
    let num_threads = num_cpus::get();
    let mutex = Arc::new(Mutex::new(0));
    let barrier = Arc::new(Barrier::new(num_threads));
    c.bench_function(&format!("std_mutex_multi_thread ({})", num_threads), |b| {
        b.iter(|| {
            let mut handles = vec![];
            for _ in 0..num_threads {
                let m = Arc::clone(&mutex);
                let b = Arc::clone(&barrier);
                handles.push(thread::spawn(move || {
                    b.wait();
                    let _guard = m.lock().unwrap();
                }));
            }
            for handle in handles {
                handle.join().unwrap();
            }
        })
    });
}

fn benchmark_rwlock_single_thread_read(c: &mut Criterion) {
    let rwlock = Arc::new(RwLock::new(0));
    c.bench_function("std_rwlock_single_thread_read", |b| {
        b.iter(|| {
            let r = Arc::clone(&rwlock);
            let _guard = r.read().unwrap();
        })
    });
}

fn benchmark_rwlock_single_thread_write(c: &mut Criterion) {
    let rwlock = Arc::new(RwLock::new(0));
    c.bench_function("std_rwlock_single_thread_write", |b| {
        b.iter(|| {
            let r = Arc::clone(&rwlock);
            let _guard = r.write().unwrap();
        })
    });
}

fn benchmark_rwlock_multi_thread_read(c: &mut Criterion) {
    let num_threads = num_cpus::get();
    let rwlock = Arc::new(RwLock::new(0));
    let barrier = Arc::new(Barrier::new(num_threads));
    c.bench_function(
        &format!("std_rwlock_multi_thread_read ({})", num_threads),
        |b| {
            b.iter(|| {
                let mut handles = vec![];
                for _ in 0..num_threads {
                    let r = Arc::clone(&rwlock);
                    let b = Arc::clone(&barrier);
                    handles.push(thread::spawn(move || {
                        b.wait();
                        let _guard = r.read().unwrap();
                    }));
                }
                for handle in handles {
                    handle.join().unwrap();
                }
            })
        },
    );
}

fn benchmark_rwlock_multi_thread_write(c: &mut Criterion) {
    let num_threads = num_cpus::get();
    let rwlock = Arc::new(RwLock::new(0));
    let barrier = Arc::new(Barrier::new(num_threads));
    c.bench_function(
        &format!("std_rwlock_multi_thread_write ({})", num_threads),
        |b| {
            b.iter(|| {
                let mut handles = vec![];
                for _ in 0..num_threads {
                    let r = Arc::clone(&rwlock);
                    let b = Arc::clone(&barrier);
                    handles.push(thread::spawn(move || {
                        b.wait();
                        let _guard = r.write().unwrap();
                    }));
                }
                for handle in handles {
                    handle.join().unwrap();
                }
            })
        },
    );
}

fn benchmark_mutex_contended(c: &mut Criterion) {
    let num_threads = num_cpus::get();
    let mutex = Arc::new(Mutex::new(0));
    c.bench_function(&format!("std_mutex_contended ({})", num_threads), |b| {
        b.iter(|| {
            let mut handles = vec![];
            for _ in 0..num_threads {
                let m = Arc::clone(&mutex);
                handles.push(thread::spawn(move || {
                    let _guard = m.lock().unwrap();
                }));
            }
            for handle in handles {
                handle.join().unwrap();
            }
        })
    });
}

fn benchmark_rwlock_contended_read(c: &mut Criterion) {
    let num_threads = num_cpus::get();
    let rwlock = Arc::new(RwLock::new(0));
    c.bench_function(
        &format!("std_rwlock_contended_read ({})", num_threads),
        |b| {
            b.iter(|| {
                let mut handles = vec![];
                for _ in 0..num_threads {
                    let r = Arc::clone(&rwlock);
                    handles.push(thread::spawn(move || {
                        let _guard = r.read().unwrap();
                    }));
                }
                for handle in handles {
                    handle.join().unwrap();
                }
            })
        },
    );
}

fn benchmark_rwlock_contended_write(c: &mut Criterion) {
    let num_threads = num_cpus::get();
    let rwlock = Arc::new(RwLock::new(0));
    c.bench_function(
        &format!("std_rwlock_contended_write ({})", num_threads),
        |b| {
            b.iter(|| {
                let mut handles = vec![];
                for _ in 0..num_threads {
                    let r = Arc::clone(&rwlock);
                    handles.push(thread::spawn(move || {
                        let _guard = r.write().unwrap();
                    }));
                }
                for handle in handles {
                    handle.join().unwrap();
                }
            })
        },
    );
}

fn benchmark_barrier(c: &mut Criterion) {
    let num_threads = num_cpus::get();
    let barrier = Arc::new(Barrier::new(num_threads));
    c.bench_function("std_barrier", |b| {
        b.iter(|| {
            let mut handles = vec![];
            for _ in 0..num_threads {
                let b = Arc::clone(&barrier);
                handles.push(thread::spawn(move || {
                    b.wait();
                }));
            }
            for handle in handles {
                handle.join().unwrap();
            }
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(1000);
    targets = benchmark_mutex_single_thread, benchmark_mutex_multi_thread, benchmark_rwlock_single_thread_read, benchmark_rwlock_single_thread_write, benchmark_rwlock_multi_thread_read, benchmark_rwlock_multi_thread_write, benchmark_mutex_contended, benchmark_rwlock_contended_read, benchmark_rwlock_contended_write, benchmark_barrier
}
